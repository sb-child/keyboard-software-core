pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
  ((r as u32) << 16) | ((g as u32) << 24) | ((b as u32) << 8)
}

pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> u32 {
  (((r as f32 * a) as u32) << 16)
    | (((g as f32 * a) as u32) << 24)
    | (((b as f32 * a) as u32) << 8)
}
