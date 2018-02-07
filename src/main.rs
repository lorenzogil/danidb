extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    let mut editor = Editor::<()>::new();
    if let Err(_) = editor.load_history("history.txt") {
        println!("No previous history.");
    }

    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => {
                editor.add_history_entry(&line);
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    editor.save_history("history.txt").unwrap();
}
