//OWNERSHIP

fn main() {
    let s1 = String::from("Hello"); // s1 owns the String
    let s2 = s1; // Ownership of the String is moved to s2

    // println!("{}", s1); // Error: s1 no longer owns the value
    println!("{}", s2); // Works fine, since s2 owns the String





//BORROWING
    let s3 = String::from("Hello");
    let s4 = &s3; // Immutable borrow

    println!("{}", s3); // This works because s2 only borrowed the value
    println!("{}", s4); // We can use s2 to read the value


    //MUTABLE BORROWING
    let mut s6 = String::from("Hello");
    let s7 = &mut s6; // Mutable borrow

    s7.push_str(", world!");
    println!("{}", s6); // Prints the modified value

// println!("{}", s1); // Error: can't use s1 while it is borrowed mutably


/*
Rule of borrowing: You can have either multiple immutable references (&T) 
or exactly one mutable reference (&mut T), but not both at the same time. 
This prevents data races and ensures safe concurrent access. 
*/


//LIFETIMES
fn longest<'a>(s9: &'a str, s10: &'a str) -> &'a str {
    if s9.len() > s10.len() {
        s9
    } else {
        s10
    }
}
let str11 = String::from("Hello");
    let str12 = String::from("World");

    let result = longest(&str11, &str12);
    println!("The longest string is {}", result);


    /* Here, 'a is a lifetime parameter, 
    ensuring that the references s1 and s2 must live at least as long as the returned reference. 
    */

}