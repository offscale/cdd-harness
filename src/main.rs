mod rpc;

pub fn main() {
    println!("--> {:?}", rpc::request("ws://127.0.0.1:7779", "p"));

    // if let Err(error) = ws::connect("ws://127.0.0.1:7779", |out| {
    //     if out.send("Hello WebSocket").is_err() {
    //         println!("Websocket couldn't queue an initial message.");
    //     } else {
    //         println!("Client sent message 'Hello WebSocket'. ");
    //     }

    //     move |msg| {
    //         println!("msg: {:?}", msg);
    //         out.close(ws::CloseCode::Normal)
    //     }
    // }) {
    //     // Inform the user of failure
    //     println!("Failed to create WebSocket due to: {:?}", error);
    // };
}

// fn rpc_call(method: &str, )

// #[derive(Deserialize, Debug, Clone)]
// pub struct RPCRequest {
//     pub jsonrpc: String,
// }
