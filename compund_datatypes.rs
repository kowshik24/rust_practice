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
    
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]

    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices:&[&String] = &[ &"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slices: {:?}", book_slices);


    // Strings vs String Slices(&str)
    // Strings [growable, mutable, owned string type]

    let mut stone_cold: String = String::from("Hell, ");
    

    stone_cold.push_str("Yeah!");

    println!("Storne cold says: {:?}", stone_cold);

    // B - &str (String Slice)

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice value: {}",slice);


    

}

// fn print(){
//     println!("SLICE: {}",slice);
// }