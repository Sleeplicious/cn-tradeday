# tradeday

查询中国 A 股交易日。数据范围：2004–2026，已嵌入二进制，无需外部文件。

## 用法

```bash
# 判断是否为交易日
tradeday --date 2026-01-05          # 输出 true / false

# 上一个交易日（不含当天）
tradeday --date 2026-01-10 --previous   # 2026-01-09

# 下一个交易日（不含当天）
tradeday --date 2026-01-10 --next       # 2026-01-12
```

## 退出码

| 码 | `--date` | `--previous` / `--next` |
|----|----------|-------------------------|
| 0 | 是交易日 | 成功，已输出日期 |
| 1 | 非交易日 | —（不会出现） |
| 2 | 参数错误、日期格式无效 | 找不到上/下一个交易日、参数错误 |

`--previous` / `--next` 只有 **0（成功）** 和 **2（失败）**，没有 1。

## 预编译产物

`dist/` 目录：

- `tradeday.exe` — Windows
- `tradeday-linux` — Linux x86_64

Linux 首次使用：

```bash
chmod +x tradeday-linux
./tradeday-linux --date 2026-01-05
```

## 编译

```bash
cargo build-win      # Windows
cargo build-linux    # Linux（需 Zig + cargo-zigbuild）
```
