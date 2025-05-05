// Chapter 1: Primitive Data Types

fn main(){

    
    // println!("Hello World! in Rust");

    let x: i32 = 10;
    let y: i32 = -30;

    println!("The number x is : {}", x);
    println!("The number y is : {}", y);

    let pie: f32 = 3.1416;
    println!("The value of pie is : {}", pie);


    //let is_cool: bool = true; // this is immutable variable
    let mut is_cool: bool = true; // this is mutable variable
    let temperature: i32 = 44;
    if temperature > 30{
        is_cool = false;
    }
    println!("Is it cool?: {}", is_cool);

    // Now define with unsigned data types

    let a: u32 = 100;
    println!("The value of a is: {}", a);

    // Charater data types - char

    let c: char = 'a';
    println!("The first letter of the alphabet is: {}", c)



}