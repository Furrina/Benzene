#[derive(Debug)]
pub struct BenzeneError {
    line: usize,
    message: String,
}

impl BenzeneError {
    pub fn error(line: usize, message: String) -> BenzeneError {
        BenzeneError { line, message }
    }

    pub fn report(&self, location: String) {
        eprintln!("[line {}] Error {}: {}", self.line, location, self.message);
    }
}
