// String type is allocated on heap and the size need not be known during compile time. 

// The variable is actually moved insted of shallow or deep copy as in other languages. 

// Use the clone method for deep copy

// If the type implements a copy trait, then the old variable is still usable after assignment. 

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello there");
    let s2 = s1;
    println!("{}", s2);

    let s1 = String::from("Hello there");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Passing by reference without transferring ownership
    // Having references is called borrowing

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable references, only one mutable reference is possible at a time in the same scope.
    // Mutable and immutable references also conflict.
    // The references scope start from where it is introduced and continues through the last time
    // the reference is used.
    let mut s = String::from("hello");
    // This will fail
    // let r1 = &s;
    // let r2 = &s;

}

fn calculate_length(s: &String) -> usize {
    s.len()
}
