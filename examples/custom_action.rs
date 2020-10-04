use colored::Colorize;
use crossterm::event::{KeyCode, KeyModifiers};

use rucline::actions::{Action, Event, Overrider};
use rucline::completion::Basic;
use rucline::{Context, Prompt};

struct EarlyExit;

impl Overrider for EarlyExit {
    fn override_for(&self, event: Event, _: &dyn Context) -> Option<Action> {
        if event.modifiers == KeyModifiers::CONTROL {
            if let KeyCode::Char('d') = event.code {
                // Cleanly exit when the combination CTRL+D is pressed
                quit::with_code(0);
            }
        }

        // Fallback to default action
        None
    }
}

#[quit::main]
fn main() {
    // Simulate a list of acceptable inputs
    let possible_commands = vec!["run", "walk", "fly"];

    // Simulate a history of previous inputs
    let command_history = vec!["run", "fly"];
    let mode_history = vec![
        "slow",
        "fast",
        "normal",
        "very slowly almost stopping",
        "very quickly almost lightspeed",
    ];

    // Initial prompt
    if let Ok(Some(command)) = Prompt::from(format!("{}> ", "vai".green()))
        .erase_after_read(true)
        .suggester(&Basic::new(&possible_commands))
        .completer(&Basic::new(&command_history))
        .overrider(&EarlyExit)
        .read_line()
    {
        // Accept command if it exists
        if possible_commands.contains(&command.as_str()) {
            // Show the sub-prompt
            if let Ok(Some(mode)) = Prompt::from(format!(
                "{}|{}> ",
                "vai".green(),
                command.as_str().bright_green()
            ))
            .completer(&Basic::new(&mode_history))
            .overrider(&EarlyExit)
            .read_line()
            {
                // We will do as commanded
                println!("Ok! Will {} {}", command, mode);
            }
        } else {
            // Command not recognized
            eprintln!("{} invalid command", "Error".red());
        }
    }
}
