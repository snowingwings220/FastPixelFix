use std::env::args;

use image::{GenericImage, GenericImageView, ImageReader, Rgb};
use std::thread;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn usage() {
    println!("Usage: fast_pixel_fix <image_file>");
    println!("Example: fast_pixel_fix image.png");
}

fn main() {
    let processors: usize = num_cpus::get();

    let img_name = match args().nth(1) {
        Some(name) => name,
        None => {
            usage();
            return;
        }
    };

    let img = match ImageReader::open(&img_name) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error decoding image: {}", e);
                panic!("Failed to decode image");
            }
        },
        Err(e) => {
            eprintln!("Error opening image file: {}", e);
            panic!("Failed to open image file");
        }
    };

    let img_s = Arc::new(img);

    let (width, height) = img_s.dimensions();

    let slices = (0..processors)
        .map(|i| {
            let start = (height * i as u32) / processors as u32;
            let end = if i == processors - 1 {
                height // Last slice gets all remaining pixels
            } else {
                (height * (i + 1) as u32) / processors as u32
            };
            (start, end)
        })
        .collect::<Vec<_>>();
    let sum = Arc::new(Mutex::new([0u32; 3]));
    let total_pixels = Arc::new(Mutex::new(0u32));


    // compute the average color of the image

    let mut handles_average: Vec<std::thread::JoinHandle<()>> = Vec::new();

    println!("Dispatching {} threads to find average color", processors);
    println!("Slices: {:?}", slices);
    println!("Slice height: {} px", height / processors as u32);

    for (start, end) in slices.clone() {
        let img_clone = Arc::clone(&img_s);
        let mut slice_sum: [u32;3] = [0; 3];
        let sum_clone = Arc::clone(&sum);
        let total_pixels_clone = Arc::clone(&total_pixels);

        let handle = thread::spawn(move || {
            
            let mut count = 0;

            for y in start..end {
                for x in 0..width {
                    let pixel = img_clone.get_pixel(x, y);
                    if pixel[3] == 0 {
                        continue; // skip transparent pixels
                    }
                    slice_sum[0] += pixel[0] as u32;
                    slice_sum[1] += pixel[1] as u32;
                    slice_sum[2] += pixel[2] as u32;
                    count += 1;
                }
            }
            let mut sum = sum_clone.lock().unwrap();
            sum[0] += slice_sum[0];
            sum[1] += slice_sum[1];
            sum[2] += slice_sum[2];
            drop(sum); // let other threads do their thing earlier, idk if rust already does this

            let mut total_pixels = total_pixels_clone.lock().unwrap();
            *total_pixels += count as u32;
            drop(total_pixels)
        });
        handles_average.push(handle);
    }
    println!("All {} threads finished", handles_average.len());
    for handle in handles_average {
        handle.join().unwrap();
    }

    let final_sum = sum.lock().unwrap();
    let total_pixels = *total_pixels.lock().unwrap();
    println!("{} enumerated", total_pixels);
    let average_color = Rgb([
        (final_sum[0] / total_pixels) as u8,
        (final_sum[1] / total_pixels) as u8,
        (final_sum[2] / total_pixels) as u8,
    ]);
    println!("Average color: {:?}", average_color);


    // compute the alpha transform
    println!("[Compute alpha transform]\n\n");
    println!("Dispatching {} threads to fix pixels", processors);
    let before_fix = Instant::now();

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let final_image = (*img_s).clone();
    let final_image = Arc::new(Mutex::new(final_image));
    for (start, end) in slices {
        let img_clone = Arc::clone(&img_s);
        let final_image_clone = Arc::clone(&final_image);
        let handle = thread::spawn(move || {
            for y in start..end {
                for x in 0..width {
                    let mut pixel = img_clone.get_pixel(x, y);
                    if pixel[3] == 0 {
                        pixel[0] = average_color[0];
                        pixel[1] = average_color[1];
                        pixel[2] = average_color[2];

                        let mut final_image_mut = final_image_clone.lock().unwrap();
                        final_image_mut.put_pixel(x, y, pixel);
                        drop(final_image_mut); 
                    }
                    
                }
            }
        });
        handles.push(handle);
    }
    
    println!("All {} threads finished", handles.len());
    for handle in handles {
        handle.join().unwrap();
    }
    let after_fix = Instant::now();

    // this much effort for some commas lol
    let total_pixels_comma = total_pixels.to_string();
    let mut total_pixels_comma2 = String::from("");
    for (index, character) in total_pixels_comma.chars().enumerate() {
        if index > 0 && (total_pixels_comma.len() - index) % 3 == 0 {
            total_pixels_comma2.push(',');
        }
        total_pixels_comma2.push(character);
    }

    println!("Fixing {} pixels took: {:.2} seconds", total_pixels_comma2 , after_fix.duration_since(before_fix).as_secs_f32());
    println!("[Done]");


    println!("[Saving image]");
    let final_image = Arc::try_unwrap(final_image).unwrap().into_inner().unwrap();
    
    match final_image.save(img_name) {
        Ok(_) => {
            println!("Image written!");
        },
        Err(e) => {
            eprintln!("Error saving image: {}", e);
            panic!("Failed to save image");
        }
    }

    println!("[Done]");

}
