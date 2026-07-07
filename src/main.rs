// 程序入口：解析命令行参数，调用 lib.rs 里的逻辑

use std::process::ExitCode;

use clap::Parser;
use tradeday::TradingCalendar; // 使用 crate 名引用 lib.rs 中的类型

// `#[derive(Parser)]`：clap 自动生成命令行解析代码
#[derive(Parser)]
#[command(
    name = "tradeday",
    about = "Query China A-share trading days",
    version = concat!(
        env!("CARGO_PKG_VERSION"),
        " (data: ",
        env!("DATA_FIRST"),
        " .. ",
        env!("DATA_LAST"),
        ")"
    )
)]
struct Cli {
    /// 基准日期，格式 YYYY-MM-DD（必填）
    #[arg(long, value_name = "YYYY-MM-DD", required = true)]
    date: String,

    /// 返回上一个交易日（与 --next 互斥，需搭配 --date）
    #[arg(long, conflicts_with = "next")]
    previous: bool,

    /// 返回下一个交易日（与 --previous 互斥，需搭配 --date）
    #[arg(long, conflicts_with = "previous")]
    next: bool,
}

// `main` 可以返回 `ExitCode`，让进程以指定退出码结束
// 0=成功，1=业务否定（非交易日），2=参数/运行错误
fn main() -> ExitCode {
    let cli = Cli::parse(); // clap 自动解析 std::env::args()
    let calendar = TradingCalendar::from_embedded();

    match run(&calendar, cli) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("error: {error}"); // 错误信息输出到 stderr
            ExitCode::from(2)
        }
    }
}

// 把具体逻辑拆到单独函数，便于阅读和测试
// `&TradingCalendar`：借用 calendar，不转移所有权
fn run(calendar: &TradingCalendar, cli: Cli) -> Result<ExitCode, tradeday::QueryError> {
    let date = TradingCalendar::parse_date(&cli.date)?;

    if cli.previous {
        let previous = calendar.previous_trading_day(date)?;
        println!("{previous}");
        return Ok(ExitCode::SUCCESS);
    }

    if cli.next {
        let next = calendar.next_trading_day(date)?;
        println!("{next}");
        return Ok(ExitCode::SUCCESS);
    }

    // 仅 --date：判断是否为交易日
    let is_trading_day = calendar.is_trading_day(date);
    println!("{}", is_trading_day);
    Ok(if is_trading_day {
        ExitCode::SUCCESS // 0
    } else {
        ExitCode::from(1) // 1：方便 shell 脚本判断
    })
}
