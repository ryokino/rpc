# RPC Implementation Action Plan

## プロジェクト概要

Unix Domain Socketを使用したJSON-RPC実装の学習プロジェクト。

- **Server**: Rust
- **Client**: TypeScript (Node.js)
- **通信**: AF_UNIX Socket
- **プロトコル**: JSON形式

## 実装するRPC関数

1. `floor(double x)` - 小数点以下切り捨て
2. `nroot(int n, int x)` - n乗根の計算
3. `reverse(string s)` - 文字列の反転
4. `validAnagram(string str1, string str2)` - アナグラム判定
5. `sort(string[] strArr)` - 文字列配列のソート

## メッセージ形式

### Request
```json
{
   "method": "floor", 
   "params": [3.7], 
   "param_types": ["double"],
   "id": 1
}
```

### Response
```json
{
   "result": "3",
   "result_type": "int",
   "id": 1
}
```

### Error
```json
{
   "error": {
      "code": -32601,
      "message": "Method not found"
   },
   "id": 1
}
```

## ディレクトリ構成
```
rpc/
├── server/           # Rust実装
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── rpc.rs
└── client/           # TypeScript実装
    ├── package.json
    └── src/
        ├── client.ts
        └── types.ts
```

## 実装ステップ

### Step 1: 基盤セットアップ
- [x] Rustプロジェクトの依存関係設定（serde, serde_json, tokio）
- [x] TypeScriptプロジェクトの設定
- [x] 基本的な型定義の作成 

### Step 2: Server実装（Rust）
- [ ] Unix Domain Socketサーバーの起動
- [ ] JSONメッセージの受信・送信
- [ ] ハッシュマップでメソッド管理
- [ ] 5つのRPC関数の実装

### Step 3: Client実装（TypeScript）
- [ ] Unix Domain Socketへの接続
- [ ] JSONメッセージの送受信
- [ ] RPC呼び出し用のAPI作成

### Step 4: 動作確認
- [ ] サーバー起動
- [ ] クライアントからの呼び出しテスト
- [ ] 全メソッドの動作確認

## Socket設定

- **パス**: `/tmp/rpc.sock`
- **プロトコル**: AF_UNIX
- **形式**: JSON文字列 + 改行区切り

## 実装方針

### Rust側（Server）
- `HashMap<String, fn()>` でメソッド管理
- tokioで非同期処理
- serde_jsonでJSON処理

### TypeScript側（Client）
- Node.jsのnetモジュール使用
- Promise/async-awaitで非同期処理
- 型安全なAPI提供

---
