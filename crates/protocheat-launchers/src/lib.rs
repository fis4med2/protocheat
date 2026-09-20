use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherReport {
    pub lutris_present: bool,
    pub lutris_version: Option<String>,
    pub heroic_present: bool,
    pub bottles_present: bool,
    pub steam_present: bool,
    pub wine_runners: Vec<String>,
    pub eac_runtime: Option<PathBuf>,
    pub battleye_runtime: Option<PathBuf>,
    pub warnings: Vec<String>,
}

fn which(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn cmd_version(cmd: &str, args: &[&str]) -> Option<String> {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("/tmp"))
}

pub fn detect() -> LauncherReport {
    let lutris_runners_dir = home().join(".local/share/lutris/runners/wine");
    let mut wine_runners = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&lutris_runners_dir) {
        for e in entries.flatten() {
            wine_runners.push(e.file_name().to_string_lossy().to_string());
        }
    }
    // Proton and GE installs under Steam compatdata area are also relevant.
    let steam_compat = home().join(".steam/steam/steamapps/common");
    if let Ok(entries) = std::fs::read_dir(&steam_compat) {
        for e in entries.flatten() {
            let n = e.file_name().to_string_lossy().to_string();
            if n.to_lowercase().contains("proton") || n.to_lowercase().contains("wine") {
                wine_runners.push(n);
            }
        }
    }

    let eac_lutris = home().join(".local/share/lutris/runtime/eac_runtime");
    let be_lutris = home().join(".local/share/lutris/runtime/battleye_runtime");
    let eac_steam = steam_compat.join("Proton EasyAntiCheat Runtime");
    let be_steam = steam_compat.join("Proton BattlEye Runtime");

    let eac_runtime = [eac_lutris, eac_steam].into_iter().find(|p| p.exists());
    let battleye_runtime = [be_lutris, be_steam].into_iter().find(|p| p.exists());

    let mut warnings = Vec::new();
    if let Some(ref p) = eac_runtime {
        if p.to_string_lossy().contains(' ') {
            warnings.push(format!(
                "EAC runtime path contains spaces ({}). Some launchers fail to expand it unless symlinked without spaces.",
                p.display()
            ));
        }
    } else {
        warnings.push("No EAC runtime found in Lutris or Steam paths.".to_string());
    }
    if std::env::var("FLATPAK_ID").is_ok() {
        warnings
            .push("Running inside Flatpak sandbox. Host runtimes may be invisible.".to_string());
    }

    LauncherReport {
        lutris_present: which("lutris"),
        lutris_version: cmd_version("lutris", &["--version"]),
        heroic_present: which("heroic") || which("heroic-electron"),
        bottles_present: which("bottles"),
        steam_present: which("steam") || steam_compat.exists(),
        wine_runners,
        eac_runtime,
        battleye_runtime,
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_runs_without_panic() {
        let r = detect();
        let _ = serde_json::to_string(&r).unwrap();
    }
}
