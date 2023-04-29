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
- Constants can only be set to an **expression**, not the result of a **function call** or any other value that is computed at **runtime**.
- Constants are valid for the entire runtime of an application, within the scope they were declared. 

# Shadowing a Variable

- Declare a *new variable* with the same name as a *previous variable* but created with a new **binding**. 
- This is referred to as **shadowing** because the new variable *shadows* the previous variable. 
- The previous variable still exists, however it cannot be referred to within this scope any longer. 

```rust
// declare a new variable with the same name as a previous variable, also creating a new binding

fn main() {
    let x = 5;
    let x = x + 1; // x is now 6
}
```

- The variable's value can be updated using the **let** keyword.