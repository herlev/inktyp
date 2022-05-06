use std::{
  io::Write,
  process::{Command, Stdio},
};

fn get_targets() -> Vec<String> {
  String::from_utf8(
    Command::new("xclip")
      .arg("-selection")
      .arg("clipboard")
      .arg("-target")
      .arg("TARGETS")
      .arg("-o")
      .output()
      .unwrap()
      .stdout,
  )
  .unwrap()
  .lines()
  .map(|s| s.to_string())
  .collect()
}

pub fn set_clipboard(data: &str, target: &str) {
  let mut p = Command::new("xclip")
    .arg("-selection")
    .arg("clipboard")
    .arg("-target")
    .arg(target)
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();
  p.stdin.take().unwrap().write_all(data.as_bytes()).unwrap();
  p.wait().unwrap();
}

pub fn get_clipboard(target: &str) -> String {
  assert!(get_targets().contains(&target.to_string()));
  String::from_utf8(
    Command::new("xclip")
      .arg("-selection")
      .arg("clipboard")
      .arg("-target")
      .arg(target)
      .arg("-o")
      .output()
      .unwrap()
      .stdout,
  )
  .unwrap()
}
