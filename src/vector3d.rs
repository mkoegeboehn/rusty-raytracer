use num::Float;
use std::convert::From;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Generic three-dimensional vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3d<T> {
    /// Creates a new Vector3d with the given x, y, and z values.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector = Vector3d::new(1, 2, 3);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    /// Creates a new Vector3d with the x, y, and z set to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector = Vector3d::with_value(1);
    /// ```
    pub fn with_value(val: T) -> Self
    where
        T: Clone,
    {
        Self {
            x: val.clone(),
            y: val.clone(),
            z: val,
        }
    }
    /// Takes the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector1 = Vector3d::new(1, 2, 3);
    /// let vector2 = Vector3d::new(4, 5, 6);
    /// let dot_product = vector1.dot(vector2);
    ///
    /// assert_eq!(32, dot_product);
    /// ```
    pub fn dot(self, rhs: Vector3d<T>) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Takes the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector1 = Vector3d::new(1, 2, 3);
    /// let vector2 = Vector3d::new(4, 5, 6);
    /// let dot_product = vector1.dot(vector2);
    ///
    /// assert_eq!(32, dot_product);
    /// ```
    pub fn cross(self, rhs: Vector3d<T>) -> Vector3d<T>
    where
        T: Copy + Sub<Output = T> + Mul<Output = T>,
    {
        Vector3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    /// Returns the length/magnitude of the vector squared.
    pub fn length_squared(&self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: Float> Vector3d<T> {
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
}

impl<T: Default> Default for Vector3d<T> {
    fn default() -> Self {
        Vector3d::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T: Add<Output = T>> Add for Vector3d<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Copy + Add<T, Output = T>> AddAssign for Vector3d<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl<T: Sub<Output = T>> Sub for Vector3d<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Copy + Sub<T, Output = T>> SubAssign for Vector3d<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl<T: Neg<Output = T>> Neg for Vector3d<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector3d<T> {
    type Output = Self;

    fn mul(self, scale: T) -> Self::Output {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl<T: Copy + Mul<T, Output = T>> MulAssign<T> for Vector3d<T> {
    fn mul_assign(&mut self, scale: T) {
        *self = *self * scale
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vector3d<T> {
    type Output = Self;

    fn div(self, scale: T) -> Self::Output {
        Self {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}

impl<T: Copy + Div<T, Output = T>> DivAssign<T> for Vector3d<T> {
    fn div_assign(&mut self, scale: T) {
        *self = *self / scale
    }
}

impl<T> From<[T; 3]> for Vector3d<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3d<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}
