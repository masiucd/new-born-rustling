
fn double(x:u8) -> u8 {
    x *2
}


fn main()  {
    let x = 45;
    println!("hello,{}",x);
    println!("hello,{}",x);

    let res = double(x);
    println!("{}", res);

    let y = x;

    let res2 = double(y);
    println!("{}", res2);
}