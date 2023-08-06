use std::{collections::HashMap, env, process::Command};

pub fn configure_rclone(rclone_paths: &HashMap<&'static str, String>) {
    for (key, value) in rclone_paths.iter() {
        env::set_var(key, value);
    }

    let rclone_bin = env::var("RCLONE_BIN").unwrap();

    let mut output = Command::new(&rclone_bin)
        .arg("config")
        .spawn()
        .expect("Failed to run rclone config");

    let status = output
        .wait()
        .expect("Failed to wait for rclone config process");

    if !status.success() {
        eprintln!("Rclone configuration process exited with status {}", status)
    }
}

pub fn bisync_paths(path1: &str, path2: &str, rclone_settings: &HashMap<&'static str, String>) {
    // let rclone_bin = rclone_settings.get("RCLONE_BIN").unwrap();

    // let output = Command::new(rclone_bin).args(["bisync", path1, path2]);
    ()
}
