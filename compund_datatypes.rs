// Chapter 2: Compound Data Types

// arrarys, tuples , slices and strings(slice string)

fn main(){

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array:  {:?}", numbers);

    // let mix = [1, 2, "Apple", 3.4]
    // println!("Mixed Array: {:?}", mix);

    let fruits: [&str; 5] = ["Apple", "Banana", "Orange", "Mango", "Grapes"];
    println!("Fruits: {:?}", fruits);

    println!("1st element of the arrary is : {:?}", fruits[0]);
    println!("2nd element of the arrary is : {:?}", fruits[1]);
    println!("3rd element of the arrary is : {:?}", fruits[2]);
    println!("4th element of the arrary is : {:?}", fruits[3]);
    println!("5th element of the arrary is : {:?}", fruits[4]);

    // Tuples

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);




    

}