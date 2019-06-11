# log-slacker

```
git clone https://github.com/drbh/log-slacker.git
cd log-slacker
```

### update to your url, file and regex in `src/main.rs`


add your webhook url
```rust
let slack = Slack::new("https://hooks.slack.com/services/").unwrap();
```


set your regex pattern
```rust
let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
```

update the file

```rust
let mut log_watcher = LogWatcher::register("./myfile".to_string()).unwrap();
```

```
cargo build --release
```


then run it!
```bash
 ./target/release/log-slacker 
```


Example output
```txt
Line 
Line example
Line 
Line 2019-06-17
ok
```

a Line is printed for each line that is added to the file, and `ok` means a slack message was sent to the url