use std::sync::mpsc::{self, RecvError};

pub struct Receiver<T> {
  rx: mpsc::Receiver<T>,
}

impl<T> Receiver<T> {
  pub fn new(rx: mpsc::Receiver<T>) -> Self {
    Self { rx }
  }
  // pub fn latest(&self) -> Option<T> {
  //   let mut val = None;
  //   while let Ok(v) = self.rx.try_recv() {
  //     val = Some(v);
  //   }
  //   val
  // }
  pub fn latest_blocking(&self) -> Result<T, RecvError> {
    let mut val = None;
    while let Ok(v) = self.rx.try_recv() {
      val = Some(v);
    }
    match val {
      Some(v) => Ok(v),
      None => self.rx.recv(),
    }
  }
}
