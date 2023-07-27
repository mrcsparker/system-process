use system_process::system_process::SystemProcess;

fn main() {
    let process = SystemProcess::new("ls").args(&["-l", "-a"]).run();

    if let Some(output) = process.output() {
        println!("process output: {}", output);
    }

    if let Some(exit_code) = process.exit_code() {
        println!("process exit code: {}", exit_code);
    }
}
