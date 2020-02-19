use cdd::*;

/// Perform an RPC socket request
pub fn request(host: &str, request: &str) -> Result<String, ws::Error> {
    use websocket::{ClientBuilder, Message};

    let mut client = ClientBuilder::new(host)
        .unwrap()
        .connect_insecure()
        .unwrap();

    client.send_message(&Message::text(request)).unwrap(); // Send message

    if let websocket::OwnedMessage::Text(response) = client.recv_message().unwrap() {
        println!("response -> {:?}", response);
        Ok(response)
    } else {
        Ok("".into())
    }
}

// pub fn parse(host: &str, code: &str) -> Result<Project, Box<dyn std::error::Error>> {}
