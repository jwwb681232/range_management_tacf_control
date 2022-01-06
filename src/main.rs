//#![windows_subsystem = "windows"]

extern crate ws;
extern crate chrono;
extern crate dotenv;
extern crate redis;

mod websocket;
mod tacf;

use redis::Commands;
use websocket::bridge;
use log::{Level, log};

struct Router {
    sender: ws::Sender,
    inner: Box<dyn ws::Handler>,
}

impl ws::Handler for Router {
    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err)
    }

    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let out = self.sender.clone();

        match req.resource() {
            // "/" => self.inner = Box::new(TacfReceiver),
            "/tacf_control" => self.inner = Box::new(TacfControl { ws: out }),
            _ => self.inner = Box::new(NotFound),
        }

        self.inner.on_request(req)
    }
}

struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}


struct TacfControl {
    ws: ws::Sender,
}

impl ws::Handler for TacfControl {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let _: () = con.publish("websocket_to_tacf", msg.to_string()).unwrap();

        //println!("websocket send: {}", msg.to_string());
        log!(Level::Warn, "websocket send: {}",  msg.to_string());
        //self.ws.broadcast(msg)

        if msg.to_string() != "RequestScenarioList".to_string() {
            self.ws.broadcast(msg).unwrap();
            //self.ws.close(ws::CloseCode::Normal).unwrap()

        }
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {

    }
}

struct TacfReceiver;

impl ws::Handler for TacfReceiver {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
    }
}

fn main() {
    env_logger::init();

    //一个客户端专门从tacf_to_websocket 里取数据广播
    let inner_sender_thread = std::thread::spawn(||{
        bridge::client()
    });


    //tacf 数据交互
    let tacf_thread = std::thread::spawn(|| {
        tacf::handler::run().unwrap()
    });


    //websocket 服务端
    ws::listen(format!("127.0.0.1:8084"), |out| {
        Router {
            sender: out,
            inner: Box::new(TacfReceiver),
        }
    }).unwrap();


    let _ = inner_sender_thread.join();
    let _ = tacf_thread.join();

}
