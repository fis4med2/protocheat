use protocheat_core::AntiCheatKind;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepotCheck {
    pub dir: PathBuf,
    pub has_windows_dll: bool,
    pub has_linux_so: bool,
    pub ok_for_proton: bool,
}

pub fn check_depot(dir: &Path) -> DepotCheck {
    let dll = dir.join("EasyAntiCheat_x64.dll");
    let so_names = ["easyanticheat_x64.so", "libeasyanticheat.so"];
    let has_windows_dll = dll.exists();
    let has_linux_so = so_names.iter().any(|n| dir.join(n).exists());
    DepotCheck {
        dir: dir.to_path_buf(),
        has_windows_dll,
        has_linux_so,
        ok_for_proton: has_linux_so,
    }
}

pub fn env_hint(
    kind: AntiCheatKind,
    eac_runtime: Option<&Path>,
    be_runtime: Option<&Path>,
) -> String {
    match kind {
        protocheat_core::AntiCheatKind::EasyAntiCheatKamu
        | protocheat_core::AntiCheatKind::EasyAntiCheatEos => match eac_runtime {
            Some(p) => format!("PROTON_EAC_RUNTIME={}", p.display()),
            None => "PROTON_EAC_RUNTIME not found. Install the Proton EasyAntiCheat Runtime or use the Lutris runtime.".to_string(),
        },
        protocheat_core::AntiCheatKind::BattlEye => match be_runtime {
            Some(p) => format!("PROTON_BATTLEYE_RUNTIME={}", p.display()),
            None => "PROTON_BATTLEYE_RUNTIME not found. Install the Proton BattlEye Runtime.".to_string(),
        },
        _ => "No Proton runtime applies. Kernel or native anti-cheat.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depot_check_missing_dir_is_not_ok() {
        let c = check_depot(Path::new("/nonexistent-protocheat-depot"));
        assert!(!c.ok_for_proton);
    }
}
