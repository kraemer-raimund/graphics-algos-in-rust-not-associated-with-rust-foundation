use crate::{
    render_buffer::{Pixel, RenderBuffer},
    shapes::Circle,
};

pub fn draw(render_buffer: &mut RenderBuffer, circle: Circle) -> &RenderBuffer {
    // We draw the circle not by iterating along its circumference, but
    // by iterating along its horizontal diameter and calculating the y
    // coordinate(s) as a trigonometric function of x.
    // We only need to calculate one quadrant of the circle, since we
    // can simply mirror it into the remaining quadrants.
    for x in 0..circle.radius() {
        let radians = (x as f32 / circle.radius() as f32).acos();
        let y = radians.sin() * circle.radius() as f32;
        let quadrants = [
            (x as i16, y as i16),
            (x as i16, -y as i16),
            (-(x as i16), y as i16),
            (-(x as i16), -y as i16),
        ];
        quadrants.into_iter().for_each(|quadrant| {
            draw_pixel(
                render_buffer,
                (
                    (circle.x() as i16 + quadrant.0) as u16,
                    (circle.y() as i16 + quadrant.1) as u16,
                ),
            );
        });
    }

    render_buffer
}

fn draw_pixel(render_buffer: &mut RenderBuffer, position: (u16, u16)) {
    let index = (position.1 as u32 * render_buffer.width() as u32) as usize + position.0 as usize;
    render_buffer.pixels_mut()[usize::from(index)] = Pixel::new(0, 0, 0, 255);
}
