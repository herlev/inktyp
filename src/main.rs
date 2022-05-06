use eframe::egui;
use egui::Vec2;

mod latex;
mod pdf2svg;
mod xclip;
use regex::Regex;

// TODO: add error handling, instead of all the unwraps
// TODO: remap keys in inkscape

struct MyApp {
  latex: String,
}

impl MyApp {
  fn new(latex: String) -> Self {
    Self { latex }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.horizontal(|ui| {
        let input = ui.text_edit_singleline(&mut self.latex);
        if !input.has_focus() {
          input.request_focus();
        }
        if ui.input().key_pressed(egui::Key::Enter) && !self.latex.is_empty() {
          equation_to_svg(&self.latex).unwrap();
          _frame.quit();
        }
        if ui.input().key_pressed(egui::Key::Escape) {
          _frame.quit();
        }
      });
    });
  }
}

fn run_app() {
  let args = std::env::args();
  if args.len() != 2 {
    return;
  }

  let equation = match (&args.collect::<Vec<_>>()[1]).as_str() {
    "new" => "".into(),
    "edit" => load_equation(),
    _ => return,
  };

  let options = eframe::NativeOptions {
    always_on_top: true,
    resizable: false,
    decorated: false,
    initial_window_size: Some(Vec2 { x: 300., y: 35. }),
    ..Default::default()
  };

  eframe::run_native(
    "inktex",
    options,
    Box::new(|_cc| Box::new(MyApp::new(equation))),
  );
}

fn main() {
  run_app();
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
  let pdf = latex::compile(&tex).unwrap();
  let svg = pdf2svg::convert(&pdf).unwrap();
  let svg = pdf2svg::group_and_add_desc(&svg, &format!("latex: {equation}"));
  copy_svg_to_clipboard(&svg);
  Ok(svg)
}

pub fn copy_svg_to_clipboard(svg: &str) {
  xclip::set_clipboard(svg, "image/x-inkscape-svg");
}

pub fn load_equation() -> String {
  let clipboard = xclip::get_clipboard("image/x-inkscape-svg");
  let re = Regex::new(r">latex: (.*)</desc>").unwrap();
  re.captures(&clipboard)
    .unwrap()
    .get(1)
    .unwrap()
    .as_str()
    .to_string()
}
