#[derive(Debug, Clone, Copy)]
pub struct Key {
  pub physical_code: u8,
  pub keycode: Option<u8>,
  pub x: f32,
  pub y: f32,
}

impl Key {
  pub fn new(physical_code: u8, keycode: Option<u8>, x: f32, y: f32) -> Self {
    Self { physical_code, keycode, x, y }
  }
}
