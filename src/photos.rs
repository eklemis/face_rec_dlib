use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

pub fn extract_unique_child_ids(dir_path: &str) -> HashSet<String> {
    let mut child_ids = HashSet::new();

    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && is_image_file(e.path()))
    {
        if let Some(child_id) = extract_child_id_from_filename(entry.path()) {
            child_ids.insert(child_id);
        }
    }

    child_ids
}

fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("png"))
        .unwrap_or(false)
}

fn extract_child_id_from_filename(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.split('_').next())
        .map(|child_id| child_id.to_string())
}
