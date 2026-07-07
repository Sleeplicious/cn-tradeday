// build.rs 会把 tradeday/*.txt（2004~2026）合并后嵌入二进制

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
    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        combined.push_str(&fs::read_to_string(path).expect("read tradeday file"));
        if !combined.ends_with('\n') {
            combined.push('\n');
        }
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    fs::write(Path::new(&out_dir).join("trading_days.txt"), combined).expect("write trading_days.txt");
}
