use crate::GlobalSystem;
use sysinfo::{Process, ProcessExt, SystemExt};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyProcess {
    pid: usize,
    name: String,
    cpu_usage: f64,
    run_time: u64,
    run_time_str: String,
}

impl From<&Process> for MyProcess {
    fn from(process: &Process) -> Self {
        let run_time = process.run_time();
        let run_time_str = format!(
            "{}:{:02}:{:02}",
            run_time / 3600,
            run_time % 3600 / 60,
            run_time % 60
        );
        let cpu_usage = (process.cpu_usage() as f64 * 10.0).round() / 10.0;
        if process.name().to_string() == "clion" {
            println!("{} {}", process.start_time(), process.run_time());
        }
        Self {
            pid: process.pid().into(),
            name: process.name().to_string(),
            cpu_usage,
            run_time,
            run_time_str,
        }
    }
}

#[tauri::command]
pub fn sys_info(sys: tauri::State<'_, GlobalSystem>) -> Vec<MyProcess> {
    let mut sys = sys.0.lock().unwrap();
    sys.refresh_all();
    sys.refresh_disks();
    let h = sys.processes();
    let mut v = vec![];
    h.iter().for_each(|(_, p)| {
        let my_process = MyProcess::from(p);
        v.push(my_process);
    });
    v.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap().reverse());
    v
}
