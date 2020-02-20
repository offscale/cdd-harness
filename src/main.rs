mod methods;
mod rpc;
mod tests;

const CODE: &str = r#"
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Pet {
	pub id: i32,
	pub name: String,
	pub tag: String,
}

#[derive(Queryable, Debug)]
pub struct Error {
	pub code: i32,
	pub message: String,
}
"#;

pub fn main() {
    // println!("--> {:?}", rpc::parse("ws://127.0.0.1:7779", CODE));

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
