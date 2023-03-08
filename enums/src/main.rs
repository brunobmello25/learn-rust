#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn main() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
