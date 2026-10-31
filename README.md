# SigmaOS Architecture

Architecture and hardware abstraction code for SigmaOS. The crate exposes `arch` and re-exports its `Arch` interface. The current module tree contains shared HAL and portability code, x86_64 support, and target-gated AArch64 and RISC-V modules.

The architecture tree also includes experimental and synthesis modules. Their presence does not mean that every routine performs real hardware setup; inspect the target-specific implementation before using it during boot or privileged execution.

## Build

This crate depends on the sibling [`SigmaOS-kernel`](../SigmaOS-kernel) checkout. From the SigmaOS checkout:

```sh
cargo check --manifest-path SigmaOS-arch/Cargo.toml --lib
```

## Layout

- `src/arch/hal.rs` and shared modules provide common architecture-facing code.
- `src/arch/x86_64/` contains x86_64 modules.
- `src/arch/aarch64/` and `src/arch/riscv64/` are gated by the corresponding Rust target architecture.

## License

GPL-3.0-or-later. See [`Cargo.toml`](Cargo.toml).
