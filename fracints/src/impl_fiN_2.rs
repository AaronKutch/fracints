macro_rules! impl_fiN_2 {
    (
        $ty:ident,
        $tyD:ident,
        $iX:ident,
        $uX:ident,
        $iD:ident,
        $uD:ident,
        $ishift:expr, // for $ty
        $ushift:expr, // for $ty
        $clo:expr,
        $chi:expr
    ) => {
        impl $ty {
            /// Numerically exact multiplication of `self` and `other`
            ///
            /// # Overflow Behavior
            ///
            /// `fiN::MIN.full_mul(fiN::MIN)` -> `niY::MIN`, where niY is a normalized integer type
            /// with double the number of bits of fiN.
            ///
            /// # Examples
            /// 
            /// // keep the note on one line
            /// ```
            /// #[macro_use]
            /// use normints::*;
            ///
            /// // note: `from_str_radix` and `to_string_radix` sometimes have small rounding errors when converting
            /// // and displaying, but the `full_mul` on the fiN between those functions is truly exact.
            /// assert_eq!(
            ///     fi32::from_str_radix(&"0.123456789",10).unwrap()
            ///         .wrapping_full_mul(fi32::from_str_radix(&"0.123456789",10).unwrap())
            ///         .to_string_radix(10),
            ///     "0.01524157879479210234".to_string()
            /// );
            /// 
            /// // overflow corner case
            /// assert_eq!(fi32::MIN.wrapping_full_mul(fi32::MIN), fi64::MIN);
            /// ```
            pub fn wrapping_full_mul(self, other: Self) -> $tyD {
                $tyD(($iD::from(self.0) * $iD::from(other.0)) << 1)
            }

            /// Division of `self` and `other`, which returns a integer representing a fixed point
            /// number with X + 1 bits in the integer part and X - 1 bits in the fractional part
            /// Note: this is not numerically exact, but it cannot overflow with any input. Be
            /// careful because it can still produce a `fiN::MIN` value.
            /// 
            /// # Examples
            ///
            /// ```
            /// use std::i64;
            /// #[macro_use]
            /// use normints::*;
            /// 
            /// let a = fi32::from_str_radix(&"0.765432198",10).unwrap();
            /// let b = fi32::from_str_radix(&"0.153456789",10).unwrap();
            /// assert_eq!(a.wrapping_full_div(b), 10711504782i64);
            /// // a divided by b should equal 4.9879331047...
            /// // 10711504782/2^31 equals     4.9879331058...
            ///
            /// let c = fi32::from_str_radix(&"0.123456789",10).unwrap();
            /// let d = fi32::from_str_radix(&"0.987654321",10).unwrap();
            /// // c is less than d, and is then able to fit into a `fi32`
            /// assert_eq!(fi32(c.wrapping_full_div(d) as i32).to_string(), "0.1249999986".to_string());
            /// 
            /// let c = fi32::from_str_radix(&"0.123456789",10).unwrap();
            /// let d = fi32::from_str_radix(&"-0.987654321",10).unwrap();
            /// // works with signs
            /// assert_eq!(fi32(c.wrapping_full_div(d) as i32).to_string(), "-0.1249999986".to_string());
            /// 
            /// // not an edge case
            /// assert_eq!(fi32::MIN.wrapping_full_div(fi32::MIN), 2147483648i64);
            /// ```
            pub fn wrapping_full_div(self, other: Self) -> $iD {
                ($iD::from(self.0) << $ishift) / $iD::from(other.0)
            }

            pub fn from_rounded(other: $tyD) -> $ty {
                if (other.0 & (1 << ($ishift - 1))) != 0 {
                    let tmp = $ty((other.0 >> $ushift) as $iX);
                    if tmp == $ty::ONE {
                        // prevent overflow
                        $ty::ONE
                    } else {
                        tmp + $ty::ULP
                    }
                } else {
                    $ty((other.0 >> $ushift) as $iX)
                }
            }

            pub fn from_truncated(other: $tyD) -> $ty {
                $ty((other.0 >> $ushift) as $iX)
            }

            pub fn cos_sin_pi(self) -> ($ty, $ty) {
                let (x, y) = $tyD::from(self).cos_sin_pi_taylor();
                (
                    $ty::from_rounded(x),
                    $ty::from_rounded(y)
                )
            }

            /*/// Calculates `sin((2pi/4) * theta)`
            /// The max error is 0.5 ULPs from the true value
            pub fn cos_2pidiv4(theta: $ty) -> $ty {
                $tyD::sin_2pidiv4_fast_taylor($tyD::from(theta))
            }*/
/*
            /// Calculates `sin((2pi/2^(X+3)) * theta)`
            /// Returns a tuple of the cosine and sine of the input, which is assumed to be in units of 2pi/2^(X+3) radians (e.g. for fi32 it is sin((2pi/2^35) * theta) and for fi64 it is sin((2pi/2^67) * theta)). The reason for the `+3` has to do with internal optimizations and has the added bonus that the function is more continuous than the other trig functions (e.g. a circle is graphed using max precision using this function, the pixels will be around TODO ULPs from each other, contrasted with TODO for the fourier based functions).
            /// This function has the properties of being inaccurate by TODO, the output is very close to normal, it is relatively continuous
            /// It is the fastest way to compute normal vectors except for using lookup tables
            pub fn cos_sin_2pidiviXplus3_fast(theta: $iD) -> ($ty, $ty) {
                //take a circular arc of the unit circle from 0 to 2pi/8 radians, and make the rational bezier curve that generates that arc. Translate the points so that the middle control point is at the origin. This eliminates some terms, and when we calculate on this curve we undo the translation (but minus 1 ULP to prevent overflow), followed by reflections to get the answer.
                //let translation = ($ty::ONE,($clo.sqrt2minus1));
                let mask_lo: $iD = {
                    let mut temp = 0;
                    for shift in 0..$ushift {
                        temp |= 1 << shift;
                    }
                    temp
                };
                let mask_hi: $iD = 111 << $ushift;
                //let function_lookup = [box |x| {x+1},box |x| {x-1}];
                //function_lookup[mask_hi & theta]()
                ($ty(0),$ty(0))
            }*/
        }

        /*impl $tyD {
            /// Computes the square root of `self`, and returns it as a normint with half the bits of `self`
            /// note: the performance of this function is poor with numbers close to zero
            ///
            /// # Overflow Behavior
            ///
            /// if the number is negative, this returns zero
            pub fn wrapping_halfsize_sqrt(&self) -> $ty {
                //find the most significant binary digit that is 1. Unrolling the first 8 iterations for speed
                let val = self.0 as $uD;
                if val <= 0 {return $ty::ZERO} //handles 0 and the most significant digit
                let most_significant_place: usize =
                    if (val & (1 << ($ishift - 1))) != 0 {$ishift - 1} else
                    if (val & (1 << ($ishift - 2))) != 0 {$ishift - 2} else
                    if (val & (1 << ($ishift - 3))) != 0 {$ishift - 3} else
                    if (val & (1 << ($ishift - 4))) != 0 {$ishift - 4} else
                    if (val & (1 << ($ishift - 5))) != 0 {$ishift - 5} else
                    if (val & (1 << ($ishift - 6))) != 0 {$ishift - 6} else
                    if (val & (1 << ($ishift - 7))) != 0 {$ishift - 7} else
                    if (val & (1 << ($ishift - 8))) != 0 {$ishift - 8} else
                    {
                        let i: usize = $ishift - 9;
                        loop {
                            //val == 0 is caught above so loop terminates at or before i == 0
                            if (val & (1 << i)) != 0 {break i}
                            i -= 1;
                        }
                    };
                //the lookup table has the square roots of 2^(0) up to 2^($ushift)
                //the maximum error of this approximation to the true square root, for any input, is (2^(X/2))*(sqrt(2)-1)
                let mut temp = $clo.lut_wrapping_halfsize_sqrt[most_significant_place] as $uD;
                for _ in 0..$clo.iters_wrapping_halfsize_sqrt {
                    temp = (val + (temp*temp)) / (temp << 1);
                }
                //TODO: final checks
                $tyD(temp as $iX)

/*
                let mut val0 = $ty::ZERO;
                let mut diff = $ty::wrapping_normint_div_int($ty::MAX,2) + $ty::ULP;
                while diff > $ty::ULP {
                    let val1 = val0 + diff;
                    let val1_sqr = val1*val1;
                    if val1_sqr <= *self {
                        val0 = val1;
                    }
                    diff = $ty::wrapping_normint_div_int(diff,2);
                }
                Some(val0)
                */
            }
        }*/

        /// Lossless and unfailing conversion of a normint to one with double the number of bits.
        /// There is not a conversion the other way to stay consistent with the primitives and to
        /// avoid rounding error. Use TODO for truncation and TODO for rounding
        impl From<$ty> for $tyD {
            fn from(x: $ty) -> Self {
                $tyD($iD::from(x.0) << $ushift)
            }
        }
    }
}
