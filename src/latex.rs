use std::{fs, io::Write, process::Command};

// Parse pdflatex stderr (pdflatex -interaction nonstopmode -file-line-error main.tex)
// search for lines with ".*:[0-9]+: .*"
// and 3 lines below, unless pattern occurs again
pub fn compile(tex: &str) -> Result<Vec<u8>, ()> {
  let dir = tempfile::tempdir().expect("Couldn't create temporary directory");
  let file_path = dir.path().join("main.tex");
  {
    let mut f =
      fs::File::create(&file_path).unwrap_or_else(|_| panic!("Couldn't create {file_path:#?}"));
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
    Err(())
  }
}
