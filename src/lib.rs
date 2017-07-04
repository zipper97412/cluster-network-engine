//TODO enlever les allow quand inutile
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate nanomsg;
extern crate serde;

mod communication;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use communication::client::{Client};
        let client = Client::new("ipc:///tmp/pipeline.ipc");

    }
}
