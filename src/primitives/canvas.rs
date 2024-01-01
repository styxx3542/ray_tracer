use crate::primitives::color::Color;
use std::fs::File;
use std::io::prelude::*;
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    length: usize,
    grid: Vec<Vec<Color>>,
}
impl Canvas {
    pub fn new(width: usize, length: usize) -> Canvas {
        Canvas {
            width,
            length,
            grid: vec![vec![Color::new(0.0, 0.0, 0.0); width]; length],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        if width >= self.width || height >= self.length {
            panic!("Pixel out of bounds - {width}, {height}");
        }
        self.grid[height][width] = color;
    }

    pub fn pixel_at(&self, width: usize, height: usize) -> Color {
        self.grid[height][width]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.length));
        ppm.push_str("255\n");
        for row in self.grid.iter() {
            let mut row_str = String::new();
            for pixel in row.iter() {
                let s = format!(
                    "{} {} {} ",
                    (pixel.red() * 255.0) as u8,
                    (pixel.green() * 255.0) as u8,
                    (pixel.blue() * 255.0) as u8
                );
                if row_str.len() + s.len() > 70 {
                    ppm.push_str(row_str.trim());
                    ppm.push_str("\n");
                    row_str = String::new();
                }
                row_str.push_str(&s);
            }

            ppm.push_str(row_str.trim());
            ppm.push_str("\n");
        }
        ppm
    }

    pub fn save_as_ppm(&self, filename: &str) -> std::io::Result<()> {
        let filename = format!("{}.ppm", filename);
        let mut file = File::create(filename)?;
        file.write_all(self.to_ppm().as_bytes())?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.length, 20);
        assert!(canvas
            .grid
            .iter()
            .all(|v| v.iter().all(|c| c == &Color::new(0.0, 0.0, 0.0))));
    }

    #[test]
    fn write_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);
        assert_eq!(canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn canvas_to_ppm() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let expected = "P3\n5 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n";
        assert_eq!(ppm, expected);
    }

    #[test]
    fn ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let expected = "P3\n5 3\n255\n";
        assert_eq!(&ppm[..expected.len()], expected);
    }

    #[test]
    fn ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        let ppm = canvas.to_ppm();
        let expected = "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(ppm, expected);
    }
}
