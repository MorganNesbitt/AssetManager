use image::Rgba;
use image::math::utils::clamp;

// https://docs.rs/image/0.21.2/image/

#[derive(Debug)]
struct Dimension {
    width: u32,
    height: u32
}

#[derive(Debug)]
struct Rectangle {
    position: Position,
    dimension: Dimension
}

#[derive(Debug)]
struct BooleanRectSides {
    top: bool,
    left: bool,
    bottom: bool,
    right: bool
}

enum DesiredBoxLocation {
    BottomLeft,
}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32
}

pub fn strip_transparency() {
    let file_path = "./src/resources/Desert/planks_W.png";

    let image = image::open(file_path).expect("Expect image to load");
    let workable_image = image.as_rgba8().expect("Expect to be convertable to rgba8");

    // TODO validate bounding boxes against image dimension
    // maybe clamp

    // TODO maybe be given three?

    // load in most common bounding box
    let bounding_box = Dimension {
        width: 256,
        height: 132
    };

    let common_position = DesiredBoxLocation::BottomLeft;
    let (image_width, image_height) = workable_image.dimensions();

    let transparent_offset = 2;
    let width_with_space = clamp(bounding_box.width + transparent_offset, 0, image_width);
    let height_with_space = clamp(bounding_box.height + transparent_offset, 0, image_height);

    let modified_bounding_box = Dimension {
        width: width_with_space,
        height: height_with_space
    };

    let Position { x, y } = determine_coordinates_by_position_and_bounding_box(
        workable_image,
        common_position,
        &modified_bounding_box
    );

    let x_offset = clamp(width_with_space - bounding_box.width, 0, 2);
    let y_offset = clamp(height_with_space - bounding_box.height, 0, 2);

    let sub_image = image::SubImage::new(workable_image, x, y, width_with_space, height_with_space);
    let workable_sub_image: image::RgbaImage = sub_image.to_image();

    let results = is_sub_image_pixel_perfect(&workable_sub_image, (x_offset, y_offset));
    let (perfect_sides, valid_check_sides) = results;

    // New algo
    // Read in image
    //

    // NOTE: current algorithm assumes theres no transparent gaps?
    // If it was perfect, generate image
    // Otherwise generate new bounds
}

fn is_sub_image_pixel_perfect(workable_image: &image::RgbaImage, offsets:(u32, u32)) -> (BooleanRectSides, BooleanRectSides) {
    let (x_offset, y_offset) = offsets;
    let (image_width, image_height) = workable_image.dimensions();

    // FIXME: breaks because of integer division
    // FIXME: is applying offset to each side.
    let perfect_bounding_box = Rectangle {
        position: Position { x: x_offset / 2, y: y_offset / 2 },
        dimension: Dimension { width: image_width - x_offset, height: image_height - y_offset }
    };

    let valid_check_bounding_box = Rectangle {
        position: Position { x: 0, y: 0 },
        dimension: Dimension { width: image_width, height: image_height }
    };

    let perfect_sides = ignore_name(workable_image, &perfect_bounding_box);
    let valid_check_sides = ignore_name(workable_image, &valid_check_bounding_box);

    (perfect_sides, valid_check_sides)
}

fn ignore_name(workable_image: &image::RgbaImage, bounding_box: &Rectangle) -> BooleanRectSides {
    let Position { x: starting_x, y: starting_y } = bounding_box.position;
    let Dimension { width: desired_width, height: desired_height } = bounding_box.dimension;

    // 3 is the alpha in rgba.
    let top = (starting_x..desired_width).into_iter().fold(false, |acc, x| {
        if acc { return acc; }
        workable_image.get_pixel(x, starting_y).data[3] > 0
    });

    let bottom = (starting_x..desired_width).into_iter().fold(false, |acc, x| {
        if acc { return acc; }
        workable_image.get_pixel(x, (starting_y + desired_height) - 1).data[3] > 0
    });

    let left = (starting_y..desired_height).into_iter().fold(false, |acc, y| {
        if acc { return acc; }
        workable_image.get_pixel(starting_x, y).data[3] > 0
    });

    let right = (starting_x..desired_width).into_iter().fold(false, |acc, y| {
        if acc { return acc; }
        workable_image.get_pixel((starting_x + desired_width) - 1, y).data[3] > 0
    });

    BooleanRectSides {
        top,
        bottom,
        left,
        right
    }
}

fn determine_coordinates_by_position_and_bounding_box(workable_image: &image::RgbaImage, common_position: DesiredBoxLocation, bounding_box: &Dimension) -> Position {
    let (_, height) = workable_image.dimensions();

    match common_position {
        BottomLeft => {
            let x: u32 = 0;
            let y: u32 = height - bounding_box.height;

            Position { x, y }
        }
    }
}

// TODO find the bounding box
// Consider: being able to pass an offset to add space (for testing spaced bounding boxes)???
pub fn test_bounding_perfect_and_spaced_boxes() -> (usize, usize) {
    let barrel_file_path = "./src/resources/Desert/planks_W.png";

    // Actual height is 132, but we are testing spaced bounding boxes
    // let sample_bounding_box = (256, 132);
    let sample_bounding_box = (256, 134);
    // let sample_coordinates = (0, 512 - (sample_bounding_box.1 + 3));
    let sample_coordinates = (0, 512 - (sample_bounding_box.1 + 2));

    let barrel_image = image::open(barrel_file_path).expect("Expect image to load");
    let workable_image = barrel_image.as_rgba8().expect("Expect to be convertable to rgba8");
    // Create a view into the image
    let sub_image = image::SubImage::new(workable_image, sample_coordinates.0, sample_coordinates.1, sample_bounding_box.0, sample_bounding_box.1);
    let bounded_image = sub_image.to_image();

    let zero_indexed_width = sample_bounding_box.0 - 1;
    let zero_indexed_height = sample_bounding_box.1 - 1;

    let rectangle_pixel_capacity = sample_bounding_box.0 * 2 + sample_bounding_box.1 * 2;
    let mut pixels_to_check: Vec<u8> = Vec::with_capacity(rectangle_pixel_capacity as usize);
    // top line in the rectangle
    for x in 0..sample_bounding_box.0 {
        pixels_to_check.push(bounded_image.get_pixel(x, 1).data[3]);
    }

    // bottom line in the rectangle
    for x in 0..sample_bounding_box.0 {
        pixels_to_check.push(bounded_image.get_pixel(x, zero_indexed_height - 1).data[3]);
    }

    // left line in the rectangle
    for y in 1..sample_bounding_box.1 {
        pixels_to_check.push(bounded_image.get_pixel(0, y).data[3]);
    }

    for y in 1..sample_bounding_box.1 {
        pixels_to_check.push(bounded_image.get_pixel(zero_indexed_width, y).data[3]);
    }


    let rectangle_pixel_capacity = sample_bounding_box.0 * 2 + sample_bounding_box.1 * 2;
    let mut spaced_pixels_to_check: Vec<u8> = Vec::with_capacity(rectangle_pixel_capacity as usize);

    // top line in the rectangle
    for x in 0..sample_bounding_box.0 {
        spaced_pixels_to_check.push(bounded_image.get_pixel(x, 0).data[3]);
    }

    // bottom line in the rectangle
    for x in 0..sample_bounding_box.0 {
        spaced_pixels_to_check.push(bounded_image.get_pixel(x, zero_indexed_height).data[3]);
    }

    let non_zero_alphas: Vec<u8> = pixels_to_check.into_iter().filter(|alpha| alpha > &0).collect();
    let spaced_alphas: Vec<u8> = spaced_pixels_to_check.into_iter().filter(|alpha| alpha > &0).collect();
    (non_zero_alphas.len(), spaced_alphas.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_bounding_boxes() {
        let (perfect_bounding_box, _) = test_bounding_perfect_and_spaced_boxes();
        assert_ne!(perfect_bounding_box, 0, "Perfect bounding box should have non zero alpha values");
    }

    #[test]
    fn spaced_bounding_boxes() {
        let (_, spaced_bounding_box) = test_bounding_perfect_and_spaced_boxes();
        assert_eq!(spaced_bounding_box, 0, "Spaced has no non-zero alphas as its perfect bounding box + 1 in all directions");
    }
}
