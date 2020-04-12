use std::io;
use std::io::Write;
use std::process::Command;

use syn;
use syn::Stmt;
use quote::quote;
use std::path::PathBuf;

use crate::utils;

/// Assemble the parsed statements from the .whip file into a valid rust main module
fn assemble_build_tokens(_whip_statements: &Vec<Stmt>, _statements: &Vec<Stmt>) -> proc_macro2::TokenStream {
    quote! {
        #(#_whip_statements)*

        fn main() {
            #(#_statements)*
        }
    }
}

/// Compile the parsed statements into a build executable in the given directory
pub fn compile(whip_statements: &Vec<Stmt>, statements: &Vec<Stmt>, directory: &PathBuf) -> io::Result<PathBuf> {
    let main_tokens = assemble_build_tokens(whip_statements, statements);
    let tmp_file_path = utils::write_tmp_file(&main_tokens.to_string(), directory)?;

    println!("Compiling the whip build binary...");
    //TODO Call rustc through the library interface instead of depending on its existence
    let intermediate_executable_path = directory.join(utils::TMP_DIR).join("build");

    let output = Command::new("rustc")
        .arg(tmp_file_path)
        .arg("-o")
        .arg(&intermediate_executable_path)
        .output()?;

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(intermediate_executable_path)
}
