use num_traits::Num;

use crate::scalar;
use crate::Vector;

// Type
pub type Vector3D<N> = Vector<N, 4>;

// Methods
impl<N: Copy + Num> Vector3D<N> {
    /// Returns dx unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::{vector, Vector3D};
    ///
    /// assert_eq!(Vector3D::unit_dx(), vector!{ dx: 1, dy: 0, dz: 0 });
    /// ```
    #[inline]
    pub fn unit_dx() -> Self {
        Vector { scalar: scalar![N::one(), N::zero(), N::zero(), N::zero()] }
    }

    /// Returns dy unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::{vector, Vector3D};
    ///
    /// assert_eq!(Vector3D::unit_dy(), vector!{ dx: 0, dy: 1, dz: 0 });
    /// ```
    #[inline]
    pub fn unit_dy() -> Self {
        Vector { scalar: scalar![N::zero(), N::one(), N::zero(), N::zero()] }
    }

    /// Returns dz unit vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::{vector, Vector3D};
    ///
    /// assert_eq!(Vector3D::unit_dz(), vector!{ dx: 0, dy: 0, dz: 1 });
    /// ```
    #[inline]
    pub fn unit_dz() -> Self {
        Vector { scalar: scalar![N::zero(), N::zero(), N::one(), N::zero()] }
    }

    /// Returns ref on dx element of vector
    ///
    /// ### Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2, dz: 3 }.dx(), &1);
    /// ```
    #[inline]
    pub fn dx(&self) -> &N {
        &self.scalar[0]
    }

    /// Returns mutable ref on dx element of vector
    ///
    /// ### Example
    /// ```
    /// use pythagore::vector;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2, dz: 3 };
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
    /// ### Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2, dz: 3 }.dy(), &2);
    /// ```
    #[inline]
    pub fn dy(&self) -> &N {
        &self.scalar[1]
    }

    /// Returns mutable ref on dy element of vector
    ///
    /// ### Example
    /// ```
    /// use pythagore::vector;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2, dz: 3 };
    /// *v.dy_mut() = 5;
    ///
    /// assert_eq!(v.dy(), &5);
    /// ```
    #[inline]
    pub fn dy_mut(&mut self) -> &mut N {
        &mut self.scalar[1]
    }

    /// Returns ref on dz element of vector
    ///
    /// ### Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: 2, dz: 3 }.dz(), &3);
    /// ```
    #[inline]
    pub fn dz(&self) -> &N {
        &self.scalar[2]
    }

    /// Returns mutable ref on dz element of vector
    ///
    /// ### Example
    /// ```
    /// use pythagore::vector;
    ///
    /// let mut v = vector!{ dx: 1, dy: 2, dz: 3 };
    /// *v.dz_mut() = 5;
    ///
    /// assert_eq!(v.dz(), &5);
    /// ```
    #[inline]
    pub fn dz_mut(&mut self) -> &mut N {
        &mut self.scalar[2]
    }
}
