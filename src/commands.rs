pub mod index;
pub mod init;

pub mod query_backlinks;
pub mod query_links;
pub mod query_search;
pub mod query_tags;
pub mod query_unresolved;

pub mod analyze_bloat;
pub mod analyze_related;

pub mod diagnose_broken_links;
pub mod diagnose_orphans;

pub mod stats;
pub mod view_describe;

pub mod tui;

pub use index::index_vault;
pub use init::initialize_database;

pub use query_backlinks::get_backlinks;
pub use query_links::get_forward_links;
pub use query_search::search_vault;
pub use query_tags::list_notes_by_tag;
pub use query_unresolved::list_unresolved_links;

pub use analyze_bloat::show_bloat;
pub use analyze_related::analyze_related;

pub use diagnose_broken_links::diagnose_broken_links_cmd;
pub use diagnose_orphans::diagnose_orphans;

pub use stats::show_stats;
pub use view_describe::get_note_describe;

pub use tui::show_tui;
