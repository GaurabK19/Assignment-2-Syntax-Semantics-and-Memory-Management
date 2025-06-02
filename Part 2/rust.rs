fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1);          
    println!("{}", s2);    // s2 owns the data now

    // Borrowing example
    let length = calculate_length(&s2); // Borrow s2
    println!("Length: {}", length);
} 

fn calculate_length(s: &String) -> usize {
    s.len() // Borrowed reference, does not take ownership
}
