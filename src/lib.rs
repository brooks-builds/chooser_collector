use std::fs::File;
use std::io::Write;

use arguments::Arguments;
use choice::Choice;
use eyre::{bail, Result};
use interactive_mode::InteractiveMode;

mod arguments;
mod choice;
mod interactive_mode;

trait CollectionMode {
    fn run(&self, sender: crossbeam::channel::Sender<Choice>) -> Result<()>;
}

pub struct MainState {
    pub arguments: Arguments,
    receiver: crossbeam::channel::Receiver<Choice>,
    sender: crossbeam::channel::Sender<Choice>,
    collector: Box<dyn CollectionMode>,
}

impl MainState {
    pub fn new() -> Result<Self> {
        let arguments = Arguments::new()?;
        let (sender, receiver) = crossbeam::channel::unbounded();
        let collector = if arguments.interactive_mode {
            Box::new(InteractiveMode::default())
        } else {
            bail!("no collector available");
        };
        Ok(Self {
            arguments,
            receiver,
            sender,
            collector,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.collector.run(self.sender.clone())?;
        let mut choices = vec![];
        self.receiver.try_iter().for_each(|choice| {
            choices.push(choice);
        });
        self.save_to_disk(choices, &self.arguments)?;
        Ok(())
    }

    fn save_to_disk(&self, choices: Vec<Choice>, arguments: &Arguments) -> Result<()> {
        let mut buffer = File::create(&arguments.output_file)?;
        let json = serde_json::to_string(&choices)?;
        buffer.write_all(json.as_bytes())?;
        Ok(())
    }
}
