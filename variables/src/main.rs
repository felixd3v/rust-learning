afn main() {
    let mut m: i32 = 2;
    println!("The value of m is equal to: {}", m);
    m = 4;
    println!("The value of m is equal to: {}", m);

    let n: bool = true;
    println!("The value of n equals: {}", n);

    //shadowing value of n
    // I need to declare the variable "again"
    let n: bool = false;
    println!("The value of n equals: {}", n);

}
