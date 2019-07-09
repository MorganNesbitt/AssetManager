// https://docs.rs/image/0.21.2/image/
pub fn strip_transparency() {
    // Transparency top, bottom, left, and right
    let barrel_file_path = "./src/resources/Desert/barrel_E.png";
    // Transparency above.
    // let dirt_tile_file_path = "src/resources/Desert/dirtTiles_N.png";

    // Currently:
    // we can strip images to a bound box

    // Next Up:
    // Have a way to tell the program where images are located
    //   -- bottom / center etc
    // Pixel perfect bounding boxes
    // Read in predefined bounding boxes width / heights
    //   -- For guessing purposes
    //   -- One you get a bounding box with transparency
    //     -- shrink and create a picture perfect
    let barrel_image = image::open(barrel_file_path).expect("Expect image to load");
    let workable_image = barrel_image.as_rgba8().expect("Expect to be convertable to rgba8");
    let sub_image = image::SubImage::new(workable_image, 0, 360, 256, 128);
    let resulting_image = sub_image.to_image().save("./src/resources/Stripped/Desert/barrel_E.png").expect("Expecting to save subimage");
}
