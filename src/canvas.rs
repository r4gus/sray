use super::color::{Color, DefaultColors};

pub struct Canvas {
    canvas: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    
    /// Create a new canvas using the given width and height.
    ///
    /// Each pixel of the canvas is initialized to black.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::canvas::Canvas;
    ///
    /// let c = Canvas::new(10, 20);
    ///
    /// assert_eq!(10, c.width());
    /// assert_eq!(20, c.height());
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: vec![Color::BLACK; width * height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
    
    /// Returns a reference to the pixel at the position specified by the `x` and `y` coordinates.
    ///
    /// * If the specified indices are in bounds, returns a reference to the color at that
    /// position, `None` otherwise.
    ///
    /// The position `(0, 0)` is at the upper left corner of the canvas. `x`
    /// specifies the pixel within a row from left to right and `y` specifies 
    /// the row that contains the pixel.
    ///
    /// Ranges:
    /// * `x` - `[0, width)`
    /// * `y` - `[0, height)`
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::canvas::Canvas;
    /// use sray::color::{Color, DefaultColors};
    ///
    /// let c = Canvas::new(10, 20);
    ///
    /// assert_eq!(&Color::BLACK, c.pixel_at(0, 0).unwrap());
    /// assert_ne!(&Color::RED, c.pixel_at(5, 5).unwrap());
    ///
    /// // The call to `pixal_at` returns `None` because the
    /// // specified indices are out of bounds.
    /// assert_eq!(None, c.pixel_at(10, 5));
    /// assert_eq!(None, c.pixel_at(5, 20));
    /// ```
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        if x < self.width && y < self.height {
            Some(&self.canvas[x + y * self.width])
        } else {
            None
        }
    }
    
    /// Set a color at the position specified by `x` and `y`.
    ///
    /// * The function call won't have an effect if the specified coordinates
    /// are out of bounds.
    ///
    /// Ranges:
    /// * `x` - `[0, width)`
    /// * `y` - `[0, height)`
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::canvas::Canvas;
    /// use sray::color::{Color, DefaultColors};
    ///
    /// let mut c = Canvas::new(10, 20);
    ///
    /// assert_eq!(&Color::BLACK, c.pixel_at(0, 0).unwrap());
    ///
    /// c.write_pixel(0, 0, Color::ORANGE);
    /// assert_eq!(&Color::ORANGE, c.pixel_at(0, 0).unwrap());
    /// ```
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.canvas[x + y * self.width] = color;
        }
    }
    
    /// Translate the given canvas into the __PPM__ file format.
    ///
    /// # Examples
    ///
    ///
    /// Every plain PPM file begins with a header consisting of a _magic number_,
    /// the images width and height in pixels and the maximum color value.
    /// ```
    /// use sray::canvas::Canvas;
    /// use sray::color::{Color, DefaultColors};
    ///
    /// let mut c = Canvas::new(10, 20);
    ///
    /// assert!(c.to_ppm().contains("P3\n10 20\n255"));
    /// ```
    /// 
    /// Following the header is the pixel data, which contains
    /// each pixel represented as three integers (red, green and blue),
    /// scaled between 0 and 255.
    /// ```
    /// use sray::canvas::Canvas;
    /// use sray::color::{Color, DefaultColors};
    ///
    /// let mut c = Canvas::new(5, 3);
    /// c.write_pixel(0, 0, Color::RED);
    /// c.write_pixel(2, 1, Color::AZURE);
    /// c.write_pixel(4, 2, Color::BLUE);
    ///
    /// let body = "P3\n\
    ///             5 3\n\
    ///             255\n\
    ///             255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
    ///             0 0 0 0 0 0 0 128 255 0 0 0 0 0 0\n\
    ///             0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
    ///
    /// assert_eq!(body, c.to_ppm());
    /// ```
    pub fn to_ppm(&self) -> String {
        const SCALE: f64 = 255.0;
        
        // Build header
        let mut ppm = format!("P3\n{} {}\n{}", self.width, self.height, SCALE as u32);
        
        // Process body
        for (i, color) in self.canvas.iter().enumerate() {
            // Limit the number of colors per row
            if i % 5 == 0 {
                ppm += "\n"
            } else {
                ppm += " ";
            }

            ppm += &format!("{} {} {}", 
                           ((color.r() * SCALE).ceil() as u32).clamp(0, 255),
                           ((color.g() * SCALE).ceil() as u32).clamp(0, 255),
                           ((color.b() * SCALE).ceil() as u32).clamp(0, 255));
        }

        // Insert a newline character at the end of the file
        ppm += "\n";

        ppm
    }
}
