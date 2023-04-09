use comemo::Prehashed;
use egui_extras::RetainedImage;
use typst::{
  doc::Document,
  eval::Library,
  font::{Font, FontBook},
  geom::RgbaColor,
  syntax::Source,
  util::Buffer,
  World,
};

use crate::svg::pdf2svg;

struct MyWorld {
  source: Source,
  library: Prehashed<Library>,
  book: Prehashed<FontBook>,
  fonts: Vec<Font>,
}

impl World for MyWorld {
  fn library(&self) -> &Prehashed<typst::eval::Library> {
    &self.library
  }

  fn main(&self) -> &typst::syntax::Source {
    &self.source
  }

  fn resolve(&self, _path: &std::path::Path) -> typst::diag::FileResult<typst::syntax::SourceId> {
    todo!()
  }

  fn source(&self, _id: typst::syntax::SourceId) -> &typst::syntax::Source {
    todo!()
  }

  fn book(&self) -> &Prehashed<typst::font::FontBook> {
    &self.book
  }

  fn font(&self, id: usize) -> Option<typst::font::Font> {
    Some(self.fonts[id].clone())
  }

  fn file(&self, _path: &std::path::Path) -> typst::diag::FileResult<typst::util::Buffer> {
    todo!()
  }
}

pub struct TypstMath {
  world: MyWorld,
}

fn source_from_equation(eq: &str) -> Source {
  let preamble = "#set page(width: auto, height: auto, margin: 10pt)";
  let source = format!("{preamble}\n$ {eq} $");
  Source::detached(source)
}

fn document_to_png(document: Document, max_width_or_height: f64) -> RetainedImage {
  let frame = &document.pages[0];
  let size = frame.size();
  let pixels_per_pt = max_width_or_height / size.x.to_pt().max(size.y.to_pt());
  let pixmap = typst::export::render(frame, pixels_per_pt as f32, RgbaColor::new(0, 0, 0, 0).into());
  let img =
    egui::ColorImage::from_rgba_unmultiplied([pixmap.width() as usize, pixmap.height() as usize], pixmap.data());
  RetainedImage::from_color_image("", img)
}

fn document_to_svg(document: Document) -> Vec<u8> {
  let pdf = typst::export::pdf(&document);
  pdf2svg(&pdf).unwrap().into()
}

impl TypstMath {
  pub fn new() -> Self {
    let bytes = include_bytes!("../fonts/NewCMMath-Regular.otf");
    let buffer = Buffer::from_static(bytes);
    let fonts = vec![Font::new(buffer, 0).unwrap()];
    Self {
      world: MyWorld {
        source: source_from_equation(""),
        library: Prehashed::new(typst_library::build()),
        book: Prehashed::new(FontBook::from_fonts(&fonts)),
        fonts,
      },
    }
  }
  pub fn equation_to_svg(&mut self, eq: &str) -> Result<Vec<u8>, String> {
    self.world.source = source_from_equation(eq);
    match typst::compile(&self.world) {
      Ok(document) => Ok(document_to_svg(document)),
      Err(errors) => {
        // dbg!(_errors);
        Err(format!(
          "{:?}",
          errors.iter().map(|e| e.message.to_string()).collect::<Vec<_>>()
        ))
      }
    }
  }
  pub fn equation_to_png(&mut self, eq: &str, max_width_or_height: f64) -> Option<RetainedImage> {
    self.world.source = source_from_equation(eq);
    match typst::compile(&self.world) {
      Ok(document) => Some(document_to_png(document, max_width_or_height)),
      Err(_errors) => {
        // dbg!(errors);
        None
      }
    }
  }
}
