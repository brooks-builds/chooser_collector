use colorsys::Rgb;
use eyre::Result;

use crate::choice::Choice;
use crate::CollectionMode;

#[derive(Default)]
pub struct InteractiveMode {}

impl InteractiveMode {
    fn read_line(&self, description: &str) -> Result<String> {
        println!("{}: ", description);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_owned())
    }

    fn convert_string_to_color(&self, string: String) -> Result<Option<(u8, u8, u8)>> {
        if string.is_empty() {
            return Ok(None);
        }

        let result = Rgb::from_hex_str(&string)?;
        Ok(Some(result.into()))
    }
}

impl CollectionMode for InteractiveMode {
    fn run(
        &self,
        sender: crossbeam::channel::Sender<Choice>,
    ) -> std::result::Result<(), eyre::Report> {
        println!("Collecting choices interactively\n");
        loop {
            let name = self.read_line("Name")?;
            if name.is_empty() {
                break;
            }
            let color_string = self.read_line("Color")?;
            let color = self.convert_string_to_color(color_string)?;

            let choice = Choice::new(name, color);
            sender.send(choice)?;
        }

        Ok(())
    }
}
