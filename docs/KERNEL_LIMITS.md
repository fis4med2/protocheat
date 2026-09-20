# Kernel limits

Wine implements NT sync with wineserver plus `/dev/ntsync`. Calls such as `NtCreateEvent`, `NtCreateMutant`, `NtCreateSemaphore` and `NtWaitForMultipleObjects` map to `NTSYNC_IOC_*` ioctls. The 64 object wait limit comes from NT. Pulse and wait-for-all need direct wait queue control, which is why they live in the driver.

## How it works on Windows

Kernel anti-cheat ships a signed driver that runs at the same privilege as the OS. EAC, BattlEye, Vanguard and Ricochet all use this model on Windows. The driver strips handle access to the game process with routines such as `ObRegisterCallbacks`, watches system calls, and talks to a user-mode service through IOCTLs. Because it loads as part of the kernel, it sees process creation and image loads before cheats can hide.

## How it behaves on Proton and Linux

Windows kernel drivers do not load under Proton or Wine. For Steam Deck and desktop Linux, EAC and BattlEye ship a build that runs strictly in user mode. No kernel driver loads. Detection is narrower than on Windows by design, and the title only works if the developer enabled that build. This is why the same anti-cheat brand can work in one game and fail in another.

## Boot and hardware barriers

Some systems require load at Windows boot. Vanguard is the clearest case. Others require hardware trust such as TPM 2.0, Secure Boot and IOMMU. Proton has no equivalent boot slot and cannot attest those properties for Windows. Those titles stay unsupported. The repo treats this as architecture, not as a missing patch.

Research in this repo stays read-only: list which calls are missing in a VM trace and document the result. No fake driver is shipped.
