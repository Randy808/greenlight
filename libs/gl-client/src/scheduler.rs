use crate::pb::scheduler::scheduler_client::SchedulerClient;
use crate::tls::{self, TlsConfig};

use crate::node::GrpcClient;
use crate::{node, pb, signer::Signer, utils};
use anyhow::Result;
use lightning_signer::bitcoin::Network;
use log::debug;
use tonic::transport::Channel;

type Client = SchedulerClient<Channel>;

#[derive(Clone)]
pub struct Scheduler {
    /// Our local `node_id` used when talking to the scheduler to
    /// identify us.
    node_id: Vec<u8>,
    client: Client,
    network: Network,
}

impl Scheduler {
    pub async fn with(
        node_id: Vec<u8>,
        network: Network,
        uri: String,
        tls: &TlsConfig,
    ) -> Result<Scheduler> {
        debug!("Connecting to scheduler at {}", uri);
        let channel = tonic::transport::Endpoint::from_shared(uri)?
            .tls_config(tls.inner.clone())?
            .tcp_keepalive(Some(crate::TCP_KEEPALIVE))
            .http2_keep_alive_interval(crate::TCP_KEEPALIVE)
            .keep_alive_timeout(crate::TCP_KEEPALIVE_TIMEOUT)
            .keep_alive_while_idle(true)
            .connect_lazy();

        let client = SchedulerClient::new(channel);

        Ok(Scheduler {
            client,
            node_id,
            network,
        })
    }

    pub async fn new(node_id: Vec<u8>, network: Network) -> Result<Scheduler> {
        let tls = crate::tls::TlsConfig::new()?;
        let uri = utils::scheduler_uri();
        Self::with(node_id, network, uri, &tls).await
    }

    //RANDY_COMMENTED
    pub async fn register(
        &self,
        signer: &Signer,
        invite_code: Option<String>,
    ) -> Result<pb::scheduler::RegistrationResponse> {
        //Get the invite code
        let code = invite_code.unwrap_or_default();
        //call the 'inner_register' on the signer and the invite code
        return self.inner_register(signer, code).await;
    }

    //G
    /// We split the register method into one with an invite code and one
    /// without an invite code in order to keep the api stable. We might want to
    /// remove the invite system in the future and so it does not make sense to
    /// change the signature of the register method.
    /// G_E

    //RANDY_COMMENTED
    //Gets the challenge using the scheduler client while passing in the node id,
    // use the signer to sign it, and send the challenge and invite code to 'register'
    async fn inner_register(
        &self,
        signer: &Signer,
        invite_code: String,
    ) -> Result<pb::scheduler::RegistrationResponse> {
        //log we're getting a challenge 
        log::debug!("Retrieving challenge for registration");

        // Use the scheduler client to get the challenge using the
        //node_id/pubkey from the signer (which only needed tlsconfig for initialization)
        let challenge = self
            .client
            .clone()
            //Pass in the challenge 'scope' which is a protobuf model
            .get_challenge(pb::scheduler::ChallengeRequest {
                scope: pb::scheduler::ChallengeScope::Register as i32,
                node_id: self.node_id.clone(),
            })
            .await?
            .into_inner();

        //log we got a response and log the response
        log::trace!("Got a challenge: {}", hex::encode(&challenge.challenge));

        //Sign the challenge using the signer
        let signature = signer.sign_challenge(challenge.challenge.clone())?;
        
        //Generate the self signed cert passing in the node_id,
        //'default' for device name, and 'localhost' for alt name
        let device_cert = tls::generate_self_signed_device_cert(
            &hex::encode(self.node_id.clone()),
            "default".into(),
            vec!["localhost".into()],
        );

        //Serialize the cert
        let device_csr = device_cert.serialize_request_pem()?;

        //log
        debug!("Requesting registration with csr:\n{}", device_csr);

        //get startup messages from signer
        let startupmsgs = signer
            .get_startup_messages()
            .into_iter()
            .map(|m| m.into())
            .collect();

        
        //Use the scheduler client to register the signer info with the
        //node id, invite code, network, signed challenge, signer init msg,
        //etc
        let mut res = self
            .client
            .clone()
            .register(pb::scheduler::RegistrationRequest {
                node_id: self.node_id.clone(),
                bip32_key: signer.bip32_ext_key(),
                network: self.network.to_string(),
                challenge: challenge.challenge,
                signer_proto: signer.version().to_owned(),
                init_msg: signer.get_init(),
                signature,
                csr: device_csr.into_bytes(),
                invite_code,
                startupmsgs,
            })
            .await?
            .into_inner();

        //G
        // This step ensures backwards compatibility with the backend. If we did
        // receive a device key, the backend did not sign the csr and we need to
        // return the response as it is. If the device key is empty, the csr was
        // signed and we return the client side generated private key.
        //G_E

        //If the device_key is empty in the registration response
        if res.device_key.is_empty() {

            //Say we receive a device_cert instead
            debug!("Received signed certificate:\n{}", &res.device_cert);

            //G
            // We intercept the response and replace the private key with the
            // private key of the device_cert. This private key has been generated
            // on and has never left the client device.
            //G_END

            //Use the private key from the device cert as the device key
            res.device_key = device_cert.serialize_private_key_pem();
        }

        //G
        // We ask the signer for a signature of the public key to append the
        // public key to any payload that is sent to a node.
        //G_END

        //Get the public key from the device cert
        let public_key = device_cert.get_key_pair().public_key_raw();

        //
        debug!(
            "Asking singer to sign public key {}",
            hex::encode(public_key)
        );

        //Sign the device's public key
        let r = signer.sign_device_key(public_key)?;

        //Log the signature
        debug!("Got signature: {}", hex::encode(r));

        //Return an ok response with the challenge response
        Ok(res)
    }

    pub async fn recover(&self, signer: &Signer) -> Result<pb::scheduler::RecoveryResponse> {
        let challenge = self
            .client
            .clone()
            .get_challenge(pb::scheduler::ChallengeRequest {
                scope: pb::scheduler::ChallengeScope::Recover as i32,
                node_id: self.node_id.clone(),
            })
            .await?
            .into_inner();

        let signature = signer.sign_challenge(challenge.challenge.clone())?;
        let name = format!("recovered-{}", hex::encode(&challenge.challenge[0..8]));
        let device_cert = tls::generate_self_signed_device_cert(
            &hex::encode(self.node_id.clone()),
            &name,
            vec!["localhost".into()],
        );
        let device_csr = device_cert.serialize_request_pem()?;
        debug!("Requesting recovery with csr:\n{}", device_csr);

        let mut res = self
            .client
            .clone()
            .recover(pb::scheduler::RecoveryRequest {
                node_id: self.node_id.clone(),
                challenge: challenge.challenge,
                signature,
                csr: device_csr.into_bytes(),
            })
            .await?
            .into_inner();

        // This step ensures backwards compatibility with the backend. If we did
        // receive a device key, the backend did not sign the csr and we need to
        // return the response as it is. If the device key is empty, the csr was
        // signed and we return the client side generated private key.
        if res.device_key.is_empty() {
            debug!("Received signed certificate:\n{}", &res.device_cert);
            // We intercept the response and replace the private key with the
            // private key of the device_cert. This private key has been generated
            // on and has never left the client device.
            res.device_key = device_cert.serialize_private_key_pem();
        }

        // We ask the signer for a signature of the public key to append the
        // public key to any payload that is sent to a node.
        let public_key = device_cert.get_key_pair().public_key_raw();
        debug!(
            "Asking singer to sign public key {}",
            hex::encode(public_key)
        );
        let r = signer.sign_device_key(public_key)?;
        debug!("Got signature: {}", hex::encode(r));

        Ok(res)
    }

    //RANDY_COMMENTED
    //Schedule the node using the node_id and return an 
    //initialized node struct connected to this link
    pub async fn schedule<T>(&self, tls: TlsConfig) -> Result<T>
    where
        T: GrpcClient,
    {
        //Create a scheduler client and schedule the node
        //by sending in the node_id
        let sched = self
            .client
            .clone()
            .schedule(pb::scheduler::ScheduleRequest {
                node_id: self.node_id.clone(),
            })
            .await?
            .into_inner();

        //Get the grpc uri from the schedule response
        let uri = sched.grpc_uri;

        //Return a new node struct with the id, the network, and the tls, and 
        //attach it to the uri returned by the schedule call
        node::Node::new(self.node_id.clone(), self.network, tls)
            .connect(uri)
            .await
    }

    //RANDY_COMMENTED
    //Call 'export_node' on the scheduler client and get an ExportNodeResponse
    pub async fn export_node(&self) -> Result<pb::scheduler::ExportNodeResponse> {
        
        //Return an Ok respponse with an empty ExportNodeRequest
        Ok(self
            .client
            .clone()
            .export_node(pb::scheduler::ExportNodeRequest {})
            .await?
            .into_inner())
    }


    pub async fn get_invite_codes(&self) -> Result<pb::scheduler::ListInviteCodesResponse> {
        let res = self
            .client
            .clone()
            .list_invite_codes(pb::scheduler::ListInviteCodesRequest {})
            .await?;
        Ok(res.into_inner())
    }
}
