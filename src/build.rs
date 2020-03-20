use quote::quote;
use syn::Stmt;
use std::io;
use std::io::Write;
use std::fs::File;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::process::Command;


pub fn compile(statements: &Vec<Stmt>, directory: String) -> String {
    let whip_tokens = assemble_build_tokens(statements, directory);
    let source_hash = write_intermediate_file(whip_tokens.to_string().as_bytes()).unwrap();

    println!("Compiling the whip build binary...");
    //TODO Call rustc through the library interface instead of depending on its existence
    let intermediate_executable_path = format!("/tmp/whip-up/{}", source_hash);
    let output = Command::new("rustc")
        .arg(format!("/tmp/whip-up/{}.rs", source_hash))
        .args(&["-o", intermediate_executable_path.clone().as_str()])
        .output()
        .expect("Failed to run build");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    intermediate_executable_path
}

fn assemble_build_tokens(_statements: &Vec<Stmt>, _directory: String) -> proc_macro2::TokenStream {
    quote! {
        use std::path::PathBuf;
        use std::process;
        use std::process::Command;

        struct Compiler {
            path: PathBuf
        }

        struct Target {
            name: String,
            compiler: Compiler,
            files: Vec<PathBuf>
        }

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

        fn main() {
            #(#_statements)*
        }
    }
}

fn write_intermediate_file(code: &[u8]) -> io::Result<String> {
    let mut hasher = DefaultHasher::new();

    hasher.write(code);

    let hash = hasher.finish().to_string().to_owned();
    let file_path = format!("/tmp/whip-up/{}.rs", hash);

    println!("Writing intermediate file {} ...", file_path);
    let mut file = File::create(file_path.clone())?;
    file.write_all(code)?;
    Ok(hash)
}
