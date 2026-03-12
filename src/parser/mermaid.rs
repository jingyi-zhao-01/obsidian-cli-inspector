use super::{build_markdown_link, Link, LinkType};

pub fn extract_mermaid_links(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut in_mermaid_block = false;

    for line in content.lines() {
        let trimmed = line.trim_start();
        if let Some(fence_lang) = trimmed.strip_prefix("```") {
            if in_mermaid_block {
                in_mermaid_block = false;
            } else if fence_lang.trim().to_lowercase().starts_with("mermaid") {
                in_mermaid_block = true;
            }
            continue;
        }

        if !in_mermaid_block {
            continue;
        }

        if let Some(target) = parse_click_href_target(trimmed) {
            if let Some(mut link) = build_markdown_link("", target, false) {
                link.link_type = LinkType::Mermaid;
                links.push(link);
            }
        }
    }

    links
}

fn parse_click_href_target(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("click ") {
        return None;
    }

    let href_pos = trimmed.find("href")?;
    let after_href = &trimmed[href_pos + 4..];
    let start_quote = after_href.find('"')?;
    let after_quote = &after_href[start_quote + 1..];
    let end_quote = after_quote.find('"')?;
    let target = after_quote[..end_quote].trim();
    if target.is_empty() {
        None
    } else {
        Some(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mermaid_links_basic() {
        let content = "```mermaid\nclick A href \"Target Note\"\n```";
        let links = extract_mermaid_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "Target Note");
        assert_eq!(links[0].link_type, LinkType::Mermaid);
    }

    #[test]
    fn test_extract_mermaid_links_ignores_non_mermaid() {
        let content = "click A href \"Target\"\n```graph\nclick B href \"Other\"\n```";
        let links = extract_mermaid_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_mermaid_links_with_heading() {
        let content = "```mermaid\nclick A href \"Note.md#Section\"\n```";
        let links = extract_mermaid_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "Note");
        assert_eq!(links[0].heading_ref, Some("Section".to_string()));
    }

    #[test]
    fn test_parse_click_href_target_handles_missing_quotes() {
        assert_eq!(parse_click_href_target("click A href Target"), None);
    }
}
