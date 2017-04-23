use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{self, ErrorKind, Write};

/// Just a `println!()` like macro printing on stderr.
macro_rules! errln {
    ($($arg:tt)*) => { {
        use std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
}

fn main() {
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
    compile_less(&manifest_dir);
}

/// Compiles `.less` files with `lessc` which is assumed to be installed on
/// the local machine.
fn compile_less(manifest_dir: &Path) {
    // In and out paths
    let less_dir = manifest_dir.join(
        env::var("LESS_DIR").unwrap_or("less/".into())
    );
    let out_dir = manifest_dir.join(
        env::var("STATIC_ASSET_DIR").unwrap_or("static/".into())
    );

    // If it's being compiled in release mode, we want to minify the
    // resulting CSS.
    //
    // We might want to check for Rocket environments instead of the cargo
    // build profile later (TODO).
    let is_debug = env::var("PROFILE").unwrap() == "debug";
    let minify_flag = ["--clean-css"];
    let flags = if is_debug { &[] as &[_] } else { &minify_flag };

    // Execute the compiler
    let res = Command::new("lessc")
        .args(flags)
        .arg(&less_dir.join("main.less"))
        .arg(&out_dir.join("main.css"))
        .output();

    // Check if anything went wrong
    match res {
        Err(e) => {
            errln!("An IO error occured while running the less-compiler:");
            errln!(" >> {}", e);

            if e.kind() == ErrorKind::NotFound {
                errln!("!! Make sure you have installed `lessc` and it's in your $PATH! !!");
            }
            errln!("");

            panic!("error compiling less files");
        }
        Ok(output) => {
            // If everything went well, we don't expect any output
            if !output.status.success() || !output.stdout.is_empty() {
                errln!("`lessc` exited unsuccessful!");

                errln!("--- stdout ---");
                io::stderr().write_all(&output.stdout)
                    .expect("IO error printing to stderr");
                errln!("--- stderr ---");
                io::stderr().write_all(&output.stderr)
                    .expect("IO error printing to stderr");

                panic!("error compiling less files");
            }
        }
    }
}
