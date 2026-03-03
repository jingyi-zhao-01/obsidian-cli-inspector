use crate::logger::Logger;

pub fn analyze_related(note: &str, limit: usize, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("analyze.related", "Related command not yet implemented");
        let _ = log.print_and_log("analyze.related", &format!("  note: {note}"));
        let _ = log.print_and_log("analyze.related", &format!("  limit: {limit}"));
    } else {
        println!("Related command not yet implemented");
        println!("  note: {note}");
        println!("  limit: {limit}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_related_without_logger() {
        analyze_related("test_note", 10, None);
    }

    #[test]
    fn test_analyze_related_with_empty_note() {
        analyze_related("", 0, None);
    }
}
