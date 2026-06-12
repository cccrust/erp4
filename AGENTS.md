# AGENTS.md

## 專案概要

Rust + SQLite CLI ERP 系統。單一 crate，二進位名稱為 `erp4`。

## 開發指令

```bash
cargo build                          # 除錯版
cargo build --release                # 正式版 (target/release/erp4)
cargo test                           # 執行所有測試
cargo fmt                            # 格式化
cargo clippy                         # 靜態分析
cargo check                          # 型別檢查
```

無 CI、無 pre-commit hook、無 codegen、無 migration 系統。

## 環境變數

- `ERP4_DB` — SQLite 資料庫路徑（預設 `erp4.db`）

## 測試

- 26 個單元測試，分布於 `src/model/*.rs` 與 `src/cli/fmt.rs`
- 使用 In-memory SQLite，Schema 載入來自 `src/db.sql`（`include_str!`）
- 整合測試腳本：`case1.sh`（基礎流程）、`case2.sh`（v0.2 新功能）、`case3.sh`（v0.3 新功能），需手動執行
- 腳本使用 `ERP4=${ERP4:-cargo run --}`，可覆寫為已編譯二進位

## 架構重點

- **入口:** `src/main.rs` → 讀取 `ERP4_DB`、開啟 SQLite、分派 CLI 指令
- **CLI 層:** `src/cli/` (`clap` derive 模式)
- **模型層:** `src/model/` — CRUD + 業務邏輯 + 測試
- **資料庫初始化:** `src/db.rs` (`CREATE TABLE IF NOT EXISTS`)，9 張表

## 商務邏輯

- 庫存自動連動：`order confirmed` 扣庫、`order cancelled` 還原；`purchase-order received` 入庫、`purchase-order cancelled` 還原
- 狀態白名單：
  - Order: `pending`, `confirmed`, `shipped`, `delivered`, `cancelled`
  - PurchaseOrder: `pending`, `approved`, `received`, `cancelled`
  - Invoice: `unpaid`, `paid`, `overdue`, `cancelled`
- 發票自動編號格式：`INV-YYYY-NNNN`（`sequences` 表）

## 寫碼慣例

- **所有 CLI 輸出、註解為繁體中文**
- 錯誤處理使用 `anyhow::Result`
- `#![allow(dead_code, unused)]` 存在於 `src/main.rs` crate root
- 共用格式化工具位於 `src/cli/fmt.rs`（千分位、CSV、著色）
- `colored` crate 原生支援 `NO_COLOR` 環境變數
