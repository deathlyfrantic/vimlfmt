#[derive(Debug, PartialEq, Clone)]
pub struct Modifier {
    pub name: String,
    pub bang: bool,
    pub count: usize,
}

impl Modifier {
    pub fn new(name: &str) -> Modifier {
        Modifier {
            name: name.to_string(),
            bang: false,
            count: 0,
        }
    }
}
