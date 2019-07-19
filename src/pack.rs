use indicatif::ProgressBar;
use rayon::prelude::*;
use sheep::{AmethystFormat, InputSprite, MaxrectsOptions, MaxrectsPacker};
use std::path::{Path, PathBuf};
use std::{fs::File, io::prelude::*};
use walkdir::WalkDir;

pub fn pack_tiles<P>(input: P, output: P, progress_bar: &ProgressBar)
where
    P: AsRef<Path>,
{
    pack_tiles_from_path(input.as_ref(), output.as_ref(), progress_bar)
}

// TODO maybe add sample / take to test directories with big images
// TODO add tests
pub fn pack_tiles_from_path(input: &Path, output: &Path, progress_bar: &ProgressBar) {
    if !input.is_dir() {
        panic!("Can only pack directory. Was given {:?}", input);
    } else if !output.is_dir() {
        panic!("Can only output to directory. Was given {:?}", output);
    }

    let images = generate_images_from_path(input, progress_bar);
    write_images_to_file(output, images, progress_bar);
}

fn write_images_to_file(
    base_path: &Path,
    images: Vec<image::DynamicImage>,
    progress_bar: &ProgressBar,
) {
    progress_bar.set_length(images.len() as u64);
    progress_bar.println("Loading images as sprites");

    let sprites = images
        .into_par_iter()
        .map(|dynamic_image| {
            let img = dynamic_image
                .as_rgba8()
                .expect("Failed to convert image to rgba8");
            let dimensions = img.dimensions();
            let bytes = img
                .pixels()
                .flat_map(|it| it.data.iter().copied())
                .collect::<Vec<u8>>();

            progress_bar.inc(1);
            InputSprite {
                dimensions,
                bytes: bytes.clone(),
            }
        })
        .collect::<Vec<InputSprite>>();

    progress_bar.finish_and_clear();
    progress_bar.println("Preparing packing images into spritesheets of max 4096x4096");

    // We'll just repeat the same sprite 16 times and pack it into a texture.

    // Do the actual packing! 4 defines the stride, since we're using rgba8 we
    // have 4 bytes per pixel.
    let options = MaxrectsOptions::default().max_width(4096).max_height(4096);
    let sprite_sheets = sheep::pack::<MaxrectsPacker>(sprites, 4, options);

    progress_bar.set_length(sprite_sheets.len() as u64);
    progress_bar.println("packing images into spritesheets");

    for (index, sprite_sheet) in progress_bar.wrap_iter(sprite_sheets.into_iter().enumerate()) {
        let meta = sheep::encode::<AmethystFormat>(&sprite_sheet, ());

        // Next, we save the output to a file using the image crate again.
        let outbuf = image::RgbaImage::from_vec(
            sprite_sheet.dimensions.0,
            sprite_sheet.dimensions.1,
            sprite_sheet.bytes,
        )
        .expect("Failed to construct image from sprite sheet bytes");

        let packed_name = format!("packed{}", index);
        outbuf
            .save(base_path.join(format!("{}.png", packed_name)))
            .expect("Failed to save image");

        let mut meta_file = File::create(base_path.join(format!("{}.ron", packed_name)))
            .expect("Failed to create meta file");
        let meta_str = ron::ser::to_string(&meta).expect("Failed to encode meta file");

        meta_file
            .write_all(meta_str.as_bytes())
            .expect("Failed to write meta file");
    }

    progress_bar.finish_and_clear();
    progress_bar.println("Finished packing and saved files");
}

pub fn generate_images_from_path(
    path: &Path,
    progress_bar: &ProgressBar,
) -> Vec<image::DynamicImage> {
    let paths: Vec<PathBuf> = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".png"))
        .map(|entry| entry.into_path())
        .collect();

    progress_bar.set_length(paths.len() as u64);
    progress_bar.println("Opening images from paths");

    progress_bar
        .wrap_iter(paths.into_iter())
        .map(|entry| image::open(entry.as_path()).expect("Failed to open image"))
        .collect()
}
