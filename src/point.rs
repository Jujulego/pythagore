// use num_traits::{Num, Zero};
// use std::ops;
//
// use crate::Vector;
// use crate::vector;
//
// /// `Point<T>` structure for 2 dimension points
// ///
// /// ## Usage
// /// ```
// /// use pythagore::Point;
// ///
// /// let a = Point { x: 1, y: 2 };
// /// let b = Point { x: 1, y: 2 };
// ///
// /// assert_eq!(a, b);
// /// assert_eq!(Point::origin(), Point { x: 0, y: 0 });
// /// ```
// #[derive(Debug, Eq)]
// pub struct Point<T> {
//     pub x: T,
//     pub y: T,
// }
//
// // Values
// impl<T: Zero> Point<T> {
//     pub fn origin() -> Self {
//         Point {
//             x: T::zero(),
//             y: T::zero(),
//         }
//     }
// }
//
// // Operators
// impl<T: PartialEq> PartialEq for Point<T> {
//     fn eq(&self, other: &Point<T>) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }
//
// impl<T: Copy + Num> ops::Add<Vector<T, 2>> for Point<T> {
//     type Output = Point<T>;
//
//     fn add(self, rhs: Vector<T, 2>) -> Self::Output {
//         Point {
//             x: self.x + *rhs.dx(),
//             y: self.y + *rhs.dy(),
//         }
//     }
// }
//
// impl<T: Copy + Num> ops::AddAssign<Vector<T, 2>> for Point<T> {
//     fn add_assign(&mut self, rhs: Vector<T, 2>) {
//         self.x += *rhs.dx();
//         self.y += *rhs.dy();
//     }
// }
//
// impl<T: Num> ops::Sub for Point<T> {
//     type Output = Vector<T, 2>;
//
//     fn sub(self, rhs: Point<T>) -> Self::Output {
//         vector![self.x - rhs.x, self.y - rhs.y]
//     }
// }
//
// impl<T: Num> ops::Sub<Vector<T, 2>> for Point<T> {
//     type Output = Point<T>;
//
//     fn sub(self, rhs: Vector<T, 2>) -> Self::Output {
//         Point {
//             x: self.x - rhs.dx(),
//             y: self.y - rhs.dy(),
//         }
//     }
// }
//
// impl<T: Num> ops::SubAssign<Vector<T, 2>> for Point<T> {
//     fn sub_assign(&mut self, rhs: Vector<T, 2>) {
//         self.x -= rhs.dx();
//         self.y -= rhs.dy();
//     }
// }
//
// // Tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_should_return_origin() {
//         assert_eq!(Point::origin(), Point { x: 0, y: 0 });
//     }
//
//     #[test]
//     fn it_should_be_equal() {
//         let a = Point { x: 1, y: 2 };
//         let b = Point { x: 1, y: 2 };
//
//         assert_eq!(a, b);
//     }
//
//     #[test]
//     fn it_should_not_be_equal_x() {
//         let a = Point { x: 1, y: 2 };
//         let b = Point { x: 2, y: 2 };
//
//         assert_ne!(a, b);
//     }
//
//     #[test]
//     fn it_should_not_be_equal_y() {
//         let a = Point { x: 1, y: 2 };
//         let b = Point { x: 1, y: 1 };
//
//         assert_ne!(a, b);
//     }
//
//     #[test]
//     fn it_should_return_new_add_translated_point() {
//         let a = Point { x: 1, y: 2 };
//         let v = Vector { dx: 3, dy: 4 };
//
//         assert_eq!(a + v, Point { x: 4, y: 6 });
//     }
//
//     #[test]
//     fn it_should_add_translate_point() {
//         let mut a = Point { x: 1, y: 2 };
//         a += Vector { dx: 3, dy: 4 };
//
//         assert_eq!(a, Point { x: 4, y: 6 });
//     }
//
//     #[test]
//     fn it_should_return_difference_between_points() {
//         let a = Point { x: 1, y: 2 };
//         let b = Point { x: 3, y: 4 };
//
//         assert_eq!(a - b, Vector { dx: -2, dy: -2 });
//     }
//
//     #[test]
//     fn it_should_return_new_sub_translated_point() {
//         let a = Point { x: 1, y: 2 };
//         let v = Vector { dx: 3, dy: 4 };
//
//         assert_eq!(a - v, Point { x: -2, y: -2 });
//     }
//
//     #[test]
//     fn it_should_sub_translate_point() {
//         let mut a = Point { x: 1, y: 2 };
//         a -= Vector { dx: 3, dy: 4 };
//
//         assert_eq!(a, Point { x: -2, y: -2 });
//     }
// }