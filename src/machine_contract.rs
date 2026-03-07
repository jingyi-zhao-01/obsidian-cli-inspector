use crate::{config::Config, db::Database, query};
use serde_json::Value;

pub struct ResultDataBuilder;

impl ResultDataBuilder {
    fn empty_query_result() -> Value {
        serde_json::json!({ "total": 0, "items": [] })
    }

    fn query_result(items: Vec<Value>) -> Value {
        serde_json::json!({ "total": items.len(), "items": items })
    }

    pub fn build_query_result_data(config: &Config, command: &str, params: &Value) -> Value {
        let db_path = config.database_path();
        if !db_path.exists() {
            return Self::empty_query_result();
        }

        let db = match Database::open(&db_path) {
            Ok(db) => db,
            Err(_) => return Self::empty_query_result(),
        };

        match command {
            "search.notes" => {
                let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
                let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

                let results = match db
                    .conn()
                    .execute_query(|conn| query::search_chunks(conn, query, limit))
                {
                    Ok(results) => results,
                    Err(_) => return Self::empty_query_result(),
                };

                let items = results
                    .iter()
                    .map(|result| {
                        serde_json::json!({
                            "chunk_id": result.chunk_id,
                            "note_id": result.note_id,
                            "note_path": result.note_path,
                            "note_title": result.note_title,
                            "heading_path": result.heading_path,
                            "chunk_text": result.chunk_text,
                            "rank": result.rank
                        })
                    })
                    .collect();

                Self::query_result(items)
            }
            "search.backlinks" => {
                let note = params.get("note").and_then(|v| v.as_str()).unwrap_or("");

                let results = match db
                    .conn()
                    .execute_query(|conn| query::get_backlinks(conn, note))
                {
                    Ok(results) => results,
                    Err(_) => return Self::empty_query_result(),
                };

                let items = results
                    .iter()
                    .map(|result| {
                        serde_json::json!({
                            "note_id": result.note_id,
                            "note_path": result.note_path,
                            "note_title": result.note_title,
                            "is_embed": result.is_embed,
                            "alias": result.alias,
                            "heading_ref": result.heading_ref,
                            "block_ref": result.block_ref
                        })
                    })
                    .collect();

                Self::query_result(items)
            }
            "search.links" => {
                let note = params.get("note").and_then(|v| v.as_str()).unwrap_or("");

                let results = match db
                    .conn()
                    .execute_query(|conn| query::get_forward_links(conn, note))
                {
                    Ok(results) => results,
                    Err(_) => return Self::empty_query_result(),
                };

                let items = results
                    .iter()
                    .map(|result| {
                        serde_json::json!({
                            "note_id": result.note_id,
                            "note_path": result.note_path,
                            "note_title": result.note_title,
                            "is_embed": result.is_embed,
                            "alias": result.alias,
                            "heading_ref": result.heading_ref,
                            "block_ref": result.block_ref
                        })
                    })
                    .collect();

                Self::query_result(items)
            }
            "search.unresolved" => {
                let results = match db.conn().execute_query(query::get_unresolved_links) {
                    Ok(results) => results,
                    Err(_) => return Self::empty_query_result(),
                };

                let items = results
                    .iter()
                    .map(|result| {
                        serde_json::json!({
                            "note_id": result.note_id,
                            "note_path": result.note_path,
                            "note_title": result.note_title,
                            "is_embed": result.is_embed,
                            "alias": result.alias,
                            "heading_ref": result.heading_ref,
                            "block_ref": result.block_ref
                        })
                    })
                    .collect();

                Self::query_result(items)
            }
            "search.tags" => {
                let list_all = params
                    .get("list")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let tag = params.get("tag").and_then(|v| v.as_str());

                if list_all || tag.is_none() {
                    let tags = match db.conn().execute_query(query::list_tags) {
                        Ok(tags) => tags,
                        Err(_) => return Self::empty_query_result(),
                    };

                    let items = tags
                        .iter()
                        .map(|tag_name| serde_json::json!({ "tag": tag_name }))
                        .collect();

                    Self::query_result(items)
                } else if let Some(tag_name) = tag {
                    let results = match db
                        .conn()
                        .execute_query(|conn| query::get_notes_by_tag(conn, tag_name))
                    {
                        Ok(results) => results,
                        Err(_) => return Self::empty_query_result(),
                    };

                    let items = results
                        .iter()
                        .map(|result| {
                            serde_json::json!({
                                "note_id": result.note_id,
                                "note_path": result.note_path,
                                "note_title": result.note_title,
                                "tags": result.tags
                            })
                        })
                        .collect();

                    Self::query_result(items)
                } else {
                    Self::empty_query_result()
                }
            }
            _ => Self::empty_query_result(),
        }
    }

    pub fn build_view_stats_result_data(config: &Config) -> Value {
        let db_path = config.database_path();
        if !db_path.exists() {
            return serde_json::json!({ "status": "success" });
        }

        let db = match Database::open(&db_path) {
            Ok(db) => db,
            Err(_) => return serde_json::json!({ "status": "success" }),
        };

        match db.get_stats() {
            Ok(stats) => serde_json::json!({
                "notes": stats.note_count,
                "links": stats.link_count,
                "tags": stats.tag_count,
                "chunks": stats.chunk_count,
                "unresolved_links": stats.unresolved_links
            }),
            Err(_) => serde_json::json!({ "status": "success" }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_empty_query_result() {
        let result = ResultDataBuilder::empty_query_result();
        assert_eq!(result.get("total").unwrap(), 0);
        assert!(result.get("items").unwrap().is_array());
    }

    #[test]
    fn test_query_result_with_items() {
        let items = vec![serde_json::json!({"id": 1}), serde_json::json!({"id": 2})];
        let result = ResultDataBuilder::query_result(items);
        assert_eq!(result.get("total").unwrap(), 2);
        assert!(result.get("items").unwrap().is_array());
    }

    #[test]
    fn test_query_result_empty_items() {
        let items: Vec<Value> = vec![];
        let result = ResultDataBuilder::query_result(items);
        assert_eq!(result.get("total").unwrap(), 0);
    }

    #[test]
    fn test_build_query_result_data_no_database() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            vault_path: temp_dir.path().to_path_buf(),
            database_path: Some(temp_dir.path().join("nonexistent.db")),
            log_path: None,
            exclude: Default::default(),
            search: Default::default(),
            graph: Default::default(),
            llm: None,
        };

        let params = serde_json::json!({});
        let result = ResultDataBuilder::build_query_result_data(&config, "search.notes", &params);
        assert_eq!(result.get("total").unwrap(), 0);
    }

    #[test]
    fn test_build_query_result_data_unknown_command() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            vault_path: temp_dir.path().to_path_buf(),
            database_path: Some(temp_dir.path().join("nonexistent.db")),
            log_path: None,
            exclude: Default::default(),
            search: Default::default(),
            graph: Default::default(),
            llm: None,
        };

        let params = serde_json::json!({});
        let result =
            ResultDataBuilder::build_query_result_data(&config, "unknown.command", &params);
        assert_eq!(result.get("total").unwrap(), 0);
    }

    #[test]
    fn test_build_view_stats_result_data_no_database() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            vault_path: temp_dir.path().to_path_buf(),
            database_path: Some(temp_dir.path().join("nonexistent.db")),
            log_path: None,
            exclude: Default::default(),
            search: Default::default(),
            graph: Default::default(),
            llm: None,
        };

        let result = ResultDataBuilder::build_view_stats_result_data(&config);
        assert_eq!(result.get("status").unwrap(), "success");
    }
}
