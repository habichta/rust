fn main() {
    let mut s = String::from("Hello");
    let l = calculate_length(&s);
    change_s(&mut s);
    println!("{}, {}", l, s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_s(s: &mut String) {
    s.push_str("dsf");
}
