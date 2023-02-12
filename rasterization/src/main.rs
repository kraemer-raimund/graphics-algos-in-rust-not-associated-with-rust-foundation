use show_image::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pixel_data = &[255; 1024 * 1024 * 3];
    
    let image = ImageView::new(ImageInfo::rgb8(1024, 1024), pixel_data);

    let window = create_window("Rendered Image", Default::default())?;
    window.set_image("image", image)?;

    std::io::stdin().read_line(&mut String::new()).unwrap();
    
    Ok(())
}
