mod methods;
mod rpc;
mod tests;

pub fn main() {
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

    let result = methods::update("ws://127.0.0.1:7779", "", project);

    println!("--> {:?}", result);
    println!("{:#}", result.unwrap().code);
}
