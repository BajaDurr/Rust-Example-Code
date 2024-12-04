fn main() {
    let mut s = String::from("Hello");

    // Mutable borrow: `s_ref` has the ability to modify `s`
    let s_ref = &mut s;

    s_ref.push_str(", world!"); // Modify the value through the mutable reference

    // After modification, we can print the modified string using `s_ref`
    println!("{}", s_ref); // Prints: Hello, world!

    // Now, `s` can be used again since `s_ref` is no longer in use
    println!("{}", s); // Prints: Hello, world!
}
