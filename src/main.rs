mod cmd;
mod fun;
mod history;
mod ps1;
mod coloransi;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {
    let ps1 = ps1::render_ps1();
    let mut rl = DefaultEditor::new().unwrap();
    let history = history::History::new();
    let commands = history.load();

    for cmd in commands {
        rl.add_history_entry(cmd);
    }

    loop {
        let readline = rl.readline(&ps1);
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                history.append(&input);

                if input.trim() == "exit" {
                    break;
                }

                if let Err(e) = cmd::execute(&input) {
                    eprintln!("{}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C received. Type 'exit' to quit.");
                continue;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D received. Exiting.");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
}
