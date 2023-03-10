use std::{process::Command, path::Path};

pub fn delete_file(directory: String, file_name: String) -> Result<(), String> {
    let path = Path::new(&directory).join(&file_name);

    let result = std::fs::remove_file(&path);
    if result.is_err() {
        return Err("File is not exist to be deleted".to_string());
    }
    return Ok(());
}
pub fn show_in_folder(directory: String, file_name: String) {
    let binding = Path::new(&directory).join(&file_name);
    let path = binding.to_str().unwrap();

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            // let new_path = match metadata(&path).unwrap().is_dir() {
            //     true => path,
            //     false => {
            //         let mut path2 = PathBuf::from(path);
            //         path2.pop();
            //         path2.into_os_string().into_string().unwrap()
            //     }
            // };
            // Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"file://{path}\"").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", path]).spawn().unwrap();
    }
}
