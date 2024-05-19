use comemo::Prehashed;
use egui_extras::RetainedImage;
use typst::foundations::Bytes;
use typst::model::Document;
use typst::text::{Font, FontBook};
use typst::visualize::Color;
use typst::{
  diag::FileResult,
  eval::Tracer,
  foundations::Datetime,
  syntax::{FileId, Source},
  Library, World,
};

struct MyWorld {
  source: Source,
  library: Prehashed<Library>,
  book: Prehashed<FontBook>,
  fonts: Vec<Font>,
}

impl World for MyWorld {
  fn main(&self) -> typst::syntax::Source {
    self.source.clone()
  }

  fn book(&self) -> &Prehashed<FontBook> {
    &self.book
  }

  fn font(&self, id: usize) -> Option<Font> {
    Some(self.fonts[id].clone())
  }

  fn library(&self) -> &Prehashed<Library> {
    &self.library
  }

  fn source(&self, _id: FileId) -> FileResult<Source> {
    todo!()
  }
  fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
    todo!()
  }

  fn file(&self, _id: FileId) -> FileResult<Bytes> {
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
  let frame = &document.pages[0].frame;
  let size = frame.size();
  let pixels_per_pt = max_width_or_height / size.x.to_pt().max(size.y.to_pt());
  let pixmap = typst_render::render(frame, pixels_per_pt as f32, Color::BLACK.with_alpha(0.));
  let img =
    egui::ColorImage::from_rgba_unmultiplied([pixmap.width() as usize, pixmap.height() as usize], pixmap.data());
  RetainedImage::from_color_image("", img)
}

fn document_to_svg(document: Document) -> Vec<u8> {
  typst_svg::svg(&document.pages[0].frame).into_bytes()
}

impl TypstMath {
  pub fn new() -> Self {
    let bytes = include_bytes!("../fonts/NewCMMath-Regular.otf");
    let buffer = Bytes::from_static(bytes);

    let fonts = vec![Font::new(buffer, 0).unwrap()];
    Self {
      world: MyWorld {
        source: source_from_equation(""),
        library: Prehashed::new(Library::default()),
        book: Prehashed::new(FontBook::from_fonts(&fonts)),
        fonts,
      },
    }
  }
  pub fn equation_to_svg(&mut self, eq: &str) -> Result<Vec<u8>, String> {
    self.world.source = source_from_equation(eq);
    let mut tracer = Tracer::default();
    match typst::compile(&self.world, &mut tracer) {
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
    let mut tracer = Tracer::default();
    match typst::compile(&self.world, &mut tracer) {
      Ok(document) => Some(document_to_png(document, max_width_or_height)),
      Err(_errors) => {
        // dbg!(errors);
        None
      }
    }
  }
}
