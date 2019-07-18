use image::Rgba;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn strip_transparency<P>(input: P, output: P)
where
    P: AsRef<Path>,
{
    strip_transparency_from_input_output(input.as_ref(), output.as_ref())
}

fn strip_transparency_from_input_output(input_path: &Path, output_path: &Path) {
    if input_path.is_dir() && !output_path.is_dir() {
        panic!(
            "Input is directory, output must be directory. Received {:?}",
            output_path
        )
    } else if input_path.is_file() && output_path.extension().expect("Unexpected output file type") != "png" {
        panic!(
            "Input is file, output must be file. Received {:?}",
            output_path
        )
    }

    if input_path.is_file() {
        strip_transparency_from_path(input_path.to_str().unwrap())
            .save(output_path)
            .expect("Expected to save sub image");
        return;
    }

    let paths: Vec<PathBuf> = WalkDir::new(input_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".png"))
        .map(|entry| entry.into_path())
        .collect();

    paths
        .into_par_iter()
        .for_each(|path_buf| {
            let path = path_buf.as_path().to_str().expect("Could not convert path to string");
            let output_path_name = output_path.join(path_buf.file_name().unwrap());
            strip_transparency_from_path(path)
                .save(output_path_name)
                .expect("Expected to save sub image");
        });
}

fn strip_transparency_from_image(image: &image::RgbaImage) -> image::RgbaImage {
    let (columns, rows) = image.dimensions();

    let mut top_row = (false, 0);
    let mut bottom_row = (false, 0);
    let mut left_column = (false, 0);
    let mut right_column = (false, 0);

    let mut raw_pixels: Vec<(u32, u32, &image::Rgba<u8>)> = image.enumerate_pixels().collect();

    for (
        _,
        y,
        Rgba {
            data: [_, _, _, alpha],
        },
    ) in &raw_pixels
    {
        if !top_row.0 && *alpha > 0 {
            top_row = (true, *y);
            break;
        }
    }

    for x in 0..columns {
        for y in 0..rows {
            let index: usize = ((y * columns) + x) as usize;
            let (
                _,
                _,
                Rgba {
                    data: [_, _, _, alpha],
                },
            ) = raw_pixels[index];
            if !left_column.0 && *alpha > 0 {
                left_column = (true, x);
                break;
            }
        }

        if left_column.0 {
            break;
        }
    }

    for x in (0..columns).rev() {
        for y in 0..rows {
            let index: usize = ((y * columns) + x) as usize;
            let (
                _,
                _,
                Rgba {
                    data: [_, _, _, alpha],
                },
            ) = raw_pixels[index];
            if !right_column.0 && *alpha > 0 {
                right_column = (true, x);
                break;
            }
        }

        if right_column.0 {
            break;
        }
    }

    raw_pixels.reverse();

    for (
        _,
        y,
        Rgba {
            data: [_, _, _, alpha],
        },
    ) in &raw_pixels
    {
        if !bottom_row.0 && *alpha > 0 {
            bottom_row = (true, *y);
            break;
        }
    }

    let x = left_column.1;
    let y = top_row.1;
    let width = right_column.1 - left_column.1;
    let height = bottom_row.1 - top_row.1;

    let sub_image = image::SubImage::new(image, x, y, width + 1, height + 1);
    sub_image.to_image()
}

fn strip_transparency_from_path(input_path: &str) -> image::RgbaImage {
    let full_image = image::open(input_path).expect("Expect image to load");
    let image = full_image
        .as_rgba8()
        .expect("Expect to be convertable to rgba8");
    strip_transparency_from_image(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn perfect_bounding_boxes() {
    //     let (perfect_bounding_box, _) = test_bounding_perfect_and_spaced_boxes();
    //     assert_ne!(perfect_bounding_box, 0, "Perfect bounding box should have non zero alpha values");
    // }

    // #[test]
    // fn spaced_bounding_boxes() {
    //     let (_, spaced_bounding_box) = test_bounding_perfect_and_spaced_boxes();
    //     assert_eq!(spaced_bounding_box, 0, "Spaced has no non-zero alphas as its perfect bounding box + 1 in all directions");
    // }
}
