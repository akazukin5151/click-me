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
fn set_executable(path: &PathBuf) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| format!("Open file failed: {}", e))?;
    let mut p = file.metadata()
        .map_err(|e| format!("File metadata failed: {}", e))?
        .permissions();
    p.set_mode(755);
    fs::set_permissions(path, p).map_err(|e| format!("Set permissions failed: {}", e))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_executable(path: &PathBuf) -> Result<(), String> {
    Ok(())
}

fn execute_bin(path: PathBuf) -> Result<ExitStatus, &'static str> {
    Command::new(path).arg("--cors").status().map_err(|_| "Failed to launch command")
}

#[cfg(unix)]
const EXE_NAME: &'static str = "http-server";

#[cfg(target_os = "windows")]
const EXE_NAME: &'static str = "http-server.exe";

fn main() -> Result<(), String> {
    let exe_path = dirs::cache_dir()
        .ok_or("Get cache dir failed")?
        .join(EXE_NAME);

    if exe_path.exists() {
        fs::remove_file(&exe_path)
            .map_err(|e| format!("Failed to remove file: {}", e))?;
    };
    let exe = include_bin();
    fs::write(&exe_path, exe)
        .map_err(|e| format!("Failed to write exe: {}", e))?;
    set_executable(&exe_path)?;

    let current_exe_path = env::current_exe()
        .map_err(|e| format!("Get current exe failed: {}", e))?;
    let current_exe_dir = current_exe_path.parent()
        .ok_or("Get current exe parent failed")?;
    env::set_current_dir(current_exe_dir)
        .map_err(|e| format!("Setting current dir failed: {}", e))?;

    thread::spawn(move || {
        thread::sleep(Duration::new(0, 10));
        open::that("http://0.0.0.0:8000/index.html")
            .map_err(|e| format!("Failed to open url: {}", e)).unwrap();
    });
    execute_bin(exe_path)
        .map_err(|e| format!("Failed to execute bin: {}", e))?;

    Ok(())
}
