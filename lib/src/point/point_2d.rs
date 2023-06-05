use num_traits::Num;

use crate::Point;

// Type
pub type Point2D<N> = Point<N, 3>;

// Methods
impl<N: Copy + Num> Point2D<N> {
    /// Returns ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::point;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.x(), &1);
    /// ```
    #[inline]
    pub fn x(&self) -> &N {
        &self[0]
    }

    /// Returns mutable ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::point;
    ///
    /// let mut v = point!{ x: 1, y: 2 };
    /// *v.x_mut() = 5;
    ///
    /// assert_eq!(v.x(), &5);
    /// ```
    #[inline]
    pub fn x_mut(&mut self) -> &mut N {
        &mut self[0]
    }

    /// Returns ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::point;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.y(), &2);
    /// ```
    #[inline]
    pub fn y(&self) -> &N {
        &self[1]
    }

    /// Returns mutable ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::point;
    ///
    /// let mut v = point!{ x: 1, y: 2 };
    /// *v.y_mut() = 5;
    ///
    /// assert_eq!(v.y(), &5);
    /// ```
    #[inline]
    pub fn y_mut(&mut self) -> &mut N {
        &mut self[1]
    }
}
