use std::io; //io(入出力)ライブラリ

fn main() {
    loop {
        let mut degree_type = String::new();
        let mut degree = String::new();

        println!("変換前の温度の形式を入力して下さい(C/F) (終了するには 'Q' を入力):");
        io::stdin()
            .read_line(&mut degree_type)
            .expect("行の読み込みに失敗しました");
        let degree_type = degree_type.trim();

        // ユーザーが 'Q' を入力した場合、ループを終了
        if degree_type.eq_ignore_ascii_case("Q") {
            break;
        }

        println!("変換前の温度の数値を入力して下さい:");
        io::stdin()
            .read_line(&mut degree)
            .expect("行の読み込みに失敗しました");

        //入力された温度をString型からf64型へ
        let degree: f64 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数値を正しく入力して下さい。");
                continue;
            }
        };

        let result = temperature_converter(degree_type, degree);
        println!("変換結果:{}", result);
    }
}

fn temperature_converter(deree_type: &str, degree: f64) -> f64 {
    if deree_type == "C" {
        return degree * (9.0 / 5.0) + 32.0;
    } else if deree_type == "F" {
        return (degree - 32.0) * (5.0 / 9.0);
    } else {
        println!("書式エラー");
        return 0.0; // or any other default value
    }
}
