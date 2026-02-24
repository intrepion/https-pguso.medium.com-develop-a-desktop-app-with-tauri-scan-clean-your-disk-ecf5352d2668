use std::path::PathBuf;
use trash::delete;
use walkdir::WalkDir;s

#[derive(serde::Serialize)]
struct FileInfo {
    pub path: String,
    pub size: u64,
    pub modified: String,
}

#[tauri::command]
fn scan_folder(path: String) -> Result<Vec<FileInfo>, String> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        if metadata.is_file() {
            let size = metadata.len();
            let modified = metadata
                .modified()
                .map_err(|e| e.to_string())
                .and_then(|time| {
                    let datetime: chrono::DateTime<chrono::Local> = time.into();
                    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
                })
                .unwrap_or_else(|_| "unknown".to_string());

            files.push(FileInfo {
                path: entry.path().display().to_string(),
                size,
                modified,
            });
        }
    }

    Ok(files)
}

#[tauri::command]
fn move_to_trash(path: String) -> Result<(), String> {
    let pathbuf = PathBuf::from(path);
    delete(&pathbuf).map_err(|e| e.to_string())
}

#[tauri::command]
fn my_custom_command() {
    println!("Called from JS!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error running Tauri app");
}
