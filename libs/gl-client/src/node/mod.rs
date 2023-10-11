use crate::pb::cln::node_client as cln_client;
use crate::pb::node_client::NodeClient;
use crate::pb::scheduler::{scheduler_client::SchedulerClient, ScheduleRequest};
use crate::tls::TlsConfig;
use crate::utils;
use anyhow::{anyhow, Result};
use lightning_signer::bitcoin::Network;
use log::{debug, info, trace};
use tonic::transport::{Channel, Uri};
use tower::ServiceBuilder;

/// A client to the remotely running node on the greenlight
/// infrastructure. It is configured to authenticate itself with the
/// device mTLS keypair and will sign outgoing requests with the same
/// mTLS keypair.
pub type Client = NodeClient<service::AuthService>;

pub type GClient = GenericClient<service::AuthService>;

pub type ClnClient = cln_client::NodeClient<service::AuthService>;

pub trait GrpcClient {
    fn new_with_inner(inner: service::AuthService) -> Self;
}

/// A builder to configure a [`Client`] that can either connect to a
/// node directly, assuming you have the `grpc_uri` that the node is
/// listening on, or it can talk to the
/// [`crate::scheduler::Scheduler`] to schedule the node and configure
/// the [`Client`] accordingly.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Node {
    node_id: Vec<u8>,
    network: Network,
    tls: TlsConfig,
}

impl GrpcClient for Client {
    fn new_with_inner(inner: service::AuthService) -> Self {
        Client::new(inner)
    }
}

impl GrpcClient for GClient {
    fn new_with_inner(inner: service::AuthService) -> Self {
        GenericClient::new(inner)
    }
}

impl GrpcClient for ClnClient {
    fn new_with_inner(inner: service::AuthService) -> Self {
        ClnClient::new(inner)
    }
}

impl Node {

    //RANDY_COMMENTED
    //Create a new Node
    pub fn new(node_id: Vec<u8>, network: Network, tls: TlsConfig) -> Node {
        //Return a Node initialized with a node_id, a network, and a tls config
        Node {
            node_id,
            network,
            tls,
        }
    }

    //RANDY_COMMENTED
    //Initializes a tls cert depending on the node_uri and uses that tls service
    //when establishing a grpc connection to the node
    pub async fn connect<C>(&self, node_uri: String) -> Result<C>
    where
        C: GrpcClient,
    {
        //Create a Uri object from the node_uri string
        let node_uri = Uri::from_maybe_shared(node_uri)?;

        //Log we're connecting to the node
        info!("Connecting to node at {}", node_uri);

        //G
        // If this is not yet a node-domain address we need to also
        // accept "localhost" as domain name from the certificate.
        //G_END

        //Get the host from the node_uri
        let host = node_uri.host().unwrap();

        //If the host starts with 'gl'
        let tls = if host.starts_with("gl") {
            //log the host we're usin
            trace!(
                "Using real hostname {}, expecting the node to have a matching certificate",
                host
            );

            //clone the tls config on the node
            self.tls.clone()
        } else {

            //Log that we're 'overriding' a hostname since the host doesn't start with 'gl'
            trace!(
                "Overriding hostname, since this is not a gl node domain: {}",
                host
            );

            //Get the tls config
            let mut tls = self.tls.clone();

            //Set the tls config's inner to the domain name 'localhost' but does so
            //non-mutably so returns a new tls inner.
            tls.inner = tls.inner.domain_name("localhost");

            //Return the tls config
            tls
        };

        //Match the tls private key and make an authlayer if present, error if the private key isn't
        let layer = match tls.private_key {
            Some(k) => service::AuthLayer::new(k)?,
            None => {
                return Err(anyhow!(
                    "Cannot connect a node::Client without first configuring its identity"
                ))
            }
        };

        //Crete a tonic/grpc channel using the node_uri
        let chan = tonic::transport::Endpoint::from_shared(node_uri.to_string())?
            //Set the tls config to tls inner
            .tls_config(tls.inner)?
            .tcp_keepalive(Some(crate::TCP_KEEPALIVE))
            .http2_keep_alive_interval(crate::TCP_KEEPALIVE)
            .keep_alive_timeout(crate::TCP_KEEPALIVE_TIMEOUT)
            .keep_alive_while_idle(true)
            //connect to the node
            .connect_lazy();

         //Use a service builder to add an auth layer and the channel as the servuce
        //This service is a 'tower' service which is an abstraction/interface that 
        //networking clients and servers can use to implement generically created middlewares
        let chan = ServiceBuilder::new().layer(layer).service(chan);

        //Return a new grpclient that uses the 'chan' that was just built
        Ok(C::new_with_inner(chan))
    }

    pub async fn schedule_with_uri<C>(self, scheduler_uri: String) -> Result<C>
    where
        C: GrpcClient,
    {
        debug!(
            "Contacting scheduler at {} to get the node address",
            scheduler_uri
        );

        let channel = Channel::from_shared(scheduler_uri)?
            .tls_config(self.tls.inner.clone())?
            .connect()
            .await?;
        let mut scheduler = SchedulerClient::new(channel);

        let node_info = scheduler
            .schedule(ScheduleRequest {
                node_id: self.node_id.clone(),
            })
            .await
            .map(|v| v.into_inner())?;

        debug!("Node scheduled at {}", node_info.grpc_uri);

        self.connect(node_info.grpc_uri).await
    }

    pub async fn schedule<C>(self) -> Result<C>
    where
        C: GrpcClient,
    {
        let uri = utils::scheduler_uri();
        self.schedule_with_uri(uri).await
    }
}

mod generic;
mod service;
pub use generic::GenericClient;

mod stasher {
    use bytes::Bytes;
    use http::HeaderMap;
    use http_body::Body;
    use pin_project::pin_project;
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };
    use tonic::body::BoxBody;
    use tonic::Status;

    #[pin_project]
    #[derive(Debug)]
    pub(crate) struct StashBody {
        value: Option<Bytes>,
    }

    impl StashBody {
        pub(crate) fn new(val: Bytes) -> Self {
            Self { value: Some(val) }
        }
    }

    impl Body for StashBody {
        type Data = Bytes;
        type Error = Status;

        fn is_end_stream(&self) -> bool {
            self.value.is_none()
        }

        fn poll_data(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
            Poll::Ready(self.project().value.take().map(Ok))
        }

        fn poll_trailers(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<Option<HeaderMap>, Status>> {
            Poll::Ready(Ok(None))
        }
    }

    impl From<StashBody> for BoxBody {
        fn from(v: StashBody) -> BoxBody {
            BoxBody::new(v)
        }
    }
}
