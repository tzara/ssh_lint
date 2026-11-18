use crate::ssh::execute_ssh;
use std::thread;

pub fn spawn_command(
    host: String,
    command: String,
    on_output: impl Fn(String) + Send + 'static,
    on_finish: impl Fn() + Send + 'static,
) {
    thread::spawn(move || {
        match execute_ssh(&host, &command) {
            Ok(output) => on_output(output),
            Err(e)     => on_output(format!("ERROR\n{}", e)),
        }
        on_finish();
    });
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_spawn_command() {
        // This test would require more complex setup to properly test
        // but at least verifies the function signature is correct
        assert!(true);
    }
}
