use std::process::{Command, Stdio};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct ModelResult {
    pub success: bool,
    pub output: String,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct ModelExecutor;

impl ModelExecutor {
    pub fn new() -> Self {
        ModelExecutor
    }

    pub fn execute_model(&self, model_path: &str, input_data: &str) -> Result<ModelResult, Box<dyn std::error::Error>> {
        let child = Command::new("python3")
            .arg(model_path)
            .arg(input_data)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(ModelResult {
            success: output.status.success(),
            output: stdout,
            error: stderr,
        })
    }

    pub fn execute_model_with_stdin(&self, model_path: &str, input_data: &str) -> Result<ModelResult, Box<dyn std::error::Error>> {
        let mut child = Command::new("python3")
            .arg(model_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input_data.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(ModelResult {
            success: output.status.success(),
            output: stdout,
            error: stderr,
        })
    }
}