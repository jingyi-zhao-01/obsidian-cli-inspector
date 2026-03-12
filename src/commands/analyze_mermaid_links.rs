use crate::config::Config;
use crate::logger::Logger;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const MGMT_CLASS_LINE: &str = "class PromA,GrafA,PromB,GrafB mgmt";
const INTERNAL_LINK_MARKER: &str = "%% ---------- Internal Links ----------";
const INTERNAL_LINK_BLOCK: &str = r##"
%% ---------- Internal Links ----------
click Client href "#Clients / Internet"
click DNS href "#Global DNS / Traffic Manager"
click IstiodA href "#Istiod"
click IstiodB href "#Istiod"
click APIA href "#Kube API Server"
click APIB href "#Kube API Server"
click SchedulerA href "#Scheduler"
click SchedulerB href "#Scheduler"
click ControllerA href "#Controller Manager"
click ControllerB href "#Controller Manager"
click IngressA href "#Istio Ingress Gateway"
click IngressB href "#Istio Ingress Gateway"
click ServiceA1 href "#Kubernetes Service"
click ServiceA2 href "#Kubernetes Service"
click ServiceB1 href "#Kubernetes Service"
click ServiceB2 href "#Kubernetes Service"
click PodA1 href "#Pod (Envoy + App)"
click PodA2 href "#Pod (Envoy + App)"
click PodB1 href "#Pod (Envoy + App)"
click PodB2 href "#Pod (Envoy + App)"
click PodA3 href "#Pod (Envoy + App)"
click PodA4 href "#Pod (Envoy + App)"
click PodB3 href "#Pod (Envoy + App)"
click PodB4 href "#Pod (Envoy + App)"
click PromA href "#Prometheus"
click PromB href "#Prometheus"
click GrafA href "#Grafana"
click GrafB href "#Grafana"

class Client,DNS,IstiodA,APIA,SchedulerA,ControllerA,IstiodB,APIB,SchedulerB,ControllerB,IngressA,ServiceA1,ServiceB1,PodA1,PodA2,PodB1,PodB2,IngressB,ServiceA2,ServiceB2,PodA3,PodA4,PodB3,PodB4,PromA,GrafA,PromB,GrafB internal-link;
"##;

pub fn update_mermaid_links(config: &Config, path: &str, logger: Option<&Logger>) -> Result<()> {
    let file_path = resolve_note_path(&config.vault_path, path);
    let content = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
    let (updated, changed) = apply_mermaid_links(&content)?;

    if !changed {
        let message = format!("No updates needed for {}", file_path.display());
        if let Some(log) = logger {
            let _ = log.print_and_log("analyze-mermaid-links", &message);
        } else {
            println!("{message}");
        }
        return Ok(());
    }

    std::fs::write(&file_path, updated)
        .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

    let message = format!("Updated Mermaid internal links for {}", file_path.display());
    if let Some(log) = logger {
        let _ = log.print_and_log("analyze-mermaid-links", &message);
    } else {
        println!("{message}");
    }

    Ok(())
}

fn resolve_note_path(vault_path: &Path, input: &str) -> PathBuf {
    let candidate = PathBuf::from(input);
    if candidate.is_absolute() {
        candidate
    } else {
        vault_path.join(candidate)
    }
}

fn apply_mermaid_links(content: &str) -> Result<(String, bool)> {
    let cleaned = remove_internal_link_section(content)?;

    if !cleaned.contains(MGMT_CLASS_LINE) {
        anyhow::bail!(
            "Mermaid management class line not found: '{MGMT_CLASS_LINE}'."
        );
    }

    let insertion = format!("{MGMT_CLASS_LINE}\n{INTERNAL_LINK_BLOCK}");
    let updated = cleaned.replacen(MGMT_CLASS_LINE, &insertion, 1);
    let changed = updated != content;
    Ok((updated, changed))
}

fn remove_internal_link_section(content: &str) -> Result<String> {
    let mut lines = Vec::new();
    let mut skipping = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if skipping {
            if is_internal_link_class_line(trimmed) {
                skipping = false;
            }
            continue;
        }

        if trimmed == INTERNAL_LINK_MARKER {
            skipping = true;
            continue;
        }

        if is_internal_link_class_line(trimmed) {
            continue;
        }

        lines.push(line);
    }

    if skipping {
        anyhow::bail!("Internal link section marker found without closing class line.");
    }

    let mut result = lines.join("\n");
    if content.ends_with('\n') {
        result.push('\n');
    }

    Ok(result)
}

fn is_internal_link_class_line(line: &str) -> bool {
    line.starts_with("class Client,DNS,") && line.ends_with("internal-link;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_mermaid_links_inserts_block() {
        let content = format!(
            "{MGMT_CLASS_LINE}\nclass Client,DNS,Old internal-link;\n"
        );
        let (updated, changed) = apply_mermaid_links(&content).expect("apply should succeed");

        assert!(changed);
        assert!(updated.contains(INTERNAL_LINK_MARKER));
        assert!(updated.contains("click Client href \"#Clients / Internet\""));
        assert!(!updated.contains("class Client,DNS,Old internal-link;"));
    }

    #[test]
    fn apply_mermaid_links_replaces_existing_block() {
        let content = format!(
            "{MGMT_CLASS_LINE}\n{INTERNAL_LINK_MARKER}\nclick Old href \"#Old\"\nclass Client,DNS,Old internal-link;\n"
        );
        let (updated, _changed) = apply_mermaid_links(&content).expect("apply should succeed");

        let marker_count = updated.matches(INTERNAL_LINK_MARKER).count();
        assert_eq!(marker_count, 1);
        assert!(!updated.contains("click Old href \"#Old\""));
    }

    #[test]
    fn apply_mermaid_links_requires_mgmt_line() {
        let err = apply_mermaid_links("no mgmt line").expect_err("should error");
        assert!(err.to_string().contains(MGMT_CLASS_LINE));
    }
}
