use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiCheatKind {
    Vac,
    EasyAntiCheatKamu,
    EasyAntiCheatEos,
    BattlEye,
    Vanguard,
    Ricochet,
    EaAntiCheat,
    GameGuard,
    Xigncode3,
    FairFight,
    BungieProprietary,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinuxStatus {
    Supported,
    Running,
    Planned,
    Broken,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEntry {
    pub name: String,
    pub kind: AntiCheatKind,
    pub status: LinuxStatus,
    pub opt_in_required: bool,
    pub needs_kernel_driver: bool,
    pub note: String,
}

impl GameEntry {
    pub fn explains_failure(&self) -> String {
        if self.needs_kernel_driver {
            format!(
                "{} uses {:?}, which needs a Windows kernel driver. Proton and Wine run in user space, so there is no place to load it. This is a vendor architecture choice, not a launcher bug.",
                self.name, self.kind
            )
        } else if self.opt_in_required {
            format!(
                "{} uses {:?} with status {:?}. The vendor ships a Proton mode, but the publisher has to enable it per title. {}",
                self.name, self.kind, self.status, self.note
            )
        } else {
            format!("{}: {}", self.name, self.note)
        }
    }
}

pub fn builtin_matrix() -> Vec<GameEntry> {
    vec![
        GameEntry {
            name: "Elden Ring".into(),
            kind: AntiCheatKind::EasyAntiCheatEos,
            status: LinuxStatus::Supported,
            opt_in_required: true,
            needs_kernel_driver: false,
            note: "FromSoftware enabled the Linux module. Online works under Proton.".into(),
        },
        GameEntry {
            name: "Apex Legends".into(),
            kind: AntiCheatKind::EasyAntiCheatKamu,
            status: LinuxStatus::Denied,
            opt_in_required: true,
            needs_kernel_driver: false,
            note: "EA withdrew Linux access. Same software as working titles, different publisher decision.".into(),
        },
        GameEntry {
            name: "Fortnite".into(),
            kind: AntiCheatKind::EasyAntiCheatEos,
            status: LinuxStatus::Denied,
            opt_in_required: true,
            needs_kernel_driver: false,
            note: "Epic owns EAC and supports Proton for other games, but did not enable it here.".into(),
        },
        GameEntry {
            name: "Arma 3".into(),
            kind: AntiCheatKind::BattlEye,
            status: LinuxStatus::Supported,
            opt_in_required: true,
            needs_kernel_driver: false,
            note: "Bohemia enabled BattlEye Proton support.".into(),
        },
        GameEntry {
            name: "PUBG: Battlegrounds".into(),
            kind: AntiCheatKind::BattlEye,
            status: LinuxStatus::Broken,
            opt_in_required: true,
            needs_kernel_driver: false,
            note: "Publisher did not enable the Proton configuration.".into(),
        },
        GameEntry {
            name: "Valorant".into(),
            kind: AntiCheatKind::Vanguard,
            status: LinuxStatus::Denied,
            opt_in_required: false,
            needs_kernel_driver: true,
            note: "Vanguard loads at Windows boot. No Linux runtime exists.".into(),
        },
        GameEntry {
            name: "League of Legends".into(),
            kind: AntiCheatKind::Vanguard,
            status: LinuxStatus::Denied,
            opt_in_required: false,
            needs_kernel_driver: true,
            note: "Ran under Wine and Lutris for years, closed in April 2024 when Vanguard was added.".into(),
        },
        GameEntry {
            name: "Call of Duty: Warzone".into(),
            kind: AntiCheatKind::Ricochet,
            status: LinuxStatus::Denied,
            opt_in_required: false,
            needs_kernel_driver: true,
            note: "Ricochet needs Secure Boot and TPM 2.0 plus a kernel driver.".into(),
        },
        GameEntry {
            name: "Destiny 2".into(),
            kind: AntiCheatKind::BungieProprietary,
            status: LinuxStatus::Denied,
            opt_in_required: false,
            needs_kernel_driver: true,
            note: "Ran via Proton with BattlEye until Season 22 in August 2023, then moved to a proprietary kernel system.".into(),
        },
        GameEntry {
            name: "Counter-Strike 2".into(),
            kind: AntiCheatKind::Vac,
            status: LinuxStatus::Supported,
            opt_in_required: false,
            needs_kernel_driver: false,
            note: "VAC is native on Linux. No opt-in step.".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_has_kernel_and_optin_cases() {
        let m = builtin_matrix();
        assert!(m.iter().any(|g| g.needs_kernel_driver));
        assert!(m.iter().any(|g| g.opt_in_required));
    }

    #[test]
    fn failure_text_mentions_cause() {
        let m = builtin_matrix();
        let v = m.iter().find(|g| g.name == "Valorant").unwrap();
        assert!(v.explains_failure().contains("kernel driver"));
    }
}
