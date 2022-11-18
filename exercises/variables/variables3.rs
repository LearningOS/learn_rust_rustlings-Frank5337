// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// mut 可变,  默认是不可变的
// 不可变有什么好处?  防止意外修改的好处

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

//继承式可变
// fn main() {
//     let mut x = 3;
//     println!("Number {}", x);
//     x = 5; // don't change this line
//     println!("Number {}", x);
// }

//内部可变性

