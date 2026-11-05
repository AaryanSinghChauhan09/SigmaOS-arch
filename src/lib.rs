#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch { X86_64, AArch64, Riscv64 }

#[derive(Debug)]
pub struct ArchContext {
    pub arch: TargetArch,
    pub paging_levels: u8,
}

impl ArchContext {
    pub fn new_x86_64() -> Self {
        Self { arch: TargetArch::X86_64, paging_levels: 4 }
    }
    pub fn is_64bit(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arch_context_creation() {
        let ctx = ArchContext::new_x86_64();
        assert_eq!(ctx.arch, TargetArch::X86_64);
        assert!(ctx.is_64bit());
    }
}
