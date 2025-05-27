pub fn info(text: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", text)
}

pub fn message(text: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", text)
}
