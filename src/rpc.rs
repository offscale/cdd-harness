use cdd::*;

/// Perform an RPC socket request
pub fn request(host: &str, request: &str) -> Result<String, Box<dyn std::error::Error>> {
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

// pub fn parse(host: &str, code: &str) -> Result<Project, Box<dyn std::error::Error>> {}
