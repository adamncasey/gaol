use sandbox::Command;

use std::io;

pub fn exec(command: &Command) -> io::Error {
    println!("exec not implemented for gaol on windows");
    io::Error::new(io::ErrorKind::Other, "exec not implemented for gaol on windows")
}

pub fn spawn(command: &Command) -> io::Result<Process> {
    println!("spawn not implemented for gaol on windows");
    Err(io::Error::new(io::ErrorKind::Other, "spawn not implemented for gaol on windows"))
}

pub struct Process {
    pub pid : i64
}
impl Process {
    pub fn wait(&self) -> io::Result<ExitStatus> {
        println!("Process::wait not implemented for gaol windows");
        Err(io::Error::new(io::ErrorKind::Other, "Process::wait not implemented for gaol windows"))
    }
}

pub enum ExitStatus {
    Code(i32),
    Signal(i32),
}

impl ExitStatus {
    #[inline]
    pub fn success(&self) -> bool {
        match *self {
            ExitStatus::Code(0) => true,
            _ => false,
        }
    }
}