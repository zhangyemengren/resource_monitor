use sysinfo::Signal::*;
use sysinfo::{CpuExt,  System, SystemExt};

#[tauri::command]
pub fn test_cpu() -> String{
    let mut system = System::new_all();
    let s = format!(
        "number of physical cores: {}",
        &system.physical_core_count()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "Unknown".to_owned()),
    );
    let x =format!(
        "total CPU usage: {}%",
        &system.global_cpu_info().cpu_usage()
    );
    format!("{} {}", s, x)
    // for cpu in &system.cpus() {
    //     writeln!(&mut io::stdout(), "{cpu:?}");
    // }
}