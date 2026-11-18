# tradeday

查询中国 A 股交易日。数据范围：1990–2026，已嵌入二进制，无需外部文件。

## 数据维护

- `holidays.txt`：编译期唯一数据源（法定放假日期）
- `tradeday/{year}.txt`：由 `build.rs` 根据节假日算出的交易日列表，可直接供其他程序内嵌数据源使用

每年国务院放假安排公布后，在 `holidays.txt` 追加对应年份，更新嵌入数据与 `tradeday/`。

## 用法

```bash
# 判断是否为交易日
tradeday --date 2026-01-05          # 输出 true / false

# 上一个交易日（不含当天，默认 T-1）
tradeday --date 2026-01-10 --previous      # 2026-01-09
tradeday --date 2026-01-10 --previous -n 2 # T-2 → 2026-01-08

# 下一个交易日（不含当天，默认 T+1）
tradeday --date 2026-01-10 --next          # 2026-01-12
tradeday --date 2026-01-10 --next -n 2     # T+2 → 2026-01-13

# 查看版本与数据范围
tradeday --version                      # tradeday 0.1.0 (data: 1990-12-19 .. 2026-12-31)
```

## 退出码

| 码 | `--date` | `--previous` / `--next`（可选 `-n`） |
|----|----------|--------------------------------------|
| 0 | 是交易日 | 成功，已输出日期 |
| 1 | 非交易日 | — |
| 2 | 参数错误、日期格式无效 | 找不到目标交易日、`n` 无效、参数错误 |

`--previous` / `--next` 只有 **0（成功）** 和 **2（失败）**，没有 1。`-n` 默认 1。

## 预编译产物

`dist/` 目录：

- `tradeday.exe` — Windows
- `tradeday-linux` — Linux x86_64

Linux 首次使用：

```bash
chmod +x tradeday-linux
./tradeday-linux --date 2026-01-05
```

## 编译 / 打包

```bash
# 仅编译（产物在 target/）
cargo build-win      # → target/release/tradeday.exe
cargo build-linux    # → target/x86_64-unknown-linux-musl/release/tradeday（需 Zig + cargo-zigbuild）
```
