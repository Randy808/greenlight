use anyhow::{Context, Error};
use gl_plugin::config::Config;
use gl_plugin::{
    hsm,
    node::PluginNodeServer,
    stager::Stage,
    storage::{SledStateStore, StateStore},
    Event,
};
use log::info;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cwd = env::current_dir()?;
    info!("Running in {}", cwd.to_str().unwrap());
    let config = Config::new().context("loading config")?;
    let stage = Arc::new(Stage::new());
    let (events, _) = tokio::sync::broadcast::channel(16);
    let state_store = get_signer_store().await?;

    start_hsm_server(config.clone(), stage.clone())?;
    start_node_server(config, stage.clone(), events.clone(), state_store).await?;

    let plugin = gl_plugin::init(stage, events).await?;
    if let Some(plugin) = plugin.start().await? {
        plugin.join().await
    } else {
        Ok(()) // This is just an invocation with `--help`, we're good to exit
    }
}

//RANDY_COMMENTED
//Starts a PluginNodeServer, wraps that with a WrappedNodeServer and NodeServer before using
//the result to build the tonic 'router' server containing an auth service and rpc wait middleware/layer
async fn start_node_server(
    config: Config,
    stage: Arc<Stage>,
    events: tokio::sync::broadcast::Sender<Event>,
    signer_state_store: Box<dyn StateStore>,
) -> Result<(), Error> {

    //Create a socket address using the config values
    let addr: SocketAddr = config
        .node_grpc_binding
        .parse()
        .context("parsing the node_grpc_binding")?;

    //Create a tls config using the identity certs in the config
    let tls = tonic::transport::ServerTlsConfig::new()
        .identity(config.identity.id.clone())
        .client_ca_root(config.identity.ca.clone());

    //Use the current directory as the rpc_path
    let mut rpc_path = std::env::current_dir().unwrap();
    //Push the lightning-rpc into the rpc_path
    rpc_path.push("lightning-rpc");

    //Log that we're starting the grpc server on the address derived from the config and
    // the rpc path of the current dir
    info!(
        "Starting grpc server on addr={} serving rpc={}",
        addr,
        rpc_path.display()
    );

    //Create a new PluginNodeServer with the stage, config, event, and signer_state_store
    let node_server = PluginNodeServer::new(
        stage.clone(),
        config.clone(),
        events.clone(),
        signer_state_store,
    )
    .await?;

    //Create a NodeServer using the PluginNodeServer (by wrapping the PluginNodeServer with WrappedNodeServer)
    let cln_node = gl_plugin::grpc::pb::node_server::NodeServer::new(
        gl_plugin::node::WrappedNodeServer::new(node_server.clone())
            .await
            .context("creating cln_grpc::pb::node_server::NodeServer instance")?,
    );

    //Create a tonic router which looks like a server protected by the 
    //signature context layer, and the rpc wait and node server services
    let router = tonic::transport::Server::builder()
        .tls_config(tls)?
        .layer(gl_plugin::node::SignatureContextLayer::new(
            node_server.ctx.clone(),
        ))
        .add_service(gl_plugin::node::RpcWaitService::new(cln_node, rpc_path))
        .add_service(gl_plugin::pb::node_server::NodeServer::new(
            gl_plugin::node::WrappedNodeServer::new(node_server).await?,
        ));

    //Spawn a new task using the router to serve from the addr in the config
    tokio::spawn(async move {
        router
            .serve(addr)
            .await
            .context("grpc interface exited with error")
    });

    //Return an ok
    Ok(())
}

//RANDY_COMMENTED
//Returns a pointer to the SledStateStore containing the state directory
async fn get_signer_store() -> Result<Box<dyn StateStore>, Error> {
    //Get the state directory as the current directory
    let mut state_dir = env::current_dir()?;
    //Push the 'signer_state' string to this directory
    state_dir.push("signer_state");

    //Return a pointer to the state_dir wrapped in a 'SledStateStore'?
    Ok(Box::new(SledStateStore::new(state_dir)?))
}

//RANDY
//Runs the Hsm server
fn start_hsm_server(config: Config, stage: Arc<Stage>) -> Result<(), Error> {
    //G
    // We run this already at startup, not at configuration because if
    // the signerproxy doesn't find the socket on the FS it'll exit.
    //G_E

    //the hsm server is initialized here
    let hsm_server = hsm::StagingHsmServer::new(
        PathBuf::from_str(&config.hsmd_sock_path).context("hsmd_sock_path is not a valid path")?,
        stage.clone(),
        config.node_info.clone(),
        config.node_config.clone(),
    );

    //run the hsm server
    tokio::spawn(hsm_server.run());
    Ok(())
}
