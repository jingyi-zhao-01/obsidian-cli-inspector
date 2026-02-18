use crate::config::Config;
use crate::config_setup;
use crate::logger::Logger;

pub type CommandResult = (&'static str, anyhow::Result<()>);

pub fn run_init_command(config_path: Option<std::path::PathBuf>, force: bool) -> CommandResult {
    let config = match config_setup::interactive_config_setup(config_path) {
        Ok(cfg) => cfg,
        Err(e) => return ("init", Err(e)),
    };

    let cmd_logger = Logger::new(config.log_dir()).ok();
    if let Some(ref log) = cmd_logger {
        let _ = log.log_section("init", "Starting Init Command");
    }

    (
        "init",
        crate::commands::initialize_database(&config, force, cmd_logger.as_ref()),
    )
}

pub fn run_stats_command(config: &Option<Config>, logger: Option<&Logger>) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => ("stats", crate::commands::show_stats(&c, logger)),
                Err(e) => ("stats", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("stats", "Starting Stats Command");
    }
    ("stats", crate::commands::show_stats(&config, logger))
}

pub fn run_index_command(
    config: &Option<Config>,
    dry_run: bool,
    force: bool,
    verbose: bool,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "index",
                    crate::commands::index_vault(&c, dry_run, force, verbose, logger),
                ),
                Err(e) => ("index", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("index", "Starting Index Command");
    }
    (
        "index",
        crate::commands::index_vault(&config, dry_run, force, verbose, logger),
    )
}

pub fn run_search_command(
    config: &Option<Config>,
    query: &str,
    limit: usize,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "search",
                    crate::commands::search_vault(&c, query, limit, logger),
                ),
                Err(e) => ("search", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("search", "Starting Search Command");
    }
    (
        "search",
        crate::commands::search_vault(&config, query, limit, logger),
    )
}

pub fn run_backlinks_command(
    config: &Option<Config>,
    note: &str,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "backlinks",
                    crate::commands::get_backlinks(&c, note, logger),
                ),
                Err(e) => ("backlinks", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("backlinks", "Starting Backlinks Command");
    }
    (
        "backlinks",
        crate::commands::get_backlinks(&config, note, logger),
    )
}

pub fn run_links_command(
    config: &Option<Config>,
    note: &str,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "links",
                    crate::commands::get_forward_links(&c, note, logger),
                ),
                Err(e) => ("links", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("links", "Starting Links Command");
    }
    (
        "links",
        crate::commands::get_forward_links(&config, note, logger),
    )
}

pub fn run_unresolved_links_command(
    config: &Option<Config>,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "unresolved-links",
                    crate::commands::list_unresolved_links(&c, logger),
                ),
                Err(e) => ("unresolved-links", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("unresolved", "Starting Unresolved Links Command");
    }
    (
        "unresolved-links",
        crate::commands::list_unresolved_links(&config, logger),
    )
}

pub fn run_tags_command(
    config: &Option<Config>,
    tag: &Option<String>,
    all: bool,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "tags",
                    crate::commands::list_notes_by_tag(&c, tag, all, logger),
                ),
                Err(e) => ("tags", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("tags", "Starting Tags Command");
    }
    (
        "tags",
        crate::commands::list_notes_by_tag(&config, tag, all, logger),
    )
}

pub fn run_describe_command(
    config: &Option<Config>,
    filename: &str,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "describe",
                    crate::commands::get_note_describe(&c, filename, logger),
                ),
                Err(e) => ("describe", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("describe", "Starting Describe Command");
    }
    (
        "describe",
        crate::commands::get_note_describe(&config, filename, logger),
    )
}

pub fn run_diagnose_orphans_command(
    config: &Option<Config>,
    exclude_templates: bool,
    exclude_daily: bool,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "diagnose-orphans",
                    crate::commands::diagnose_orphans(&c, exclude_templates, exclude_daily, logger),
                ),
                Err(e) => ("diagnose-orphans", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section("diagnose-orphans", "Starting Diagnose Orphans Command");
    }
    (
        "diagnose-orphans",
        crate::commands::diagnose_orphans(&config, exclude_templates, exclude_daily, logger),
    )
}

pub fn run_diagnose_broken_links_command(
    config: &Option<Config>,
    logger: Option<&Logger>,
) -> CommandResult {
    let config = match config {
        Some(cfg) => cfg,
        None => {
            return match config_setup::load_config(None) {
                Ok(c) => (
                    "diagnose-broken-links",
                    crate::commands::diagnose_broken_links_cmd(&c, logger),
                ),
                Err(e) => ("diagnose-broken-links", Err(e)),
            }
        }
    };

    if let Some(ref log) = logger {
        let _ = log.log_section(
            "diagnose-broken-links",
            "Starting Diagnose Broken Links Command",
        );
    }
    (
        "diagnose-broken-links",
        crate::commands::diagnose_broken_links_cmd(&config, logger),
    )
}
