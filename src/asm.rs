use std::arch::asm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn asm_tick_counter() -> u64 {
    let mut reg_eax: u32;
    let mut reg_edx: u32;

    unsafe {
        asm!("rdtsc", out("eax") reg_eax, out("edx") reg_edx);
    }

    (reg_edx as u64) << 32 | reg_eax as u64
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn asm_tick_counter_start() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            "mfence",
            "lfence",
            "rdtsc",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") rax
        );
    }
    rax
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn asm_tick_counter_stop() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            "rdtsc",
            "lfence",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") rax
        );
    }
    rax
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn asm_tick_counter_processor_id() -> (u64, u32) {
    let mut reg_eax: u32;
    let mut reg_edx: u32;
    let mut reg_ecx: u32;

    unsafe {
        asm!("rdtscp", out("eax") reg_eax, out("edx") reg_edx, out("ecx") reg_ecx);
    }

    ((reg_edx as u64) << 32 | reg_eax as u64, reg_ecx)
}

#[test]
fn test_asm_rdtsc() {
    use core::arch::x86_64::__rdtscp;
    use core::arch::x86_64::_rdtsc;

    let counter1 = asm_tick_counter();
    let counter2 = asm_tick_counter();
    let diff_tick_counter = counter2 - counter1;
    assert!(counter1 < counter2);
    assert!(diff_tick_counter > 0);

    let counter_start = asm_tick_counter_start();
    let counter_stop = asm_tick_counter_stop();
    let diff_tick_counter2 = counter_stop - counter_start;
    assert!(counter_start < counter_stop);
    assert!(diff_tick_counter2 > 0);

    let counter3 = unsafe { _rdtsc() };
    let counter4 = unsafe { _rdtsc() };
    let diff_tick_rdtsc = counter4 - counter3;
    assert!(counter3 < counter4);
    assert!(diff_tick_rdtsc > 0);

    let mut ecx: u32 = 0;
    let ptr_ecx: *mut u32 = (&mut ecx) as *mut u32;
    let counter5 = unsafe { __rdtscp(ptr_ecx) };
    let cpu_core_id_1 = ecx;

    let counter6 = unsafe { __rdtscp(ptr_ecx) };
    let cpu_core_id_2 = ecx;
    let diff_tick_rdtscp = counter6 - counter5;

    assert!(counter5 < counter6);
    assert!(diff_tick_rdtscp > 0);
    assert!(cpu_core_id_1 == cpu_core_id_2);

    let (counter7, cpu_core_id_3) = asm_tick_counter_processor_id();
    let (counter8, cpu_core_id_4) = asm_tick_counter_processor_id();
    let diff_tick_asm_rdtscp = counter8 - counter7;

    assert!(counter7 < counter8);
    assert!(cpu_core_id_1 == cpu_core_id_3);
    assert!(cpu_core_id_3 == cpu_core_id_4);
    assert!(diff_tick_asm_rdtscp > 0);

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
