use std::process::exit;
use std::sync::mpsc::channel;
use std::thread::spawn;

use chooser_collector::MainState;
use eyre::Result;
use twitch_chat_wrapper::ChatMessage;

fn main() -> Result<()> {
    let (send_to_chat, receive_from_chooser) = channel::<String>();
    let (send_to_chooser, receive_from_twitch) = channel::<ChatMessage>();
    spawn(|| {
        twitch_chat_wrapper::run(receive_from_chooser, send_to_chooser).unwrap();
    });
    let mut main_state = match MainState::new(send_to_chat, receive_from_twitch) {
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

    main_state.run()
}
