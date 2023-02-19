use crate::{
    render_buffer::{Pixel, RenderBuffer},
    shapes::Circle,
};

/// Draw a circle using the Horn method.
/// B. K. P. Horn: Circle Generators for Display Devices.
/// In: Computer Graphics and Image Processing, 5, 2, June 1976, S. 280–288, ISSN 0146-664X
/// https://people.csail.mit.edu/bkph/papers/Circle_Generators_OPT.pdf
pub fn draw(render_buffer: &mut RenderBuffer, circle: Circle) -> &RenderBuffer {
    // We start at the point to the far right on the circle, from which we then move upwards
    // and to the left along the circle.
    let mut x = circle.radius() as f32;
    let mut y = 0.0;
    // The control variable d is used for determining whether the pixel at (x, y) is within
    // or outside of the circle (see below).
    let mut d = -x;

    // We move upwards (y increases in each step) and to the left (x decreases in each step) along
    // the circle, so we know that we completed one octant when the x and y coordinates meet.
    while x >= y {
        // We mirror the pixel coordinates of one octant of the circle into the remaining octants.
        let octants = [
            (x, y),
            (x, -y),
            (-x, y),
            (-x, -y),
            (y, x),
            (y, -x),
            (-y, x),
            (-y, -x),
        ];
        octants.into_iter().for_each(|octant| {
            draw_pixel(
                render_buffer,
                (
                    (circle.x() as f32 + octant.0) as u16,
                    (circle.y() as f32 + octant.1) as u16,
                ),
            );
        });
        // The following represents the equation d_i = (x_i - 1/2)^2 + y_i^2 - r^2 > 0 as an
        // optimized incremental implementation.
        // After each step, we move 1 pixel upwards (y += 1.0). Depending on the value of the
        // control variable d, we sometimes also move 1 pixel to the left (x -= 1.0).
        d = d + 2.0 * y + 1.0;
        y += 1.0;
        if d > 0.0 {
            d = d - 2.0 * x + 2.0;
            x -= 1.0;
        }
    }

    render_buffer
}

fn draw_pixel(render_buffer: &mut RenderBuffer, position: (u16, u16)) {
    let index = (position.1 as u32 * render_buffer.width() as u32) as usize + position.0 as usize;
    render_buffer.pixels_mut()[usize::from(index)] = Pixel::new(0, 0, 0, 255);
}
