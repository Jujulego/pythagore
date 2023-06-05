use num_traits::Num;

use crate::{force, Force};

// Type
pub type Force2D<N> = Force<N, 3>;

// Methods
impl<N: Copy + Num> Force2D<N> {
    /// Returns dx unit force
    ///
    /// ## Example
    /// ```
    /// use pythagore::{force, Force2D};
    ///
    /// assert_eq!(Force2D::unit_dx(), force!{ dx: 1, dy: 0 });
    /// ```
    #[inline]
    pub fn unit_dx() -> Self {
        force![N::one(), N::zero()]
    }

    /// Returns dy unit force
    ///
    /// ## Example
    /// ```
    /// use pythagore::{force, Force2D};
    ///
    /// assert_eq!(Force2D::unit_dy(), force!{ dx: 0, dy: 1 });
    /// ```
    #[inline]
    pub fn unit_dy() -> Self {
        force![N::zero(), N::one()]
    }

    /// Returns ref on dx element of force
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1, dy: 2 }.dx(), &1);
    /// ```
    #[inline]
    pub fn dx(&self) -> &N {
        &self[0]
    }

    /// Returns mutable ref on dx element of force
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// let mut v = force!{ dx: 1, dy: 2 };
    /// *v.dx_mut() = 5;
    ///
    /// assert_eq!(v.dx(), &5);
    /// ```
    #[inline]
    pub fn dx_mut(&mut self) -> &mut N {
        &mut self[0]
    }

    /// Returns ref on dy element of force
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1, dy: 2 }.dy(), &2);
    /// ```
    #[inline]
    pub fn dy(&self) -> &N {
        &self[1]
    }

    /// Returns mutable ref on dy element of force
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// let mut v = force!{ dx: 1, dy: 2 };
    /// *v.dy_mut() = 5;
    ///
    /// assert_eq!(v.dy(), &5);
    /// ```
    #[inline]
    pub fn dy_mut(&mut self) -> &mut N {
        &mut self[1]
    }
}
