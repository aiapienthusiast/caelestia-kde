use caelestia_shell::components::filedialog::{
    scan_directory, FileDialog, FileEntry, DIALOG_HEIGHT, DIALOG_MIN_HEIGHT, DIALOG_MIN_WIDTH,
    DIALOG_WIDTH, ITEM_WIDTH, SIDEBAR_WIDTH, STANDARD_PLACES,
};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_filedialog_sizes_and_constants() {
    assert_eq!(ITEM_WIDTH, 103.0);
    assert_eq!(SIDEBAR_WIDTH, 230.0);
    assert_eq!(DIALOG_WIDTH, 1000.0);
    assert_eq!(DIALOG_HEIGHT, 600.0);
    assert_eq!(DIALOG_MIN_WIDTH, 400.0);
    assert_eq!(DIALOG_MIN_HEIGHT, 300.0);

    assert_eq!(STANDARD_PLACES.len(), 7);
    assert_eq!(STANDARD_PLACES[0].0, "Home");
    assert_eq!(STANDARD_PLACES[1].0, "Downloads");
}

#[test]
fn test_file_entry_model_and_formatting() {
    let tmp_dir = std::env::temp_dir().join("caelestia_filedialog_test_entry");
    let _ = fs::create_dir_all(&tmp_dir);

    let test_file = tmp_dir.join("photo.png");
    let _ = fs::write(&test_file, vec![0u8; 1024 * 50]); // 50 KB

    let entry = FileEntry::new(test_file.clone());
    assert_eq!(entry.name, "photo.png");
    assert_eq!(entry.suffix, "png");
    assert!(!entry.is_dir);
    assert_eq!(entry.formatted_size(), "50.0 KB");

    let dir_entry = FileEntry::new(tmp_dir.clone());
    assert!(dir_entry.is_dir);
    assert_eq!(dir_entry.formatted_size(), "Folder");

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_directory_scanning_and_filtering() {
    let tmp_dir = std::env::temp_dir().join("caelestia_filedialog_test_scan");
    let _ = fs::create_dir_all(&tmp_dir);

    let sub_dir = tmp_dir.join("documents");
    let _ = fs::create_dir_all(&sub_dir);

    let img_file = tmp_dir.join("wallpaper.jpg");
    let _ = fs::write(&img_file, "data");

    let text_file = tmp_dir.join("notes.txt");
    let _ = fs::write(&text_file, "text");

    // Scan with image filter
    let filters = vec!["jpg".to_string(), "png".to_string()];
    let entries = scan_directory(&tmp_dir, &filters, false);

    // Should include directory (sub_dir) and wallpaper.jpg, but NOT notes.txt
    assert!(entries.iter().any(|e| e.name == "documents" && e.is_dir));
    assert!(entries.iter().any(|e| e.name == "wallpaper.jpg" && !e.is_dir));
    assert!(!entries.iter().any(|e| e.name == "notes.txt"));

    // Directory entries should be sorted before files
    assert!(entries[0].is_dir);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_filedialog_state_and_navigation() {
    let mut dialog = FileDialog::new();
    assert!(!dialog.is_open);
    assert_eq!(dialog.cwd, vec!["Home".to_string()]);

    dialog.open();
    assert!(dialog.is_open);

    // Select sidebar place
    dialog.select_place("Downloads");
    assert_eq!(dialog.cwd, vec!["Home".to_string(), "Downloads".to_string()]);

    // Go up
    dialog.go_up();
    assert_eq!(dialog.cwd, vec!["Home".to_string()]);

    // Close
    dialog.close();
    assert!(!dialog.is_open);
    assert!(dialog.selected_entry.is_none());
}

#[test]
fn test_filedialog_selection_validation_and_acceptance() {
    let mut dialog = FileDialog::new().with_filters("Images", vec!["png".to_string()]);

    // No file selected yet
    assert!(!dialog.is_selection_valid());
    assert!(dialog.accept_selection().is_none());

    // Select a valid file
    dialog.selected_entry = Some(FileEntry {
        name: "test.png".to_string(),
        path: PathBuf::from("/home/user/Pictures/test.png"),
        is_dir: false,
        size_bytes: 1024,
        suffix: "png".to_string(),
    });

    assert!(dialog.is_selection_valid());
    assert_eq!(
        dialog.accept_selection(),
        Some(PathBuf::from("/home/user/Pictures/test.png"))
    );

    // Select an invalid file (extension mismatch)
    dialog.selected_entry = Some(FileEntry {
        name: "document.pdf".to_string(),
        path: PathBuf::from("/home/user/document.pdf"),
        is_dir: false,
        size_bytes: 2048,
        suffix: "pdf".to_string(),
    });
    assert!(!dialog.is_selection_valid());

    // Folder selection mode
    let folder_dialog = FileDialog::for_folder();
    assert!(folder_dialog.select_folder);
    assert!(folder_dialog.is_selection_valid());
}
