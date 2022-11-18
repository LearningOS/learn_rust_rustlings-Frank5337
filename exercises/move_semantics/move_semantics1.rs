// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// 思考: 栈 vs 堆?
// 栈 -> 编译器识别类型的大小, 默认分配的空间
// 堆 -> 运行时动态分配

// 思考: 为什么需要move语义?  为了内存安全,内存管理

fn main() {
    let vec0 = Vec::new(); //动态增长 -> 堆
    
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22); // Vec::push(&mut vec, 22)
    vec.push(44);
    vec.push(66);

    vec
}
