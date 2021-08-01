use std::ops;
use super::misc::equal;

/// A color represented by its red, green and blue values.
#[derive(Debug, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    
    /// Create a new color.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::color::Color;
    ///
    /// let c = Color::new(-0.5, 0.4, 1.7);
    ///
    /// assert_eq!(-0.5, c.r());
    /// assert_eq!(0.4, c.g());
    /// assert_eq!(1.7, c.b());
    /// ```
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal(self.r, other.r) &&
        equal(self.g, other.g) &&
        equal(self.b, other.b)
    }
}

impl ops::Add<Self> for Color {
    type Output = Self;
    
    /// Add two colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::color::Color;
    ///
    /// let c1 = Color::new(0.9, 0.6, 0.75);
    /// let c2 = Color::new(0.7, 0.1, 0.25);
    ///
    /// assert_eq!(Color::new(1.6, 0.7, 1.0), c1 + c2);
    ///
    /// ```
    fn add(self, _rhs: Self) -> Self::Output {
        Self {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b
        }
    }
}

impl ops::Sub<Self> for Color {
    type Output = Self;
    
    /// Subtract two colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::color::Color;
    ///
    /// let c1 = Color::new(0.9, 0.6, 0.75);
    /// let c2 = Color::new(0.7, 0.1, 0.25);
    ///
    /// assert_eq!(Color::new(0.2, 0.5, 0.5), c1 - c2);
    ///
    /// ```
    fn sub(self, _rhs: Self) -> Self::Output {
        Self {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;
    
    /// Multiply a color with a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::color::Color;
    ///
    /// let c = Color::new(0.2, 0.3, 0.4);
    ///
    /// assert_eq!(Color::new(0.4, 0.6, 0.8), c * 2.0);
    ///
    /// ```
    fn mul(self, _rhs: f64) -> Self::Output {
        Self {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs
        }
    }
}

impl ops::Mul<Self> for Color {
    type Output = Self;
    
    /// Multiply (blend) two colors.
    ///
    /// This is also known as the _Hadamard product_ or
    /// _Schur product_.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::color::Color;
    ///
    /// let c1 = Color::new(1.0, 0.2, 0.4);
    /// let c2 = Color::new(0.9, 1.0, 0.1);
    ///
    /// assert_eq!(Color::new(0.9, 0.2, 0.04), c1 * c2);
    ///
    /// ```
    fn mul(self, _rhs: Self) -> Self::Output {
        Self {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b
        }
    }
}

/// Definitions of some widely used colors.
///
/// To get access to the defined colors one must bring
/// the `DefaultColors` trait into scope using
/// `use sray::color::{Color, DefaultColors};`.
pub trait DefaultColors {
    const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
    const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };
    const ROSE: Color = Color { r: 1.0, g: 0.0, b: 0.5 };
    const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0 };
    const VIOLET: Color = Color { r: 0.5, g: 0.0, b: 1.0 };
    const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };
    const AZURE: Color = Color { r: 0.0, g: 0.5, b: 1.0 };
    const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0 };
    const SPRING_GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.5 };
    const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
    const CHARTREUSE: Color = Color { r: 0.5, g: 1.0, b: 0.0 };
    const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0 };
    const ORANGE: Color = Color { r: 1.0, g: 0.5, b: 0.0 };
}

impl DefaultColors for Color {}
