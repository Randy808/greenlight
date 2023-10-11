//! Utilities used to authorize a signature request based on pending RPCs
use std::str::FromStr;
use lightning_signer::invoice::Invoice;
use vls_protocol_signer::approver::Approval;
use crate::signer::model::Request;
use crate::Error;

pub trait Authorizer {
    fn authorize(
        &self,
        requests: &Vec<Request>,
    ) -> Result<Vec<Approval>, Error>;
}

pub struct DummyAuthorizer {}

impl Authorizer for DummyAuthorizer {
    fn authorize(
        &self,
        _requests: &Vec<Request>,
    ) -> Result<Vec<Approval>, Error> {
        Ok(vec![])
    }
}

//Create a GreenlightAuthorizer
pub struct GreenlightAuthorizer {}


//Implement the Authorizer trait (from up top here in auth.rs) functionality for the GreenlightAuthorizer
impl Authorizer for GreenlightAuthorizer {

    //Looks through all the requests and puts the bolt11 invoice (of requests with a bolt11 invoice inside of it)
    // into a vec of 'Approvals'
    fn authorize(
        &self,
        requests: &Vec<Request>,
    ) -> Result<Vec<Approval>, Error> {

        //Approvals equals the requests iterated through, matched to be a request to pay, and returning
        //a bolt11 invoice from the request
        let approvals : Vec<_> = requests.iter().flat_map(|request| {
            match request {
                Request::GlPay(req) => {
                    // TODO error handling
                    Some(Approval::Invoice(Invoice::from_str(&req.bolt11)
                        .expect("")))
                }

                //Otherwise do nothing
                _ => None,
            }
        }).collect();
        
        //Collect approvals
        Ok(approvals)
    }
}
