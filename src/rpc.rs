use serde::{Deserialize, Serialize};

/// Perform an RPC socket request
pub fn socket_request(host: &str, request: &str) -> Result<String, Box<dyn std::error::Error>> {
    use websocket::{ClientBuilder, Message};

    println!("request -> {}", request);

    let mut client = ClientBuilder::new(host)?.connect_insecure()?;
    client.send_message(&Message::text(request))?; // Send message

    if let websocket::OwnedMessage::Text(response) = client.recv_message()? {
        println!("response -> {}", response);
        Ok(response)
    } else {
        // todo: Err here
        Ok("{}".into())
    }
}

pub fn rpc_request(
    host: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<RPCResponse, Box<dyn std::error::Error>> {
    // let json_request = serde_json::to_string(&RPCRequest {
    //     jsonrpc: "2.0".into(),
    //     method: method.into(),
    //     params: None,
    //     id: "1".into(),
    // })?;

    let json_request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": "1",
    })
    .to_string();

    let json_response = socket_request(host, &json_request)?;
    let response: RPCResponse = serde_json::from_str(&json_response)?;
    Ok(response)
}

pub fn parse(host: &str, code: &str) -> Result<RPCResponse, Box<dyn std::error::Error>> {
    rpc_request(host, "parse", serde_json::json!({ "code": code }))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RPCRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<String>,
    pub id: String,
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
