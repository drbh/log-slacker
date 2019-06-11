extern crate logwatcher;
extern crate slack_hook;

use logwatcher::LogWatcher;
use regex::Regex;
use slack_hook::{PayloadBuilder, Slack};

fn send_message(mess: String) {
    let slack = Slack::new("https://hooks.slack.com/services/").unwrap();
    let p = PayloadBuilder::new().text(mess).build().unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}", x),
    }
}

fn line_handlers(line: String) {
    println!("Line {}", line);
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if re.is_match(&line) {
        send_message(line);
    }
}

fn main() {
    let mut log_watcher = LogWatcher::register("./myfile".to_string()).unwrap();
    log_watcher.watch(line_handlers)
}
