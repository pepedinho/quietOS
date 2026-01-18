use crate::println;

#[inline(always)]
pub fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
    }
    ((high as u64) << 32) | (low as u64)
}

pub struct BenchResult {
    pub name: &'static str,
    pub cycles: u64,
}

pub fn print_results(results: &[BenchResult]) {
    for r in results {
        println!("{:<26} {:>10}", r.name, r.cycles);
    }
}

#[macro_export]
macro_rules! bench {
    ($name:expr, $block:block) => {{
        let start = $crate::bench::rdtsc();
        $block
        let end = $crate::bench::rdtsc();
        $crate::bench::BenchResult {
            name: $name,
            cycles: end - start,
        }
    }};
}

#[macro_export]
macro_rules! bench_avg {
    ($name:expr, $count:expr, $block:block) => {{
        let mut total = 0u64;
        for _ in 0..$count {
            let start = $crate::bench::rdtsc();
            $block
            let end = $crate::bench::rdtsc();
            total += end - start;
        }
        $crate::bench::BenchResult {
            name: $name,
            cycles: total / $count as u64,
        }
    }};
}
