use std::time::SystemTime;
use crate::GlobalSystem;
use sysinfo::{ProcessExt, SystemExt, UserExt};

#[derive(serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MyProcess {
    pid: usize,
    name: String,
    cpu_usage: f64,
    run_time: u64,
    run_time_str: String,
    user_name: String,
}

impl MyProcess {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pid(mut self, pid: usize) -> Self {
        self.pid = pid;
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn cpu_usage(mut self, cpu_usage: f64) -> Self {
        self.cpu_usage = cpu_usage;
        self
    }
    pub fn run_time(mut self, start_time: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let run_time = now - start_time;
        let run_time_str = format!(
            "{}:{:02}:{:02}",
            run_time / 3600,
            run_time % 3600 / 60,
            run_time % 60
        );
        self.run_time = run_time;
        self.run_time_str = run_time_str;
        self
    }
    pub fn user_name(mut self, user_name: String) -> Self {
        self.user_name = user_name;
        self
    }
    pub fn build(self) -> Self {
        Self {
            pid: self.pid,
            name: self.name,
            cpu_usage: self.cpu_usage,
            run_time: self.run_time,
            run_time_str: self.run_time_str,
            user_name: self.user_name,
        }
    }
}

#[tauri::command]
pub fn sys_info(sys: tauri::State<'_, GlobalSystem>) -> Vec<MyProcess> {
    let mut sys = sys.0.lock().unwrap();
    sys.refresh_all();
    let h = sys.processes();
    let mut v = vec![];
    h.iter().for_each(|(_, p)| {
        let pid: usize = p.pid().into();
        let name = p.name().to_string();
        let cpu_usage = (p.cpu_usage() as f64 * 10.0).round() / 10.0;
        let start_time = p.start_time();
        let user_name = if let Some(user_id) = p.user_id() {
            if let Some(user) = sys.get_user_by_id(user_id) {
                user.name().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        let my_process = MyProcess::new()
            .pid(pid)
            .name(name)
            .cpu_usage(cpu_usage)
            .run_time(start_time)
            .user_name(user_name)
            .build();
        v.push(my_process);
    });
    v.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap().reverse());
    v
}
