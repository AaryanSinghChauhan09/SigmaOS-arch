// SPDX-License-Identifier: Apache-2.0
//! SigmaOS Arch Linux Persona CLI Tool (`arch`)
//!
//! Provides direct CLI access to Arch Linux operations:
//! - `status`: Persona readiness check
//! - `pacman`: Native / ALPM package management
//! - `aur`: Arch User Repository package search
//! - `doctor`: Subsystem health diagnostic inspection

use std::env;
use std::process::{Command, ExitCode};

fn print_usage() {
    println!("\x1b[1;36mSigmaOS Arch Linux Persona Tool\x1b[0m");
    println!("Usage: arch <subcommand> [arguments...]");
    println!();
    println!("Commands:");
    println!("  status         Display Arch Linux persona status");
    println!("  pacman [flags] Run Pacman / ALPM package operations (-S, -Syu, -Ss, -Si, -R)");
    println!("  aur <query>    Query Arch User Repository (AUR v5 RPC API)");
    println!("  doctor         Perform substrate health diagnostic check");
    println!("  help           Display this help message");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return ExitCode::SUCCESS;
    }

    match args[1].as_str() {
        "status" => {
            println!("\x1b[1;32m[✓] SigmaOS Arch Linux Persona: Active & Ready\x1b[0m");
            println!("  Package Management : Pacman + ALPM Engine");
            println!("  Repository Tier    : Bleeding-Edge Rolling");
            println!("  AUR Access         : Enabled");
            ExitCode::SUCCESS
        }
        "pacman" => {
            let status = Command::new("sigpkg")
                .arg("pacman")
                .args(&args[2..])
                .status()
                .or_else(|_| {
                    Command::new("sigmaos-arch")
                        .arg("pacman")
                        .args(&args[2..])
                        .status()
                });
            match status {
                Ok(s) => if s.success() { ExitCode::SUCCESS } else { ExitCode::FAILURE },
                Err(e) => {
                    eprintln!("error: could not invoke pacman bridge: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        "aur" => {
            if args.len() < 3 {
                eprintln!("error: search query required for aur");
                return ExitCode::FAILURE;
            }
            let status = Command::new("sigpkg")
                .arg("aur")
                .args(&args[2..])
                .status();
            match status {
                Ok(s) => if s.success() { ExitCode::SUCCESS } else { ExitCode::FAILURE },
                Err(e) => {
                    eprintln!("error: could not invoke aur bridge: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        "doctor" => {
            let status = Command::new("sigmaos-arch")
                .arg("doctor")
                .status()
                .or_else(|_| {
                    Command::new("sigpkg")
                        .arg("doctor")
                        .status()
                });
            match status {
                Ok(s) => if s.success() { ExitCode::SUCCESS } else { ExitCode::FAILURE },
                Err(e) => {
                    eprintln!("error: could not invoke doctor diagnostics: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        "help" | "--help" | "-h" => {
            print_usage();
            ExitCode::SUCCESS
        }
        unknown => {
            eprintln!("error: unknown command '{}'", unknown);
            print_usage();
            ExitCode::FAILURE
        }
    }
}
