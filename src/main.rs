use std::env;
use std::process::exit;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = if args.len() > 1 { &args[1] } else { "succeed" };
    let seconds = if args.len() > 2 { args[2].parse::<u64>().unwrap_or(10) } else { 10 };

    for i in 0..seconds {
        println!("已经运行：{} 秒", i);
        sleep(Duration::from_secs(1));
    }

    match result.as_ref() {
        "succeed" => println!("运行成功！"),
        "failed" => {
            eprintln!("运行失败！");
            exit(1);
        },
        "oom" => {
            // 尝试分配大量内存，但 Rust 的内存管理通常不会导致 OOM
            let memory = vec![0; 1_000_000_000];
            println!("{:?}", memory);
        },
        "loop" => loop {
            // 卡死循环
        },
        _ => println!("未知的运行结果"),
    }
}