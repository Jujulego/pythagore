use num_traits::Num;

use crate::Point;

// Type
pub type Point2D<T> = Point<T, 3>;

// Methods
impl<T: Copy + Num> Point2D<T> {
    /// Returns ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.x(), &1);
    /// ```
    #[inline]
    pub fn x(&self) -> &T {
        &self.scalar[0]
    }

    /// Returns mutable ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2 };
    /// *v.x_mut() = 5;
    ///
    /// assert_eq!(v.x(), &5);
    /// ```
    #[inline]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.scalar[0]
    }

    /// Returns ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.y(), &2);
    /// ```
    #[inline]
    pub fn y(&self) -> &T {
        &self.scalar[1]
    }

    /// Returns mutable ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2 };
    /// *v.y_mut() = 5;
    ///
    /// assert_eq!(v.y(), &5);
    /// ```
    #[inline]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.scalar[1]
    }
}
