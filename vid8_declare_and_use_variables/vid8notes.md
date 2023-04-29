# Variables
Variable bindings have a **scope**, and are constrained to live in a **block**; a block is a collection of statements enclosed by braces `{}`. 


```rust
fn main() {
    let x: i32 = 1;
    println!("The value of x is: {}", x);
    let y: bool = true:
    println!("The value of y is: {}", y);
}
```
Output using **Cargo**:

```bash
~/variables [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo run
   Compiling variables v0.1.0 (~/MSFT_rust_series/MSFT_rust_series/vid8_declare_and_use_variables/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/variables`
The value of x is: 1
The value of y is: true
```
Variable bindings are **immutable** by default, meaning that they cannot be changed during the program's runtime. If we take the code above and append it to include changing the `x` variable, we'll get an error. 

```rust
fn main() {
    let x: i32 = 1;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);

    let y: bool = true;
    println!("The value of y is: {}", y);
}
```

Output:

```shell
~/variables [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo run
   Compiling variables v0.1.0 (~/MSFT_rust_series/vid8_declare_and_use_variables/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:4
  |
2 |    let x: i32 = 1;
  |        -
  |        |
  |        first assignment to `x`
  |        help: consider making this binding mutable: `mut x`
3 |    println!("The value of x is: {}", x);
4 |    x = 2;
  |    ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error

```

This is a basic example of **immutability**. Our `x` variable is immutable (it cannot be changed) because we didn't *explicitly* define it otherwise by using the keyword **mut**, as demostrated below:

```rust
fn main() {
    let mut x: i32 = 1;  // explicitly defines the x variable as mutable
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);
    let y: bool = true;
    println!("The value of y is: {}", y);
}
```
Output:

```bash
~/variables [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo run
   Compiling variables v0.1.0 (~/MSFT_rust_series/MSFT_rust_series/vid8_declare_and_use_variables/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/variables`
The value of x is: 1
The value of x is: 2
The value of y is: true
```

If two (2) variables with the same name are created within the same scope, both exist at the same time but you can only use the *latest* one, the first one becomes *unreachable*. 

Ex:
```rust
fn main() {
  let x = 4
  println!("The value of x is {}", x); // this will ALWAYS print 4
  let x = 6
  println!("The value of x is {}", x); // this will ALWAYS print 6
}
```
Here we created two (2) variables named `x` whose variables are both called on to print their values when we execute our program. As the program executes, it maintains references to the correct variables. After we create a *new* variable named `x`, first `x` value still exists, however it cannot be reached by it's *previous* name. 


## Shadowing a variable

Shadowing occurs when a variable declared within a *nested scope* has the same name as a variable in an *outer scope*.

```rust
fn main() {
   let mut x: i32 = 1;
   println!("The value of x is: {}", x);
   x = 2;
   println!("The value of x is: {}", x);
   let y: bool = true;
   println!("The value of y is: {}", y);
   
   {
        let x: i32 = 42;
        println!("The inner answer to x is always {}", x,);
   }
   
   println!("The outer answer to x is still {}", x);
}
```

Output:
```bash
~/variables [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo run
   Compiling variables v0.1.0 (/Users/pope/Dev/MSFT_rust_series/MSFT_rust_series/vid8_declare_and_use_variables/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/variables`
The value of x is: 1
The value of x is: 2
The value of y is: true
The inner answer to x is always 42
The outer answer to x is still 2
```