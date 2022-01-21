use crate::websocket::router::{Router, TacfReceiver};

pub struct Server{}

impl Server {
    pub fn run(){
        ws::listen(format!("0.0.0.0:8084"), |out| {
            Router {
                sender: out,
                inner: Box::new(TacfReceiver),
            }
        }).unwrap();
    }
}

