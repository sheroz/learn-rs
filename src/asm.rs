use std::arch::asm;

#[cfg(target_arch = "x86_64")]
pub fn asm_tick_counter() -> u64 {
    let mut reg_eax: u32;
    let mut reg_edx: u32;

    unsafe {
        asm!("rdtsc", out("eax") reg_eax, out("edx") reg_edx);
    }

    (reg_edx as u64) << 32 | reg_eax as u64
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn asm_tick_counter() -> u64 {
    let tick_counter: u64;
    unsafe {
        asm!(
            "mrs x0, cntvct_el0",
            out("x0") tick_counter
        );
    }
    tick_counter
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn asm_tick_counter_frequency() -> u64 {
    let tick_frequency: u64;
    unsafe {
        asm!(
            "mrs x0, cntfrq_el0",
            out("x0") tick_frequency
        );
    }
    tick_frequency
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

#[cfg(not(target_arch = "x86_64"))]
pub fn asm_tick_counter_processor_id() -> u64 {
    unsupported_architecture();
    0
}

#[cfg(target_arch = "x86_64")]
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

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn asm_tick_counter_start() -> u64 {
    asm_tick_counter()
}

#[cfg(target_arch = "x86_64")]
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

#[cfg(target_arch = "aarch64")]
#[inline]
 pub fn asm_tick_counter_stop() -> u64 {
    asm_tick_counter()
}

#[allow(dead_code)]
fn unsupported_architecture() {
    panic!("\nError! Architecture found: {}\nCurrently, only x86_64 architecture is supported!", std::env::consts::ARCH);
}

#[cfg(test)]
mod tests {
    
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_asm_aarch64_tick_counter() {
        use std::{thread::sleep, time::Duration};

        use super::*;
        const DURATION_IN_MILLISECONDS: u64 = 200; 

        let counter_start = asm_tick_counter_start();
        sleep(Duration::from_millis(DURATION_IN_MILLISECONDS));
        let counter_stop = asm_tick_counter_stop();
        println!("counter_start: {}. counter_end: {}", counter_start, counter_stop);
        assert!(counter_start < counter_stop);

        let counter_diff = counter_stop - counter_start;
        assert!(counter_diff > 0);

        let calculated_frequency = counter_diff * (1000 / DURATION_IN_MILLISECONDS);
        println!("calculated_frequency: {}", calculated_frequency);

        let counter_frequency = asm_tick_counter_frequency();
        println!("cpu_counter_frequency: {}", counter_frequency);
        assert!(counter_frequency > 0);
    }
    
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_asm_x86_64_counters() {
        use super::*;
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
    #[cfg(target_arch = "x86_64")]
    fn test_asm_x86_64() {
        use std::arch::asm;

        let i: u64 = 3;
        let mut o: u64 = 1;
        assert_eq!(o, 1);
        assert_ne!(o, i);
        unsafe {
            asm!("mov {0}, {1}", out(reg) o, in(reg) i);
        }
        assert_eq!(o, i);
    }
}
