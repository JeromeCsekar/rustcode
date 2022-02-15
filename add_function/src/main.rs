fn add (a: i32, b: i32) -> i32
{
    a+b
}

fn display(result: i32)
{
    println!("{:?}", result);
}

fn main() 
{
    let result = add(13,15);
    println!("Addition of Two  numbers:");
    display(result);
}
