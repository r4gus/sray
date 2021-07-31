use std::ops;

#[derive(Debug)]
struct Tuple4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl ops::Add<Self> for Tuple4 {
    type Output = Self;
    
    fn add(self, _rhs: Self) -> Self::Output {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl ops::Sub<Self> for Tuple4 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

impl ops::Neg for Tuple4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl ops::Mul<f64> for Tuple4 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self::Output {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs
        }
    }
}

impl ops::Div<f64> for Tuple4 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self::Output {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            w: self.w / _rhs
        }
    }
}

impl PartialEq for Tuple4 {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) &&
        equal(self.y, other.y) &&
        equal(self.z, other.z) &&
        equal(self.w, other.w)
    }
}

/// A point in 3d space.
#[derive(Debug, PartialEq)]
pub struct Point3(Tuple4);

impl Point3 {

    /// Create a point in 3d space.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::Point3;
    ///
    /// let p = Point3::new(4.3, -4.2, 3.1);
    ///
    /// assert_eq!(4.3, p.x());
    /// assert_eq!(-4.2, p.y());
    /// assert_eq!(3.1, p.z());
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple4 { x, y, z, w: 1.0 })
    }
    
    /// Get the x coordinate of the given point.
    pub fn x(&self) -> f64 {
        self.0.x
    }

    /// Get the y coordinate of the given point.
    pub fn y(&self) -> f64 {
        self.0.y
    }

    /// Get the z coordinate of the given point.
    pub fn z(&self) -> f64 {
        self.0.z
    }
}

impl ops::Add<Vector3> for Point3 {
    type Output = Self;
    
    /// Add a vector to a point.
    ///
    /// The result is a new point. The distance from the old
    /// point to the new point is described by the vector
    /// added.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::{Point3, Vector3};
    ///
    /// let p = Point3::new(3.0, -2.0, 5.0);
    /// let v = Vector3::new(-2.0, 3.0, 1.0);
    /// let pv = p + v;
    ///
    /// assert_eq!(Point3::new(1.0, 1.0, 6.0), pv);
    /// ```
    fn add(self, _rhs: Vector3) -> Self::Output {
        Self(self.0 + _rhs.0)
    }
}

impl ops::Sub<Self> for Point3 {
    type Output = Vector3;
    
    /// Subtract a point from another point.
    ///
    /// The resulting vector describes the distance and direction
    /// from the second point to the first one.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::{Point3, Vector3};
    ///
    /// let p1 = Point3::new(3.0, 2.0, 1.0);
    /// let p2 = Point3::new(5.0, 6.0, 7.0);
    /// let pp = p1 - p2;
    ///
    /// assert_eq!(Vector3::new(-2.0, -4.0, -6.0), pp);
    /// ```
    fn sub(self, _rhs: Self) -> Self::Output {
        Vector3(self.0 - _rhs.0)
    }
}

impl ops::Sub<Vector3> for Point3 {
    type Output = Self;
    
    /// Subtract a vector from a point.
    ///
    /// The result is another point. The given vector describes the
    /// distance and direction from the old point to the new one.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::{Point3, Vector3};
    ///
    /// let p = Point3::new(3.0, 2.0, 1.0);
    /// let v = Vector3::new(5.0, 6.0, 7.0);
    /// let pv = p - v;
    ///
    /// assert_eq!(Point3::new(-2.0, -4.0, -6.0), pv);
    /// ```
    fn sub(self, _rhs: Vector3) -> Self::Output {
        Self(self.0 - _rhs.0)
    }
}

/// A vector in 3d space.
#[derive(Debug, PartialEq)]
pub struct Vector3(Tuple4);

impl Vector3 {
    
    /// Create a vector in 3d space.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::Vector3;
    ///
    /// let v = Vector3::new(4.3, -4.2, 3.1);
    ///
    /// assert_eq!(4.3, v.x());
    /// assert_eq!(-4.2, v.y());
    /// assert_eq!(3.1, v.z());
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple4 { x, y, z, w: 0.0 })
    }

    /// Get the x coordinate of the given point.
    pub fn x(&self) -> f64 {
        self.0.x
    }

    /// Get the y coordinate of the given point.
    pub fn y(&self) -> f64 {
        self.0.y
    }

    /// Get the z coordinate of the given point.
    pub fn z(&self) -> f64 {
        self.0.z
    }
}

impl ops::Add<Self> for Vector3 {
    type Output = Self;
    
    /// Add a vector to another vector.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::{Vector3};
    ///
    /// let v1 = Vector3::new(3.0, -2.0, 5.0);
    /// let v2 = Vector3::new(-2.0, 3.0, 1.0);
    /// let vv = v1 + v2;
    ///
    /// assert_eq!(Vector3::new(1.0, 1.0, 6.0), vv);
    /// ```
    fn add(self, _rhs: Vector3) -> Self::Output {
        Self(self.0 + _rhs.0)
    }
}

impl ops::Sub<Self> for Vector3 {
    type Output = Self;
    
    /// Subtract a vector from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::{Vector3};
    ///
    /// let v1 = Vector3::new(3.0, 2.0, 1.0);
    /// let v2 = Vector3::new(5.0, 6.0, 7.0);
    /// let vv = v1 - v2;
    ///
    /// assert_eq!(Vector3::new(-2.0, -4.0, -6.0), vv);
    /// ```
    fn sub(self, _rhs: Self) -> Self::Output {
        Self(self.0 - _rhs.0)
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;
    
    /// Negate the given vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::Vector3;
    ///
    /// let v = Vector3::new(1.0, -2.0, 3.0);
    ///
    /// assert_eq!(Vector3::new(-1.0, 2.0, -3.0), -v);
    /// ```
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;
    
    /// Multiply a vector by a scalar/ fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::Vector3;
    ///
    /// let v = Vector3::new(1.0, -2.0, 3.0);
    ///
    /// assert_eq!(Vector3::new(3.5, -7.0, 10.5), v * 3.5);
    /// ```
    fn mul(self, _rhs: f64) -> Self::Output {
        Self(self.0 * _rhs)
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;
    
    /// Divide a vector by a scalar.
    ///
    /// This has the same effect as multiplying the vector by the
    /// inverse of the scalar, e.g. instead of dividing by `2` one
    /// can simply multiply by `0.5`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sray::math::Vector3;
    ///
    /// let v = Vector3::new(1.0, -2.0, 3.0);
    ///
    /// assert_eq!(Vector3::new(0.5, -1.0, 1.5), v / 2.0);
    /// ```
    fn div(self, _rhs: f64) -> Self::Output {
        Self(self.0 / _rhs)
    }
}

/// Compare two f64 floating point numbers for equality.
fn equal(lhs: f64, rhs: f64) -> bool {
    const EPSILON: f64 = 1e-10;
    (lhs - rhs).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::{equal, Tuple4};

    #[test]
    fn compare_floating_point_number() {
        let x = 3.5_f64;
        let y = -3.5_f64;

        assert!(equal(x, -y));
        assert!(equal(-x, y));
        assert!(!equal(x, y));
        assert!(equal(x, y.powi(2).sqrt()));
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple4 { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let t2 = Tuple4 { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };
        let expected = t1 + t2; 

        assert_eq!(1.0, expected.x);
        assert_eq!(1.0, expected.y);
        assert_eq!(6.0, expected.z);
        assert_eq!(1.0, expected.w);
    }

    #[test]
    fn subtracting_two_tuples() {
        let t1 = Tuple4 { x: 3.0, y: 2.0, z: 1.0, w: 1.0 };
        let t2 = Tuple4 { x: 5.0, y: 6.0, z: 7.0, w: 0.0 };
        let expected = t1 - t2; 

        assert_eq!(-2.0, expected.x);
        assert_eq!(-4.0, expected.y);
        assert_eq!(-6.0, expected.z);
        assert_eq!(1.0, expected.w);
    }

    #[test]
    fn comparing_two_tuples() {
        let t1 = Tuple4 { x: 3.0, y: 2.0, z: 1.0, w: 1.0 };
        let t2 = Tuple4 { x: 3.0, y: 2.0, z: 1.0, w: 1.0 };
        let t3 = Tuple4 { x: 5.0, y: 6.0, z: 7.0, w: 0.0 };
        
        assert!(t1 == t2);
        assert!(t2 == t1);
        assert!(t1 != t3);
        assert!(t3 != t1);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t = Tuple4 { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        assert_eq!(Tuple4{ x: 3.5, y: -7.0, z: 10.5, w: -14.0 }, t * 3.5);
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let t = Tuple4 { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        assert_eq!(Tuple4{ x: 0.5, y: -1.0, z: 1.5, w: -2.0 }, t * 0.5);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t = Tuple4 { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        assert_eq!(Tuple4{ x: 0.5, y: -1.0, z: 1.5, w: -2.0 }, t / 2.0);
    }
}
