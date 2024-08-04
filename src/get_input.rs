use std::io::Write;


/// Gets input string from user
///
/// # Panics
///
/// Panics if reading from stdin failed.
pub fn input(msg: &str) -> String {
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    buf
}
