use colored::Colorize;
use std::{sync::mpsc, thread};

/// spawns an mpsc channel which will append whatever it receives--in this case,
/// twitch chat's most common prefix--to a variable
pub fn electoral_handler() -> mpsc::Sender<String> {
    // rx will continuously receive the most common prefix of user inputs
    // at an interval
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        println!(
            "{}{}",
            "[handler]: ".green().bold(),
            "spawned handler thread".green().italic(),
        );

        let mut program = String::new();
        while let Ok(val) = rx.recv() {
            // process new most common prefix (just append and print)
            program += &val;

            println!(
                "{}{} {}",
                "[handler]: ".green().bold(),
                "program:".green().italic(),
                format!("{program:?}").green()
            );
        }
    });

    tx
}
