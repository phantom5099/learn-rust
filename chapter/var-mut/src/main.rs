fn main() {
    let tup: (i32, char, f64) = (300, 'w', 3.14);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);

    let b: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("b = {:?}", b);

    let c = [1; 5]; //包含 5 个元素，这些元素的值最初都将被设置为 3
    println!("c = {:?}", c);
}
