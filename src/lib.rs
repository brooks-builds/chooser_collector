use arguments::Arguments;
use eyre::Result;

mod arguments;

pub struct MainState {
    pub arguments: Arguments,
}

impl MainState {
    pub fn new() -> Result<Self> {
        let arguments = Arguments::new()?;
        Ok(Self { arguments })
    }
}
