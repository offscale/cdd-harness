#![allow(dead_code)]
#![allow(unused_imports)]

// use crate::rpc;

// const RUST_SERVER: &str = "ws://127.0.0.1:7779";

// #[test]
// fn test_parse_rust_empty() {
//     let response = rpc::parse(RUST_SERVER, "");
//     assert_eq!(
//         response.unwrap(),
//         rpc::RPCResponse {
//             jsonrpc: "2.0".into(),
//             id: None,
//             error: Some(rpc::RPCErrorCode {
//                 code: -32700,
//                 message: "Parse error".into()
//             }),
//         }
//     );
// }

// #[test]
// fn test_parse_rust_valid() {
//     let response = rpc::parse(RUST_SERVER, "var a=5;");

//     assert_eq!(
//         response.unwrap(),
//         rpc::RPCResponse {
//             jsonrpc: "2.0".into(),
//             id: None,
//             error: Some(rpc::RPCErrorCode {
//                 code: -32700,
//                 message: "Parse error".into()
//             }),
//         }
//     );
// }
