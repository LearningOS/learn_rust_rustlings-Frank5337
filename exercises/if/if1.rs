// if1.rs

//如果是浮点数, 该如何比较大小? 或者是 字符串 该如何比较大小?

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `rustlings hint if1` for hints
    if a > b { a } else { b }
}

// Don't mind this for now :)
#[cfg(test)] //属性宏
mod tests { // 这是定义模块嘛?
    use super::*; // 为什么同一个文件还需要use ?

    #[test] // 这是什么?
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8)); // 断言?
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
