use super::color::Color;

pub struct PixelBuffer {
    buff: Vec<u32>, // xrgb
    width: usize,
    height: usize,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, buff: vec![0; width * height] }
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm_out = String::with_capacity(self.buff.len() * 12 + 24);
        let width = self.width;
        let height = self.height;
        ppm_out.push_str(format!("P3\n{width} {height}\n255\n").as_str());

        for j in (0..height).rev() {
            for i in 0..width {
                let pixel = self.at(i, j);
                let r = (*pixel & 0xFF0000) >> (8*2);
                let g = (*pixel & 0xFF00) >> (8*1);
                let b = *pixel & 0xFF;

                ppm_out.push_str(format!("{r} {g} {b}\n").as_str());
            }
        }

        ppm_out
    }

    pub fn at(&self, x: usize, y: usize) -> &u32 {
        &self.buff[x + self.width*y]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color, samples: i32) {
        self.buff[x + self.width*y] = color.output_32bit(samples);
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.buff
    }
}
