use std::process;
use sysinfo::Process;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

/// Traverses up the parent processes to see if the name of any of them matches a `KNOWN_SHELL`
pub fn find() -> Option<(String, u32)> {
    const KNOWN_SHELLS: [&str; 4] = ["bash", "zsh", "fish", "tcsh"];

    if System::IS_SUPPORTED {
        let sys = System::new_all();
        let mut process: Option<&Process> = sys.process(Pid::from_u32(process::id()));
        while process.is_some() {
            process = get_parent(&sys, process.unwrap());
            if let Some(shell) = process {
                if KNOWN_SHELLS.contains(&(shell.name())) {
                    return Some((shell.name().to_string(), shell.pid().as_u32()));
                }
            }
        }
        None
    } else {
        None
    }
}

fn get_parent<'a>(sys: &'a System, process: &Process) -> Option<&'a Process> {
    if let Some(ppid) = process.parent() {
        if let Some(parent) = sys.process(ppid) {
            Some(parent)
        } else {
            None
        }
    } else {
        None
    }
}
