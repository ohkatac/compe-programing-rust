use std::io;
  
fn read_value() -> String {
  let mut buf = String::new();
  io::stdin().read_line(&mut buf).ok();
  let n: &str = buf.trim();
  return n.to_string();
}
fn read_values() -> Vec<String> {
  let mut buf = String::new();
  io::stdin().read_line(&mut buf).ok();
  let line: Vec<String> = buf
    .trim()
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();
  return line;
}
