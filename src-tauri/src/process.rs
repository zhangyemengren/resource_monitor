use sysinfo::{Process, ProcessExt, System, SystemExt};

#[derive(serde::Serialize)]
pub struct MyProcess {
    pid: usize,
    name: String,
}

impl From<Process> for MyProcess  {
    fn from(process: Process) -> Self {
        Self {
            pid: process.pid().into(),
            name: process.name().to_string(),
        }
    }
}
impl From<&Process> for MyProcess  {
    fn from(process: &Process) -> Self {
        Self {
            pid: process.pid().into(),
            name: process.name().to_string(),
        }
    }
}

#[tauri::command]
pub fn sys_info() -> Vec<MyProcess>{
    let mut sys = System::new_all();
    sys.refresh_all();
    let h = sys.processes();
    let mut v = vec![];
    h.iter().for_each(|(_, p)| {
        v.push(MyProcess::from(p));
    });
    v
}
