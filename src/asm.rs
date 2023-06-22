use std::arch::asm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn  asm_tick_counter() -> u64 {
    let mut reg_eax:u32;
    let mut reg_edx:u32;

    unsafe {
        asm!("rdtsc", out("eax") reg_eax, out("edx") reg_edx);
    }

    (reg_edx as u64) << 32 | reg_eax as u64
}

#[test]
fn test_asm_rdtsc() {
    use core::arch::x86_64::_rdtsc;

    let counter1 = asm_tick_counter();
    let counter2 = asm_tick_counter();

    let counter3 = unsafe {_rdtsc()};
    let counter4 = unsafe {_rdtsc()};

    assert!(counter1 < counter2);
    assert!(counter3 < counter4);
}

/// adding documentation 
/// test_asm1: using the inline assembly feature
#[test]
fn test_asm1() {
        let i: u64 = 3;
        let mut o: u64 = 1;
        assert_eq!(o, 1); 
        assert_ne!(o, i); 
        unsafe {
            asm!("mov {0}, {1}", out(reg) o, in(reg) i);
        }
        assert_eq!(o, i); 
}

