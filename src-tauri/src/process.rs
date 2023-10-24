use sysinfo::{Pid, System, SystemExt};

#[tauri::command]
pub fn sys_info() -> Vec<usize>{
    let mut sys = System::new_all();
    sys.refresh_all();
    let h = sys.processes();
    let mut v = vec![];
    h.iter().for_each(|(pid, _)| {
        v.push((*pid).into());
    });
    v
}
