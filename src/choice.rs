use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Choice {
    name: String,
    red: u8,
    green: u8,
    blue: u8,
}

impl Choice {
    pub fn new(name: String, color: Option<(u8, u8, u8)>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        if let Some((r, g, b)) = color {
            red = r;
            green = g;
            blue = b;
        }

        Self {
            name,
            red,
            green,
            blue,
        }
    }
}
