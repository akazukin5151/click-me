use std::path::PathBuf;
use std::time::Duration;
use std::process::ExitStatus;
use std::process::Command;
use std::fs;
use std::thread;
use std::env;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_os = "linux")]
fn include_bin() -> &'static [u8] {
    include_bytes!("../http-server-vendor/http-server-linux")
}

#[cfg(target_os = "macos")]
fn include_bin() -> &'static [u8] {
    include_bytes!("../http-server-vendor/bin/simple-http-server")
}

#[cfg(target_os = "windows")]
fn include_bin() -> &'static [u8] {
    include_bytes!("../http-server-vendor/http-server-windows.exe")
}

#[cfg(unix)]
fn set_executable(path: &PathBuf) {
    let file = fs::File::open(path).unwrap();
    let mut p = file.metadata().unwrap().permissions();
    p.set_mode(755);
    fs::set_permissions(path, p).unwrap();
}

#[cfg(target_os = "windows")]
fn set_executable(path: &PathBuf) {
}

fn execute_bin(path: PathBuf) -> Result<ExitStatus, &'static str> {
    Command::new(path).arg("--cors").status().map_err(|_| "Failed to launch command")
}

#[cfg(unix)]
const EXE_NAME: &'static str = "http-server";

#[cfg(target_os = "windows")]
const EXE_NAME: &'static str = "http-server.exe";

fn main() {
    let exe_path = dirs::cache_dir().unwrap().join(EXE_NAME);
    if exe_path.exists() {
        fs::remove_file(&exe_path).unwrap();
    };
    let exe = include_bin();
    fs::write(&exe_path, exe).unwrap();
    set_executable(&exe_path);

    let current_exe_path = env::current_exe().unwrap();
    let current_exe_dir = current_exe_path.parent().unwrap();
    env::set_current_dir(current_exe_dir).unwrap();

    thread::spawn(move || {
        thread::sleep(Duration::new(0, 10));
        open::that("http://0.0.0.0:8000/index.html").unwrap();
    });
    execute_bin(exe_path).unwrap();
}
