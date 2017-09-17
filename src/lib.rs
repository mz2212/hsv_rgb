#![crate_type = "lib"]
#![crate_name = "hsv_rgb"]

pub struct RGBColor {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

pub struct HSVColor {
	pub h: u8,
	pub s: u8,
	pub v: u8,
}

impl HSVColor {
	pub fn new(h: u8, s: u8, v: u8) -> HSVColor {
		HSVColor {
			h: h,
			s: s,
			v: v,
		}
	}
	pub fn to_rgb(&self) -> RGBColor {
		let mut rgb = RGBColor::new(0, 0, 0);

		if self.s == 0 {
			rgb.r = self.v;
			rgb.g = self.v;
			rgb.b = self.v;
			return rgb
		}

		let region = self.h / 43;
		let remainder = (self.h - (region * 43)) * 6;

		let p: u32 = (self.v as u32 * (255 - self.s as u32)) >> 8;
		let q: u32 = (self.v as u32 * (255 - ((self.s as u32 * remainder as u32) >> 8))) >> 8;
		let t: u32 = (self.v as u32 * (255 - ((self.s as u32 * (255 - remainder as u32)) >> 8))) >> 8;

		match region {
			0 => {
				rgb.r = self.v;
				rgb.g = t as u8;
				rgb.b = p as u8;
			},
			1 => {
				rgb.r = q as u8;
				rgb.g = self.v;
				rgb.b = p as u8;
			},
			2 => {
				rgb.r = p as u8;
				rgb.g = self.v;
				rgb.b = t as u8;
			},
			3 => {
				rgb.r = p as u8;
				rgb.g = q as u8;
				rgb.b = self.v;
			},
			4 => {
				rgb.r = t as u8;
				rgb.g = p as u8;
				rgb.b = self.v;
			}
			_ => {
				rgb.r = self.v;
				rgb.g = p as u8;
				rgb.b = q as u8;
			},
		}
		rgb
	}
}

impl RGBColor {
	pub fn new(r: u8, g: u8, b: u8) -> RGBColor {
		RGBColor {
			r: r,
			g: g,
			b: b,
		}
	}
}