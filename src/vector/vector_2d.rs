use num_traits::Num;

use crate::scalar;
use crate::Vector;

// Type
pub type Vector2D<N> = Vector<N, 3>;

// Methods
impl<N: Copy + Num> Vector2D<N> {
    /// Returns dx unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::{vector, Vector2D};
    ///
    /// assert_eq!(Vector2D::unit_dx(), vector!{ dx: 1, dy: 0 });
    /// ```
    #[inline]
    pub fn unit_dx() -> Self {
        Vector { scalar: scalar![N::one(), N::zero(), N::zero()] }
    }

    /// Returns dy unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::{vector, Vector2D};
    ///
    /// assert_eq!(Vector2D::unit_dy(), vector!{ dx: 0, dy: 1 });
    /// ```
    #[inline]
    pub fn unit_dy() -> Self {
        Vector { scalar: scalar![N::zero(), N::one(), N::zero()] }
    }

    /// Returns ref on dx element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2 }.dx(), &1);
    /// ```
    #[inline]
    pub fn dx(&self) -> &N {
        &self.scalar[0]
    }

    /// Returns mutable ref on dx element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2 };
    /// *v.dx_mut() = 5;
    ///
    /// assert_eq!(v.dx(), &5);
    /// ```
    #[inline]
    pub fn dx_mut(&mut self) -> &mut N {
        &mut self.scalar[0]
    }

    /// Returns ref on dy element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2 }.dy(), &2);
    /// ```
    #[inline]
    pub fn dy(&self) -> &N {
        &self.scalar[1]
    }

    /// Returns mutable ref on dy element of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2 };
    /// *v.dy_mut() = 5;
    ///
    /// assert_eq!(v.dy(), &5);
    /// ```
    #[inline]
    pub fn dy_mut(&mut self) -> &mut N {
        &mut self.scalar[1]
    }
}
