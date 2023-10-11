// Implementation of the server-side hsmd. It collects requests and passes
// them on to the clients which actually have access to the keys.
use crate::pb::{hsm_client::HsmClient, Empty, HsmRequest, HsmRequestContext};
use crate::wire::{DaemonConnection, Message};
use anyhow::{anyhow, Context};
use anyhow::{Error, Result};
use log::{debug, error, info, warn};
use std::convert::TryFrom;
use std::env;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::str;
use std::sync::atomic;
use std::sync::Arc;
#[cfg(unix)]
use tokio::net::UnixStream as TokioUnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;
use which::which;

type GrpcClient = HsmClient<tonic::transport::Channel>;

//Get the socket path using the environment variable and return the string
fn get_sock_path() -> Result<String> {
    Ok(env::var("HSMD_SOCK_PATH").unwrap_or("hsmd.sock".to_string()))
}

//A NodeConnection object
struct NodeConnection {
    //Create a property for the connection
    conn: DaemonConnection,
    //Create an option wrapping the hsm request context
    context: Option<HsmRequestContext>,
}

//Create a function that returns the version as a string
fn version() -> String {
    //Get the path for the lightning_hsmd program
    let path = which("lightning_hsmd").expect("could not find HSM executable in PATH");

    //Create a shell command using the lightning_hsmd path and pass in the argument 'version'
    let version = Command::new(path)
        .args(&["--version"])
        .output()
        .expect("failed to execute process");

    //Return the version of lightning_hsmd
    str::from_utf8(&version.stdout).unwrap().trim().to_string()
}

//Creates a stream of data from file descriptor 3
fn setup_node_stream() -> Result<DaemonConnection, Error> {
    //Gets a stream using the file descriptor '3'
    //Maybe we can make a constant here to specify what we're using to connect
    
    //In Core lightning it seems the *native* hsmd takes this port
    //TODO: Define magic number
    let ms = unsafe { UnixStream::from_raw_fd(3) };
    //Return the connection  that contains a tokio wrapped unix stream that leads to the 
    //file descriptor '3'
    Ok(DaemonConnection::new(TokioUnixStream::from_std(ms)?))
}

//This function is a wrapper for 'process_requests' that kicks off a one-time(?) async process
//Only called from 'process_requests'
fn start_handler(local: NodeConnection, counter: Arc<atomic::AtomicUsize>, grpc: GrpcClient) {

    //spawn a new process
    tokio::spawn(async {

        //Call process_requests on the local node connection, the counter in the args, and the grpc
        match process_requests(local, counter, grpc)
            .await
            .context("processing requests")
        {
            Ok(()) => panic!("why did the hsmproxy stop processing requests without an error?"),
            Err(e) => warn!("hsmproxy stopped processing requests with error: {}", e),
        }
    });
}

//Either registers a new node connection or uses
//the node connection passed in, sends it to grpc server, and
//forwards it on to existing node connection in argument
async fn process_requests(
    node_conn: NodeConnection,
    request_counter: Arc<atomic::AtomicUsize>,
    //This server should actually be an interface to the hsm.rs
    mut server: GrpcClient,
) -> Result<(), Error> {

    //Get the daemon connection for the given node connection
    let conn = node_conn.conn;

    //Get the option containing the HsmRequestContext
    let context = node_conn.context;

    //Log that we're pinging the server
    info!("Pinging server");

    //Call ping on the server to get an empty 'ok' response
    server.ping(Empty::default()).await?;

    //Endlessly loop
    loop {

        //Try and read a msg from the daemon connection
        if let Ok(msg) = conn.read().await {

            //Match the message types
            match msg.msgtype() {
                //If the message type is a 9 
                //then parse the message into a hsm request context,
                //create a new node connection with fresh uninitialized unix stream (also using hsm request context),
                //start the handler, and write a msg (with the remote stream) on the connection
                9 => {
                    //log that we received a message from a node
                    debug!("Got a message from node: {:?}", &msg.body);

                    //G
                    // This requests a new client fd with a given context,
                    // handle it locally, and defer the creation of the client
                    // fd on the server side until we need it.
                    //G_EMD

                    //Create an hsm request context from the message
                    let ctx = HsmRequestContext::from_client_hsmfd_msg(&msg)?;

                    //log we got a request for a new client
                    debug!("Got a request for a new client fd. Context: {:?}", ctx);

                    //Create a new stream that are connected to each other
                    let (local, remote) = UnixStream::pair()?;

                    //Create a node connection
                    let local = NodeConnection {
                        //Create a new daemon connection with the 'local' unix stream
                        conn: DaemonConnection::new(TokioUnixStream::from_std(local)?),
                        //Defined the context to be the converted message
                        context: Some(ctx),
                    };

                    //Get the file descriptor of the remote unix stream
                    let remote = remote.as_raw_fd();

                    //Create a new message with the file descriptors (the first vec is an empty buff for the body
                    //while the second is the vec of descriptors)
                    let msg = Message::new_with_fds(vec![0, 109], &vec![remote]);

                    //Clone the grpc server
                    let grpc = server.clone();

                    //G
                    // Start new handler for the client
                    //G_END

                    //Call 'start_handler' on the local stream, the request_counter (passed as arg), and the cloned grpc server
                    start_handler(local, request_counter.clone(), grpc);

                    //Try and write the message to the connection and return if there's an error
                    if let Err(e) = conn.write(msg).await {
                        error!("error writing msg to node_connection: {:?}", e);
                        return Err(e);
                    }
                }
                _ => {
                    //G
                    // By default we forward to the remote HSMd
                    //G_END

                    //Create a tonic request if the msg type isn't identified using
                    //the msg and existing node connection in param
                    let req = tonic::Request::new(HsmRequest {
                        context: context.clone(),
                        raw: msg.body.clone(),
                        request_id: request_counter.fetch_add(1, atomic::Ordering::Relaxed) as u32,
                        requests: Vec::new(),
                        signer_state: Vec::new(),
                    });

                    let start_time = tokio::time::Instant::now();

                    //log that we got a message from the node, and log the
                    //constructed/reconstructed tonic request
                    debug!("Got a message from node: {:?}", &req);

                    //Forward the request to the grpc server and save the response
                    let res = server.request(req).await?.into_inner();

                    //Parse the response back into a message
                    let msg = Message::from_raw(res.raw);

                    //Create timer for delta                   
                    let delta = start_time.elapsed();

                    //Say we got a response from hsmd
                    debug!(
                        "Got respone from hsmd: {:?} after {}ms",
                        &msg,
                        delta.as_millis()
                    );

                     //Write the response on the existing node connection
                    conn.write(msg).await?
                }
            }
        } else {
            //Error out if we weren't able to get a message from the 
            //node connection
            error!("Connection lost");

            //return an error
            return Err(anyhow!("Connection lost"));
        }
    }
}

//RANDY
use std::path::PathBuf;

//returns an hsm client that uses a channel made from using the socket path
//from the 'get_sock_path' method
async fn grpc_connect() -> Result<GrpcClient, Error> {

    //G
    // We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    // Connect to a Uds socket
    //G_END

    //Create a channel using the endpoint with no domain and port 50051
    //typically this port is reserved for grpc client connections
    let channel = Endpoint::try_from("http://[::]:50051")?

        //connect the tonic endpoint with this connector lmbda that accepts a uri and returns a stream
        .connect_with_connector(service_fn(|_: Uri| {
            //Get the socket path from the env var
            let sock_path = get_sock_path().unwrap();

            //Create an empty path
            let mut path = PathBuf::new();
            
            //If the socket path is relative
            if !sock_path.starts_with('/') {
                //Push the current directory to the path before adding the socket path
                path.push(env::current_dir().unwrap());
            }

            //push the socket path to the path
            path.push(&sock_path);

            //Get the path as a string
            let path = path.to_str().unwrap().to_string();

            //Say where the hsmserver socket is
            info!("Connecting to hsmserver at {}", path);

            //Return a unix stream to the socket
            TokioUnixStream::connect(path)
        }))
        .await
        //log that we couldnt connect if it fails
        .context("could not connect to the socket file")?;

    //return an hsm client that uses the channel
    Ok(HsmClient::new(channel))
}

//Calls process requests using the node connection, grpc connection, and request counter
pub async fn run() -> Result<(), Error> {
    //Get arguments from the environment
    let args: Vec<String> = std::env::args().collect();

    //G
    // Start the counter at 1000 so we can inject some message before
    // real requests if we want to.
    //G_END
    
    //Create a counter using an atomic usize
    let request_counter = Arc::new(atomic::AtomicUsize::new(1000));

    //If the argument length is 2 and the second argument is version (first arg is the program name)
    if args.len() == 2 && args[1] == "--version" {
        //Then log the version
        println!("{}", version());
        //And return
        return Ok(());
    }

    //Log that we're starting the hsmproxy
    info!("Starting hsmproxy");

    //Run setup node stream which returns a connection from file descriptor '3'
    let node = setup_node_stream()?;

    //Returns a grpc connection represented as an 'HsmClient' containing a channel
    let grpc = grpc_connect().await?;

    //Calls process requests using the node connection, grpc connection, and request counter
    process_requests(
        NodeConnection {
            conn: node,
            context: None,
        },
        request_counter,
        grpc,
    )
    .await
}
