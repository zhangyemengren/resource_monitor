use sysinfo::{Process, ProcessExt, SystemExt};
use crate::GlobalSystem;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyProcess {
    pid: usize,
    name: String,
    cpu_usage: String,
}

impl From<Process> for MyProcess  {
    fn from(process: Process) -> Self {
        Self {
            pid: process.pid().into(),
            name: process.name().to_string(),
            cpu_usage: format!("{:.1}%", process.cpu_usage() * 100.0),
        }
    }
}
impl From<&Process> for MyProcess  {
    fn from(process: &Process) -> Self {
        Self {
            pid: process.pid().into(),
            name: process.name().to_string(),
            cpu_usage: format!("{:.1}%", process.cpu_usage() * 100.0),
        }
    }
}

#[tauri::command]
pub fn sys_info(sys: tauri::State<'_, GlobalSystem>,) -> Vec<MyProcess>{
    let mut sys = sys.0.lock().unwrap();
    sys.refresh_all();
    let h = sys.processes();
    let mut v = vec![];
    h.iter().for_each(|(_, p)| {
        v.push(MyProcess::from(p));
    });
    v.sort_by(|a, b| a.name.cmp(&b.name));
    v
}
