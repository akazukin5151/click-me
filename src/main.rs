use std::path::Path;
use std::process::ExitStatus;
use std::process::Command;
use std::fs;
use std::thread;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_os = "linux")]
fn include_bin() -> &'static [u8] {
    include_bytes!("../http-server-vendor/http-server-linux")
}

#[cfg(target_os = "macos")]
fn include_bin() -> &'static [u8] {
    include_bytes!("../http-server-vendor/http-server-macos")
}

#[cfg(target_os = "windows")]
fn include_bin() -> &'static [u8] {
    include_bytes!("../http-server-vendor/http-server-windows.exe")
}

#[cfg(unix)]
fn set_executable(path: &'static str) {
    let file = fs::File::open(path).unwrap();
    let mut p = file.metadata().unwrap().permissions();
    p.set_mode(755);
    fs::set_permissions(path, p).unwrap();
}

#[cfg(target_os = "windows")]
fn set_executable(path: &'static str) {
}

fn execute_bin(path: &'static str) -> Result<ExitStatus, &str> {
    Command::new(path).arg("--cors").status().map_err(|_| "Failed to launch command")
}

#[cfg(unix)]
const EXE_PATH: &'static str = "/tmp/http-server";

#[cfg(target_os = "windows")]
const EXE_PATH: &'static str = "/tmp/http-server.exe";

fn main() {
    if Path::new(EXE_PATH).exists() {
        fs::remove_file(EXE_PATH).unwrap();
    };
    let exe = include_bin();
    fs::write(EXE_PATH, exe).unwrap();
    set_executable(EXE_PATH);
    thread::spawn(move || {
        execute_bin(EXE_PATH).unwrap();
    });
    open::that("http://0.0.0.0:8000/index.html").unwrap();
}
