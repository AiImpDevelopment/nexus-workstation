//! Auto System Agent - Native Nexus Module
//!
//! Offline-first system validation, error detection, and intelligent
//! upgrade evaluation. Runs entirely locally without external APIs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall: HealthStatus,
    pub checks: Vec<HealthCheck>,
    pub scan_duration_ms: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: HealthStatus,
    pub severity: Severity,
    pub message: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// File change detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChanges {
    pub new_files: Vec<String>,
    pub modified_files: Vec<String>,
    pub deleted_files: Vec<String>,
    pub total_scanned: usize,
}

/// Service status for monitored endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub url: String,
    pub is_up: bool,
    pub response_ms: Option<u64>,
}

/// Check if a required path exists
fn check_path(path: &str) -> HealthCheck {
    let exists = Path::new(path).exists();
    HealthCheck {
        name: format!("path:{}", path.split('/').last().unwrap_or(path)),
        status: if exists { HealthStatus::Healthy } else { HealthStatus::Critical },
        severity: Severity::High,
        message: if exists {
            format!("OK: {path}")
        } else {
            format!("MISSING: {path}")
        },
        category: "filesystem".to_string(),
    }
}

/// Run all system integrity checks (fully offline)
fn run_integrity_checks() -> Vec<HealthCheck> {
    let mut checks = Vec::new();

    // Nexus core files
    let nexus_files = [
        "/opt/ork-station/Nexus/src-tauri/src/lib.rs",
        "/opt/ork-station/Nexus/src-tauri/Cargo.toml",
        "/opt/ork-station/Nexus/package.json",
        "/opt/ork-station/Nexus/.mcp.json",
    ];
    for path in &nexus_files {
        checks.push(check_path(path));
    }

    // MCP server files
    let mcp_files = [
        "/opt/ork-station/mcp-servers/semantic/server.py",
        "/opt/ork-station/mcp-servers/harmony-loop/server.py",
        "/opt/ork-station/mcp-servers/unlimited-context/server.py",
    ];
    for path in &mcp_files {
        checks.push(check_path(path));
    }

    // System config
    let config_files = [
        "/opt/ork-station/CLAUDE.md",
        "/opt/ork-station/services/archivar/intelligence/__init__.py",
    ];
    for path in &config_files {
        checks.push(check_path(path));
    }

    // Check disk space
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("df")
            .args(["--output=avail", "-B1", "/opt/ork-station"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().nth(1) {
                if let Ok(bytes) = line.trim().parse::<u64>() {
                    let gb = bytes as f64 / 1_073_741_824.0;
                    checks.push(HealthCheck {
                        name: "disk:space".to_string(),
                        status: if gb > 10.0 {
                            HealthStatus::Healthy
                        } else if gb > 5.0 {
                            HealthStatus::Degraded
                        } else {
                            HealthStatus::Critical
                        },
                        severity: if gb < 5.0 { Severity::Critical } else { Severity::Medium },
                        message: format!("{gb:.1} GB free on /opt/ork-station"),
                        category: "system".to_string(),
                    });
                }
            }
        }
    }

    // Check Nexus Rust modules
    let nexus_modules = [
        "router/classifier.rs",
        "router/targets.rs",
        "chat.rs",
        "docker/mod.rs",
        "github/mod.rs",
        "agents/mod.rs",
        "evaluation/mod.rs",
        "settings.rs",
    ];
    let src_dir = "/opt/ork-station/Nexus/src-tauri/src";
    for module in &nexus_modules {
        let full_path = format!("{src_dir}/{module}");
        let exists = Path::new(&full_path).exists();
        checks.push(HealthCheck {
            name: format!("nexus:{module}"),
            status: if exists { HealthStatus::Healthy } else { HealthStatus::Critical },
            severity: Severity::High,
            message: if exists {
                format!("Module OK: {module}")
            } else {
                format!("Module MISSING: {module}")
            },
            category: "nexus".to_string(),
        });
    }

    // Check MCP config completeness
    if let Ok(content) = std::fs::read_to_string("/opt/ork-station/Nexus/.mcp.json") {
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
            let server_count = config
                .get("mcpServers")
                .and_then(|s| s.as_object())
                .map(|o| o.len())
                .unwrap_or(0);
            checks.push(HealthCheck {
                name: "nexus:mcp-servers".to_string(),
                status: if server_count >= 12 {
                    HealthStatus::Healthy
                } else if server_count >= 8 {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Critical
                },
                severity: Severity::Medium,
                message: format!("{server_count} MCP servers configured (target: 20)"),
                category: "nexus".to_string(),
            });
        }
    }

    checks
}

/// Check service health via HTTP (async)
async fn check_services() -> Vec<ServiceStatus> {
    let services = [
        ("Ollama", "http://localhost:11434/api/tags"),
        ("Semantic MCP", "http://localhost:8002/"),
        ("Harmony MCP", "http://localhost:8001/"),
        ("RLM", "http://localhost:8015/status"),
        ("Docker", "http://localhost:2375/version"),
    ];

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap_or_default();

    let mut results = Vec::new();
    for (name, url) in &services {
        let start = Instant::now();
        let is_up = match client.get(*url).send().await {
            Ok(resp) => resp.status().is_success() || resp.status().as_u16() == 404,
            Err(_) => false,
        };
        results.push(ServiceStatus {
            name: name.to_string(),
            url: url.to_string(),
            is_up,
            response_ms: Some(start.elapsed().as_millis() as u64),
        });
    }
    results
}

/// Run a complete system scan
#[tauri::command]
pub async fn system_scan() -> Result<SystemHealth, String> {
    let start = Instant::now();

    // Run offline integrity checks
    let mut checks = run_integrity_checks();

    // Run service health checks
    let services = check_services().await;
    for svc in &services {
        checks.push(HealthCheck {
            name: format!("service:{}", svc.name),
            status: if svc.is_up {
                HealthStatus::Healthy
            } else {
                HealthStatus::Degraded
            },
            severity: Severity::Medium,
            message: if svc.is_up {
                format!(
                    "{} is up ({}ms)",
                    svc.name,
                    svc.response_ms.unwrap_or(0)
                )
            } else {
                format!("{} is DOWN", svc.name)
            },
            category: "services".to_string(),
        });
    }

    // Determine overall health
    let has_critical = checks.iter().any(|c| c.status == HealthStatus::Critical);
    let has_degraded = checks.iter().any(|c| c.status == HealthStatus::Degraded);
    let overall = if has_critical {
        HealthStatus::Critical
    } else if has_degraded {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    };

    Ok(SystemHealth {
        overall,
        checks,
        scan_duration_ms: start.elapsed().as_millis() as u64,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

/// Get quick health summary (lightweight, no network calls)
#[tauri::command]
pub fn system_health_quick() -> Result<HashMap<String, serde_json::Value>, String> {
    let checks = run_integrity_checks();
    let healthy = checks.iter().filter(|c| c.status == HealthStatus::Healthy).count();
    let total = checks.len();
    let issues: Vec<_> = checks
        .iter()
        .filter(|c| c.status != HealthStatus::Healthy)
        .map(|c| c.message.clone())
        .collect();

    let mut result = HashMap::new();
    result.insert(
        "score".to_string(),
        serde_json::json!(format!("{healthy}/{total}")),
    );
    result.insert(
        "healthy".to_string(),
        serde_json::json!(healthy == total),
    );
    result.insert("issues".to_string(), serde_json::json!(issues));
    result.insert("checked_at".to_string(), serde_json::json!(chrono::Utc::now().to_rfc3339()));

    Ok(result)
}
