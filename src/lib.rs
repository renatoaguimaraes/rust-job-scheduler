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

use log::{info, error};
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::process::Command;
use std::result::Result;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{thread};
use uuid::Uuid;
use chrono::prelude::Utc;

#[tonic::async_trait]
pub trait Worker {
    fn start(&self, cmd: Cmd) -> Result<String, String>;
    fn query(&self, uid: String) -> Result<Status, String>;
    fn stop(&self, uid: String) -> Result<bool, String>;
    async fn stream(&self, uid: String, tx: &mut mpsc::Sender<String>);
}

#[derive(Clone)]
pub enum WorkerEnum {
    InMemoryWorker(InMemoryWorker),
}

#[tonic::async_trait]
impl Worker for WorkerEnum {
    fn start(&self, cmd: Cmd) -> Result<String, String> {
        match self {
            WorkerEnum::InMemoryWorker(in_memory) => in_memory.start(cmd),
        }
    }

    fn query(&self, uid: String) -> Result<Status, String> {
        match self {
            WorkerEnum::InMemoryWorker(in_memory) => in_memory.query(uid),
        }
    }

    fn stop(&self, uid: String) -> Result<bool, String> {
        match self {
            WorkerEnum::InMemoryWorker(in_memory) => in_memory.stop(uid),
        }
    }

    async fn stream(&self, uid: String, tx: &mut mpsc::Sender<String>) {
        match self {
            WorkerEnum::InMemoryWorker(in_memory) => in_memory.stream(uid, tx).await,
        }
    }
}

#[derive(Clone)]
pub struct InMemoryWorker {
    data: Arc<Mutex<HashMap<String, Status>>>,
}

impl Default for InMemoryWorker {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl Worker for InMemoryWorker {
    fn start(&self, cmd: Cmd) -> Result<String, String> {
        info!("Process starting: {:?}", cmd);
        // uuid to map the process status
        let uid = Uuid::new_v4();
        // create log file to stream later
        let mut dir = temp_dir();
        let file_name = format!("{}.log", uid.to_string());
        dir.push(file_name);
        let log_file = File::create(dir).unwrap();
        info!("Process log: {:?}", log_file);
        // spawn the command
        let mut child = match Command::new(cmd.name)
            .args(cmd.args)
            .stdout(log_file)
            .spawn()
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        info!(
            "Process running: uid({}) pid({})",
            uid.to_string(),
            child.id()
        );
        // create initial process status
        let status = Status {
            uid: uid.to_string(),
            pid: child.id(),
            exit_code: 0,
            exited: true,
        };
        // aquire lock and store initial process status
        let mut map = match self.data.lock() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        map.insert(uid.to_string(), status);
        // clone map references before pass to the thread
        let data = self.data.clone();
        // tread to wait process exit to get process info
        thread::spawn(move || {
            // wait for process
            let exit_status = child.wait().unwrap();
            // create status with process info
            let status = Status {
                uid: uid.to_string(),
                pid: child.id(),
                exit_code: exit_status.code().unwrap(),
                exited: exit_status.success(),
            };
            info!("Process finished: {:?}", status);
            // aquire lock and update process status
            let mut map = data.lock().unwrap();
            map.entry(uid.to_string()).or_insert(status);
        });
        Ok(uid.to_string())
    }

    fn query(&self, uid: String) -> Result<Status, String> {
        let map = self.data.lock().unwrap();
        let status = map.get(&uid).unwrap();
        // manual copy, Status has a property String which
        // doesn't implments the trait Copy
        Ok(Status {
            uid: status.uid.clone(),
            pid: status.pid,
            exit_code: status.exit_code,
            exited: status.exited,
        })
    }

    fn stop(&self, uid: String) -> Result<bool, String> {
        let map = self.data.lock().unwrap();
        let status = map.get(&uid).unwrap();
        unsafe {
            match libc::kill(status.pid as i32, libc::SIGTERM) {
                0 => return Ok(true),
                _ => return Err(format!("Error while stop process: {}", uid)),
            };
        }
    }

    async fn stream(&self, uid: String, tx: &mut mpsc::Sender<String>) {
      match tx.send(format!("uid: {} -> {}", uid, Utc::now().to_string())).await {
        Ok(res) => info!("Log stream sent with success: {:?}", res),
        Err(err) => error!("Error to send log stream {:?}", err),
      }
    }
}
