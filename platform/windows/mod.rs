
use sandbox::{ChildSandboxMethods, Command, SandboxMethods};

use std::io;

pub mod process;

use platform::windows::process::Process;
use profile::{self, AddressPattern, OperationSupport, OperationSupportLevel, PathPattern, Profile};

impl OperationSupport for profile::Operation {
    fn support(&self) -> OperationSupportLevel {
        OperationSupportLevel::NeverAllowed
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
}

pub struct Sandbox {
    profile: Profile,
}

impl Sandbox {
    pub fn new(profile: Profile) -> Sandbox {
        println!("Error: gaol sandbox not implemented for windows.");
        Sandbox {
            profile: profile,
        }
    }
}

impl SandboxMethods for Sandbox {
    fn profile(&self) -> &Profile {
        &self.profile
    }

    fn start(&self, command: &mut Command) -> io::Result<Process> {
        println!("Error: gaol sandbox not implemented for windows.");
        Err(io::Error::new(io::ErrorKind::Other, "Error: gaol sandbox not implemented for windows."))
    }
}

pub struct ChildSandbox {
    profile: Profile,
}

impl ChildSandbox {
    pub fn new(profile: Profile) -> ChildSandbox {
        println!("Error: gaol sandbox not implemented for windows.");
        ChildSandbox {
            profile: profile,
        }
    }
}

impl ChildSandboxMethods for ChildSandbox {
    fn activate(&self) -> Result<(),()> {
        println!("Error: gaol sandbox not implemented for windows.");
        Err(())
    }
}