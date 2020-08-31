// #![windows_subsystem = "windows"]
use clap::{Arg, App};

use enigo::*;

use std::time::Duration;
use std::thread;

fn main() {
    let matches = App::new("防止vdi进入睡眠模式")
        .version("1.0")
        .author("wuyongsheng <wuyongsheng@qq.com>")
        .about("通过不停移动鼠标的方式，防止vdi进入睡眠模式")
        .arg(Arg::with_name("span")
            .short("s")
            .takes_value(true)
            .help("鼠标直线移动距离"))
        .arg(Arg::with_name("interval")
            .short("i")
            .takes_value(true)
            .help("多少秒执行一次"))
        .get_matches();

    let mut span: i32 = matches.value_of("span").unwrap_or("200").parse::<i32>().unwrap();
    let interval: u64 = matches.value_of("interval").unwrap_or("5").parse::<u64>().unwrap();

    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 500);

    loop {
        thread::sleep(Duration::from_secs(interval));
        span *= -1;
        enigo.mouse_move_relative(span, 0);
    }
}
