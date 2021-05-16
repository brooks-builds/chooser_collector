use std::time::Duration;

use eyre::{bail, Result};

pub const HELP: &str = "\
Chooser Collector

Create a JSON choices file meant to be used with the random chooser physics program. Names must be unique, if a name is used twice the second will be ignored.

USAGE:

chooser_collector [OPTIONS]

FLAGS:

-h, --help            Prints help information

OPTIONS:

-i, --interactive (default) Collect choices by being interactively asked for the choice name and the color one at a time
-f, --file filename         Read choice names from a file and generate a choices file from the names
-t, --twitch DESCRIPTION    Collect choices by reading from twitch chat. With the Description of what chatters are entering for.
-c, --command command       Command to listen for in Twitch chat. Whatever this is will have a ! added to the front of it. By default \"here\" is used
-o, --output                File name to write to. By default the file name is choices.json
-d, --deadline              How long will we wait for twitch chat to collect choices before stopping listening to chat
-s, --standard-out          Print the resulting json file to standard out instead of to a file


EXAMPLES:

chooser_collector

chooser_collector -f game_names -o game_choices.json

chooser_collector -t -c addme
";
const DEFAULT_TWITCH_COMMAND: &str = "here";
const DEFAULT_OUTPUT_FILE: &str = "choices.json";
const DEFAULT_DEADLINE: Duration = Duration::from_secs(60);
const DEFAULT_DESCRIPTION: &str = "something awesome";

#[derive(Debug)]
pub struct Arguments {
    pub interactive_mode: bool,
    pub twitch_mode: bool,
    pub file: Option<String>,
    pub twitch_command: String,
    pub output_file: String,
    pub help: bool,
    pub deadline: Duration,
    pub description: String,
    pub standard_out: bool,
}

impl Arguments {
    pub fn new() -> Result<Self> {
        let mut pico_arguments = pico_args::Arguments::from_env();
        let mut arguments = Self::default();

        if pico_arguments.contains(["-h", "--help"]) {
            arguments.help = true;
            return Ok(arguments);
        }

        arguments.set_interactive_mode(&mut pico_arguments);
        arguments.set_twitch_mode(&mut pico_arguments)?;
        arguments.set_file(&mut pico_arguments)?;
        arguments.set_twitch_command(&mut pico_arguments)?;
        arguments.set_output_file(&mut pico_arguments)?;
        arguments.set_deadline(&mut pico_arguments)?;
        arguments.set_standard_out(&mut pico_arguments);

        arguments.validate()?;

        Ok(arguments)
    }

    fn set_interactive_mode(&mut self, pico_arguments: &mut pico_args::Arguments) {
        if pico_arguments.contains(["-i", "--interactive"]) {
            self.interactive_mode = true;
        }
    }

    fn set_twitch_mode(&mut self, pico_arguments: &mut pico_args::Arguments) -> Result<()> {
        let keys = ["-t", "--twitch"];
        let description = pico_arguments
            .opt_value_from_str(keys)?
            .unwrap_or_else(|| "".to_owned());
        self.twitch_mode = !description.is_empty();
        if self.twitch_mode {
            self.interactive_mode = false;
        }

        self.description = description;
        Ok(())
    }

    fn set_file(&mut self, pico_arguments: &mut pico_args::Arguments) -> Result<()> {
        if let Some(file) = pico_arguments.opt_value_from_str(["-f", "--file"])? {
            self.file = Some(file);
            self.interactive_mode = false;
        }
        Ok(())
    }

    fn set_twitch_command(&mut self, pico_arguments: &mut pico_args::Arguments) -> Result<()> {
        if let Some(command) = pico_arguments.opt_value_from_str(["-c", "--command"])? {
            self.twitch_command = command;
        }
        Ok(())
    }

    fn set_output_file(&mut self, pico_arguments: &mut pico_args::Arguments) -> Result<()> {
        if let Some(filename) = pico_arguments.opt_value_from_str(["-o", "--output"])? {
            self.output_file = filename;
        }
        Ok(())
    }

    fn set_deadline(&mut self, pico_arguments: &mut pico_args::Arguments) -> Result<()> {
        if let Some(deadline) = pico_arguments.opt_value_from_str(["-d", "--deadline"])? {
            self.deadline = Duration::from_secs(deadline);
        }
        Ok(())
    }

    fn set_standard_out(&mut self, pico_arguments: &mut pico_args::Arguments) {
        self.standard_out = pico_arguments.contains(["-s", "--standard-out"]);
    }

    fn validate(&self) -> Result<()> {
        if self.interactive_mode && self.twitch_mode {
            bail!("Cannot have interactive mode and twitch mode set at the same time");
        }

        if (self.interactive_mode || self.twitch_mode) && matches!(self.file, Some(_)) {
            bail!("Cannot read choices from a file if we are getting choices interactively or through Twitch Chat");
        }

        Ok(())
    }

    pub fn help_text(&self) -> &str {
        HELP
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            interactive_mode: true,
            twitch_mode: false,
            file: None,
            twitch_command: DEFAULT_TWITCH_COMMAND.to_owned(),
            output_file: DEFAULT_OUTPUT_FILE.to_owned(),
            help: false,
            deadline: DEFAULT_DEADLINE,
            description: DEFAULT_DESCRIPTION.to_string(),
            standard_out: false,
        }
    }
}
