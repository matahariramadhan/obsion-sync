use std::{collections::HashMap, io, io::Write};

use obsidian_to_notion::{sync, versioning};
fn main() {
    let rclone_bin = read_user_input("Enter RCLONE_BIN:");
    let rclone_config = read_user_input("Enter RCLONE_CONFIG:");
    let mut rclone_settings: HashMap<&str, String> = HashMap::new();

    rclone_settings.insert("RCLONE_BIN", rclone_bin);
    rclone_settings.insert("RCLONE_CONFIG", rclone_config);
    sync::configure_rclone(&rclone_settings);

    let git_name = read_user_input("Enter your git name:");
    let git_email = read_user_input("Enter your git email:");
    let path1 = read_user_input("Enter PATH1:");
    versioning::backup_path(&path1, (&git_name, &git_email));

    let path2 = read_user_input("Enter PATH2:");
    versioning::backup_path(&path2, (&git_name, &git_email));

    sync::bisync_paths(&path1, &path2, &rclone_settings);

    // Translate obsidian markdown to notion markdown
    // bisync notion from onedrive
}

fn read_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
