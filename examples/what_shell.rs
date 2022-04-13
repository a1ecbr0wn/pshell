use pshell;

fn main() {
    // `find` returns the name of the shell in a string and the pid as a u32
    let (sh, pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
    println!(
        "This application has been run from pid `{}`, which is a {} shell",
        pid, sh
    );
}
