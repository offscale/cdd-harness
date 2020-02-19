use serde::{Deserialize, Serialize};

/// Perform an RPC socket request
pub fn socket_request(host: &str, request: &str) -> Result<String, Box<dyn std::error::Error>> {
    use websocket::{ClientBuilder, Message};

    let mut client = ClientBuilder::new(host)?.connect_insecure()?;
    client.send_message(&Message::text(request))?; // Send message

    if let websocket::OwnedMessage::Text(response) = client.recv_message()? {
        println!("response -> {:?}", response);
        Ok(response)
    } else {
        // todo: Err here
        Ok("{}".into())
    }
}

pub fn rpc_request(host: &str, request: &str) -> Result<RPCResponse, Box<dyn std::error::Error>> {
    let json = socket_request(host, request)?; // fix
    let response: RPCResponse = serde_json::from_str(&json)?;
    Ok(response)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub error: Option<RPCErrorCode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RPCErrorCode {
    pub code: i32,
    pub message: String,
}

// pub fn parse(host: &str, code: &str) -> Result<Project, Box<dyn std::error::Error>> {}
