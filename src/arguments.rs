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
-t, --twitch                Collect choices by reading from twitch chat
-c, --command command       Command to listen for in Twitch chat. Whatever this is will have a ! added to the front of it. By default \"here\" is used
-o, --output                File name to write to. By default the file name is choices.json

EXAMPLES:

chooser_collector

chooser_collector -f game_names -o game_choices.json

chooser_collector -t -c addme
";
const DEFAULT_TWITCH_COMMAND: &str = "HERE";
const DEFAULT_OUTPUT_FILE: &str = "choices.json";

#[derive(Debug)]
pub struct Arguments {
    pub interactive_mode: bool,
    pub twitch_mode: bool,
    pub file: Option<String>,
    pub twitch_command: String,
    pub output_file: String,
    pub help: bool,
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
        arguments.set_twitch_mode(&mut pico_arguments);
        arguments.set_file(&mut pico_arguments)?;
        arguments.set_twitch_command(&mut pico_arguments)?;
        arguments.set_output_file(&mut pico_arguments)?;

        arguments.validate()?;

        Ok(arguments)
    }

    fn set_interactive_mode(&mut self, pico_arguments: &mut pico_args::Arguments) {
        if pico_arguments.contains(["-i", "--interactive"]) {
            self.interactive_mode = true;
        }
    }

    fn set_twitch_mode(&mut self, pico_arguments: &mut pico_args::Arguments) {
        self.twitch_mode = pico_arguments.contains(["-t", "--twitch"]);
        if self.twitch_mode {
            self.interactive_mode = false;
        }
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
        }
    }
}
