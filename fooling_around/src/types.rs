/*
Primitives

    Integer u8 - unsigned 8bit / i128 - signed 128 bit integer, or anything in between
    Floats: f32, f64
    Characters (char) - single character
    Tuples
*/

//While Rust is satically type language, the compiler can usually infer what we want to use. Usually.

pub fn run(){
    // Default is i32 for this one;
    let x = 1;

    //And this one is f64
    let y = 2.5;

    //Explicit type
    let z: i64 = 47474727378;

    //Find max size
    //std - standard library
    println!("Max i32; {}", std::i32::MAX);
    println!("Max i64; {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //From expressions
    let is_greater = 10 < 5;

    //Char - chars are single unicode characers in ''
    let a1 = 'a';

    //Chars can also use emoji, because why not!
    let parting_face = '\u{1F973}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, parting_face));
}