use egui::load::SizedTexture;
use egui::ImageSource::Texture;
use egui::TextureHandle;
use egui::TextureOptions;

use crate::svg;
use crate::typst_math::TypstMath;

pub struct InkTyp {
  equation: String,
  texture_handle: TextureHandle,
  tm: TypstMath,
}

pub const WINDOW_SIZE: f64 = 300.;

impl InkTyp {
  pub fn new(cc: &eframe::CreationContext, equation: String) -> Self {
    cc.egui_ctx.set_visuals(egui::Visuals::light());
    let mut tm = TypstMath::new();
    let image = tm
      .equation_to_png(&equation, WINDOW_SIZE)
      .unwrap_or(egui::ColorImage::from_rgba_unmultiplied([0, 0], &[]));
    let texture_handle = cc.egui_ctx.load_texture("", image, TextureOptions::default());

    Self {
      equation,
      texture_handle,
      tm,
    }
  }
  fn update_img(&mut self) {
    if let Some(img) = self.tm.equation_to_png(&self.equation, WINDOW_SIZE) {
      self.texture_handle.set(img, TextureOptions::default());
    }
  }
}

impl eframe::App for InkTyp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.horizontal(|ui| {
        let input = ui.text_edit_singleline(&mut self.equation);
        if !input.has_focus() {
          input.request_focus();
        }

        if input.changed {
          self.update_img();
        }

        if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !self.equation.is_empty() {
          let svg = String::from_utf8(self.tm.equation_to_svg(&self.equation).unwrap()).unwrap();
          let svg = svg::add_description(&svg, &format!("typst: {}", self.equation));
          print!("{svg}");
          std::process::exit(0);
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
          std::process::exit(1);
        }
      });
      ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        ui.image(Texture(SizedTexture::from_handle(&self.texture_handle)));
      })
    });
  }
}
