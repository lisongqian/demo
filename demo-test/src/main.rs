use std::io;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

fn kill_child(child: &Child) {
    let pid = child.id() as i32;
    let ret = unsafe { libc::kill(pid, libc::SIGTERM) };
    if ret != 0 {
        eprintln!("error:{:?}", io::Error::last_os_error());
    }
}

fn main() {
    let child = Command::new("./demo")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    sleep(Duration::from_secs(2));
    kill_child(&child);
    let output = child.wait_with_output().unwrap();
    println!(
        "\n\n==== Start child stdout ====\n\n{}\n\n==== End child stdout ====",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "\n\n==== Start child stderr ====\n\n{}\n\n==== End child stderr ====",
        String::from_utf8_lossy(&output.stderr)
    );
}
