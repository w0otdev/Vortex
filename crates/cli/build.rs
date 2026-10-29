use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=vortex.rc");
    println!("cargo:rerun-if-changed=vortex.ico");
    println!("cargo:rerun-if-changed=vortex_icon_64.rgba");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let res_path = out_dir.join("vortex.res");
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let rc_file = manifest_dir.join("vortex.rc");

    let mut rc_candidates = vec![
        PathBuf::from("rc.exe"),
        PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\rc.exe"),
        PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\rc.exe"),
        PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\rc.exe"),
    ];
    if let Ok(entries) = std::fs::read_dir(r"C:\Program Files (x86)\Windows Kits\10\bin") {
        for entry in entries.flatten() {
            let candidate = entry.path().join("x64").join("rc.exe");
            if candidate.is_file() && !rc_candidates.contains(&candidate) {
                rc_candidates.push(candidate);
            }
        }
    }

    for rc in rc_candidates {
        if let Ok(status) = Command::new(&rc)
            .args(["/nologo", "/fo"])
            .arg(&res_path)
            .arg(&rc_file)
            .status()
            && status.success()
        {
            println!("cargo:rustc-link-arg-bins={}", res_path.display());
            return;
        }
    }
    println!("cargo:warning=rc.exe not found; skipping Windows icon resource");
}
