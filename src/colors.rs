pub fn info(text: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", text)
}

pub fn message(text: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", text)
}

pub fn error(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn warning(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

#[macro_export]
/// Display a purple message to the user, used for all confirmations / server event
macro_rules! display_info {
    ( $fmt:expr $(, $args:expr )* ) => {{
        let formatted = format!($fmt $(, $args )*);
        println!("\x1b[35m{}\x1b[0m", formatted);
    }};
}

#[macro_export]
/// Display a red message to the user, used for non critical error
macro_rules! display_error {
    ( $fmt:expr $(, $args:expr )* ) => {{
        let formatted = format!($fmt $(, $args )*);
        println!("{}", crate::colors::error(&formatted));
    }};
}
