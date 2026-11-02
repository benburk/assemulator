use std::io::{IsTerminal, Write};

/// Gets input string from user, prompting with `msg` when stdin is a terminal.
/// Exits the process at end of input.
///
/// # Panics
///
/// Panics if reading from stdin failed.
pub fn input(msg: &str) -> String {
    let interactive = std::io::stdin().is_terminal();
    if interactive {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(msg.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
    let mut buf = String::new();
    if std::io::stdin().read_line(&mut buf).unwrap() == 0 {
        if interactive {
            println!();
        }
        std::process::exit(0);
    }

    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    buf
}
