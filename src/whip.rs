use std::path::PathBuf;
use std::process;
use std::process::Command;

#[allow(dead_code)]
pub struct Compiler {
    path: PathBuf
}

#[allow(dead_code)]
pub struct Target {
    name: String,
    compiler: Compiler,
    files: Vec<PathBuf>
}

#[allow(dead_code)]
impl Target {
    pub fn build(&self) {
        let output = Command::new(
            self.compiler.path
                .clone()
                .into_os_string()
                .to_str()
                .unwrap())
            .args(&["-o", self.name.as_str()])
            .args(self.files.clone())
            .output()
            .expect("Failed to run build");

        use std::io::{self, Write};

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        if !output.status.success() {
            println!("Build failed");
            process::exit(1);
        }
    }
}
