fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let ten = Some(10);
    let eleven = plus_one(ten);

    println!("ten: {:?}", ten); // ten: Some(10)
    println!("eleven: {:?}", eleven); // eleven: Some(11)

    let no_value: Option<i32> = None;
    let result = plus_one(no_value);

    println!("no_value: {:?}", no_value); // no_value: None
    println!("result: {:?}", result); // result: None

    // Some値に対して複数回plus_oneを呼び出す
    let mut number = Some(2);
    for _ in 0..3 {
        number = plus_one(number);
        println!("number: {:?}", number);
    }
    // 出力: number: Some(3), number: Some(4), number: Some(5)
}
