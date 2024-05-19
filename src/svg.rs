use crate::xml;

// This function assummes the svg is generated using pdf2svg
pub fn add_description(svg: &str, desc: &str) -> String {
  let desc = xml::escape(desc);
  let template = format!("<desc>{desc}</desc>");
  let mut lines: Vec<_> = svg.lines().collect();
  lines.insert(lines.len() - 1, &template);
  lines.join("\n")
}
