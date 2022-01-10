use std::{fs::File, io::Write};

pub struct PPM {
    name: String,
    width: u32,
    height: u32,
    buffer: String,
}

impl PPM {
    pub fn from(name: String, width: u32, height: u32) -> Self {
        let buffer = format!("P3\n{}\n{}\n255\n", width, height);
        PPM {
            name,
            width,
            height,
            buffer,
        }
    }

    pub fn push(&mut self, r: u8, g: u8, b: u8) {
        self.buffer += &format!("{} {} {}\n", r, g, b);
    }

    pub fn write(self) -> std::io::Result<()> {
        let mut file = File::create(self.name)?;
        file.write_all(self.buffer.as_bytes())?;
        Ok(())
    }
}

#[test]
fn create_ppm_image() {
    let mut file = PPM::from(String::from("test.ppm"), 256, 256);

    for x in 0..=255 {
        for y in 0..=255 {
            file.push(x, y, 0);
        }
    }
    file.write().unwrap();
}
