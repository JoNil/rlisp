use std::io::Command;
use std::io::process::ProcessExit;
use std::os;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    let output = match Command::new("make")
            .arg("all")
            .cwd(&Path::new("src/"))
            .output() {
        Ok(output) => output,
        Err(e) => panic!("Failed to run make: {}", e),
    };

    if ProcessExit::ExitStatus(0) != output.status {
        println!("stdout: {}", String::from_utf8_lossy(output.output.as_slice()));
        println!("stderr: {}", String::from_utf8_lossy(output.error.as_slice()));
        panic!("Failed to compile mpc");
    }

    println!("cargo:rustc-flags=-L {} -l mpc:static", out_dir);
}