#[macro_export]
macro_rules! forward_ref_unop {
    ($trt:ident, $t:ty, $op:ident, $($cst:tt)*) => {
        impl$($cst)* $trt for &$t {
            type Output = <$t as $trt>::Output;

            #[inline]
            fn $op(self) -> Self::Output {
                (*self).$op()
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
                (*self).$op(*rhs);
            }
        }
    };
}

#[macro_export]
macro_rules! forward_ref_binop {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<&$rhs> for $lhs {
            type Output = <$lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                self.$op(*rhs)
            }
        }

        impl$($cst)+ $trt<$rhs> for &$lhs {
            type Output = <$lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: $rhs) -> Self::Output {
                (*self).$op(rhs)
            }
        }

        impl$($cst)+ $trt<&$rhs> for &$lhs {
            type Output = <$lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                (*self).$op(*rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_binop {
    ($trt:ident, $lhs:ty, $op:ident, $rhs:ty, $($cst:tt)*) => {
        impl$($cst)+ $trt<$lhs> for $rhs {
            type Output = <$lhs as $trt<$rhs>>::Output;

            #[inline]
            fn $op(self, rhs: $lhs) -> Self::Output {
                rhs.$op(self)
            }
        }

        forward_ref_binop!($trt, $rhs, $op, $lhs, $($cst)+);
    };
}