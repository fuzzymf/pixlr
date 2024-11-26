use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use resvg::render;
use resvg::tiny_skia::{Pixmap, Transform};
use std::fs;
use usvg::{fontdb, PostProcessingSteps, Tree, TreeParsing, TreePostProc};

fn svg_to_pixel_art(
    svg_path: &str,
    output_path: &str,
    pixel_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the SVG file and parse it
    let svg_data = fs::read(svg_path)?;

    let opt: usvg::Options = usvg::Options::default();
    let mut tree: Tree = Tree::from_data(&svg_data, &opt).unwrap();
    tree.postprocess(PostProcessingSteps::default(), &fontdb::Database::new());

    // Get the size of the SVG tree and create a pixmap
    let size = tree.size;
    let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();

    // render the SVG tree into the pixmap
    render(&tree, Transform::default(), &mut pixmap.as_mut());

    // Create a DynamicImage from the pixmap
    let image = DynamicImage::ImageRgba8(
        ImageBuffer::<Rgba<u8>, _>::from_raw(
            pixmap.width(),
            pixmap.height(),
            pixmap.data().to_vec(),
        )
        .ok_or("Failed to convert pixmap data")?,
    );

    // Pixelate the image
    let (width, height) = image.dimensions();
    let small_width = width / pixel_size;
    let small_height = height / pixel_size;

    // Resize down to pixelate
    let small_image = image.resize_exact(
        small_width,
        small_height,
        image::imageops::FilterType::Nearest,
    );

    // Resize back up to original dimensions
    let pixelated_image =
        small_image.resize_exact(width, height, image::imageops::FilterType::Nearest);

    //Save the pixelated image
    pixelated_image.save(output_path)?;
    println!("Pixelated image saved at {}", output_path);

    Ok(())
}

fn png_to_pixel_art(
    img_path: &str,
    output_path: &str,
    pixel_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the PNG file
    let img = image::open(img_path)?;

    // Pixelate the image
    let (width, height) = img.dimensions();
    let small_width = width / pixel_size;
    let small_height = height / pixel_size;

    // Resize down to pixelate
    let small_image = img.resize_exact(
        small_width,
        small_height,
        image::imageops::FilterType::Nearest,
    );

    // Resize back up to original dimensions
    let pixelated_image =
        small_image.resize_exact(width, height, image::imageops::FilterType::Nearest);

    // Save the pixelated image
    pixelated_image.save(output_path)?;
    println!("Pixelated image saved at {}", output_path);

    Ok(())
}

/*
The main function takes in the image file,
switches the image to a pixmap,
and then pixelates the image.
The pixelated image is then saved to the output path.
 */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input.svg/input.png> <output.png> <pixel_size>",
            args[0]
        );
        std::process::exit(1);
    }

    let img_path = &args[1];
    let output_path = &args[2];
    let pixel_size: u32 = args[3].parse().expect("Pixel size must be a number");

    let extension = img_path.split('.').last().unwrap();

    match extension {
        "svg" => {
            if let Err(e) = svg_to_pixel_art(img_path, output_path, pixel_size) {
                eprintln!("Error: {}", e);
            }
        }
        "png" => {
            if let Err(e) = png_to_pixel_art(img_path, output_path, pixel_size) {
                eprintln!("Error: {}", e);
            }
        }
        _ => {
            eprintln!("Unsupported file format. Only SVG and PNG are supported.");
            std::process::exit(1);
        }
    }
}
