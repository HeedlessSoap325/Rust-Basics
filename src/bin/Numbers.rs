//////////////////////////////////////////////////////////////////////////////////////
/// 
/// Integer Types:
/// | Size    | Signed | Unsigned |
/// | 8-bit   | i8     | u8       |
/// | 16-bit  | i16    | u16      |
/// | 32-bit  | i32    | u32      |
/// | 64-bit  | i64    | u64      |
/// | 128-bit | i128   | u128     |
/// | arch    | isize  | usize    |
/// 
/// arch => Depends on the Achitecture of the System, so either 32-bit or 64-bit.
///         -> usize gives you the guarantee to be always big enough to hold
///            any pointer or any offset in a data structure.
/// 
/// Float Types:
/// | Size   | "Signed" |
/// | 32-bit | f32      |
/// | 64-bit | f64      |
/// 
/// => Stored using IEEE-754
///  
//////////////////////////////////////////////////////////////////////////////////////

fn main(){
    assigndifferenttype();
    conversion();
    comparetypes();
    max();
    datatypesaddition();
    numbersystems();
    floatingpoint();
    floatingpointprecision1();
    floatingpointprecision2();
    range();
    rangewithstd();
    computations();
}

fn assigndifferenttype(){
    let x: i32 = 5;

    //let mut y: u32 = 5; //This is WRONG

    let mut _y = 5; //Type is defaut to 132

    _y = x; //This will Throm an ERROR, because you cant assign Variables with different Types

    let _z = 10; //Type of z: i32

    println!("Success!");
}

fn conversion(){
    let _v: u16 = 38_u8 as u16; //This converts 38 of form u8 to u16 using the "as" Keyword

    println!("Success!");
}

fn comparetypes(){
    //let x: i32 = 5; //This is WRONG, i32 != u32

    let x: u32 = 5;

    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String{
    format!("{}", std::any::type_name::<T>())
}

fn max(){
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

fn datatypesaddition(){
    //let v1 = 251_u8 + 8; //This WONT WORk, because u8 can only hold values up to 255

    let v1 = 251_u16 + 8;

    //let v2 = i8::checked_add(251, 8).unwrap(); //This also wont work, same reason as above

    let v2 = i16::checked_add(251, 8).unwrap();

    println!("{},{}", v1, v2);
}

fn numbersystems(){
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    
    //assert!(v == 1579); //This is WRONG v = 1597

    assert!(v == 1597);
    println!("Success!");
}

fn floatingpoint(){
    let x = 1_000.000_1; //f64
    let y: f32 = 0.12; //f32
    let z = 0.01_f64; //f64

    assert_eq!(type_of(&x), "f64".to_string());
    assert_eq!(type_of(&y), "f32".to_string());
    assert_eq!(type_of(&z), "f64".to_string());

    println!("Success!")
}

fn floatingpointprecision1(){
    //assert!(0.1 + 0.2 == 0.3); //This WONT WORK, because 0.1 + 0.2 = 0.30000000000000004

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); //because f32 is less precise

    println!("Success!");
}

fn floatingpointprecision2(){
    //assert!(0.1 + 0.2 == 0.3); //This WONT WORK, because 0.1 + 0.2 = 0.30000000000000004

    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32); //because f32 is less precise

    println!("Success!");
}

fn range(){
    let mut sum: i32 = 0;
    for i in -3..2{ //-3..2 is the Range of the for-loop, but 2 is EXCLUDED
        sum += i;
    }

    //assert!(sum == -3); //This is WRONG, because 2 is EXCLUDED

    assert!(sum == -5);

    for c in 'a'..='z'{ //Here, "z" in INCUDED, because of the "="
        println!("{}", c as u8); //a is 97, b is 98 ... z is 122 (ASCII)
    }
}

use std::ops::{Range, RangeInclusive};
fn rangewithstd(){
    assert_eq!((1..5), Range{start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

fn computations(){
    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5); //Moving the bit 1 five tims to the left
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //Moving 0x80 as binary 2 times to the right
}