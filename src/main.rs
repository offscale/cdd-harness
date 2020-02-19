mod rpc;
mod tests;

pub fn main() {
    println!("--> {:?}", rpc::rpc_request("ws://127.0.0.1:7779", "p"));
}
