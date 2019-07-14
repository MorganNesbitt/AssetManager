use image::Rgba;
use std::path::Path;

pub fn strip_transparency<P>(path: P)
where
    P: AsRef<Path>,
{
    strip_transparency_from_path(path.as_ref())
}

fn strip_transparency_from_path(path: &Path) {
    if !path.is_file() {
        panic!("Only support files currently")
    }

    let image = image::open(path).expect("Expect image to load");
    let workable_image = image.as_rgba8().expect("Expect to be convertable to rgba8");

    let (columns, rows) = workable_image.dimensions();

    let mut top_row = (false, 0);
    let mut bottom_row = (false, 0);
    let mut left_column = (false, 0);
    let mut right_column = (false, 0);

    let mut raw_pixels: Vec<(u32, u32, &image::Rgba<u8>)> =
        workable_image.enumerate_pixels().collect();

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

    let output_file_path = "./src/resources/Stripped/Desert/planks_W.png";

    let x = left_column.1;
    let y = top_row.1;
    let width = right_column.1 - left_column.1;
    let height = bottom_row.1 - top_row.1;

    let sub_image = image::SubImage::new(workable_image, x, y, width + 1, height + 1);
    let bounded_image = sub_image.to_image();

    bounded_image
        .save(output_file_path)
        .expect("Expected to save sub image");
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
