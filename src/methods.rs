use crate::rpc::*;
use cdd::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParseResult {
    pub project: Project,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateResult {
    pub code: String,
    pub project: Project,
}

pub fn parse(
    host: &str,
    code: &str,
) -> Result<RPCResponse<ParseResult>, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&rpc_request(
        host,
        "parse",
        serde_json::json!({ "code": code }),
    )?)?)
}

pub fn update(
    host: &str,
    code: &str,
    project: Project,
) -> Result<RPCResponse<UpdateResult>, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&rpc_request(
        host,
        "update",
        serde_json::json!({ "code": code, "project": project }),
    )?)?)
}
