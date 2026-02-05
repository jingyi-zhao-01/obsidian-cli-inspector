use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedNote {
    pub title: String,
    pub frontmatter: HashMap<String, String>,
    pub tags: Vec<String>,
    pub links: Vec<Link>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    pub alias: Option<String>,
    pub heading_ref: Option<String>,
    pub block_ref: Option<String>,
    pub is_embed: bool,
}

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse(content: &str) -> ParsedNote {
        let (frontmatter, rest) = Self::extract_frontmatter(content);
        let tags = Self::extract_tags(&frontmatter, &rest);
        let links = Self::extract_wikilinks(&rest);
        let title = Self::extract_title(&frontmatter, &rest);

        ParsedNote {
            title,
            frontmatter,
            tags,
            links,
            text: rest.to_string(),
        }
    }

    fn extract_frontmatter(content: &str) -> (HashMap<String, String>, &str) {
        let mut map = HashMap::new();

        if !content.starts_with("---") {
            return (map, content);
        }

        let rest = &content[3..];
        if let Some(end_pos) = rest.find("---") {
            let frontmatter_text = &rest[..end_pos];
            let content_after = &rest[end_pos + 3..].trim_start();

            for line in frontmatter_text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some(colon_pos) = line.find(':') {
                    let key = line[..colon_pos].trim().to_lowercase();
                    let value = line[colon_pos + 1..].trim().to_string();

                    // Special handling for tags which might be arrays
                    if key == "tags" {
                        let tags_str = value.trim_start_matches('[').trim_end_matches(']');
                        for tag in tags_str.split(',') {
                            let clean_tag = tag.trim().trim_matches('"').trim_matches('\'');
                            if !clean_tag.is_empty() {
                                map.insert(
                                    format!("tag_{}", clean_tag),
                                    clean_tag.to_string(),
                                );
                            }
                        }
                    } else {
                        map.insert(key, value);
                    }
                }
            }

            return (map, content_after);
        }

        (map, content)
    }

    fn extract_title(frontmatter: &HashMap<String, String>, content: &str) -> String {
        // Try to get from frontmatter
        if let Some(title) = frontmatter.get("title") {
            return title.clone();
        }

        // Try to extract from first heading
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return trimmed[2..].trim().to_string();
            }
        }

        // Fallback to empty string
        String::new()
    }

    fn extract_tags(frontmatter: &HashMap<String, String>, content: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // From frontmatter
        for (key, value) in frontmatter {
            if key.starts_with("tag_") {
                tags.push(value.clone());
            }
        }

        // From inline tags in content
        for word in content.split_whitespace() {
            if word.starts_with('#') && word.len() > 1 {
                let tag = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '_')
                    .trim_start_matches('#');
                if !tag.is_empty() && !tags.contains(&tag.to_string()) {
                    tags.push(tag.to_string());
                }
            }
        }

        tags.sort();
        tags.dedup();
        tags
    }

    fn extract_wikilinks(content: &str) -> Vec<Link> {
        let mut links = Vec::new();
        let content_chars: Vec<char> = content.chars().collect();
        let mut pos = 0;

        while pos < content_chars.len() {
            // Check for wikilink or embed
            if pos + 1 < content_chars.len() {
                let is_embed = content_chars[pos] == '!' && content_chars[pos + 1] == '[';
                let is_wikilink = content_chars[pos] == '[' && content_chars[pos + 1] == '[';

                if is_embed {
                    // ![[...]]
                    if let Some(link) = Self::parse_wikilink(&content_chars, pos + 1, true) {
                        links.push(link);
                    }
                    pos += 1;
                } else if is_wikilink {
                    // [[...]]
                    if let Some(link) = Self::parse_wikilink(&content_chars, pos, false) {
                        links.push(link);
                    }
                    pos += 1;
                }
            }
            pos += 1;
        }

        links
    }

    fn parse_wikilink(chars: &[char], start: usize, is_embed: bool) -> Option<Link> {
        if start + 3 >= chars.len() || chars[start] != '[' || chars[start + 1] != '[' {
            return None;
        }

        let mut end = start + 2;
        while end + 1 < chars.len() && !(chars[end] == ']' && chars[end + 1] == ']') {
            end += 1;
        }

        if end + 1 >= chars.len() {
            return None;
        }

        let content: String = chars[start + 2..end].iter().collect();
        let (text, alias, heading_ref, block_ref) = Self::parse_link_content(&content);

        Some(Link {
            text,
            alias,
            heading_ref,
            block_ref,
            is_embed,
        })
    }

    fn parse_link_content(content: &str) -> (String, Option<String>, Option<String>, Option<String>) {
        let mut alias = None;
        let mut heading_ref = None;
        let mut block_ref = None;

        // Check for pipe (alias)
        let text = if let Some(pipe_pos) = content.find('|') {
            let t = content[..pipe_pos].trim().to_string();
            alias = Some(content[pipe_pos + 1..].trim().to_string());
            t
        } else {
            content.to_string()
        };

        // Check for heading reference
        let text = if let Some(hash_pos) = text.find('#') {
            let heading = text[hash_pos + 1..].trim().to_string();
            let t = text[..hash_pos].trim().to_string();

            if heading.starts_with('^') {
                block_ref = Some(heading[1..].to_string());
            } else {
                heading_ref = Some(heading);
            }
            t
        } else {
            text
        };

        (text, alias, heading_ref, block_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wikilink_simple() {
        let parsed = MarkdownParser::parse("This is [[note]] link");
        assert_eq!(parsed.links.len(), 1);
        assert_eq!(parsed.links[0].text, "note");
    }

    #[test]
    fn test_parse_wikilink_with_alias() {
        let parsed = MarkdownParser::parse("This is [[note|alias]] link");
        assert_eq!(parsed.links.len(), 1);
        assert_eq!(parsed.links[0].text, "note");
        assert_eq!(parsed.links[0].alias, Some("alias".to_string()));
    }
}
