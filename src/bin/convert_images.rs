use std::path::Path;
use image::{ImageBuffer, Rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let samples = vec![
        "chapter_5",
        "chapter_7",
        "chapter_8",
        "sphere_in_sphere"
    ];

    for sample in samples {
        let ppm_path = format!("samples/{}.ppm", sample);
        let jpg_path = format!("docs/images/{}.jpg", sample);
        
        let img = image::open(&ppm_path)?;
        
        img.save(&jpg_path)?;
        
        println!("Converted {} to {}", ppm_path, jpg_path);
    }

    Ok(())
} 