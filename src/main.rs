use std::env;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Write, Seek};

mod fileheader;
mod i2r;

fn main() -> Result<(), Error> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is correct
    if args.len() != 4 {
        eprintln!("Usage: {} <input_image> <input_save> <output_save>", args[0]);
        return Ok(());
    }

    // Extract input file paths
    let image_path = &args[1];
    let save_path = &args[2];
    let output_path = &args[3];

    // Open the save file
    let mut save_file = File::open(save_path)?;

    // Read save file header
    let header = fileheader::Fileheader::read_from_file(&mut save_file)?;

    // Check if the save file is compressed
    if header.compressed {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Compressed save files are not supported.",
        ));
    }

    // Extract raw pixel data from the image
    let image_result = image::open(image_path);
    let image = match image_result {
        Ok(image) => image,
        Err(err) => return Err(Error::new(ErrorKind::Other, format!("Error loading image: {}", err))),
    };

    let raw_data = i2r::extract_raw_data(&header, &image)?;

    // Create the modified image
    let modified_image = i2r::create_image(&raw_data);

    // Open the output save file
    let mut output_file = File::create(output_path)?;

    // Copy the content of the save file up to the end of the header
    let header_end = header.header_end as usize;
    fs::copy(save_path, output_path)?;

    // Write the modified raw pixel data starting at header_end
    output_file.seek(std::io::SeekFrom::Start(header_end as u64))?;
    output_file.write_all(&raw_data)?;

    println!("Modified save file has been saved to {}", output_path);

    Ok(())
}
