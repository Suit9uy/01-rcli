fn main() {
    println!("Hello, rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); // ✅ 基本測試，保證至少有一個 test 可跑
    }
}
