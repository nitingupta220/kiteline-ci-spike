//! Release-pipeline spike: smallest binary that exercises the per-OS process scan.
use std::time::Instant;

#[cfg(target_os = "linux")]
fn count_procs() -> usize {
    std::fs::read_dir("/proc").map(|d| d.filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().chars().all(|c| c.is_ascii_digit())).count()).unwrap_or(0)
}

#[cfg(target_os = "macos")]
fn count_procs() -> usize {
    libproc::processes::pids_by_type(libproc::processes::ProcFilter::All).map(|v| v.len()).unwrap_or(0)
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    if arg == "--version" || arg == "-V" { println!("kiteline {}", env!("CARGO_PKG_VERSION")); return; }
    let t = Instant::now(); let n = count_procs();
    println!("kiteline {} {}-{}: scanned {} processes in {:?}", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH, n, t.elapsed());
}
