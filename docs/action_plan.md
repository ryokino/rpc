# RPC Implementation Action Plan

## プロジェクト概要

Unix Domain Socketを使用したJSON-RPC実装の学習プロジェクト。

- **Server**: Rust
- **Client**: TypeScript (Node.js)
- **通信**: AF_UNIX Socket
- **プロトコル**: JSON形式のカスタムRPC

## RPC関数仕様

実装するRPC関数とその詳細：

### 1. floor(double x)
- **機能**: 10進数xを最も近い整数に切り捨て
- **入力**: `double`
- **出力**: `int`
- **例**: `floor(3.7)` → `3`

### 2. nroot(int n, int x)  
- **機能**: 方程式 r^n = x におけるrの値を計算（n乗根）
- **入力**: `int n, int x`
- **出力**: `double`
- **例**: `nroot(3, 27)` → `3.0` (立方根)
- **エラーケース**: n=0, x<0でnが偶数

### 3. reverse(string s)
- **機能**: 文字列sの逆である新しい文字列を返す
- **入力**: `string`
- **出力**: `string` 
- **例**: `reverse("hello")` → `"olleh"`

### 4. validAnagram(string str1, string str2)
- **機能**: 2つの文字列が互いにアナグラムかを判定
- **入力**: `string, string`
- **出力**: `bool`
- **例**: `validAnagram("listen", "silent")` → `true`

### 5. sort(string[] strArr)
- **機能**: 文字列配列をソートして返す
- **入力**: `string[]`
- **出力**: `string[]`
- **例**: `sort(["zebra", "apple", "banana"])` → `["apple", "banana", "zebra"]`

### ハッシュマップベースの実装設計

Rustでは以下のような構造でメソッドを管理：

```rust
use std::collections::HashMap;

type RpcHandler = fn(&[serde_json::Value]) -> Result<serde_json::Value, RpcError>;

struct RpcServer {
    methods: HashMap<String, RpcHandler>,
}

impl RpcServer {
    fn new() -> Self {
        let mut methods = HashMap::new();
        methods.insert("floor".to_string(), handle_floor);
        methods.insert("nroot".to_string(), handle_nroot);
        methods.insert("reverse".to_string(), handle_reverse);
        methods.insert("validAnagram".to_string(), handle_validAnagram);
        methods.insert("sort".to_string(), handle_sort);
        
        RpcServer { methods }
    }
}
```

### Request Examples
```json
// floor関数の呼び出し
{
   "method": "floor", 
   "params": [3.7], 
   "param_types": ["double"],
   "id": 1
}

// nroot関数の呼び出し  
{
   "method": "nroot",
   "params": [3, 27],
   "param_types": ["int", "int"], 
   "id": 2
}

// reverse関数の呼び出し
{
   "method": "reverse",
   "params": ["hello"],
   "param_types": ["string"],
   "id": 3
}

// validAnagram関数の呼び出し
{
   "method": "validAnagram", 
   "params": ["listen", "silent"],
   "param_types": ["string", "string"],
   "id": 4
}

// sort関数の呼び出し
{
   "method": "sort",
   "params": [["zebra", "apple", "banana"]],
   "param_types": ["string[]"],
   "id": 5
}
```

### Response Examples
```json
// floor関数のレスポンス
{
   "result": "3",
   "result_type": "int",
   "id": 1
}

// nroot関数のレスポンス
{
   "result": "3.0", 
   "result_type": "double",
   "id": 2
}

// reverse関数のレスポンス
{
   "result": "olleh",
   "result_type": "string", 
   "id": 3
}

// validAnagram関数のレスポンス
{
   "result": "true",
   "result_type": "bool",
   "id": 4
}

// sort関数のレスポンス
{
   "result": ["apple", "banana", "zebra"],
   "result_type": "string[]",
   "id": 5
}
```

### Error Response
```json
{
   "error": {
      "code": -32603,
      "message": "Internal error",
      "data": "Division by zero"
   },
   "id": 1
}
```

## 全体アーキテクチャ

### システム構成
```
Client (TypeScript/Node.js)
    ↓ Unix Domain Socket
    ↓ JSON Request/Response  
Server (Rust)
    ↓ Method Dispatch
    ↓ Business Logic
Result ← Computation
```

### ディレクトリ構成
```
rpc/
├── server/           # Rust実装
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── rpc/
│   │   │   ├── mod.rs
│   │   │   ├── server.rs
│   │   │   ├── message.rs
│   │   │   └── handlers.rs
│   │   └── lib.rs
│   └── tests/
├── client/           # TypeScript/Node.js実装
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   │   ├── client.ts
│   │   ├── types.ts
│   │   └── rpc-client.ts
│   └── tests/
└── docs/
    ├── action-plan.md
    ├── architecture.md
    └── learning-log.md
```

## Phase別実装計画

### Phase 1: 基盤セットアップ 🏗️

**目標**: 開発環境とプロジェクト構造の準備

#### Server側 (Rust)
- [ ] `Cargo.toml`の依存関係設定
  - `serde` (JSONシリアライゼーション)
  - `serde_json` (JSON処理)
  - `tokio` (非同期ランタイム)
  - `anyhow` (エラーハンドリング)
- [ ] プロジェクト構造の作成
- [ ] 基本的な型定義 (`RpcRequest`, `RpcResponse`)

#### Client側 (TypeScript)
- [ ] `package.json`、`tsconfig.json`の設定
- [ ] 必要なパッケージのインストール
  - `@types/node`
  - `typescript`
- [ ] 基本的なTypeScript型定義

#### 共通
- [ ] Unix Domain Socketパスの決定 (`/tmp/rpc.sock`)
- [ ] エラーコードの標準化

**学習ポイント**:
- RustとTypeScriptプロジェクトのセットアップ
- serdeクレートによるJSON処理
- Unix Domain Socketの基礎概念

---

### Phase 2: Server側実装 (Rust) 🦀

**目標**: RPC サーバーの完全実装

#### 2.1 Socket Server基盤
- [ ] Unix Domain Socketサーバーの起動
- [ ] クライアント接続の受け入れ
- [ ] 接続ごとのハンドリング (tokio spawn)
- [ ] グレースフルシャットダウン

#### 2.2 メッセージ処理システム  
- [ ] JSONリクエストのパース
- [ ] メッセージバリデーション
- [ ] エラーハンドリングシステム
- [ ] レスポンス生成

#### 2.3 Method Dispatcher & Handler Registry
- [ ] メソッド名に基づく処理振り分け
- [ ] パラメータの型チェック  
- [ ] ハッシュマップベースのメソッドレジストリ
- [ ] 動的メソッド登録システム

#### 2.4 ビジネスロジック実装
- [ ] `floor` メソッド: `floor(double x)` - 10進数xを最も近い整数に切り捨て
- [ ] `nroot` メソッド: `nroot(int n, int x)` - 方程式 r^n = x におけるrの値を計算
- [ ] `reverse` メソッド: `reverse(string s)` - 文字列sの逆文字列を返す
- [ ] `validAnagram` メソッド: `validAnagram(string str1, string str2)` - 2つの文字列がアナグラムかを判定
- [ ] `sort` メソッド: `sort(string[] strArr)` - 文字列配列をソートして返す

**学習ポイント**:
- Tokioによる非同期プログラミング
- Rustの所有権システムとライフタイム
- エラー型の設計とanyhowクレート
- パターンマッチングとenum活用

---

### Phase 3: Client側実装 (TypeScript) 🔧

**目標**: 使いやすいRPCクライアントの実装

#### 3.1 Socket接続管理
- [ ] Unix Domain Socket接続
- [ ] 接続プール（再利用）
- [ ] 接続エラーハンドリング
- [ ] 自動リトライ機能

#### 3.2 RPC呼び出しインターフェース
- [ ] Promise/async-awaitベースのAPI
- [ ] リクエストID管理（UUID生成）
- [ ] タイムアウト処理
- [ ] 同時複数リクエストのサポート

#### 3.3 型安全なAPI
- [ ] メソッド別の型定義
- [ ] パラメータ型の検証
- [ ] レスポンス型の保証
- [ ] エラー型の定義

#### 3.4 ユーザビリティ
- [ ] メソッド呼び出し用ヘルパー関数
- [ ] ログ機能
- [ ] デバッグモード

**学習ポイント**:
- Node.jsのnetモジュール
- TypeScriptの高度な型システム
- Promise/async-awaitパターン
- エラーハンドリングのベストプラクティス

## 学習記録の方針

各Phase完了時に`learning-log.md`に記録:
- 実装で学んだ技術的なポイント
- 遭遇した問題とその解決方法
- 設計判断の理由と代替案の検討
- 次のPhaseに向けた課題

## 成功基準

### Phase 1完了基準
- [ ] 両プロジェクトのビルドが通る
- [ ] 基本的な型定義が完成
- [ ] 開発環境が整っている

### Phase 2完了基準  
- [ ] Rustサーバーが起動する
- [ ] 基本的なRPCメソッドが動作する
- [ ] エラーハンドリングが適切に動作する

### Phase 3完了基準
- [ ] TypeScriptクライアントから接続できる
- [ ] すべてのメソッドが型安全に呼び出せる
- [ ] エラーが適切にハンドリングされる

## 参考資料・学習リソース

### RPC関連
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [gRPC概念](https://grpc.io/docs/what-is-grpc/core-concepts/)

### Rust関連
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Serde Guide](https://serde.rs/)

### Unix Socket関連
- Unix Domain Socket プログラミングガイド
- ネットワークプログラミングの基礎

### TypeScript関連
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Node.js net module](https://nodejs.org/api/net.html)

---
