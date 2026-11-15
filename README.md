# SigmaOS-arch

> **SigmaOS Arch Linux persona** — CLI tool, subcommand dispatch, Arch integration.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Inspired by [Arch Linux](https://archlinux.org).

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## API

```rust
use sigmaos_arch::Tool;

let tool = Tool::new("arch").expect("empty name rejected");
tool.run("install neovim").expect("unknown subcommands rejected");
```

## Component Role

The `SigmaOS-arch` crate provides the Arch Linux minimalist persona for SigmaOS:
- Pacman-compatible package translation
- Rolling-release update model
- Makepkg / PKGBUILD integration

Full docs: [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

## License

Apache-2.0
