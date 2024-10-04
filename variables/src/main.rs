const HELLO: u32 = 2;

fn main() {
    let x: u32 = 5;
    let y: u32 = 6;
    let mut z = 8;

    println!("{}", z);
    z = 9;
    println!("{}", x + y);

    println!("{}", z + HELLO);

    let f = 5;
    println!("{}", f);
    let f = 6;
    println!("{}", f);
    {
        let f = 7;
        println!("{}", f);
    }

    println!("{}", f);

    let t = (1, 'b', 3.5);

    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);

    let a = [1, 2, 3, 4, 5];

    for i in 0..a.len() {
        println!("{}", a[i]);
    }

    let a = [3; 5];

    for i in 0..a.len() {
        println!("{}", a[i]);
    }

    let c = ['f', 's'];

    let b = (a, a, c);

    for i in 0..b.0.len() {
        println!("{}", b.0[i]);
    }
}
