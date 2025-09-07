// 07_ownership.rs
// 核心内容：[Rust核心] 详细解释所有权三大法则：所有者、移动（Move）和克隆（Clone）。通过示例展示栈和堆的数据。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 所有权是Rust用以管理内存的核心系统。其他语言要么有垃圾回收器（GC），要么需要程序员手动管理内存。
 * Rust通过所有权系统在编译时分析内存的分配和释放，从而实现了内存安全，且没有GC的运行时开销。
 *
 * 1. 栈 (Stack) vs 堆 (Heap)
 *    - 栈：存储大小已知且固定的数据。操作速度快（后进先出）。函数调用、局部变量（如i32, bool, f64, char, 元组和数组等固定大小类型）都存储在栈上。
 *    - 堆：存储在编译时大小未知或可能变化的数据。操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个指向该位置的指针。这个过程称为“在堆上分配内存”。操作相对较慢。`String`, `Vec<T>`, `HashMap` 等动态大小的类型的数据存储在堆上。
 *
 * 2. 所有权三大法则
 *    a) 每个值在Rust中都有一个变量，称为其“所有者”（Owner）。
 *    b) 在任何时候，一个值只能有一个所有者。
 *    c) 当所有者离开作用域（scope）时，该值将被“丢弃”（dropped），其占用的内存会被自动释放。
 *
 * 3. 变量赋值与所有权转移 (Move)
 *    - 对于存储在栈上的简单类型（如i32），赋值操作会复制一份数据。这类类型被称为实现了 `Copy` trait。
 *      `let x = 5;`
 *      `let y = x; // x的值被复制给y，x和y都是有效的`
 *
 *    - 对于存储在堆上的数据（如String），情况则完全不同。一个 `String` 类型由三部分组成：
 *      一个指向堆上实际数据的指针、一个长度（length）和一个容量（capacity）。这三部分存储在栈上。
 *      `let s1 = String::from("hello");`
 *      `let s2 = s1; // 这里不是复制堆上的数据！`
 *
 *      在 `let s2 = s1;` 之后，Rust认为 `s1` 不再有效，以防止“二次释放”（double free）错误。
 *      这被称为所有权的“移动”（Move）。`s1` 的所有权被移动给了 `s2`。
 *      此时如果再使用 `s1`，编译器会报错。
 *
 * 4. 克隆 (Clone) - 显式复制堆数据
 *    - 如果我们确实需要深度复制堆上的数据（而不仅仅是移动所有权），我们可以调用 `clone()` 方法。
 *      `let s1 = String::from("hello");`
 *      `let s2 = s1.clone(); // 堆上的 "hello" 数据被复制了一份`
 *      `// 现在 s1 和 s2 都有效，分别指向不同的内存地址`
 *
 * 5. Copy Trait - 栈上数据的特殊情况
 *    - 对于完全存储在栈上的类型，复制它们的成本很低。因此，这些类型默认实现了 `Copy` trait。
 *    - 实现了 `Copy` trait 的类型在赋值后，旧的变量仍然可用。
 *    - 常见的 `Copy` 类型包括：
 *      - 所有整型 (u32, i32, etc.)
 *      - 布尔型 (bool)
 *      - 所有浮点型 (f64, f32)
 *      - 字符型 (char)
 *      - 只包含 `Copy` 类型的元组 `(i32, i32)`
 *
 * 6. 函数与所有权
 *    - 将变量传递给函数，与给变量赋值的语义相同：
 *      - 对于 `Copy` 类型，是按位复制。
 *      - 对于非 `Copy` 类型（如String），是所有权移动。
 *    - 函数返回值也会转移所有权。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

fn main() {
    // 1. 作用域和 Drop
    {
        let s = String::from("scope"); // s 从此刻开始有效
        println!("Inside inner scope: {}", s);
    } // 这个作用域结束，s 不再有效，内存被释放

    // 2. String 类型和 Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被移动到 s2

    // 下面这行代码如果取消注释，会编译失败，因为 s1 的所有权已经移动
    // println!("s1 is: {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("s2 is: {}", s2); // s2 现在是 "hello" 的所有者

    // 3. Clone - 深度复制
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3 的数据被克隆给 s4
    println!("s3 = {}, s4 = {}", s3, s4); // s3 和 s4 都有效

    // 4. Copy Trait - 栈上数据
    let x = 5; // i32 实现了 Copy trait
    let y = x; // x 的值被复制给 y
    println!("x = {}, y = {}", x, y); // x 和 y 都有效，因为 i32 是 Copy 类型

    // 5. 函数与所有权转移
    let s = String::from("ownership"); // s 进入作用域
    takes_ownership(s);             // s 的所有权被移动到函数 takes_ownership 中
                                    // 从这里开始 s 不再有效

    let z = 10;                     // z 进入作用域
    makes_copy(z);                  // z 的值被复制到函数 makes_copy 中
                                    // z 在这里仍然有效

    let s_back = gives_ownership(); // 函数返回一个 String，所有权移动给 s_back
    println!("Got ownership back: {}", s_back);

    let s_given = String::from("take and give back");
    let s_received = takes_and_gives_back(s_given); // 所有权移动到函数，然后又移回来
    println!("Received back: {}", s_received);
} // main 作用域结束，所有仍然有效的变量（s2, s3, s4, x, y, z, s_back, s_received）被 drop

fn takes_ownership(some_string: String) { // some_string 获得所有权
    println!("Inside takes_ownership: {}", some_string);
} // some_string 离开作用域，被 drop，内存释放

fn makes_copy(some_integer: i32) { // some_integer 获得一份值的拷贝
    println!("Inside makes_copy: {}", some_integer);
} // some_integer 离开作用域，无事发生

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回 String，并将所有权移出
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 接收 String 的所有权，然后又将其返回
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 预测所有权:
 *    分析下面的代码片段，在不运行的情况下，指出哪一行会发生编译错误，并解释为什么。
 *
 *    fn challenge_one() {
 *        let a = String::from("apple");
 *        let b = a;
 *        let c = b.clone();
 *        println!("a = {}, b = {}, c = {}", a, b, c);
 *    }
 *
 * 2. 计算字符串长度:
 *    编写一个名为 `calculate_length` 的函数，它接收一个 `String`，并返回它的长度（一个 `usize`）。
 *    但是，这个函数必须将 `String` 的所有权返回给调用者，同时返回长度。
 *    提示：函数可以返回一个元组 `(String, usize)`。
 *    在 `main` 函数中调用它，并确保在调用后你仍然可以使用原来的字符串变量。
 *    (这个问题预示了下一课“引用和借用”的重要性！)
 *
 */