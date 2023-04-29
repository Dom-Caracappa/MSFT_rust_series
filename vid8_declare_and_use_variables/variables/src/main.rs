fn main() {
   let x: i32 = 1;
   println!("The value of x is: {}", x);
   let x = 2;
   println!("The value of x is: {}", x);
   let y: bool = true;
   println!("The value of y is: {}", y);
   let y = false;
   println!("The value of y is: {}", y);
   
   {
        let x: i32 = 42;
        println!("The inner answer to x is always {}", x,);
   }
   println!("The outer answer to x is still {}", x);


   const STRING: &str = "hello";
   println!("The value of the string constant is: {}", STRING);
}
