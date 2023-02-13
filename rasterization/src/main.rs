use show_image::{create_window, ImageInfo, ImageView, WindowOptions};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pixel_data = &[255; 1024 * 1024 * 3];

    let image = ImageView::new(ImageInfo::rgb8(512, 512), pixel_data);

    let window = create_window(
        "Rendered Image",
        WindowOptions::new().set_size([512, 512]),
    )?;
    window.set_image("image", image)?;
    let _ = window.wait_until_destroyed();

    Ok(())
}
