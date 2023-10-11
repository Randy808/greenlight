//G
/// A simple staging mechanism for incoming requests so we can invert from
/// pull to push. Used by `hsmproxy` to stage requests that can then
/// asynchronously be retrieved and processed by one or more client
/// devices.
//G_E


use crate::pb;
use anyhow::{anyhow, Error};
use log::{debug, trace, warn};
use std::collections;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{broadcast, mpsc, Mutex};

//Creates a struct with a hashmap of ids to requests, a notification sender,
//and hsm connection ids
#[derive(Debug)]
pub struct Stage {
    requests: Mutex<collections::HashMap<u32, Request>>,
    notify: broadcast::Sender<Request>,
    hsm_connections: Arc<AtomicUsize>,
}

//Create a struct called request which contains a protobuf hsm request 
//and a hsm response of type mspc sender (?)
#[derive(Clone, Debug)]
pub struct Request {
    pub request: pb::HsmRequest,
    pub response: mpsc::Sender<pb::HsmResponse>,
}

impl Stage {


    //Create a new stage using new values for each field
    pub fn new() -> Self {
        //Create a channel with a 1000 byte capacity but only save a sender
        let (notify, _) = broadcast::channel(1000);

        //Return a stage
        Stage {
            //Create a new hashmap for requests
            requests: Mutex::new(collections::HashMap::new()),
            //Add the newly-created notify sender to the stage
            notify: notify,
            //Set the hsm connections to a pointer to an atomic usize
            hsm_connections: Arc::new(AtomicUsize::new(0)),
        }
    }

    //Create a new channel and wrap the channel sender, and the request arg into to 'r',
    //then add r to requests and use the notify to send the request
    // and return the channel receiver to the caller.

    //So creates a new channel sender and receiver,
    //and adds the sender to a newly initialized request put in the request queue,
    // and returns a receiver for the caller to subscribe to 
    pub async fn send(
        &self,
        request: pb::HsmRequest,
    ) -> Result<mpsc::Receiver<pb::HsmResponse>, Error> {
        //Lock and extract the requests from the stager
        let mut requests = self.requests.lock().await;

        //Create a channel of buffer 1 and capture the sender as response,
        // and receiver as receiver
        let (response, receiver): (
            mpsc::Sender<pb::HsmResponse>,
            mpsc::Receiver<pb::HsmResponse>,
        ) = mpsc::channel(1);

        //Create a Request from the hsm request and the channel response
        let r = Request { request, response };

        //Insert this newly created request into requests
        requests.insert(r.request.request_id, r.clone());

        //If there was an error sending the created request in the notify stream
        if let Err(_) = self.notify.send(r) {
            //log the error
            warn!("Error notifying hsmd request stream, likely lost connection.");
        }

        //return the receiver to the created channel
        Ok(receiver)
    }

    //Give a stage stream with the backlog of requests, a way to connect to notify, and the hsm connections
    pub async fn mystream(&self) -> StageStream {
        //Lock the mutex and get the requests
        let requests = self.requests.lock().await;
        //Call fetch_add to add '1' to the number of hsmc connections
        self.hsm_connections.fetch_add(1, Ordering::Relaxed);

        StageStream {
            //Get the request values and clone them all to create a backlog
            backlog: requests.values().map(|e| e.clone()).collect(),

            //Create a new subscription to the broadcast
            bcast: self.notify.subscribe(),

            //Clone the hsm connections
            hsm_connections: self.hsm_connections.clone(),
        }
    }

    //Remove the request associated with the response in the argument, and use that
    //removed request to send the arg_response back to the requester
    pub async fn respond(&self, response: pb::HsmResponse) -> Result<(), Error> {
        //Get the requests from the locked-mutex 
        let mut requests = self.requests.lock().await;

        //Match the result of removing the response in the argument from the requests
        match requests.remove(&response.request_id) {
            //If it was removed
            Some(req) => {

                //log that it was removed
                debug!(
                    "Response for request_id={}, outstanding requests count={}",
                    response.request_id,
                    requests.len()
                );

                //And try to send the response using the request's responder (aka 'req.response')
                if let Err(e) = req.response.send(response).await {
                    //Error out if the resoinse couldn't send
                    Err(anyhow!("Error sending request to requester: {:?}", e))
                } else {
                    //Give Ok otherwise
                    Ok(())
                }
            }
            None => {
                //log that the request was not found otherwise
                trace!(
                    "Request {} not found, is this a duplicate result?",
                    response.request_id
                );
                //return ok
                Ok(())
            }
        }
    }

    pub async fn is_stuck(&self) -> bool {
        let sticky = self
            .requests
            .lock()
            .await
            .values()
            .filter(|r| r.request.raw[0..2] == [0u8, 5])
            .count();

        trace!("Found {sticky} sticky requests.");
        sticky != 0
    }
}

pub struct StageStream {
    backlog: Vec<Request>,
    bcast: broadcast::Receiver<Request>,
    hsm_connections: Arc<AtomicUsize>,
}

impl StageStream {
    pub async fn next(&mut self) -> Result<Request, Error> {
        if self.backlog.len() > 0 {
            let req = self.backlog[0].clone();
            self.backlog.remove(0);
            Ok(req)
        } else {
            match self.bcast.recv().await {
                Ok(r) => Ok(r),
                Err(e) => Err(anyhow!("error waiting for more requests: {:?}", e)),
            }
        }
    }
}

impl Drop for StageStream {
    fn drop(&mut self) {
        self.hsm_connections.fetch_sub(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep as delay_for;

    #[tokio::test]
    async fn test_live_stream() {
        let stage = Stage::new();

        let mut responses = vec![];

        for i in 0..10 {
            responses.push(
                stage
                    .send(pb::HsmRequest {
                        request_id: i,
                        context: None,
                        raw: vec![],
                        signer_state: vec![],
                        requests: vec![],
                    })
                    .await
                    .unwrap(),
            );
        }

        let mut s1 = stage.mystream().await;
        let mut s2 = stage.mystream().await;
        let f1 = tokio::spawn(async move {
            while let Ok(r) = s1.next().await {
                eprintln!("hsmd {} is handling request {}", 1, r.request.request_id);
                match r
                    .response
                    .send(pb::HsmResponse {
                        request_id: r.request.request_id,
                        raw: vec![],
                        signer_state: vec![],
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(e) => eprintln!("errror {:?}", e),
                }
                delay_for(Duration::from_millis(10)).await;
            }
        });
        let f2 = tokio::spawn(async move {
            while let Ok(r) = s2.next().await {
                eprintln!("hsmd {} is handling request {}", 2, r.request.request_id);
                match r
                    .response
                    .send(pb::HsmResponse {
                        request_id: r.request.request_id,
                        raw: vec![],
                        signer_state: vec![],
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(e) => eprintln!("errror {:?}", e),
                }
                delay_for(Duration::from_millis(10)).await;
            }
        });

        for i in 10..100 {
            responses.push(
                stage
                    .send(pb::HsmRequest {
                        request_id: i,
                        context: None,
                        raw: vec![],
                        signer_state: vec![],
                        requests: vec![],
                    })
                    .await
                    .unwrap(),
            );
        }

        for mut r in responses {
            let resp = r.recv().await.unwrap();
            eprintln!("Got response {:?}", resp);
        }

        drop(stage);
        f1.await.unwrap();
        f2.await.unwrap();
    }
}
