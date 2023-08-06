use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use tempfile::TempDir;

// Use the crate name from your Cargo.toml file
use obsidian_to_notion::{sync, versioning};

#[test]
fn test_backup_and_sync() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    // Set up the test environment
    let rclone_bin =
        "/home/matahari/development/projects/education/obsidian-to-notion/rclone/bin/rclone";
    let rclone_config = "/home/matahari/development/projects/education/obsidian-to-notion/rclone/.config/rclone.conf";
    let git_name = "Test User";
    let git_email = "test@example.com";

    let mut rclone_settings: HashMap<&str, String> = HashMap::new();
    rclone_settings.insert("RCLONE_BIN", rclone_bin.to_string());
    rclone_settings.insert("RCLONE_CONFIG", rclone_config.to_string());

    // Run the configure_rclone function
    sync::configure_rclone(&rclone_settings);

    // Create sample files in the temporary directory
    fs::write(temp_dir.path().join("file1.txt"), "Sample content").expect("Failed to create file");
    fs::write(temp_dir.path().join("file2.txt"), "Another sample").expect("Failed to create file");

    // Backup the temporary directory
    let git_config = (git_name, git_email);
    versioning::backup_path(temp_dir.path().to_str().unwrap(), git_config);

    // Assert that the backup was successful
    // Add your assertions here if needed

    // Clean up: Remove the temporary directory
    temp_dir
        .close()
        .expect("Failed to remove temporary directory");
}
