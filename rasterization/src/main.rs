use show_image::{create_window, ImageInfo, ImageView, WindowOptions};

use crate::render_buffer::RenderBuffer;

mod render_buffer;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let white_image = RenderBuffer::white(512, 512);
    let pixel_data = &white_image.raw_pixel_data();

    let image = ImageView::new(
        ImageInfo::rgba8(white_image.width().into(), white_image.height().into()),
        pixel_data,
    );

    let window = create_window(
        "Rendered Image",
        WindowOptions::new().set_size([white_image.width().into(), white_image.height().into()]),
    )?;
    window.set_image("image", image)?;
    let _ = window.wait_until_destroyed();

    Ok(())
}
