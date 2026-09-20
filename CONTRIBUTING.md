# Contributing

All text is in English so reviews stay in one language.

## DCO sign-off

Every commit must end with a sign-off line. We use the Developer Certificate of Origin. Canonical identity for this repo:

```
Signed-off-by: fis4med2 <fis4med2@users.noreply.github.com>
```

Add it with `git commit -s`. A PR without sign-off will not merge.

## How to help

- Add or fix one entry in `data/games.json` with a log or ProtonDB link.
- Fix one launcher check in `protocheat-launchers`.
- Fix docs. Small PRs merge faster than large ones.

## Issue format

Title, game, launcher, runner, output of `protocheat doctor`, expected result. Clean tokens and usernames from logs before attaching.

## PR format

What changed, how to test, output of `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test`. One concern per PR.

## No-bypass rule

Do not send code that hooks, patches, spoofs or disables anti-cheat, reads other process memory, or loads a fake driver. Such PRs are closed. See SECURITY.md.
