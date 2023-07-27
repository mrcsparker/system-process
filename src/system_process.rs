use std::process::{Command, Output};

// Struct to represent a system process
pub struct SystemProcess {
    // The process itself
    pub process: Command,
    // The output of the process, if it has been run
    pub output: Option<Output>,
}

impl SystemProcess {
    // Construct a new SystemProcess instance with the given process name
    pub fn new(process_name: &str) -> Self {
        SystemProcess {
            process: Command::new(process_name),
            output: None,
        }
    }

    // Set the command-line arguments for the process
    pub fn args(mut self, args: &[&str]) -> Self {
        self.process.args(args);
        self
    }

    // Run the process and store its output
    pub fn run(mut self) -> Self {
        self.output = self.process.output().ok();
        self
    }

    // Return the process's output as a string, if available
    pub fn output(&self) -> Option<String> {
        self.output
            .as_ref()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    }

    // Return the process's exit code, if available
    pub fn exit_code(&self) -> Option<i32> {
        self.output
            .as_ref()
            .map(|output| output.status.code().unwrap_or(-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let process = SystemProcess::new("echo").args(&["hello", "world"]).run();
        assert!(process.output.is_some());
    }

    #[test]
    fn test_output() {
        let process = SystemProcess::new("echo").args(&["hello", "world"]).run();
        assert_eq!(process.output(), Some("hello world\n".to_string()));
    }

    #[test]
    fn test_exit_code() {
        let process = SystemProcess::new("echo").args(&["hello", "world"]).run();
        assert_eq!(process.exit_code(), Some(0));
    }

    #[test]
    fn test_system_process() {
        let process = SystemProcess::new("ls").args(&["-l", "-a"]).run();

        assert!(process.output().is_some());
        assert_eq!(process.exit_code(), Some(0));

        let output = process.output().unwrap();
        assert!(output.contains("total"));
    }
}
