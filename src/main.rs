use std::path::PathBuf;
use std::fs;

fn main() {
    // get windows appdatalocal path
    let appdata_local: PathBuf = PathBuf::from(std::env::var("LOCALAPPDATA").unwrap());
    let dayz_path = appdata_local.join("DayZ");
    println!("DayZ path: {:?}", dayz_path);

    let mut removed_size: i128 = 0;
    // iterate files in dayz_path
    for entry in std::fs::read_dir(dayz_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".log") || path_str.ends_with(".mdmp") || path_str.ends_with(".RPT") {
            // get file size
            let metadata = fs::metadata(&path).unwrap();
            let size = metadata.len();
            removed_size += size as i128;
            fs::remove_file(path).unwrap();
        }
    }

    println!("Removed {}GB", removed_size / 1024 / 1024 / 1024);
    println!("Done! :)");
}
