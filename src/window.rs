use display_interface::DisplayError;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::Rectangle,
    Pixel,
};
use minifb::{Key, Window, WindowOptions};

pub struct WindowFrame {
    width: usize,
    height: usize,
    buf: Vec<u32>,
    test_buf: Vec<u32>,
    window: Window,
}

impl WindowFrame {
    pub fn new(width: usize, height: usize) -> Self {
        let mut window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{:?}", e);
        });

        // Limit to max ~30 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(33300)));

        Self {
            width,
            height,
            buf: vec![0; width * height],
            test_buf: vec![0; width * height],
            window,
        }
    }

    pub fn update(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.buf, self.width, self.height)
                .unwrap();
        }
    }
}

impl Dimensions for WindowFrame {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        Rectangle::new(
            Point::zero(),
            Size::new(self.width as u32, self.height as u32),
        )
    }
}

impl DrawTarget for WindowFrame {
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels.into_iter() {
            // print!("(x:{x},y:{y})");
            if x > 0 && (x as usize) < self.width && y > 0 && (y as usize) < self.height {
                let idx = x as usize + y as usize * self.width;
                self.buf[idx] = if color.is_on() { 0xffffff } else { 0x000000 };
                self.test_buf[idx] = self.buf[x as usize + y as usize * self.width];
            }
        }
        Ok(())
    }
}
