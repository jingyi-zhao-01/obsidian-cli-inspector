// Query module for retrieving and searching vault data
mod links;
mod search;
mod tags;

pub use links::{get_backlinks, get_forward_links, get_unresolved_links, LinkResult};
pub use search::{search_chunks, SearchResult};
pub use tags::{
    get_notes_by_tag, get_notes_by_tags_and, get_notes_by_tags_or, list_tags, TagResult,
};
