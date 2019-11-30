use std::fs::File;
use std::io::Write;

pub struct PPM {
    pub width: u32,
    pub height: u32,
    pub max: u32,
    file: File,
}

impl PPM {
    pub fn new(filename: &str, height: u32, width: u32, max: u32) -> PPM {
        let mut output = File::create(filename).unwrap();
        write!(output, "P3\n{} {}\n255\n", width, height).unwrap();
        PPM {
            width: width,
            height: height,
            max: max,
            file: output,
        }
    }

    pub fn write_row(&mut self, data: &[u32; 3]) {
        write!(self.file, "{} {} {}\n", data[0], data[1], data[2]).unwrap();
    }
}
