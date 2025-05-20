
# fracints - Fractional Integers of the unit range

This crate provides a special case of fixed point numbers in the form of all fractional bits. The `fiN` signed types (where the `N` is the number of bits) represent values in the range [-1.0, 1.0), and for the `fuN` unsigned types [0, 1.0). In calculations, they are treated as if they were integers of the same size but had `/ 2^N` attached to them.

## Pros

- They are the most efficient representation of the uniform partition of the -1 to 1 range of real numbers (in the case of `fiN`) and the 0 to 1 range of real numbers (in the case of `fuN`). The size of an ULP of each type is approximately:
    - `fi8` => 7.81e-3
    - `fu8` => 3.91e-3
    - `fi16` => 3.0517e-5
    - `fu16` => 1.5259e-5
    - `fi32` => 4.6566e-10
    - `fu32` => 2.3283e-10
    - `fi64` => 1.0842e-19
    - `fu64` => 5.4210e-20
    - `fi128` => 5.8775e-39
    - `fu128` => 2.9387e-39
- No bizarre numeric behavior (bizarre meaning stuff that is not small truncation errors or errors due to obvious overflow cases) in all of the functions of this crate, except for the edge and corner cases involving `fiN::MIN`. However, this crate has special handling for preventing fiN::MIN from being produced, see the section in the docs on preventing overflow.
- Multiplication overflow is impossible (except for edge cases involving `fiN::MIN`)
- Many of the trig functions have no reliance on `f32` or `f64` trigonometry, and generate as little error as is possible (they are <= 0.5 ULP of the true value). This means that one can use perfect trigonometry on a device lacking an fpu.
- the functions for these types are very similar to the way the primitive integer types work (e.g. how the `wrapping_X`, `overflowing_X`, `checked_X`, `saturating_X` and others relate to each other), except that the bounds may be `NEG_ONE` and `ONE` instead of `MIN` and `MAX`, see the individual function docs for details.
- If for some reason a special value is needed (e.g. there is a need for a value in the fiN domain that represents NaN instead of having a custom enum which would take up more bytes), `fiN::MIN` can be used since it is suggested to avoid it in normal calculations.
- Good enough dynamic range for most applications, even without the exponent that floating point has. If `fi32` is used and 1 ULP is defined as a micrometer, it gives ~4 kilometers of room. If `fi64` is used at the nanometer scale, then it gives ~18 million kilometers. If `fi128` is used at the 1e-15 meter scale, then it gives ~36 million light years.

## Cons

- Postive numeric 1.0 cannot be exactly represented by either fiN or fuN due to the fact that integers can only represent up to (2 ^ N) - 1. Instead, the `fiN::ONE` constant uses the true numeric 1 minus 1 ULP. Negative numeric 1 (`fiN::MIN`) can be represented exactly by fiN, but as noted elsewhere it leads to all kinds of issues unless carefully handled. I believe that the disadvantage is worth it because of the special cases it avoids (such as no multiplication overflow besides a corner case) and greater idealness it enables.
- Simple operations are a little slower to compute than the equivalent floating point operation, and transcendentals are considerably slower to compute (roughly speaking, but depending on the hardware and how much instruction level parallelism there is the performance could go anywhere).

## Other notes

Initial development of all the basics is complete, but I did not get around to implementing unsigned fracints and trigonometry. There are a bunch of TODOs where I know I could improve performance of serialization and some existing algorithms. Any issues or PRs are welcome if someone finds this useful.
