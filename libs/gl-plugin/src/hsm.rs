//! Service used to talk to the `hsmd` that is passing us the signer
//! requests.

use crate::config::NodeInfo;
use crate::pb::{hsm_server::Hsm, Empty, HsmRequest, HsmResponse, NodeConfig};
use crate::stager;
use anyhow::{Context, Result};
use futures::TryFutureExt;
use log::{debug, info, trace, warn};
use std::path::PathBuf;
use std::sync::Arc;
use tonic::{Request, Response, Status};

/// The StagingHsmServer is used by the plugin to receive incoming requests
/// from the hsmproxy and stages the requests for clients of the Node
/// interface to stream and reply to.
#[derive(Clone)]
pub struct StagingHsmServer {
    stage: Arc<stager::Stage>,
    hsmd_sock_path: PathBuf,
    node_info: NodeInfo,
    node_config: NodeConfig,
}

impl StagingHsmServer {
    pub fn new(
        hsmd_sock_path: PathBuf,
        stage: Arc<stager::Stage>,
        node_info: NodeInfo,
        node_config: NodeConfig,
    ) -> StagingHsmServer {
        StagingHsmServer {
            stage,
            hsmd_sock_path,
            node_info,
            node_config,
        }
    }

    //G
    /// We have some canned responses from the signer, this gives us access.
    //G_E

    //Find the response for the message in the node_config startup messages
    fn find_canned_response(&self, msg: &Vec<u8>) -> Option<Vec<u8>> {

        //Get the startupmsgs from the node_config and find the one that matches the msg passed in and map
        //it into a response
        self.node_config
            .startupmsgs
            .iter()
            .find(|m| &m.request == msg)
            .map(|m| m.response.clone())
    }
}

//Implement the hsm trait for the stagingHsmServer
#[tonic::async_trait]
impl Hsm for StagingHsmServer {

    //If the request is in the startup msgs or a request type we notice, return a canned response,
    //and send the request to the channel to be processed, received, and returned if not.
    //I think this should receive requests from the SignerProxy.
    async fn request(&self, request: Request<HsmRequest>) -> Result<Response<HsmResponse>, Status> {

        //Takes a request wrapped HsmRequest and extracts the HsmRequest
        let req = request.into_inner();

        //Log that we received a request from the proxy (this is in the plugin where things get queued?)
        debug!("Received request from hsmproxy: {:?}", req);

        //G
        // Start by looking in the canned responses and return it if it is known
        //G_E

        //If we can find the request in the startup messages
        if let Some(response) = self.find_canned_response(&req.raw) {
            //Log that we're returning the responses we have
            debug!(
                "Returning canned response={:?} for request={:?}",
                response, req.raw
            );

            //return the ok response that contains the HsmResponse using the response
            //from the 'find_canned_response' and use an new signer state
            return Ok(Response::new(HsmResponse {
                request_id: req.request_id,
                raw: response,
                signer_state: Vec::new(),
            }));
        } else if req.get_type() == 11 {
            //If we didn't have a canned response and the request type is 11

            //Log that we're returning a 'stashed'(?) init msg (like a default?)
            debug!("Returning stashed init msg: {:?}", self.node_info.initmsg);

            //Return an ok response with the request_id, the default init message,
            // and an empty signer state
            return Ok(Response::new(HsmResponse {
                request_id: req.request_id,
                raw: self.node_info.initmsg.clone(),
                signer_state: Vec::new(), // the signerproxy doesn't care about state
            }));
        } else if req.get_type() == 33 {
            //If the request type is 33

            //Log that we're returning a stashed 'dev-memleak'(?)
            debug!("Returning stashed dev-memleak response");

            //Return an ok response that contains an HsmResponse with a request id, a hardcoded raw,
            //and an empty signer state
            return Ok(Response::new(HsmResponse {
                request_id: req.request_id,
                raw: vec![0, 133, 0],
                signer_state: Vec::new(), // the signerproxy doesn't care about state
            }));
        }

        //If the requst isn't in the startup messages 
        //and we don't have a type we can understand and respond to...

        //match a response to us sending the request to the stage that should
        //respond with a channel(?) or an error
        let mut chan = match self.stage.send(req).await {
            Err(e) => {
                return Err(Status::unknown(format!(
                    "Error while queing request from node: {:?}",
                    e
                )))
            }
            Ok(c) => c,
        };

        //*****I think this period between sending and receiving the message gets to the remote
        //signer, returned, and pushed back into the request channel by the proxy*****

        //If the data in the channel is available, return it as the response
        let res = match chan.recv().await {
            None => {
                return Err(Status::unknown(format!(
                    "Channel closed while waiting for response",
                )))
            }
            Some(r) => r,
        };

        //return the response
        Ok(Response::new(res))
    }

    //Return an empty response
    async fn ping(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        //Log that we got a ping
        trace!("Got a ping");

        //Return an ok empty response
        Ok(Response::new(Empty::default()))
    }
}

//Impelment staging hsm server
impl StagingHsmServer {

    //Create a receive stream using a socket file
    pub async fn run(self) -> Result<()> {

        //Create a path
        let mut path = std::path::PathBuf::new();

        //push the current directory onto the path
        path.push(std::env::current_dir().unwrap());

        //Push the hsmd socket path onto path with current dir
        path.push(&self.hsmd_sock_path);
        info!(
            "Configuring hsmd interface to listen on {}",
            path.to_str().unwrap()
        );

        //Create a directory for the the path's parent (not including file) if it doesn't exist
        std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap())?;

        //if the path with file exists
        if path.exists() {
            //Say that we had a socket path but are deleting it now
            warn!(
                "Socket path {} already exists, deleting",
                path.to_string_lossy()
            );

            //Remove the socket file
            std::fs::remove_file(&path).context("removing stale hsmd socket")?;
        }

        //Create an incoming async stream
        let incoming = {

            //Create a unix listener
            let uds = tokio::net::UnixListener::bind(path)?;

            //Create a stream
            async_stream::stream! {
                //loop forever in the stream
                loop {
                    // and yield on the unix listener until we can shoot back unix streams
            yield  uds.accept().map_ok(|(st, _)| crate::unix::UnixStream(st)).await;
                }
            }
        };

        //Say that the hsm server interface is starting
        info!("HSM server interface starting.");

        //Create the tonic transport server using the hsm server from our protobuf file (pb.rs)
        //The docs says this makes a http2 server that listens for grpc reqs
        tonic::transport::Server::builder()
            //This is allowing the server to listen to things coming through the hsmserver protobuf interface
            //Docs say it's a 'router'
            .add_service(crate::pb::hsm_server::HsmServer::new(self))
            //use the incoming socket stream we made above
            .serve_with_incoming(incoming)
            .await
            .context("serving HsmServer interface")
    }
}
