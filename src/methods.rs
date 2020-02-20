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

pub fn parse(host: &str, code: &str) -> Result<ParseResult, Box<dyn std::error::Error>> {
    let json_result = rpc_request(host, "parse", serde_json::json!({ "code": code }))?;
    let result: RPCResponse<ParseResult> = serde_json::from_str(&json_result)?;
    Ok(result.result.unwrap()) // fix
}

pub fn update(
    host: &str,
    code: &str,
    project: Project,
) -> Result<UpdateResult, Box<dyn std::error::Error>> {
    let json_result = rpc_request(
        host,
        "update",
        serde_json::json!({ "code": code, "project": project }),
    )?;
    let result: RPCResponse<UpdateResult> = serde_json::from_str(&json_result)?;
    Ok(result.result.unwrap()) // fix
}
