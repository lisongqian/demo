use std::thread::sleep;


fn main() {
    let mut i = 0;
    while i < 6 {
        println!("Hello, world!");
        sleep(std::time::Duration::from_secs(1));
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    use wait_timeout::ChildExt;

    #[test]
    fn it_works() {
        let mut child = Command::new("target/debug/demo")
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        if child
            .wait_timeout(std::time::Duration::from_secs(5))
            .unwrap()
            .is_none()
        {
            let _ = child.kill();
            eprintln!("Child process timed out");
        }
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
}

