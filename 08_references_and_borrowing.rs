// 08_references_and_borrowing.rs
// 核心内容：讲解引用（&）和借用（Borrowing）的概念，包括不可变引用和可变引用，以及悬垂引用问题。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 在上一课中，我们看到函数获取了值的所有权。如果我们希望函数使用一个值但不获取其所有权，该怎么办？
 * 这时就需要“借用”（Borrowing）这个概念。
 *
 * 1. 引用 (References)
 *    - 引用允许你引用（refer to）某个值，而无需获取它的所有权。
 *    - 我们使用 `&` 符号来创建一个引用。引用就像一个指针，它是一个地址，我们可以由此访问存储在该地址的数据。
 *    - 因为引用不拥有数据，所以当引用离开作用域时，它指向的数据不会被丢弃（drop）。
 *    - 我们将创建一个引用的行为称为“借用”（borrowing）。
 *
 * 2. 不可变引用 (Immutable References)
 *    - 默认情况下，引用是不可变的，就像变量一样。你不能通过一个不可变引用来修改它所指向的数据。
 *    - 语法: `let s2 = &s1;`
 *
 * 3. 可变引用 (Mutable References)
 *    - 如果你想通过引用来修改数据，你需要创建一个可变引用。
 *    - 语法: `let s2 = &mut s1;`
 *    - 这要求原始变量 `s1` 本身也必须是可变的（即 `let mut s1 = ...`）。
 *
 * 4. 借用规则 (The Rules of Borrowing)
 *    这是Rust保证内存安全的核心规则之一，必须牢记：
 *    在一个特定的作用域内，对于某一块数据，你只能拥有以下两种情况之一：
 *    a) 一个或多个不可变引用 (`&T`)。
 *    b) **有且仅有**一个可变引用 (`&mut T`)。
 *
 *    - 为什么有这个规则？
 *      这个规则在编译时就防止了“数据竞争”（data races）。数据竞争通常发生在：
 *      - 两个或多个指针同时访问同一数据。
 *      - 至少有一个指针在写数据。
 *      - 没有同步机制来协调对数据的访问。
 *      Rust通过这个借用规则，在编译阶段就彻底杜绝了这类问题的发生。
 *
 * 5. 悬垂引用 (Dangling References)
 *    - 在其他需要手动管理内存的语言中，很容易创建一个“悬垂指针”——一个指向已经被释放的内存的指针。
 *    - Rust的编译器通过“生命周期”（lifetimes，我们将在后面深入学习）来确保所有引用都是有效的。
 *    - 如果你尝试创建一个引用，而它指向的数据在引用本身还存在时就被销毁了，编译器会报错。
 *      这使得在安全的Rust代码中，悬垂引用是不可能存在的。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================
fn main() {
    // 1. 使用不可变引用来解决上一课的挑战
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 我们传递 s1 的引用，而不是所有权
    println!("The length of '{}' is {}.", s1, len); // s1 在这里仍然有效！

    // 2. 使用可变引用来修改字符串
    let mut s2 = String::from("hello"); // s2 必须是 mut
    change(&mut s2); // 传递一个可变引用
    println!("The modified string is: {}", s2);

    // 3. 借用规则演示
    let mut s3 = String::from("data");

    // a) 可以有多个不可变引用
    let r1 = &s3;
    let r2 = &s3;
    println!("Immutable borrows: {} and {}", r1, r2);
    // 在 r1 和 r2 的作用域结束后，我们才能创建可变引用

    // b) 只能有一个可变引用
    let r3 = &mut s3;
    println!("Mutable borrow: {}", r3);

    // 下面这行代码如果取消注释，会编译失败
    // let r4 = &mut s3; // error[E0499]: cannot borrow `s3` as mutable more than once at a time
    // println!("Second mutable borrow: {}", r4);

    // 同样，在存在可变引用的情况下，不能再创建不可变引用
    let mut s4 = String::from("race");
    let _r5 = &mut s4;
    // let r6 = &s4; // error[E0502]: cannot borrow `s4` as immutable because it is also borrowed as mutable
    // println!("{}, {}", r5, r6);

    // 4. 悬垂引用示例 (这段代码无法通过编译)
    // let reference_to_nothing = dangle();

    // 练习1：
    let mut my_string = String::from("hello");
    inspect(&my_string);
    let str1 = &mut my_string;
    println!("{}", str1);

    // 练习2：
    // 创建一个可变的 String
    let mut str2 = String::from("Hello");
    println!("Before calling the function: {}", str2);
    // 调用函数，传入一个可变引用
    let string_ref = add_suffix(&mut str2);
    // 函数执行后：
    // str2 本身已经被修改了。
    //println!("After modification, original string is: {}", str2);//报错,原因如下：
        //1.首先str2在可变借用后，将写入权限给了string_ref
        //2.因为rust规则：只有一个可变借用可以操作写入，所以str2暂时为不可以状态，因为下面还有继续使用string_ref。
    println!("After modification, the content is: {}", string_ref); // 使用 string_ref
    println!("Now we can use my_string again: {}", str2);//结果一致
}

// 这个函数接收一个 String 的引用，返回其长度
// `s` 是一个指向 String 的引用，它不拥有这个 String
fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` 离开作用域，但它不拥有所有权，所以什么都不会发生

// 这个函数接收一个可变的 String 引用，并修改它
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// 练习1：
fn inspect(s: &String) {
    println!("{}", s);
}

// 练习2：
fn add_suffix(s: &mut String) -> &String{
    s.push_str("!");
    s
}
// 这个函数尝试创建一个悬垂引用，但编译器会阻止我们
// fn dangle() -> &String { // dangle 返回一个 String 的引用
//     let s = String::from("dangle"); // s 是一个新的 String
//     &s // 我们返回 s 的引用
// } // s 在这里离开作用域，被 drop，内存被释放。它的引用就指向了无效的内存！
// Rust 编译器会报错：`this function's return type contains a borrowed value, but there is no value for it to be borrowed from`

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 修复借用错误:
 *    下面的代码违反了借用规则。请在不改变函数 `inspect` 签名的情况下，调整 `main` 函数中的代码来修复它。
 *
 *    fn inspect(s: &String) {
 *        println!("{}", s);
 *    }
 *
 *    fn main_challenge() {
 *        let mut my_string = String::from("hello");
 *        let str1 = &mut my_string;
 *        inspect(&my_string); // 错误发生在这里
 *        println!("{}", str1);
 *    }
 *
 * 2. 修改并返回引用:
 *    编写一个函数 `add_suffix`，它接收一个可变的 `String` 引用，给它添加后缀 "!",
 *    然后返回这个 `String` 的不可变引用。
 *    思考一下，为什么这个函数签名 `fn add_suffix(s: &mut String) -> &String` 是可行的？
 *    (提示：与生命周期有关，输入引用的生命周期会被推断为返回引用的生命周期。)
 *
 */