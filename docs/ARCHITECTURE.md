# Architecture

ProtoCheat sits above Wine and Proton. It does not replace ntdll or wineserver.

Flow: `protocheat-cli` calls `protocheat-core` for the game matrix, `protocheat-launchers` for Steam, Lutris, Heroic and Bottles detection, `protocheat-ntsync` for `/dev/ntsync`, `protocheat-syscall` for Nt to ioctl notes, and `protocheat-proton` for depot checks and runtime hints.

`protocheat-shim` builds as cdylib and only exposes `protocheat_shim_loaded`. It exists for prefix load tests. No hooks.

Data lives in `data/games.json`. Code reads it only as reference. The builtin matrix in `protocheat-core` covers the first ten titles. Community PRs extend the JSON file.
