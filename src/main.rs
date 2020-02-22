mod methods;
mod rpc;
mod tests;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    let hostname: &str = match args.get(1) {
        Some(arg) => arg,
        None => "ws://127.0.0.1:7779",
    };

    let project = cdd::Project {
        info: None,
        models: vec![cdd::Model {
            name: "Pet".into(),
            vars: vec![],
        }],
        requests: vec![cdd::Request {
            name: "listPets".into(),
            path: "/pets".into(),
            params: vec![],
            method: cdd::Method::Get,
            response_type: "".into(),
            error_type: "".into(),
        }],
    };

    println!("trying {}", hostname);

    let result = methods::update(hostname, "", project);

    println!("--> {:?}", result);
    println!("{:#}", result.unwrap().code);
}
