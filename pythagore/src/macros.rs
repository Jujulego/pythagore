#[macro_export]
macro_rules! owned_unop {
    ($trt:ident, $t:ty, $op:ident, $($cst:tt)*) => {
        impl$($cst)* $trt for $t {
            type Output = <&'static $t as $trt>::Output;

            #[inline]
            fn $op(self) -> Self::Output {
                (&self).$op()
            }
        }
    };
}

#[macro_export]
macro_rules! forward_ref_op_assign {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<&$rhs> for $lhs {
            #[inline]
            fn $op(&mut self, rhs: &$rhs) {
                self.$op(*rhs);
            }
        }
    };
}

#[macro_export]
macro_rules! owned_op_assign {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<$rhs> for $lhs {
            #[inline]
            fn $op(&mut self, rhs: $rhs) {
                self.$op(&rhs);
            }
        }
    };
}

#[macro_export]
macro_rules! forward_ref_binop {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<&$rhs> for $lhs {
            type Output = <&'static $lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                self.$op(*rhs)
            }
        }

        impl$($cst)+ $trt<$rhs> for $lhs {
            type Output = <&'static $lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: $rhs) -> Self::Output {
                (&self).$op(rhs)
            }
        }

        impl$($cst)+ $trt<&$rhs> for &$lhs {
            type Output = <&'static $lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                (&self).$op(*rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! owned_binop {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<$rhs> for &$lhs {
            type Output = <&'static $lhs as $trt<&'static $rhs>>::Output;

            #[inline]
            fn $op(self, rhs: $rhs) -> Self::Output {
                self.$op(&rhs)
            }
        }

        impl$($cst)+ $trt<&$rhs> for $lhs {
            type Output = <&'static $lhs as $trt<&'static $rhs>>::Output;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                (&self).$op(rhs)
            }
        }

        impl$($cst)+ $trt<$rhs> for $lhs {
            type Output = <&'static $lhs as $trt<&'static $rhs>>::Output;

            #[inline]
            fn $op(self, rhs: $rhs) -> Self::Output {
                (&self).$op(&rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_owned_binop {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<&$lhs> for &$rhs {
            type Output = <&'static $lhs as $trt<&'static $rhs>>::Output;

            #[inline]
            fn $op(self, rhs: &$lhs) -> Self::Output {
                rhs.$op(self)
            }
        }

        owned_binop!($trt, $rhs, $op, $lhs, $($cst)+);
    };
}