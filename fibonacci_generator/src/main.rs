use std::io;

fn main() {
    let mut input = String::new();

    println!("フィボナッチ数列のn番目の要素を生成します。nを入力してください:");

    io::stdin()
        .read_line(&mut input)
        .expect("行の読み込みに失敗しました");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("数値を正しく入力してください。");
            return;
        }
    };

    let result = fibonacci(n);
    println!("フィボナッチ数列の{}番目の要素は{}です。", n, result);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut current = 1;
    let mut next = 1;
    let mut tmp;

    for _ in 2..n+1 {
        tmp = current + next;
        current = next;
        next = tmp
    }

    return current;
}
