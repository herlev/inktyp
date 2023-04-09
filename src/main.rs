use crate::app::InkTyp;
use eframe::egui;
use egui::Vec2;

mod app;
mod svg;
mod typst_math;
mod xml;

fn run_app() {
  let mut args = std::env::args();
  if args.len() < 2 {
    return;
  }

  args.next();
  let equation = match args.next().unwrap().as_str() {
    "new" => "".into(),
    "edit" => args.next().expect("edit must take an argument"),
    _ => return,
  };

  let options = eframe::NativeOptions {
    always_on_top: true,
    decorated: false,
    initial_window_size: Some(Vec2 {
      x: app::WINDOW_SIZE as f32,
      y: app::WINDOW_SIZE as f32,
    }),
    resizable: false,
    ..Default::default()
  };

  eframe::run_native("inktyp", options, Box::new(|cc| Box::new(InkTyp::new(cc, equation)))).unwrap();
}

fn main() {
  run_app();
}
