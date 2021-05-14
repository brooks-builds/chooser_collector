use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};

use arguments::Arguments;
use choice::Choice;
use eyre::{bail, Result};
use interactive_mode::InteractiveMode;
use twitch_chat_wrapper::ChatMessage;
use twitch_mode::TwitchMode;

mod arguments;
mod choice;
mod interactive_mode;
mod twitch_mode;

trait CollectionMode {
    fn run(&mut self, sender: crossbeam::channel::Sender<Choice>) -> Result<()>;
}

pub struct MainState {
    pub arguments: Arguments,
    receiver: crossbeam::channel::Receiver<Choice>,
    sender: crossbeam::channel::Sender<Choice>,
    collector: Box<dyn CollectionMode>,
}

impl MainState {
    pub fn new(
        send_to_twitch: Sender<String>,
        receive_from_twitch: Receiver<ChatMessage>,
    ) -> Result<Self> {
        let arguments = Arguments::new()?;
        let (sender, receiver) = crossbeam::channel::unbounded();
        let collector: Box<dyn CollectionMode> = if arguments.interactive_mode {
            Box::new(InteractiveMode::default())
        } else if arguments.twitch_mode {
            Box::new(TwitchMode::new(
                send_to_twitch,
                receive_from_twitch,
                &arguments,
            )?)
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
