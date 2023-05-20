use num_traits::Num;

use crate::{Scalar, scalar};
use crate::Vector;

// Type
pub type Vector2D<T> = Vector<T, 3>;

// Methods
impl<T: Copy + Num> Vector2D<T> {
    /// Returns dx unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Vector2D::unit_dx(), vector!{ dx: 1, dy: 0 });
    /// ```
    #[inline]
    pub fn unit_dx() -> Self {
        Vector { scalar: scalar![T::one(), T::zero(), T::zero()] }
    }

    /// Returns dy unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Vector2D::unit_dy(), vector!{ dx: 0, dy: 1 });
    /// ```
    #[inline]
    pub fn unit_dy() -> Self {
        Vector { scalar: scalar![T::zero(), T::one(), T::zero()] }
    }

    /// Returns ref on dx element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2 }.dx(), &1);
    /// ```
    #[inline]
    pub fn dx(&self) -> &T {
        &self.scalar[0]
    }

    /// Returns mutable ref on dx element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2 };
    /// *v.dx_mut() = 5;
    ///
    /// assert_eq!(v.dx(), &5);
    /// ```
    #[inline]
    pub fn dx_mut(&mut self) -> &mut T {
        &mut self.scalar[0]
    }

    /// Returns ref on dy element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2 }.dy(), &2);
    /// ```
    #[inline]
    pub fn dy(&self) -> &T {
        &self.scalar[1]
    }

    /// Returns mutable ref on dy element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2 };
    /// *v.dy_mut() = 5;
    ///
    /// assert_eq!(v.dy(), &5);
    /// ```
    #[inline]
    pub fn dy_mut(&mut self) -> &mut T {
        &mut self.scalar[1]
    }
}
