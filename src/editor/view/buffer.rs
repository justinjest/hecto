
#[derive(Default)]
pub struct Buffer {
    pub buf: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        if let Some(_element) = self.buf.get(0) {
            false
        } else {
            true
        }
    }
}
