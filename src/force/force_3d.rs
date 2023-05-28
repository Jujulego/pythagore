use num_traits::Num;

use crate::scalar;
use crate::Force;

// Type
pub type Force3D<N> = Force<N, 4>;

// Methods
impl<N: Copy + Num> Force3D<N> {
    /// Returns dx unit force
    ///
    /// ## Example
    /// ```
    /// use pythagore::{force, Force3D};
    ///
    /// assert_eq!(Force3D::unit_dx(), force!{ dx: 1, dy: 0, dz: 0 });
    /// ```
    #[inline]
    pub fn unit_dx() -> Self {
        Force { scalar: scalar![N::one(), N::zero(), N::zero(), N::zero()] }
    }

    /// Returns dy unit force
    ///
    /// ## Example
    /// ```
    /// use pythagore::{force, Force3D};
    ///
    /// assert_eq!(Force3D::unit_dy(), force!{ dx: 0, dy: 1, dz: 0 });
    /// ```
    #[inline]
    pub fn unit_dy() -> Self {
        Force { scalar: scalar![N::zero(), N::one(), N::zero(), N::zero()] }
    }

    /// Returns dz unit force
    ///
    /// ## Example
    /// ```
    /// use pythagore::{force, Force3D};
    ///
    /// assert_eq!(Force3D::unit_dz(), force!{ dx: 0, dy: 0, dz: 1 });
    /// ```
    #[inline]
    pub fn unit_dz() -> Self {
        Force { scalar: scalar![N::zero(), N::zero(), N::one(), N::zero()] }
    }

    /// Returns ref on dx element of force
    ///
    /// ### Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }.dx(), &1);
    /// ```
    #[inline]
    pub fn dx(&self) -> &N {
        &self.scalar[0]
    }

    /// Returns mutable ref on dx element of force
    ///
    /// ### Example
    /// ```
    /// use pythagore::force;
    ///
    /// let mut v = force!{ dx: 1, dy: 2, dz: 3 };
    /// *v.dx_mut() = 5;
    ///
    /// assert_eq!(v.dx(), &5);
    /// ```
    #[inline]
    pub fn dx_mut(&mut self) -> &mut N {
        &mut self.scalar[0]
    }

    /// Returns ref on dy element of force
    ///
    /// ### Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }.dy(), &2);
    /// ```
    #[inline]
    pub fn dy(&self) -> &N {
        &self.scalar[1]
    }

    /// Returns mutable ref on dy element of force
    ///
    /// ### Example
    /// ```
    /// use pythagore::force;
    ///
    /// let mut v = force!{ dx: 1, dy: 2, dz: 3 };
    /// *v.dy_mut() = 5;
    ///
    /// assert_eq!(v.dy(), &5);
    /// ```
    #[inline]
    pub fn dy_mut(&mut self) -> &mut N {
        &mut self.scalar[1]
    }

    /// Returns ref on dz element of force
    ///
    /// ### Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }.dz(), &3);
    /// ```
    #[inline]
    pub fn dz(&self) -> &N {
        &self.scalar[2]
    }

    /// Returns mutable ref on dz element of force
    ///
    /// ### Example
    /// ```
    /// use pythagore::force;
    ///
    /// let mut v = force!{ dx: 1, dy: 2, dz: 3 };
    /// *v.dz_mut() = 5;
    ///
    /// assert_eq!(v.dz(), &5);
    /// ```
    #[inline]
    pub fn dz_mut(&mut self) -> &mut N {
        &mut self.scalar[2]
    }
}
