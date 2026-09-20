use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "protocheat",
    version,
    about = "Diagnose Proton and anti-cheat setup. No bypass."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Doctor {
        #[arg(long)]
        json: bool,
    },
    Check {
        name: String,
    },
    Explain {
        name: String,
    },
    Depot {
        dir: PathBuf,
    },
    Matrix,
    Syscalls,
}

fn main() {
    tracing::info!("protocheat start");
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Doctor { json } => {
            let launchers = protocheat_launchers::detect();
            let ntsync = protocheat_ntsync::check();
            if json {
                let v = serde_json::json!({
                    "launchers": launchers,
                    "ntsync": format!("{ntsync:?}"),
                });
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
            } else {
                println!("protocheat doctor");
                println!(
                    "lutris: {} {:?}",
                    launchers.lutris_present, launchers.lutris_version
                );
                println!("heroic: {}", launchers.heroic_present);
                println!("bottles: {}", launchers.bottles_present);
                println!("steam data seen: {}", launchers.steam_present);
                println!("runners: {:?}", launchers.wine_runners);
                println!("eac runtime: {:?}", launchers.eac_runtime);
                println!("battleye runtime: {:?}", launchers.battleye_runtime);
                println!("ntsync: {ntsync:?}");
                for w in launchers.warnings {
                    println!("warn: {w}");
                }
            }
        }
        Cmd::Check { name } => {
            let m = protocheat_core::builtin_matrix();
            match m
                .iter()
                .find(|g| g.name.to_lowercase().contains(&name.to_lowercase()))
            {
                Some(g) => println!("{}: {:?} -> {:?}", g.name, g.kind, g.status),
                None => println!("{name} is not in the local matrix yet. Contributions welcome."),
            }
        }
        Cmd::Explain { name } => {
            let m = protocheat_core::builtin_matrix();
            match m
                .iter()
                .find(|g| g.name.to_lowercase().contains(&name.to_lowercase()))
            {
                Some(g) => println!("{}", g.explains_failure()),
                None => println!("No entry for {name}."),
            }
        }
        Cmd::Depot { dir } => {
            let c = protocheat_proton::check_depot(&dir);
            println!("dir: {}", c.dir.display());
            println!("windows dll: {}", c.has_windows_dll);
            println!("linux so: {}", c.has_linux_so);
            println!("ok for proton: {}", c.ok_for_proton);
        }
        Cmd::Matrix => {
            for g in protocheat_core::builtin_matrix() {
                println!("{} | {:?} | {:?}", g.name, g.kind, g.status);
            }
        }
        Cmd::Syscalls => {
            for m in protocheat_syscall::table() {
                println!("{} -> {} ({})", m.nt_call, m.linux_equivalent, m.note);
            }
        }
    }
}
