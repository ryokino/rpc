/// RPC Request
export interface RpcRequest {
  method: string;
  params: any[];
  param_types: string[];
  id: number;
}

/// RPC エラー
export interface RpcError {
  code: number;
  message: string;
}

/// RPC レスポンス
export interface RpcResponse {
  result?: number;
  result_type?: string;
  error?: RpcError;
  id: number;
}
