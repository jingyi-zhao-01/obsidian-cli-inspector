use crate::logger::Logger;

pub fn show_bloat(threshold: usize, limit: usize, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("bloat", "Bloat command not yet implemented");
        let _ = log.print_and_log("bloat", &format!("  threshold: {threshold}"));
        let _ = log.print_and_log("bloat", &format!("  limit: {limit}"));
    } else {
        println!("Bloat command not yet implemented");
        println!("  threshold: {threshold}");
        println!("  limit: {limit}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_bloat_without_logger() {
        show_bloat(100, 50, None);
    }

    #[test]
    fn test_show_bloat_with_zero_values() {
        show_bloat(0, 0, None);
    }
}
