use std::process::exit;

use chooser_collector::MainState;
use eyre::Result;

fn main() -> Result<()> {
    let main_state = match MainState::new() {
        Ok(state) => state,
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1)
        }
    };

    if main_state.arguments.help {
        println!("{}", main_state.arguments.help_text());
        exit(0);
    }

    dbg!(main_state.arguments);

    Ok(())
}
