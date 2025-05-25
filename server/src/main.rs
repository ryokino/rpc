use std::{collections::HashMap, path::Path};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixListener,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

type RpcMethod = fn(&Value) -> Result<(String, String), String>;

const SERVER_PATH: &str = "/tmp/rpc.sock";

/// RPC リクエスト

#[derive(Debug, Serialize, Deserialize)]
struct RpcRequest {
    method: String,
    params: serde_json::Value, // 柔軟に受け取るため
    param_types: Option<Vec<String>>,
    id: u64,
}

/// RPC レスポンス
#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse {
    result: String,
    result_type: String,
    id: u64,
}

/// RPC エラー
#[derive(Debug, Serialize, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcErrorResponse {
    error: RpcError,
    id: u64,
}

#[tokio::main]
async fn main() {
    if Path::new(SERVER_PATH).exists() {
        std::fs::remove_file(SERVER_PATH).unwrap();
    }

    let method_table = create_method_table();

    let listener = UnixListener::bind(SERVER_PATH).unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("New client connected!");

                // streamを分割
                let (read_half, mut write_half) = stream.into_split();
                let mut reader = BufReader::new(read_half);
                let mut lines = String::new();

                match reader.read_line(&mut lines).await {
                    Ok(0) => println!("接続終了"),
                    Ok(_) => {
                        let trimmed_lines = lines.trim();
                        println!("受信: {}", trimmed_lines);

                        // JSONのパース処理
                        match serde_json::from_str::<RpcRequest>(trimmed_lines) {
                            Ok(request) => {
                                let response = if let Some(method_fn) =
                                    method_table.get(&request.method)
                                {
                                    match method_fn(&request.params) {
                                        Ok((result, result_type)) => RpcResponse {
                                            result,
                                            result_type,
                                            id: request.id,
                                        },
                                        Err(err_msg) => {
                                            let error_response = RpcErrorResponse {
                                                error: RpcError {
                                                    code: -32602,
                                                    message: err_msg,
                                                },
                                                id: request.id,
                                            };
                                            // エラーレスポンスを送信して続行
                                            if let Ok(error_json) =
                                                serde_json::to_string(&error_response)
                                            {
                                                let message = format!("{}\n", error_json);
                                                let _ =
                                                    write_half.write_all(message.as_bytes()).await;
                                            }
                                            continue;
                                        }
                                    }
                                } else {
                                    let error_response = RpcErrorResponse {
                                        error: RpcError {
                                            code: -32601,
                                            message: "Method not found".to_string(),
                                        },
                                        id: request.id,
                                    };

                                    if let Ok(error_json) = serde_json::to_string(&error_response) {
                                        let message = format!("{}\n", error_json);
                                        let _ = write_half.write_all(message.as_bytes()).await;
                                    }
                                    continue;
                                };

                                // JSONに変換する
                                match serde_json::to_string(&response) {
                                    Ok(json_response) => {
                                        let message = format!("{}\n", json_response);
                                        if let Err(e) =
                                            write_half.write_all(message.as_bytes()).await
                                        {
                                            println!("Error sending response: {}", e);
                                        } else {
                                            println!(
                                                "Response sent successfully: {}",
                                                json_response
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error converting response to JSON: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("エラー: {}", e);

                                let error_response = RpcErrorResponse {
                                    error: RpcError {
                                        code: -32602,
                                        message: "Invalid params".to_string(),
                                    },
                                    id: 0,
                                };

                                match serde_json::to_string(&error_response) {
                                    Ok(error_response_json) => {
                                        let message = format!("{}\n", error_response_json);
                                        if let Err(e) =
                                            write_half.write_all(message.as_bytes()).await
                                        {
                                            println!("Error sending error response: {}", e);
                                        } else {
                                            println!(
                                                "Error response sent successfully: {}",
                                                error_response_json
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error converting error response to JSON: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("エラー: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn create_method_table() -> HashMap<String, RpcMethod> {
    let mut methods = HashMap::new();
    methods.insert("floor".to_string(), rpc_floor as RpcMethod);
    methods.insert("nroot".to_string(), rpc_nroot as RpcMethod);
    methods.insert("reverse".to_string(), rpc_reverse as RpcMethod);
    methods.insert("valid_anagram".to_string(), rpc_valid_anagram as RpcMethod);
    methods.insert("sort".to_string(), rpc_sort as RpcMethod);
    methods
}

fn rpc_floor(params: &Value) -> Result<(String, String), String> {
    if let Some(arr) = params.as_array() {
        if let Some(num) = arr.first().and_then(|v| v.as_f64()) {
            let result = num.floor();
            return Ok((result.to_string(), "int".to_string()));
        }
    }
    Err("Invalid params".to_string())
}

fn rpc_nroot(params: &Value) -> Result<(String, String), String> {
    if let Some(arr) = params.as_array() {
        if arr.len() >= 2 {
            if let (Some(n), Some(x)) = (
                arr.first().and_then(|v| v.as_f64()),
                arr.get(1).and_then(|v| v.as_f64()),
            ) {
                let result = x.powf(1.0 / n);
                return Ok((result.to_string(), "double".to_string()));
            }
        }
    }
    Err("Invalid params".to_string())
}

fn rpc_reverse(params: &Value) -> Result<(String, String), String> {
    if let Some(arr) = params.as_array() {
        if let Some(str) = arr.first().and_then(|v| v.as_str()) {
            let result = str.chars().rev().collect::<String>();
            return Ok((result, "string".to_string()));
        }
    }
    Err("Invalid params".to_string())
}

fn rpc_valid_anagram(params: &Value) -> Result<(String, String), String> {
    if let Some(arr) = params.as_array() {
        if arr.len() >= 2 {
            if let (Some(str1), Some(str2)) = (
                arr.first().and_then(|v| v.as_str()),
                arr.get(1).and_then(|v| v.as_str()),
            ) {
                let mut char1 = str1.chars().collect::<Vec<char>>();
                let mut char2 = str2.chars().collect::<Vec<char>>();
                char1.sort();
                char2.sort();
                let is_anagram = char1 == char2;
                return Ok((is_anagram.to_string(), "bool".to_string()));
            }
        }
    }
    Err("Invalid params".to_string())
}

fn rpc_sort(params: &Value) -> Result<(String, String), String> {
    if let Some(arr) = params.as_array() {
        if let Some(str_arr) = arr.first().and_then(|v| v.as_array()) {
            let mut strings: Vec<String> = Vec::new();
            for item in str_arr {
                if let Some(s) = item.as_str() {
                    strings.push(s.to_string());
                } else {
                    return Err("Invalid params".to_string());
                }
            }
            strings.sort();
            let result = serde_json::to_string(&strings).unwrap();
            return Ok((result, "string".to_string()));
        }
    }
    Err("Invalid params".to_string())
}
