// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// reborrowing ?

fn main() {
    let mut x = 100; //i32 4字节 32 安全的在栈上存储的类型, 简单值类型
    let y = &mut x;
    //借用检查器, 只检查 借用/引用 &mut x , 为了防止悬垂指针
    *y += 100;
    let z = &mut x;

    *z += 1000;
    assert_eq!(x, 1200);

    //为什么x 不等于 1200?
    // x 是基础类型 = 发生的是copy行为
    // x 实现了 Copy trait (按位复制 按比特位复制)
    // 为什么有了move 还需要copy?
    // let mut x = 100;
    // let mut y = x; // x 被copy给y了
    // let mut z = x; // z 又copy给z
    // x = y += 100;
    // z += 1000;
    // assert_eq!(x, 1200);

}
 