pub fn escape(s: &str) -> String {
  s.replace('&', "&amp;")
    .replace('>', "&gt;")
    .replace('<', "&lt;")
    .replace('\'', "&apos;")
    .replace('"', "&quot;")
}
