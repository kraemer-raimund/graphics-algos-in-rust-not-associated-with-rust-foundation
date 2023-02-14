mod drawing;
mod render_buffer;
mod shapes;

use shapes::Circle;
use show_image::{create_window, ImageInfo, ImageView, WindowOptions};

use crate::{drawing::draw, render_buffer::RenderBuffer};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let render_buffer = &mut RenderBuffer::white(512, 512);
    let circle = Circle::new(200, 200, 40);
    draw(render_buffer, circle);

    let pixel_data = &render_buffer.raw_pixel_data();

    let image = ImageView::new(
        ImageInfo::rgba8(render_buffer.width().into(), render_buffer.height().into()),
        pixel_data,
    );

    let window = create_window(
        "Rendered Image",
        WindowOptions::new()
            .set_size([render_buffer.width().into(), render_buffer.height().into()]),
    )?;
    window.set_image("image", image)?;
    let _ = window.wait_until_destroyed();

    Ok(())
}
