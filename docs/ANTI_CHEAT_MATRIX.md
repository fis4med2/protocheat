# Anti-cheat matrix

## User-space with Proton mode

EAC Kamu, EAC EOS and BattlEye ship a Linux module. The publisher must enable it per title. Elden Ring, Arma 3 and DayZ enabled it. Apex Legends, Fortnite, PUBG and Siege use the same software but did not enable it for those titles. Symptom is init failure or service not started.

Needed files: `EasyAntiCheat_x64.dll` next to `easyanticheat_x64.so` in the depot, plus `PROTON_EAC_RUNTIME` or `PROTON_BATTLEYE_RUNTIME` pointing at the Lutris or Steam runtime. Lutris stores runtimes under `~/.local/share/lutris/runtime`. Heroic also needs the `.so` symlink into `Plugins/x86_64` for some Epic builds and an empty `LD_PRELOAD` on older releases.

## Kernel driver required

Vanguard, Ricochet, EA proprietary builds, Bungie proprietary builds after Season 22, GameGuard and XIGNCODE3 need a signed Windows driver. On Windows that driver runs in Ring 0, strips access with `ObRegisterCallbacks`, monitors system calls, and uses IOCTLs to reach its user-mode service. Proton has no slot for this. Some also need boot time load, Secure Boot, TPM 2.0 or IOMMU. League of Legends worked under Lutris until Vanguard arrived in April 2024. Destiny 2 worked under Proton with BattlEye until August 2023. Both then closed.

## Native

VAC is native on Linux. Counter-Strike 2 needs no extra step.
