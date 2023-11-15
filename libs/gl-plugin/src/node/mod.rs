use crate::config::Config;
use crate::pb::{self, node_server::Node};
use crate::rpc::LightningClient;
use crate::stager;
use crate::storage::StateStore;
use crate::{messages, Event};
use anyhow::{Context, Error, Result};
use base64::{engine::general_purpose, Engine as _};
use bytes::BufMut;
use gl_client::persist::State;
use governor::{
    clock::MonotonicClock, state::direct::NotKeyed, state::InMemoryState, Quota, RateLimiter,
};
use lazy_static::lazy_static;
use log::{debug, error, info, trace, warn};
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{broadcast, mpsc, Mutex, OnceCell};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::ServerTlsConfig, Code, Request, Response, Status};
mod wrapper;
pub use wrapper::WrappedNodeServer;
use gl_client::bitcoin;
use std::str::FromStr;


static LIMITER: OnceCell<RateLimiter<NotKeyed, InMemoryState, MonotonicClock>> =
    OnceCell::const_new();

lazy_static! {
    static ref HSM_ID_COUNT: AtomicUsize = AtomicUsize::new(0);

    /// The number of signers that are currently connected (best guess
    /// due to races). Allows us to determine whether we should
    /// initiate operations that might require signatures.
    static ref SIGNER_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ref RPC_BCAST: broadcast::Sender<super::Event> = broadcast::channel(4).0;

    static ref SERIALIZED_CONFIGURE_REQUEST: Mutex<Option<String>> = Mutex::new(None);

    static ref RPC_READY: AtomicBool = AtomicBool::new(false);
}

/// The PluginNodeServer is the interface that is exposed to client devices
/// and is in charge of coordinating the various user-controlled
/// entities. This includes dispatching incoming RPC calls to the JSON-RPC
/// interface, as well as staging requests from the HSM so that they can be
/// streamed and replied to by devices that have access to the signing keys.
#[derive(Clone)]
pub struct PluginNodeServer {
    pub tls: ServerTlsConfig,
    pub stage: Arc<stager::Stage>,
    pub rpc: Arc<Mutex<LightningClient>>,
    rpc_path: PathBuf,
    events: tokio::sync::broadcast::Sender<super::Event>,
    signer_state: Arc<Mutex<State>>,
    grpc_binding: String,
    signer_state_store: Arc<Mutex<Box<dyn StateStore>>>,
    pub ctx: crate::context::Context,
}

impl PluginNodeServer {
    pub async fn new(
        stage: Arc<stager::Stage>,
        config: Config,
        events: tokio::sync::broadcast::Sender<super::Event>,
        signer_state_store: Box<dyn StateStore>,
    ) -> Result<Self, Error> {
        let tls = ServerTlsConfig::new()
            .identity(config.identity.id)
            .client_ca_root(config.identity.ca);

        let mut rpc_path = std::env::current_dir().unwrap();
        rpc_path.push("lightning-rpc");
        info!("Connecting to lightning-rpc at {:?}", rpc_path);

        let rpc = Arc::new(Mutex::new(LightningClient::new(rpc_path.clone())));

        // Bridge the RPC_BCAST into the events queue
        let tx = events.clone();
        tokio::spawn(async move {
            let mut rx = RPC_BCAST.subscribe();
            loop {
                if let Ok(e) = rx.recv().await {
                    let _ = tx.send(e);
                }
            }
        });

        let signer_state = signer_state_store.read().await?;

        let ctx = crate::context::Context::new();

        let rrpc = rpc.clone();

        let s = PluginNodeServer {
            ctx,
            tls,
            rpc,
            stage,
            events,
            rpc_path,
            signer_state: Arc::new(Mutex::new(signer_state)),
            signer_state_store: Arc::new(Mutex::new(signer_state_store)),
            grpc_binding: config.node_grpc_binding,
        };

        tokio::spawn(async move {
            debug!("Locking grpc interface until the JSON-RPC interface becomes available.");
            use tokio::time::{sleep, Duration};

            // Move the lock into the closure so we can release it later.
            let rpc = rrpc.lock().await;
            loop {
                let res: Result<crate::responses::GetInfo, crate::rpc::Error> =
                    rpc.call("getinfo", json!({})).await;
                match res {
                    Ok(_) => break,
                    Err(e) => {
                        warn!(
                            "JSON-RPC interface not yet available. Delaying 50ms. {:?}",
                            e
                        );
                        sleep(Duration::from_millis(50)).await;
                    }
                }
            }

	    // Signal that the RPC is ready now.
	    RPC_READY.store(true, Ordering::SeqCst);

            let list_datastore_req = cln_rpc::model::requests::ListdatastoreRequest{
                key: Some(vec![
                    "glconf".to_string(),
                    "request".to_string()
                ])
            };

            let res: Result<cln_rpc::model::responses::ListdatastoreResponse, crate::rpc::Error> =
                rpc.call("listdatastore", list_datastore_req).await;

            match res {
                Ok(list_datastore_res) => {
                    if list_datastore_res.datastore.len() > 0 {
                        let serialized_configure_request = list_datastore_res.datastore[0].string.clone();
                        match serialized_configure_request {
                            Some(serialized_configure_request) => {
                                let mut cached_serialized_configure_request = SERIALIZED_CONFIGURE_REQUEST.lock().await;
                                *cached_serialized_configure_request = Some(serialized_configure_request);
                            }
                            None => {}
                        }
                    }
                }
                Err(_) => {}
            }
            
            drop(rpc);
        });

        Ok(s)
    }

    // Wait for the limiter to allow a new RPC call
    pub async fn limit(&self) {
        let limiter = LIMITER
            .get_or_init(|| async {
                let quota = Quota::per_minute(core::num::NonZeroU32::new(300).unwrap());
                RateLimiter::direct_with_clock(quota, &MonotonicClock::default())
            })
            .await;

        limiter.until_ready().await
    }

    pub async fn get_rpc(&self) -> LightningClient {
        let rpc = self.rpc.lock().await;
        let r = rpc.clone();
        drop(rpc);
        r
    }
}

#[tonic::async_trait]
impl Node for PluginNodeServer {
    type StreamCustommsgStream = ReceiverStream<Result<pb::Custommsg, Status>>;
    type StreamHsmRequestsStream = ReceiverStream<Result<pb::HsmRequest, Status>>;
    type StreamLogStream = ReceiverStream<Result<pb::LogEntry, Status>>;

    async fn stream_custommsg(
        &self,
        _: Request<pb::StreamCustommsgRequest>,
    ) -> Result<Response<Self::StreamCustommsgStream>, Status> {
        log::debug!("Added a new listener for custommsg");
        let (tx, rx) = mpsc::channel(1);
        let mut stream = self.events.subscribe();
        // TODO: We can do better by returning the broadcast receiver
        // directly. Well really we should be filtering the events by
        // type, so maybe a `.map()` on the stream can work?
        tokio::spawn(async move {
            while let Ok(msg) = stream.recv().await {
                if let Event::CustomMsg(m) = msg {
                    log::trace!("Forwarding custommsg {:?} to listener", m);
                    if let Err(e) = tx.send(Ok(m)).await {
                        log::warn!("Unable to send custmmsg to listener: {:?}", e);
                        break;
                    }
                }
            }
            panic!("stream.recv loop exited...");
        });
        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn stream_log(
        &self,
        _: Request<pb::StreamLogRequest>,
    ) -> Result<Response<Self::StreamLogStream>, Status> {
        match async {
            let (tx, rx) = mpsc::channel(1);
            let mut lines = linemux::MuxedLines::new()?;
            lines.add_file("/tmp/log").await?;

            // TODO: Yes, this may produce duplicate lines, when new
            // log entries are produced while we're streaming the
            // backlog out, but do we care?
            use tokio::io::{AsyncBufReadExt, BufReader};
            let file = tokio::fs::File::open("/tmp/log").await?;
            let mut file = BufReader::new(file).lines();

            tokio::spawn(async move {
                match async {
                    while let Some(line) = file.next_line().await? {
                        tx.send(Ok(pb::LogEntry {
                            line: line.trim().to_owned(),
                        }))
                        .await?
                    }

                    while let Ok(Some(line)) = lines.next_line().await {
                        tx.send(Ok(pb::LogEntry {
                            line: line.line().trim().to_string(),
                        }))
                        .await?;
                    }
                    Ok(())
                }
                .await as Result<(), anyhow::Error>
                {
                    Ok(()) => {}
                    Err(e) => {
                        warn!("error streaming logs to client: {}", e);
                    }
                }
            });
            Ok(ReceiverStream::new(rx))
        }
        .await as Result<Self::StreamLogStream, anyhow::Error>
        {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    //RANDY_COMMENTED
    //Create a stream, read a request from the stage, add the signer state and context info to it,
    //and forward the request using the outgoing stream. Then do some connecting of the peers using
    //datastore and node info, and return the receive of the stream.

    //So this function attaches one end of a stream to the stage from the nde, and returns the other end
    async fn stream_hsm_requests(
        &self,
        _request: Request<pb::Empty>,
    ) -> Result<Response<Self::StreamHsmRequestsStream>, Status> {

        //Get the hsm id count 'AtomicUsize' and add '1' to the value 
        //(prev val is returned and overflows are taken care of in 'fetch_add')
        let hsm_id = HSM_ID_COUNT.fetch_add(1, Ordering::SeqCst);

        //Add 1 to signer count?
        //REVISIT
        SIGNER_COUNT.fetch_add(1, Ordering::SeqCst);

        //Log that we have a new sigher with hsm id 'hsm_id'
        info!(
            "New signer with hsm_id={} attached, streaming requests",
            hsm_id
        );

        //Create a 10 byte(?) buffer in a channel
        let (tx, rx) = mpsc::channel(10);

        //Create a stream using the stage
        //We send things into the stream from 'libs/gl-plugin/src/hsm.rs::request'
        let mut stream = self.stage.mystream().await;

        //Get the signer state
        let signer_state = self.signer_state.clone();

        //Clone the context
        let ctx = self.ctx.clone();

        //Spawn an async func
        tokio::spawn(async move {

            //Show the hsm id
            trace!("hsmd hsm_id={} request processor started", hsm_id);

            //CHANGE: Added a signer heartbeat

            {
                //G
                // We start by immediately injecting a
                // vls_protocol::Message::GetHeartbeat. This serves two
                // purposes: already send the initial snapshot of the
                // signer state to the signer as early as possible, and
                // triggering a pruning on the signer, if enabled. In
                // incremental mode this ensures that any subsequent,
                // presumably time-critical messages, do not have to carry
                // the large state with them.
                //G_END

                //Lock the signer state and clone it
                let state = signer_state.lock().await.clone();

                //Turn the state into a signer state entry
                let state: Vec<gl_client::pb::SignerStateEntry> = state.into();

                //Get a signer state entry vec by converting to internal signer state
                let state: Vec<pb::SignerStateEntry> = state
                    .into_iter()
                    .map(|s| pb::SignerStateEntry {
                        key: s.key,
                        version: s.version,
                        value: s.value,
                    })
                    .collect();

                //init a heartbeat msg
                let msg = vls_protocol::msgs::GetHeartbeat {};

                //use vls ser
                use vls_protocol::msgs::SerBolt;

                //create hsm req using the signer state and heartbeat msg
                let req = crate::pb::HsmRequest {
                    //B
                    // Notice that the request_counter starts at 1000, to
                    // avoid collisions.
                    //B_END
                    request_id: 0,
                    signer_state: state,
                    raw: msg.as_vec(),
                    requests: vec![], // No pending requests yet, nothing to authorize.
                    context: None,
                };

                //Send this req using same tx we prev only for requests
                if let Err(e) = tx.send(Ok(req)).await {
                    log::warn!("Failed to send heartbeat message to signer: {}", e);
                }
            }

            //Loop forever
            loop {

                //Wait for the next thing on the stage stream
                let mut req = match stream.next().await {
                    //If there's an error then say we couldn't get the request for the stage
                    Err(e) => {
                        error!(
                            "Could not get next request from stage: {:?} for hsm_id={}",
                            e, hsm_id
                        );
                        break;
                    }

                    //Return the request from the stream otherwise
                    Ok(r) => r,
                };

                //Log what request we're sending
                trace!(
                    "Sending request={} to hsm_id={}",
                    req.request.request_id,
                    hsm_id
                );

                //clone the signer state
                let state = signer_state.lock().await.clone();

                //Change the signer state into 
                let state: Vec<gl_client::pb::SignerStateEntry> = state.into();

                //G
                // TODO Consolidate protos in `gl-client` and `gl-plugin`, then remove this map.
                //G_END

                //Turn the state into an iterator and convert each state iinto a signer state entry
                let state: Vec<pb::SignerStateEntry> = state
                    .into_iter()
                    .map(|s| pb::SignerStateEntry {
                        key: s.key,
                        version: s.version,
                        value: s.value,
                    })
                    .collect();

                //Turn the vec of signer state entries into the req's request's signer_state
                req.request.signer_state = state.into();

                //get the context snapshot and put it into requests
                req.request.requests = ctx.snapshot().await.into_iter().map(|r| r.into()).collect();

                //REVISIT
                //REVISIT_RANDY
                let serialized_configure_request = SERIALIZED_CONFIGURE_REQUEST.lock().await;

                match &(*serialized_configure_request) {
                    Some(serialized_configure_request) => {
                        let configure_request = serde_json::from_str::<crate::context::Request>(
                            serialized_configure_request,
                        )
                        .unwrap();
                        req.request.requests.push(configure_request.into());
                    }
                    None => {}
                }

                //Log the number of signer requests and state entries
                debug!(
                    "Sending signer requests with {} requests and {} state entries",
                    req.request.requests.len(),
                    req.request.signer_state.len()
                );

                //If there's an error while sending the request to the channel made earlier
                if let Err(e) = tx.send(Ok(req.request)).await {
                    //error out and break
                    warn!("Error streaming request {:?} to hsm_id={}", e, hsm_id);
                    break;
                }
            }

            //otherwise just log that the hsm send ended
            info!("Signer hsm_id={} exited", hsm_id);
            SIGNER_COUNT.fetch_sub(1, Ordering::SeqCst);
        });

        //log that we're returning the stream hsm request channel
        trace!("Returning stream_hsm_request channel");
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    //RANDY_COMMENTED
    //Receives a request, extracts its signer state, merges it with the
    //plugin's local signer state, and saves it to the signer state store
    async fn respond_hsm_request(
        &self,
        request: Request<pb::HsmResponse>,
    ) -> Result<Response<pb::Empty>, Status> {

        //Change the request into it's inner 'hsmresponse' type
        let req = request.into_inner();

        //G
        // Create a state from the key-value-version tuples. Need to
        // convert here, since `pb` is duplicated in the two different
        // crates.
        //G_END

        //Get the signer_state from the request sent to us by collecting
        //each 'SignerStateEntry' in the signer_state property
        let signer_state: Vec<gl_client::pb::SignerStateEntry> = req
            .signer_state
            .iter()
            .map(|i| gl_client::pb::SignerStateEntry {
                key: i.key.to_owned(),
                value: i.value.to_owned(),
                version: i.version,
            })
            .collect();

        //Turn the signer state into a 'persist' State
        let new_state: gl_client::persist::State = signer_state.into();

        {
            //G
            // Apply state changes to the in-memory state
            //G_E

            //Mutex lock the signer state of this plugin
            let mut state = self.signer_state.lock().await;

            //Merge the signer state of plugin with signer state from
            //processed request
            state.merge(&new_state).map_err(|e| {
                //map an error if one exists
                Status::new(
                    Code::Internal,
                    format!("Error updating internal state: {e}"),
                )
            })?;

            //G
            // Send changes to the signer_state_store for persistence
            //G_END

            //Write the new cloned, plugin-local signer state to 
            //the signer_state_stire
            self.signer_state_store
                .lock()
                .await
                .write(state.clone())
                .await
                .map_err(|e| {
                    Status::new(
                        Code::Internal,
                        format!("error persisting state changes: {}", e),
                    )
                })?;
        }

        //Send the processed request to the stage
        if let Err(e) = self.stage.respond(req).await {
            warn!("Suppressing error: {:?}", e);
        }

        //Return an empty response
        Ok(Response::new(pb::Empty::default()))
    }

    type StreamIncomingStream = ReceiverStream<Result<pb::IncomingPayment, Status>>;

    async fn stream_incoming(
        &self,
        _req: tonic::Request<pb::StreamIncomingFilter>,
    ) -> Result<Response<Self::StreamIncomingStream>, Status> {
        // TODO See if we can just return the broadcast::Receiver
        // instead of pulling off broadcast and into an mpsc.
        let (tx, rx) = mpsc::channel(1);
        let mut bcast = self.events.subscribe();
        tokio::spawn(async move {
            while let Ok(p) = bcast.recv().await {
                match p {
                    super::Event::IncomingPayment(p) => {
                        let _ = tx.send(Ok(p)).await;
                    }
                    _ => {}
                }
            }
        });

        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn configure(&self, req: tonic::Request<pb::GlConfig>) -> Result<Response<pb::Empty>, Status>  {
        self.limit().await;
        let gl_config = req.into_inner();
        let rpc = self.get_rpc().await;

        let res: Result<crate::responses::GetInfo, crate::rpc::Error> =
            rpc.call("getinfo", json!({})).await;

        let network = match res {
            Ok(get_info_response) => match get_info_response.network.parse() {
                Ok(v) => v,
                Err(_) => Err(Status::new(
                    Code::Unknown,
                    format!("Failed to parse 'network' from 'getinfo' response"),
                ))?,
            },
            Err(e) => {
                return Err(Status::new(
                        Code::Unknown,
                        format!("Failed to retrieve a response from 'getinfo' while setting the node's configuration: {}", e),
                    ));
            }
        };
    
        match bitcoin::Address::from_str(&gl_config.close_to_addr) {
            Ok(address) => {
                if address.network != network {
                    return Err(Status::new(
                        Code::Unknown,
                        format!(
                            "Network mismatch: \
                            Expected an address for {} but received an address for {}",
                            network,
                            address.network
                        ),
                    ));
                }
            }
            Err(e) => {
                return Err(Status::new(
                    Code::Unknown,
                    format!("The address {} is not valid: {}", gl_config.close_to_addr, e),
                ));
            }
        }

        let requests: Vec<crate::context::Request> = self.ctx.snapshot().await.into_iter().map(|r| r.into()).collect();
        let serialized_req = serde_json::to_string(&requests[0]).unwrap();
        let datastore_res: Result<crate::cln_rpc::model::responses::DatastoreResponse, crate::rpc::Error> =
            rpc.call("datastore", json!({
                "key": vec![
                    "glconf".to_string(),
                    "request".to_string(),
                ],
                "string": serialized_req,
            })).await;
        
        match datastore_res {
            Ok(_) => {
                let mut cached_gl_config = SERIALIZED_CONFIGURE_REQUEST.lock().await;
                *cached_gl_config = Some(serialized_req);

                Ok(Response::new(pb::Empty::default()))
            }
            Err(e) => {
                return Err(Status::new(
                    Code::Unknown,
                    format!("Failed to store the raw configure request in the datastore: {}", e),
                ))
            }
        }
    }
}

use cln_grpc::pb::node_server::NodeServer;

impl PluginNodeServer {
    pub async fn run(self) -> Result<()> {
        let addr = self.grpc_binding.parse().unwrap();

        let cln_node = NodeServer::new(
            WrappedNodeServer::new(self.clone())
                .await
                .context("creating NodeServer instance")?,
        );

        let router = tonic::transport::Server::builder()
            .max_frame_size(4 * 1024 * 1024) // 4MB max request size
            .tcp_keepalive(Some(tokio::time::Duration::from_secs(1)))
            .tls_config(self.tls.clone())?
            .layer(SignatureContextLayer {
                ctx: self.ctx.clone(),
            })
            .add_service(RpcWaitService::new(cln_node, self.rpc_path.clone()))
            .add_service(crate::pb::node_server::NodeServer::new(self.clone()));

        router
            .serve(addr)
            .await
            .context("grpc interface exited with error")
    }

    /// Reconnect all peers with whom we have a channel or previously
    /// connected explicitly to.
    pub async fn reconnect_peers(&self) -> Result<(), Error> {
        if SIGNER_COUNT.load(Ordering::SeqCst) < 1 {
            use anyhow::anyhow;
            return Err(anyhow!(
                "Cannot reconnect peers, no signer to complete the handshake"
            ));
        }

        log::info!("Reconnecting all peers");
        let mut rpc = cln_rpc::ClnRpc::new(self.rpc_path.clone()).await?;
        let peers = self.get_reconnect_peers().await?;

        //for every request in the peer requests
        for r in peers {
            //log that we're gonna call connect on the peer
            trace!("Calling connect: {:?}", &r.id);

            //call connect on the cln node
            let res = rpc.call_typed(r.clone()).await;

            //log what the response to the connect call did
            trace!("Connect returned: {:?} -> {:?}", &r.id, res);

            //match the response and log whether it was successful or not
            match res {
                Ok(r) => info!("Connection to {} established: {:?}", &r.id, r),
                Err(e) => warn!("Could not connect to {}: {:?}", &r.id, e),
            }
        }
        return Ok(());
    }

    //RANDY_COMMENTED
    async fn get_reconnect_peers(
        &self,
    ) -> Result<Vec<cln_rpc::model::requests::ConnectRequest>, Error> {

        //G
        // Now that we have an hsmd we can actually reconnect to our peers
        //G_END

        //Get rpc path
        let rpc_path = self.rpc_path.clone();
        
        //Match the cln rpc made with the rpc path
        let mut rpc = cln_rpc::ClnRpc::new(rpc_path).await?;
        
        //Use the rpc conn to call list peers by inspecting the request type (ListpeersRequest here)
        let peers = rpc
            .call_typed(cln_rpc::model::requests::ListpeersRequest {
                id: None,
                level: None,
            })
            .await?;

        //Unwrap the response's peers and filteronlt the connected ones and convert
        //the peer ids into connect requets.
        let mut requests: Vec<cln_rpc::model::requests::ConnectRequest> = peers
            .peers
            .iter()
            .filter(|&p| p.connected)
            .map(|p| cln_rpc::model::requests::ConnectRequest {
                id: p.id.to_string(),
                host: None,
                port: None,
            })
            .collect();

        let mut dspeers: Vec<cln_rpc::model::requests::ConnectRequest> = rpc
            //Call list data store
            .call_typed(cln_rpc::model::requests::ListdatastoreRequest {
                key: Some(vec!["greenlight".to_string(), "peerlist".to_string()]),
            })
            .await?
             //unwrap the data store requests and map them into peers if possible,
            //and then more connect requests?
            .datastore
            .iter()
            .map(|x| {
                //G
                // We need to replace unnecessary escape characters that
                // have been added by the datastore, as serde is a bit
                // picky on that.
                //G_END
                let mut s = x.string.clone().unwrap();
                s = s.replace('\\', "");
                serde_json::from_str::<messages::Peer>(&s).unwrap()
            })
            .map(|x| cln_rpc::model::requests::ConnectRequest {
                id: x.id,
                host: Some(x.addr),
                port: None,
            })
            .collect();

         //G
        // Merge the two peer lists;
        //G_END

        //append the datastore peers onto the peers we got from the cln node
        requests.append(&mut dspeers);

        //sort the peers
        requests.sort_by(|a, b| a.id.cmp(&b.id));

        //deduplicate any peers
        requests.dedup_by(|a, b| a.id.eq(&b.id));

        Ok(requests)
    }
}

use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct SignatureContextLayer {
    ctx: crate::context::Context,
}

impl SignatureContextLayer {
    pub fn new(context: crate::context::Context) -> Self {
        SignatureContextLayer { ctx: context }
    }
}

//RANDY_COMMENTED
//Implement the layer trait for the signature context layer
impl<S> Layer<S> for SignatureContextLayer {
    //Type the signature context service as 'Service'
    type Service = SignatureContextService<S>;

    //Create a method called layer that takes in a service and wraps that service in a SignatureContextService
    fn layer(&self, service: S) -> Self::Service {
        //Returns a SignatureContextService with an inner service and a context
        SignatureContextService {
            inner: service,
            ctx: self.ctx.clone(),
        }
    }
}

// Is the maximum message size we allow to buffer up on requests.
const MAX_MESSAGE_SIZE: usize = 4000000;

#[derive(Debug, Clone)]
pub struct SignatureContextService<S> {
    inner: S,
    ctx: crate::context::Context,
}

//RANDY_COMMENTED
//The SignatureContextService used to protect the NodeServer in plugin.rs
impl<S> Service<hyper::Request<hyper::Body>> for SignatureContextService<S>
where
    S: Service<hyper::Request<hyper::Body>, Response = hyper::Response<tonic::body::BoxBody>>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Response = S::Response;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    //RANDY_COMMENTED
    //Method called when pluginNodeServer is called
    //CHECKPOINT
    fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let reqctx = self.ctx.clone();

        Box::pin(async move {
            use tonic::codegen::Body;
            let (parts, mut body) = request.into_parts();

            let uri = parts.uri.path_and_query().unwrap();
            let _ = RPC_BCAST
                .clone()
                .send(super::Event::RpcCall(uri.to_string()));

            let pubkey = parts
                .headers
                .get("glauthpubkey")
                .map(|k| general_purpose::STANDARD_NO_PAD.decode(k).ok())
                .flatten();

            let sig = parts
                .headers
                .get("glauthsig")
                .map(|s| general_purpose::STANDARD_NO_PAD.decode(s).ok())
                .flatten();

            use bytes::Buf;
            let timestamp: Option<u64> = parts
                .headers
                .get("glts")
                .map(|s| general_purpose::STANDARD_NO_PAD.decode(s).ok())
                .flatten()
                .map(|s| s.as_slice().get_u64());

            if let (Some(pk), Some(sig)) = (pubkey, sig) {
                // Now that we know we'll be adding this to the
                // context we can start buffering the request.
                let mut buf = Vec::new();
                while let Some(chunk) = body.data().await {
                    let chunk = chunk.unwrap();
                    // We check on the MAX_MESSAGE_SIZE to avoid an unlimited sized
                    // message buffer
                    if buf.len() + chunk.len() > MAX_MESSAGE_SIZE {
                        debug!("Message {} exceeds size limit", uri.path().to_string());
                        return Ok(tonic::Status::new(
                            tonic::Code::InvalidArgument,
                            format!("payload too large"),
                        )
                        .to_http());
                    }
                    buf.put(chunk);
                }

                trace!(
                    "Got a request for {} with pubkey={}, sig={} and body size={:?}",
                    uri,
                    hex::encode(&pk),
                    hex::encode(&sig),
                    &buf.len(),
                );
                let req = crate::context::Request::new(
                    uri.to_string(),
                    <bytes::Bytes>::from(buf.clone()),
                    pk,
                    sig,
                    timestamp,
                );

                reqctx.add_request(req.clone()).await;

                let body: hyper::Body = buf.into();
                let request = hyper::Request::from_parts(parts, body);
                let res = inner.call(request).await;

                //G
                // Defer cleanup into a separate task, otherwise we'd
                // need `res` to be `Send` which we can't
                // guarantee. This is needed since adding an await
                // point splits the state machine at that point.
                //G_END

                //R
                //After we process the request, spawn a new task to clean the request from the context
                tokio::spawn(async move {
                    reqctx.remove_request(req).await;
                });
                res.map_err(Into::into)
            } else {
                //G
                // No point in buffering the request, we're not going
                // to add it to the `HsmRequestContext`
                //G_END
                let request = hyper::Request::from_parts(parts, body);
                inner.call(request).await.map_err(Into::into)
            }
        })
    }
}

mod rpcwait;
pub use rpcwait::RpcWaitService;
