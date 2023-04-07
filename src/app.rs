use std::{sync::mpsc, thread};

use crate::{latex, mpsc_receiver::Receiver, svg};

pub struct MyApp {
  latex: String,
  svg_image: egui_extras::RetainedImage,
  eq_sender: mpsc::Sender<String>,
  svg_receiver: mpsc::Receiver<String>,
}

const MINIMAL_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";

impl MyApp {
  pub fn new(cc: &eframe::CreationContext, latex: String) -> Self {
    let (tx, rx) = mpsc::channel();
    let rx = Receiver::<String>::new(rx);
    let (tx2, rx2) = mpsc::channel();
    let ctx = cc.egui_ctx.clone();
    ctx.set_visuals(egui::Visuals::light());
    thread::spawn(move || {
      while let Ok(eq) = rx.latest_blocking() {
        if let Ok(svg) = latex::equation_to_svg(&eq) {
          tx2.send(svg).unwrap();
          ctx.request_repaint();
        }
      }
    });
    tx.send(latex.clone()).unwrap();
    Self {
      eq_sender: tx,
      svg_receiver: rx2,
      latex,
      svg_image: egui_extras::image::RetainedImage::from_svg_str("", MINIMAL_SVG).unwrap(),
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.horizontal(|ui| {
        let input = ui.text_edit_singleline(&mut self.latex);
        if !input.has_focus() {
          input.request_focus();
        }

        if input.changed {
          self.eq_sender.send(self.latex.clone()).unwrap();
        }

        if let Ok(svg) = self.svg_receiver.try_recv() {
          self.svg_image = egui_extras::image::RetainedImage::from_svg_bytes_with_size(
            "test",
            svg.as_bytes(),
            egui_extras::image::FitTo::Width(300),
          )
          .unwrap();
        }
        if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !self.latex.is_empty() {
          let svg = latex::equation_to_svg(&self.latex).unwrap();
          let svg = svg::group_and_add_desc(&svg, &format!("latex: {}", self.latex));
          print!("{svg}");
          frame.close();
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
          frame.close();
          std::process::exit(1);
        }
      });
      self.svg_image.show(ui);
    });
  }
}
