use crate::logger::Logger;

pub fn show_tui(logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("tui", "TUI not yet implemented");
    } else {
        println!("TUI not yet implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_tui_without_logger() {
        show_tui(None);
    }
}
