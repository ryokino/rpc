// use serde::{Deserialize, Serialize};

// /// RPC リクエスト

// #[derive(Debug, Serialize, Deserialize)]
// pub struct RpcRequest {
//     pub method: String,
//     pub params: Vec<serde_json::Value>,
//     pub param_types: Vec<String>,
//     pub id: u64,
// }

// /// RPC レスポンス
// #[derive(Debug, Serialize, Deserialize)]
// pub struct RpcResponse {
//     pub result: Option<serde_json::Value>,
//     pub result_type: Option<String>,
//     pub error: Option<RpcError>,
//     pub id: u64,
// }

// /// RPC エラー
// #[derive(Debug, Serialize, Deserialize)]
// pub struct RpcError {
//     pub code: i32,
//     pub message: String,
// }
