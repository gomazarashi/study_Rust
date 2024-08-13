fn main() {
    let value = Some(42);

    // if let 記法を用いて、valueがSomeの場合のみ処理を行う
    if let Some(number) = value {
        println!("The number is: {}", number);
    } else {
        println!("No value found.");
    }
}
