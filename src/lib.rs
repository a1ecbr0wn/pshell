use std::process;
use sysinfo::Process;
use sysinfo::{Pid, System, IS_SUPPORTED_SYSTEM};

/// Traverses up the parent processes to see if the name of any of them matches a `KNOWN_SHELL`
pub fn find() -> Option<(String, u32)> {
    const KNOWN_SHELLS: [&str; 10] = [
        "bash",
        "zsh",
        "fish",
        "tcsh",
        "dash",
        "csh",
        "ksh",
        "cmd",
        "powershell",
        "pwsh",
    ];

    if IS_SUPPORTED_SYSTEM {
        let sys = System::new_all();
        let mut process: Option<&Process> = sys.process(Pid::from_u32(process::id()));
        while process.is_some() {
            process = get_parent(&sys, process.unwrap());
            if let Some(shell) = process {
                if let Some(shell_name) = shell.name().to_str() {
                    let mut shell_name = shell_name;
                    if shell_name.ends_with(".exe") {
                        shell_name = &shell_name[..shell_name.len() - 4];
                    }
                    if KNOWN_SHELLS.contains(&shell_name) {
                        return Some((shell_name.to_owned(), shell.pid().as_u32()));
                    }
                };
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
