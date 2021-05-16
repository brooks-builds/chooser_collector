use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{self, Duration, Instant};

use eyre::Result;
use twitch_chat_wrapper::ChatMessage;

use crate::arguments::Arguments;
use crate::choice::Choice;
use crate::CollectionMode;

pub struct TwitchMode {
    send_to_twitch: Sender<String>,
    receive_from_twitch: Receiver<ChatMessage>,
    command: String,
    deadline: Duration,
    who_has_entered: HashSet<String>,
}

impl TwitchMode {
    pub fn new(
        send_to_twitch: Sender<String>,
        receive_from_twitch: Receiver<ChatMessage>,
        arguments: &Arguments,
    ) -> Result<Self> {
        let command = arguments.twitch_command.clone();
        let deadline = arguments.deadline;
        let who_has_entered = HashSet::new();
        send_to_twitch.send(format!(
            "-- Enter for {} -- Use the command !{} to enter for the next {} seconds",
            &arguments.description,
            command,
            deadline.as_secs()
        ))?;
        Ok(Self {
            send_to_twitch,
            receive_from_twitch,
            command,
            deadline,
            who_has_entered,
        })
    }
}

impl CollectionMode for TwitchMode {
    fn run(
        &mut self,
        sender: crossbeam::channel::Sender<crate::choice::Choice>,
    ) -> eyre::Result<()> {
        let start = time::Instant::now();
        let pattern = format!("!{}", &self.command);
        loop {
            let duration = Instant::now() - start;
            if duration > self.deadline {
                self.send_to_twitch
                    .send("Thanks for entering, good luck!".to_owned())?;
                break;
            }
            let chat_message = if let Ok(message) = self.receive_from_twitch.try_recv() {
                message
            } else {
                continue;
            };
            if chat_message.message.to_lowercase().starts_with(&pattern) {
                let name = if let Some(display_name) = chat_message.display_name {
                    display_name
                } else {
                    chat_message.name
                };
                if self.who_has_entered.contains(&name) {
                    continue;
                }
                let color = chat_message.color_rgb;
                let choice = Choice::new(name.clone(), Some(color));
                sender.send(choice)?;
                self.who_has_entered.insert(name.clone());
                eprintln!("Thanks for entering {}", name);
            }
        }
        Ok(())
    }
}
