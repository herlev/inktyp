use crate::app::MyApp;
use eframe::egui;
use egui::Vec2;

mod app;
mod clipboard;
mod latex;
mod mpsc_receiver;
mod svg;

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
    resizable: false,
    decorated: false,
    initial_window_size: Some(Vec2 { x: 300., y: 300. }),
    ..Default::default()
  };

  eframe::run_native("inktex", options, Box::new(|cc| Box::new(MyApp::new(cc, equation)))).unwrap();
}

fn main() {
  run_app();
}
