use super::utils::generate_images_from_path;
use rayon::prelude::*;
use sheep::{AmethystFormat, InputSprite, MaxrectsOptions, MaxrectsPacker};
use std::path::Path;
use std::{fs::File, io::prelude::*};

pub fn pack_tiles<P>(input: P, output: P)
where
    P: AsRef<Path>,
{
    pack_tiles_from_path(input.as_ref(), output.as_ref())
}

// TODO maybe add sample / take to test directories with big images
// TODO add tests
pub fn pack_tiles_from_path(input: &Path, output: &Path) {
    if !input.is_dir() {
        panic!("Can only pack directory. Was given {:?}", input);
    } else if !output.is_dir() {
        panic!("Can only output to directory. Was given {:?}", output);
    }

    write_images_to_file(output, generate_images_from_path(input));
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
    let options = MaxrectsOptions::default().max_width(4096).max_height(4096);
    let sprite_sheets = sheep::pack::<MaxrectsPacker>(sprites, 4, options);

    for (index, sprite_sheet) in sprite_sheets.into_iter().enumerate() {
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
}
