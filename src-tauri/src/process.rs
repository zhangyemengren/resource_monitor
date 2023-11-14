use crate::GlobalSystem;
use std::time::SystemTime;
use sysinfo::{DiskUsage, ProcessExt, SystemExt, UserExt};

#[derive(serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MyProcess {
    pid: usize,
    name: String,
    cpu_usage: f64,
    run_time: u64,
    run_time_str: String,
    user_name: String,
    memory: u64,
    memory_str: String,
    read_bytes: u64,
    written_bytes: u64,
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
        let cpu_usage = (cpu_usage * 10.0).round() / 10.0;
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
    pub fn memory_usage(mut self, memory: u64) -> Self {
        let memory_str;
        if memory > 1024_u64.pow(3) {
            let round = (memory as f32 / 1024.0_f32.powf(3.0) * 100.0).round() / 100.0;
            memory_str = format!("{} GB", round);
        } else {
            let round = (memory as f32 / 1024.0_f32.powf(2.0) * 10.0).round() / 10.0;
            memory_str = format!("{} MB", round);
        }
        self.memory = memory;
        self.memory_str = memory_str;
        self
    }
    pub fn disk_usage(mut self, disk_usage: DiskUsage) -> Self {
        self.read_bytes = disk_usage.read_bytes;
        self.written_bytes = disk_usage.written_bytes;
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
            memory: self.memory,
            memory_str: self.memory_str,
            read_bytes: self.read_bytes,
            written_bytes: self.written_bytes,
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
            .pid(p.pid().into())
            .name(p.name().to_string())
            .cpu_usage(p.cpu_usage().into())
            .run_time(p.start_time())
            .user_name(user_name)
            .memory_usage(p.memory())
            .disk_usage(p.disk_usage())
            .build();
        v.push(my_process);
    });
    v.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap().reverse());
    v
}

#[tauri::command]
pub fn find_process(sys: tauri::State<'_, GlobalSystem>, search: String) -> Vec<MyProcess> {
    let all = sys_info(sys);
    all.into_iter().filter(|s| s.name.contains(&search)).collect()
}
