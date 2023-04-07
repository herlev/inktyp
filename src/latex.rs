use std::{fs, io::Write, process::Command};

use crate::svg;

// Parse pdflatex stderr (pdflatex -interaction nonstopmode -file-line-error main.tex)
// search for lines with ".*:[0-9]+: .*"
// and 3 lines below, unless pattern occurs again
pub fn compile(tex: &str) -> Result<Vec<u8>, String> {
  let dir = tempfile::tempdir().expect("Couldn't create temporary directory");
  let file_path = dir.path().join("main.tex");
  {
    let mut f = fs::File::create(&file_path).unwrap_or_else(|_| panic!("Couldn't create {file_path:#?}"));
    f.write_all(tex.as_bytes()).unwrap();
  }
  let output = Command::new("pdflatex")
    .arg("-interaction")
    .arg("nonstopmode")
    .arg("-file-line-error")
    .arg("main.tex")
    .current_dir(&dir)
    .output()
    .expect("Failed to execute pdflatex");
  if output.status.success() {
    Ok(fs::read(dir.path().join("main.pdf")).unwrap())
  } else {
    Err(String::from_utf8(output.stdout).unwrap())
  }
}

pub fn equation_to_svg(equation: &str) -> Result<String, ()> {
  let template = (
    r"
  \documentclass[12pt,border=12pt]{standalone}
  \usepackage[utf8]{inputenc}
  \usepackage[T1]{fontenc}
  \usepackage{textcomp}
  \usepackage{amsmath, bm, amssymb}
  \newcommand{\R}{\mathbb R}
  \begin{document}
  $",
    r"$
  \end{document}",
  );
  let tex = format!("{}{}{}", template.0, equation, template.1);
  let pdf = compile(&tex).map_err(|_| ())?;
  let svg = svg::pdf2svg(&pdf).unwrap();
  Ok(svg)
}
