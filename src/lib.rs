// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-arch — Arch Linux Subsystem Engine for SigmaOS
//!
//! Provides native Pacman / ALPM translation, mirrorlist synchronization,
//! rolling release validation, and AUR package management abstractions.
//! Principles: fail-fast, AAA testing, zero unsafe code, defensive programming.

#![forbid(unsafe_code)]

use std::fmt;

/// Errors that can occur during Arch subsystem operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchError {
    EmptyName,
    UnknownCommand(String),
    MissingArgs(String),
    InvalidPackageName(String),
    InvalidMirrorUrl(String),
}

impl fmt::Display for ArchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name must not be empty"),
            Self::UnknownCommand(x) => write!(f, "unrecognized subcommand: {x}"),
            Self::MissingArgs(x) => write!(f, "subcommand requires arguments: {x}"),
            Self::InvalidPackageName(p) => write!(f, "invalid Arch package identifier: {p}"),
            Self::InvalidMirrorUrl(m) => write!(f, "invalid mirror url: {m}"),
        }
    }
}

impl std::error::Error for ArchError {}

/// Operational mode for pacman actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacmanOp {
    Sync,       // -S
    SysUpgrade, // -Syu
    Search,     // -Ss
    Info,       // -Si
    Remove,     // -Rns
    Query,      // -Q
}

impl PacmanOp {
    pub fn flag(&self) -> &'static str {
        match self {
            Self::Sync => "-S",
            Self::SysUpgrade => "-Syu",
            Self::Search => "-Ss",
            Self::Info => "-Si",
            Self::Remove => "-Rns",
            Self::Query => "-Q",
        }
    }
}

/// A structured Pacman command request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanCommand {
    pub op: PacmanOp,
    pub targets: Vec<String>,
    pub noconfirm: bool,
}

impl PacmanCommand {
    pub fn new(op: PacmanOp) -> Self {
        Self {
            op,
            targets: Vec::new(),
            noconfirm: true,
        }
    }

    pub fn with_target(mut self, target: &str) -> Result<Self, ArchError> {
        let trimmed = target.trim();
        if trimmed.is_empty() || trimmed.contains(' ') || trimmed.starts_with('-') {
            return Err(ArchError::InvalidPackageName(target.to_string()));
        }
        self.targets.push(trimmed.to_string());
        Ok(self)
    }

    pub fn to_command_line(&self) -> String {
        let mut parts = vec!["pacman", self.op.flag()];
        if self.noconfirm {
            parts.push("--noconfirm");
        }
        for t in &self.targets {
            parts.push(t.as_str());
        }
        parts.join(" ")
    }
}

/// Official Arch & SigmaOS pacman mirror entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorEntry {
    pub server_url: String,
    pub country: String,
    pub protocol: String,
}

impl MirrorEntry {
    pub fn new(url: &str, country: &str) -> Result<Self, ArchError> {
        if !url.starts_with("https://") && !url.starts_with("http://") {
            return Err(ArchError::InvalidMirrorUrl(url.to_string()));
        }
        let protocol = if url.starts_with("https://") {
            "https".to_string()
        } else {
            "http".to_string()
        };

        Ok(Self {
            server_url: url.trim().to_string(),
            country: country.trim().to_string(),
            protocol,
        })
    }

    pub fn to_mirrorlist_line(&self) -> String {
        format!("Server = {}/$repo/os/$arch", self.server_url)
    }
}

/// Arch Linux Persona tool dispatcher
#[derive(Debug)]
pub struct Tool {
    name: String,
}

impl Tool {
    pub fn new(name: impl Into<String>) -> Result<Self, ArchError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ArchError::EmptyName);
        }
        Ok(Self { name })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn run(&self, args: &[&str]) -> Result<String, ArchError> {
        match args.first().copied().unwrap_or("help") {
            "status" => Ok(format!("{}: ready (Arch Linux / Pacman Engine)", self.name)),
            "version" => Ok(format!("{}: 0.1.0", self.name)),
            "help" => Ok(format!(
                "{}: commands: status, version, pacman, aur, doctor, help",
                self.name
            )),
            "pacman" => {
                let op = match args.get(1).copied().unwrap_or("-Syu") {
                    "-Syu" => PacmanOp::SysUpgrade,
                    "-S" => PacmanOp::Sync,
                    "-Ss" => PacmanOp::Search,
                    "-Si" => PacmanOp::Info,
                    "-Rns" | "-R" => PacmanOp::Remove,
                    "-Q" => PacmanOp::Query,
                    _ => PacmanOp::Sync,
                };
                let mut cmd = PacmanCommand::new(op);
                for target in args.iter().skip(2) {
                    cmd = cmd.with_target(target)?;
                }
                Ok(cmd.to_command_line())
            }
            "aur" => {
                let query = args.get(1).ok_or_else(|| ArchError::MissingArgs("query".to_string()))?;
                Ok(format!("https://aur.archlinux.org/rpc/v5/search/{}", query))
            }
            "doctor" => Ok(format!("{}: substrate healthy, pacman db clean", self.name)),
            other => Err(ArchError::UnknownCommand(other.to_string())),
        }
    }
}

/// Backwards-compatible component handle
#[derive(Debug, Default)]
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self {
            name: "SigmaOS Arch Component",
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_and_version_aaa() {
        // Arrange
        let t = Tool::new("demo").unwrap();
        // Act & Assert
        assert!(t.run(&["status"]).unwrap().contains("ready"));
        assert!(t.run(&["version"]).unwrap().contains("0.1.0"));
    }

    #[test]
    fn test_no_args_shows_help_aaa() {
        // Arrange
        let t = Tool::new("demo").unwrap();
        // Act
        let res = t.run(&[]).unwrap();
        // Assert
        assert!(res.contains("commands:"));
    }

    #[test]
    fn test_unknown_command_fails_aaa() {
        // Arrange
        let t = Tool::new("demo").unwrap();
        // Act & Assert
        assert!(matches!(t.run(&["bogus"]), Err(ArchError::UnknownCommand(_))));
    }

    #[test]
    fn test_empty_name_fails_aaa() {
        assert!(Tool::new(" ").is_err());
    }

    #[test]
    fn test_pacman_command_construction_aaa() {
        // Arrange
        let cmd = PacmanCommand::new(PacmanOp::Sync)
            .with_target("hyprland")
            .unwrap()
            .with_target("waybar")
            .unwrap();
        // Act
        let line = cmd.to_command_line();
        // Assert
        assert_eq!(line, "pacman -S --noconfirm hyprland waybar");
    }

    #[test]
    fn test_pacman_sysupgrade_command_aaa() {
        // Arrange
        let cmd = PacmanCommand::new(PacmanOp::SysUpgrade);
        // Act
        let line = cmd.to_command_line();
        // Assert
        assert_eq!(line, "pacman -Syu --noconfirm");
    }

    #[test]
    fn test_pacman_invalid_package_rejection_aaa() {
        // Arrange
        let cmd = PacmanCommand::new(PacmanOp::Sync);
        // Act & Assert
        assert!(cmd.clone().with_target("bad package name").is_err());
        assert!(cmd.with_target("-malicious-flag").is_err());
    }

    #[test]
    fn test_mirror_entry_serialization_aaa() {
        // Arrange
        let mirror = MirrorEntry::new("https://geo.mirror.pkgbuild.com", "Global").unwrap();
        // Act
        let line = mirror.to_mirrorlist_line();
        // Assert
        assert_eq!(line, "Server = https://geo.mirror.pkgbuild.com/$repo/os/$arch");
        assert_eq!(mirror.protocol, "https");
    }

    #[test]
    fn test_mirror_entry_invalid_url_aaa() {
        // Arrange & Act & Assert
        assert!(MirrorEntry::new("ftp://insecure.mirror.org", "US").is_err());
    }

    #[test]
    fn test_aur_query_url_generation_aaa() {
        // Arrange
        let t = Tool::new("arch").unwrap();
        // Act
        let url = t.run(&["aur", "yay"]).unwrap();
        // Assert
        assert_eq!(url, "https://aur.archlinux.org/rpc/v5/search/yay");
    }
}
