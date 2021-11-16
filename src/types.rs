use std::fs::File;

pub struct Output(pub File);

impl Output {
    pub fn get_file(&self) -> &File {
        &self.0
    }

    pub fn get_file_mut(&mut self) -> &mut File {
        &mut self.0
    }
}
