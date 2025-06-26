use std::fs;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub buf: Vec<String>,
}

impl Buffer {

    pub fn load(&mut self) -> Result<&Buffer, Error> {
        let args: Vec<String> = std::env::args().collect();
        if let Some(first_arg) = args.get(1) {
           let file_contents = fs::read_to_string(first_arg)?;
            for i in file_contents.lines() {
                self.buf.push(i.to_string());
            }
        }
        Ok(self)
    }

    pub fn is_empty(&self) -> bool {
        if let Some(_element) = self.buf.get(0) {
            false
        } else {
            true
        }
    }
}
