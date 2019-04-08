use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CoordsF {
	pub x: f32,
	pub y: f32,
}

#[derive(Deserialize)]
pub struct Size {
	pub w: u32,
	pub h: u32,
}

#[derive(Deserialize)]
pub struct Rect {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

#[derive(Deserialize)]
pub struct Frame {
	pub filename: String,
	pub frane: Rect,
	pub rotated: bool,
	pub trimmed: bool,
	pub spriteSourceSize: Rect,
	pub sourceSize: Size,
	pub pivot: CoordsF,
}

#[derive(Deserialize)]
pub struct Meta {
	pub app: String,
	pub version: String,
	pub image: String,
	pub format: String,
	pub size: Size,
	pub scale: String,
	pub smartupdate: String,
}

#[derive(Deserialize)]
pub struct TexturePacker {
	pub frames: Vec<Frame>,
	meta: Meta
}