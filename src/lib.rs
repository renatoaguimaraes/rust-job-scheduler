// Cmd is a job request with the program name and arguments.
#[derive(Debug)]
pub struct Cmd {
    // name program path/name
    pub name: String,
    // args program arguments
    pub args: Vec<String>,
}

// Status of the process.
#[derive(Debug)]
pub struct Status {
    // uid process identifier
    pub uid: String,
    // pid process identifier
    pub pid: u32,
    // exit_code of the exited process, or -1 if the process doesn't
    // exited or was terminated by a signal
    pub exit_code: i32,
    // exited reports whether the program has exited
    pub exited: bool,
}

// impl Status {
//     pub fn is_running(&self) -> bool {
//         self.status.exit_code == 0 && !self.status.exited
//     }
// }

use log::{info};
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::process::Command;
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

pub trait Worker {
    fn new() -> Self;
    fn start(&mut self, cmd: Cmd) -> Result<String, String>;
}

pub struct InMemoryWorker {
    data: Arc<Mutex<HashMap<String, Status>>>,
}

impl Worker for InMemoryWorker {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn start(&mut self, cmd: Cmd) -> Result<String, String> {
        info!("Process starting: {:?}", cmd);

        let uid = Uuid::new_v4();

        let mut dir = temp_dir();
        let file_name = format!("{}.log", uid.to_string());
        dir.push(file_name);

        let log_file = File::create(dir).unwrap();
        info!("Process log: {:?}", log_file);

        let mut child = match Command::new(cmd.name)
            .args(cmd.args)
            .stdout(log_file)
            .spawn()
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        info!("Process running: uid({}) pid({})", uid.to_string(), child.id());

        let status = Status {
            uid: uid.to_string(),
            pid: child.id(),
            exit_code: 0,
            exited: true,
        };

        let mut map = match self.data.lock() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        map.insert(uid.to_string(), status);

        let data = self.data.clone();
        thread::spawn(move || {
            let exit_status = child.wait().unwrap();

            let status = Status {
                uid: uid.to_string(),
                pid: child.id(),
                exit_code: match exit_status.code() {
                    Some(code) => code,
                    None => 0,
                },
                exited: exit_status.success(),
            };
            info!("Process finished: {:?}", status);

            let mut map = data.lock().unwrap();
            map.insert(uid.to_string(), status);
        });

        Ok(uid.to_string())
    }
}
