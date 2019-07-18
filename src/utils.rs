use std::path::Path;
use walkdir::WalkDir;

pub fn generate_images_from_path(path: &Path) -> Vec<image::DynamicImage> {
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".png"))
        .map(|entry| {
            let file_path = entry.path();

            image::open(file_path).expect("Failed to open image")
        })
        .collect()
}
