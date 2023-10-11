mod hsmproxy;
mod passfd;
mod pb;
mod wire;

use anyhow::Result;

//Creates an empty Proxy struct
pub struct Proxy {}

//Implements methods for the Proxy struct
impl Proxy {

    //Create a new method that returns an empty proxy
    pub fn new() -> Proxy {
        Proxy {}
    }

    //Calls the hsmproxy's associated function 'run'
    pub async fn run(&self) -> Result<()> {
        hsmproxy::run().await
    }
}
