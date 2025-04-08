/*use std::collections::HashMap;
use std::io;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use nix::errno::Errno;
use nix::sys::signal::{self, SigHandler, Signal};
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::Pid;

// Job structure to hold information about background processes
struct Job {
    id: u32,
    pid: Pid,
    command: String,
}

lazy_static::lazy_static! {
    static ref JOBS: Mutex<HashMap<Pid, Job>> = Mutex::new(HashMap::new());
    static ref JOB_COUNTER: Mutex<u32>  = Mutex::new(1);
}

// Function to get the next job ID
fn next_job_id() -> u32 {
    let mut counter = JOB_COUNTER.lock().unwrap();
    let id = *counter;
    *counter += 1;
    id
}

pub fn launch_background_process(command: &mut Command) -> io::Result<()> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());

    let child = command.spawn()?;
    let pid = Pid::from_raw(child.id() as i32);

    let job_id = next_job_id();

    let job = Job {
        id: job_id,
        pid,
        command: format!("{:?}", command), // Store the command string
    };

    JOBS.lock().unwrap().insert(pid, job);

    println!("[{}] {} started", job_id, pid);

    Ok(())
}

// Function to handle SIGCHLD signals
extern "C" fn sigchld_handler(_: i32) {
    loop {
        match waitpid(None, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, status)) => {
                if let Some(job) = JOBS.lock().unwrap().remove(&pid) {
                    println!("[{}] {} exited with status {}", job.id, pid, status);
                }
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                if let Some(job) = JOBS.lock().unwrap().remove(&pid) {
                    println!("[{}] {} terminated by signal {}", job.id, pid, signal);
                }
            }
            Ok(WaitStatus::Stopped(pid, signal)) => {
                println!("Process {} stopped by signal {}", pid, signal);
            }
            Ok(WaitStatus::Continued(pid)) => {
                println!("Process {} continued", pid);
            }
            Ok(WaitStatus::PtraceEvent(pid, signal, _)) => {
                println!("Process {} ptrace event {} ", pid, signal);
            }
            Ok(WaitStatus::PtraceSyscall(pid)) => {
                println!("Process {} ptrace syscall", pid);
            }
            Ok(WaitStatus::StillAlive) => {
                break;
            }
            Err(Errno::ECHILD) => {
                break; // No more children to wait for
            }
            Err(e) => {
                eprintln!("waitpid error: {}", e);
                break;
            }
        }
    }
}

pub fn manage_background_processes() -> io::Result<()> {
    // Set up the SIGCHLD handler
    unsafe {
        signal::signal(Signal::SIGCHLD, SigHandler::Handler(sigchld_handler))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok(())
}

pub fn list_jobs() {
    let jobs = JOBS.lock().unwrap();
    if jobs.is_empty() {
        println!("No currently running background jobs.");
    } else {
        println!("Currently running background jobs:");
        for (pid, job) in jobs.iter() {
            println!("[{}] {}  {}", job.id, pid, job.command);
        }
    }
}

pub fn bring_job_to_foreground(job_id: u32) -> io::Result<()> {
    let mut jobs = JOBS.lock().unwrap();
    let (pid, command) = match jobs.iter().find(|(_, job)| job.id == job_id) {
        Some((&pid, job)) => (pid, job.command.clone()),
        None => {
            eprintln!("Job with id {} not found", job_id);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Job not found"));
        }
    };

    jobs.remove(&pid);

    let mut cmd = Command::new(command.split_whitespace().next().unwrap_or(""));
    cmd.args(command.split_whitespace().skip(1));
    let status = cmd.status()?;

    if let Some(code) = status.code() {
        println!("Foreground job exited with status: {}", code);
    } else if status.success() {
        println!("Foreground job completed successfully.");
    } else {
        println!("Foreground job terminated abnormally.");
    }

    Ok(())
}
*/
