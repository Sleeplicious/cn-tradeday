// build.rs 会把 tradeday/*.txt 合并后嵌入二进制，并导出数据起止日期

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let tradeday_dir = Path::new("tradeday");
    println!("cargo:rerun-if-changed=tradeday");

    let mut files: Vec<_> = fs::read_dir(tradeday_dir)
        .expect("tradeday directory not found")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
        .collect();
    files.sort();

    let mut combined = String::new();
    let mut dates = Vec::new();

    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let text = fs::read_to_string(path).expect("read tradeday file");
        for line in text.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            dates.push(line.to_owned());
        }
        combined.push_str(&text);
        if !combined.ends_with('\n') {
            combined.push('\n');
        }
    }

    dates.sort();
    dates.dedup();

    let (first, last) = dates
        .first()
        .zip(dates.last())
        .expect("tradeday data must contain at least one date");

    println!("cargo:rustc-env=DATA_FIRST={first}");
    println!("cargo:rustc-env=DATA_LAST={last}");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    fs::write(Path::new(&out_dir).join("trading_days.txt"), combined).expect("write trading_days.txt");
}
