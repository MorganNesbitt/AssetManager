use rayon::prelude::*;
use sheep::{AmethystFormat, InputSprite, SimplePacker};
use std::path::Path;
use std::{fs::File, io::prelude::*};
use walkdir::WalkDir;

pub fn pack_tiles<P>(input: P, output: P)
where
    P: AsRef<Path>,
{
    pack_tiles_from_path(input.as_ref(), output.as_ref())
}

pub fn pack_tiles_from_path(input: &Path, output: &Path) {
    if !input.is_dir() {
        panic!("Can only pack directory. Was given {:?}", input);
    } else if !output.is_dir() {
        panic!("Can only output to directory. Was given {:?}", output);
    }

    println!("Starting to iterate of Desert Folder");
    let desert_images = generate_images_from_path(input);
    println!("count of desert images {:?}", desert_images.len());

    write_images_to_file(output, desert_images);
}

fn write_images_to_file(base_path: &Path, images: Vec<image::DynamicImage>) {
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
            InputSprite {
                dimensions,
                bytes: bytes.clone(),
            }
        })
        .collect::<Vec<InputSprite>>();

    // We'll just repeat the same sprite 16 times and pack it into a texture.

    // Do the actual packing! 4 defines the stride, since we're using rgba8 we
    // have 4 bytes per pixel.
    let sprite_sheet = sheep::pack::<SimplePacker>(sprites, 4);

    // Now, we can encode the sprite sheet in a format of our choosing to
    // save things such as offsets, positions of the sprites and so on.
    let meta = sheep::encode::<AmethystFormat>(&sprite_sheet, ());

    // Next, we save the output to a file using the image crate again.
    let outbuf = image::RgbaImage::from_vec(
        sprite_sheet.dimensions.0,
        sprite_sheet.dimensions.1,
        sprite_sheet.bytes,
    )
    .expect("Failed to construct image from sprite sheet bytes");

    outbuf
        .save(base_path.join("packed.png"))
        .expect("Failed to save image");

    // Lastly, we serialize the meta info using serde. This can be any format
    // you want, just implement the trait and pass it to encode.
    let mut meta_file =
        File::create(base_path.join("packed.ron")).expect("Failed to create meta file");
    let meta_str = ron::ser::to_string(&meta).expect("Failed to encode meta file");

    meta_file
        .write_all(meta_str.as_bytes())
        .expect("Failed to write meta file");
}

fn generate_images_from_path(path: &Path) -> Vec<image::DynamicImage> {
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".png"))
        .map(|entry| {
            let file_path = entry.path();

            image::open(file_path).expect("Failed to open image")
        })
        .take(2)
        .collect()
}
