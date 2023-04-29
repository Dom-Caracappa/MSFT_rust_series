# Declare Variables

- Bound to varibles using keyword **let**.
- Variables are **immutable** by default.

``` rust
fn main() {
    let x = 5;
    x = 6; // causes an error, cause this value can't change
}

fn main() {
    let mut x = 5;
    x = 6 // updates value to 6
}
```

# About Immutability in Variables

- Immutable variables can create predictability in our code.
- However, sometimes it's more convenient to make **mutable** variables we can change.
- Both strategies have trade-offs:
    - Larger data-structures: Changing mutable variables may be faster/easier than creating new ones. 
    - Smaller data-structures: Creating new variables might be easier to work with. 
- Immutable variables can be compared to **constants**.
    - Like *immutable variables*, constants possess a value that cannot be changed. 


# Declare a Constant

- Use the keyword **const**, the name of the constant, the *type of value*, the actual value you wish to assign. 

```rust
fn main(){
    const SCORE_LIMIT: u32 = 100;  // u32 is an unsigned 32-bit integer
}
```

- Conventions for constants dictate that names should be *capitalized*, with *underscores* between words.