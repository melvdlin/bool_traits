# bool_traits

Use boolean expressions as trait bounds.

# Examples

```rust
#![feature(generic_const_exprs)]

use bool_traits::*;

fn create_small_u8_array<const N: usize>() -> [u8; N]
    where
        (): True<{ N <= 4 }>,
{
    [0; N]
}
// this compiles:
let array = create_small_u8_array::<3 > ();
// this does not:
let array = create_small_u8_array::<5 > ();
```