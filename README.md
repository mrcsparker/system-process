# SystemProcess

Are you tired of boring old command line tools? Look no further!
With SystemProcess, you can add some excitement to your life by running system commands and printing their output and exit code.

```rust
use system_process::SystemProcess;

fn main() {
    let process = SystemProcess::new("ls").args(&["-l", "-a"]).run();

    if let Some(output) = process.output() {
        println!("🤖 process output: {}", output);
    }

    if let Some(exit_code) = process.exit_code() {
        println!("🤖 process exit code: {}", exit_code);
    }
}
```

The `SystemProcess::new()` function is used to create a new SystemProcess instance, passing in the command to be run. The `args()` method is used to specify any additional arguments to be passed to the command. Finally, the run() method is used to execute the command and make your dreams come true.

The `output()` method returns the standard output of the process, and the `exit_code()` method returns the exit code of the process. These values can be accessed using pattern matching, as shown in the example above.

The `SystemProcess` struct provides several other methods for managing system processes, including the ability to kill a running process, set timeouts, and more. Refer to the documentation for more information. Happy hacking! 💻
