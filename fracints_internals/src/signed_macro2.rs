#[macro_export]
macro_rules! impl_signed_double {
    ($ty:ident, $tyD:ident, $iX:ident, $uX:ident, $iD:ident, $uD:ident) => {
        impl FracintDouble for $ty {
            type Double = $tyD;

            fn saturating_widening_mul(self, rhs: Self) -> $tyD {
                if (self == Self::MIN) && (rhs == Self::MIN) {
                    $tyD::MAX
                } else {
                    $tyD(($iD::from(self.0).wrapping_mul($iD::from(rhs.0))) << 1)
                }
            }
        }

        impl FracintHalf for $tyD {
            type Half = $ty;

            fn split(self) -> ($ty, $ty) {
                ($ty(self.0 as $iX), $ty((self.0 >> $ty::BITS) as $iX))
            }

            fn truncate(self) -> $ty {
                $ty((self.0 >> $ty::BITS) as $iX)
            }
        }

        impl From<$ty> for $tyD {
            /// Lossless conversion
            fn from(x: $ty) -> Self {
                $tyD($iD::from(x.0) << $ty::BITS)
            }
        }
    };
}
