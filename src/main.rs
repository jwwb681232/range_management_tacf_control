extern crate ws;
extern crate redis;

mod websocket;
mod tacf;

fn main() {
    env_logger::init();

    //一个客户端专门从tacf_to_websocket 里取数据广播
    let inner_sender_thread = std::thread::spawn(|| {
        websocket::bridge::client()
    });

    //tacf 数据交互
    let tacf_thread = std::thread::spawn(|| {
        tacf::handler::run().unwrap()
    });

    //websocket 服务端
    websocket::server::Server::run();


    let _ = inner_sender_thread.join();
    let _ = tacf_thread.join();
}
