use chrono::prelude::*;

use std::time::Duration;
use std::process::{Command, Stdio};
use std::thread;

fn set_root(name: String) {
     let res = Command::new("xsetroot")
        .stdout(Stdio::null())
        .arg("-name")
        .arg(name)
        .spawn();

    match res {
        Ok(mut child) => {
            let _ = child.wait();
        },
        Err(_) => {},
    };
}

fn get_formatted_time() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%H:%M:%S").to_string()
}

fn get_formatted_date() -> String {
    let current_date: DateTime<Local> = Local::now();
    current_date.format("%a, %B %d").to_string()
}

// static mut INDEX: usize = 0;
// fn marquee(text: String, width: u8) -> String {
//     let width: usize = width.into();
//     let offset = unsafe {
//         let offset = INDEX % text.len();
//         INDEX = INDEX + 1;
//         offset
//     };
//
//     let len = text.len();
//     let start = offset;
//     let end = (offset+width)%len;
//
//     if end < start {
//         let mut out_text = String::from(&text[start..len]);
//         out_text.push_str(&text[0..end]);
//         out_text
//     } else {
//         String::from(&text[start..end])
//     }
//
// }

fn main() {

    loop {
        let time = get_formatted_time();
        let date = get_formatted_date();

        let title = format!(
            "{} | {}",
            date,
            time
        );

        set_root(title);
        thread::sleep(Duration::from_millis(100))
    }
}
