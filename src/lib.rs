//交易日查询

use std::str::FromStr;

use chrono::NaiveDate;

// `include_str!` 是编译期宏：运行时无需读外部文件
const TRADING_DAYS: &str = include_str!(concat!(env!("OUT_DIR"), "/trading_days.txt"));


// 表示查询过程中可能发生的错误
// `#[derive(...)]` 自动实现常用 trait，例如 Debug（打印调试信息）、Clone（克隆）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    InvalidDate(String),
    NoPreviousTradingDay(NaiveDate),
    NoNextTradingDay(NaiveDate),
}

// `impl` 为类型实现方法或 trait
// `Display` trait：让错误可以用 `{}` 格式化输出
impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `match` 模式匹配：根据 enum 变体分别处理
        match self {
            Self::InvalidDate(raw) => write!(f, "invalid date: {raw}"),
            Self::NoPreviousTradingDay(date) => {
                write!(f, "no previous trading day before {date}")
            }
            Self::NoNextTradingDay(date) => write!(f, "no next trading day after {date}"),
        }
    }
}

// 实现标准库的 `Error` trait，表明这是一个错误类型
impl std::error::Error for QueryError {}

// `struct`：结构体，把相关数据组合在一起
// 这里用有序日期列表表示 2004~2026 全部交易日
#[derive(Debug, Clone)]
pub struct TradingCalendar {
    days: Vec<NaiveDate>, // `Vec`：动态数组，类似其他语言的 List
}

impl TradingCalendar {
    /// 从编译期嵌入的 tradeday/*.txt 加载日历
    pub fn from_embedded() -> Self {
        Self::from_text(TRADING_DAYS)
    }

    /// 从文本解析交易日列表
    ///
    /// `&str` 表示借用字符串，不取得所有权（调用方仍拥有原字符串）
    pub fn from_text(text: &str) -> Self {
        // 链式迭代器：逐行处理文本
        let mut days = text
            .lines() // 按行迭代
            .map(str::trim) // 去掉首尾空白
            .filter(|line| !line.is_empty() && !line.starts_with('#')) // 跳过空行和注释
            .filter_map(|line| NaiveDate::parse_from_str(line, "%Y-%m-%d").ok()) // 解析失败则丢弃
            .collect::<Vec<_>>(); // 收集成 Vec

        days.sort_unstable(); // 排序，为二分查找做准备
        days.dedup(); // 去重

        // `Self { days }` 是结构体字面量，创建 TradingCalendar 实例
        Self { days }
    }

    /// 判断某天是否为交易日
    ///
    /// `&self` 表示不可变借用自身，类似其他语言的 `this` / `self`
    pub fn is_trading_day(&self, date: NaiveDate) -> bool {
        // 二分查找：在有序数组中 O(log n) 查找
        // `.is_ok()` 把 Result 转成 bool（找到=true，没找到=false）
        self.days.binary_search(&date).is_ok()
    }

    /// 返回严格意义上的「上一个交易日」（不含当天）
    ///
    /// 返回类型 `Result<T, E>`：
    /// - `Ok(value)` 表示成功
    /// - `Err(error)` 表示失败（例如已经是最早的交易日）
    pub fn previous_trading_day(&self, date: NaiveDate) -> Result<NaiveDate, QueryError> {
        let index = match self.days.binary_search(&date) {
            Ok(found) => found,       // 当天是交易日：位置就是 found
            Err(insert_at) => insert_at, // 当天不是交易日：insert_at 是第一个更大的位置
        };

        if index == 0 {
            // 前面没有更早的交易日了
            return Err(QueryError::NoPreviousTradingDay(date));
        }

        Ok(self.days[index - 1])
    }

    /// 返回严格意义上的「下一个交易日」（不含当天）
    pub fn next_trading_day(&self, date: NaiveDate) -> Result<NaiveDate, QueryError> {
        let index = match self.days.binary_search(&date) {
            Ok(found) => found + 1, // 当天是交易日：从下一天开始找
            Err(insert_at) => insert_at, // 当天不是交易日：insert_at 就是下一个交易日
        };

        // `.get(index)` 安全取下标，越界返回 None
        // `.copied()` 把 `Option<&NaiveDate>` 变成 `Option<NaiveDate>`
        // `.ok_or(...)` 把 None 转成 Err
        self.days
            .get(index)
            .copied()
            .ok_or(QueryError::NoNextTradingDay(date))
    }

    /// 把 "YYYY-MM-DD" 字符串解析成日期
    pub fn parse_date(raw: &str) -> Result<NaiveDate, QueryError> {
        // `map_err`：解析失败时把错误映射成我们的 QueryError
        NaiveDate::from_str(raw).map_err(|_| QueryError::InvalidDate(raw.to_owned()))
        // `to_owned()` 把 &str 复制成 String（取得所有权）
    }
}

// `#[cfg(test)]`：只在 `cargo test` 时编译这段代码，不会进入正式二进制
#[cfg(test)]
mod tests {
    use super::*; // 引入父模块的所有公开项

    #[test]
    fn detects_trading_day() {
        let calendar = TradingCalendar::from_embedded();
        // `assert!`：条件必须为 true，否则测试失败
        assert!(calendar.is_trading_day(NaiveDate::from_ymd_opt(2004, 1, 2).unwrap()));
        assert!(calendar.is_trading_day(NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()));
        assert!(!calendar.is_trading_day(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()));
    }

    #[test]
    fn finds_neighbors_around_weekend() {
        let calendar = TradingCalendar::from_embedded();
        let saturday = NaiveDate::from_ymd_opt(2026, 1, 10).unwrap();

        assert_eq!(
            calendar.previous_trading_day(saturday).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 9).unwrap()
        );
        assert_eq!(
            calendar.next_trading_day(saturday).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 12).unwrap()
        );
    }

    #[test]
    fn finds_neighbors_on_trading_day() {
        let calendar = TradingCalendar::from_embedded();
        let tuesday = NaiveDate::from_ymd_opt(2026, 1, 6).unwrap();

        assert_eq!(
            calendar.previous_trading_day(tuesday).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            calendar.next_trading_day(tuesday).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()
        );
    }
}
