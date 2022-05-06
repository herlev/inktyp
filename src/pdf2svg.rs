use std::process::Command;

pub fn convert(pdf: &Vec<u8>) -> Result<String, ()> {
  let dir = tempfile::tempdir().expect("Couldn't create temporary directory");
  std::fs::write(dir.path().join("main.pdf"), pdf).expect("Couldn't write pdf file");
  let status = Command::new("pdf2svg")
    .arg("main.pdf")
    .arg("main.svg")
    .current_dir(&dir)
    .status()
    .unwrap();
  if status.success() {
    Ok(std::fs::read_to_string(dir.path().join("main.svg")).expect("Couldn't read svg file"))
  } else {
    Err(())
  }
}
// This function assummes the svg is generated using pdf2svg
pub fn group_and_add_desc(svg: &str, desc: &str) -> String {
  // TODO: escape xml characters in equation
  // https://stackoverflow.com/questions/1091945/what-characters-do-i-need-to-escape-in-xml-documents
  let template = ("<g><g>", format!("<desc>{desc}</desc></g></g>"));
  let mut lines: Vec<_> = svg.lines().collect();
  let i = lines.iter().position(|&l| l == "</defs>").unwrap();
  lines.insert(i + 1, template.0);
  lines.insert(lines.len() - 1, &template.1);
  lines.join("\n")
}
