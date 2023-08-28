#[allow(dead_code)]
#[derive(Debug)]
pub struct Asparagus {
    name: String,
}

impl Asparagus {
    pub fn default() -> Self {
        Self {
            name: String::from("default"),
        }
    }
}
