fn main(){
    initialization();
    mutable();
    scope();
    shadowing();
    shadowing2();
    unusedvars1();
    unusedvars2();
    destructuring();
    destructuringassignments();
}

fn initialization(){

    //let x: i32; //This variable is uninitalized, but used => Error

    let x: i32 = 5;
    let y: i32; //This variable is uninitialaized, but not used => Warning

    assert_eq!(x, 5);
    println!("Success!");
}

fn mutable(){
    let mut x: i32 = 5; //Allows Variable x to be assigned a new value

    //let y: i32 = 5; //DOES NOT allow Variable y to be assigned a new value

    let mut y: i32 = 5;

    x += 2; //This is fine
    y +=2; //This is WRONG, because y is not mutable

    assert_eq!(x, 7);
    assert_eq!(y, 7);
    println!("Success!");
}

fn scope(){ //Main Scope
    let x: i32 = 10;
    let y: i32 = 5;

    {//Secondary Scope
        //let y: i32 = 5; //This Variable is only valid inside the Secondary Scope!
        println!("The value of x is {} and value of y is {}", x, y);
    }//End of Secondary Scope

    println!("The value of x is {} and value of y is {}", x, y);
} //End of Main Scope

fn shadowing(){
    let x: i32 = 5;

    {//Seconary Scope
        let x = 12; //This Variable Decleration overshadows the previosly definde Variable x
        
        //assert_eq!(x, 5); //PANIC
        
        assert_eq!(x, 12);
    }//End of Secondary Scope

    //assert_eq!(x, 12); //PANIC

    assert_eq!(x, 5);

    let x = 42;// New declaration of Variable x, THIS IS FINE
    println!("{}", x);
}

fn shadowing2(){
    let mut x: i32 = 1;
    x = 7;

    //Shadowing and re-binding

    //let x = x; //This is WRONG, because x must be mutable for x += 3 to work.

    let mut x = x; //useless
    x += 3;


    let y: i32 = 4;

    //Shadowing
    let y: &str = "I am a String!";

    println!("{}", y);
    println!("Success!")
}

fn unusedvars1(){
    //let x = 1; //This would print a Warning

    let _x = 1;
}

#[allow(unused_variables)] //Prevents warning
fn unusedvars2(){
    let x = 1; //This would print a Warning
}

fn destructuring(){
    //let (x, y) = (1, 2); //This WONT WORK, because the Tuples Variable "x" is immutable

    let (mut x, y) = (1, 2); //let mut x = 1; let y = 2;

    x += 2;
    
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn destructuringassignments(){
    let (x, y); //let x; let y;

    (x, ..) = (3, 4); // x = 3
    [.., y] = [1, 2]; // y = 2

    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}