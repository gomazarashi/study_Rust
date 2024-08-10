fn main() {
    impl Message {
        fn call(&self) {
            // method body would be defined here
            // メソッド本体はここに定義される
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
}
