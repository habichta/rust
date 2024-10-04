fn main() {
    let s = String::from("sdfsa");
    another_functions(5, &s);
}

fn another_functions(x: u32, value: &String) {
    println!("Hello {x} {value}");

    let y = {
        let x = 4;
        x + 1
    };

    println!("{y}");

    let y2 = double(y);

    println!("{y2}")
}

fn double(x: i32) -> i32 {
    x * 2
}
