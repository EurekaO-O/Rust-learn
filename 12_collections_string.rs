// 12_collections_string.rs
// 核心内容：讲解String类型，它与&str字符串切片的区别，以及字符串的常用操作。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * Rust 对字符串的处理主要围绕两种类型：`String` 和 `&str` (字符串切片)。
 *
 * 1. `&str` - 字符串切片 (String Slice)
 *    - `&str` 是对存储在其他地方（通常是二进制文件或另一个 `String`）的 UTF-8 编码字符串数据的引用。
 *    - 字符串字面量（例如 `"hello"`）的类型就是 `&'static str`。
 *    - 它是不可变的，大小固定，是对一段字符串数据的“视图”或“借用”。
 *
 * 2. `String` - 可增长、可变的字符串
 *    - `String` 类型由标准库提供，是一个可增长、可变、有所有权的 UTF-8 编码字符串。
 *    - 它的数据存储在堆上，因此可以在运行时改变大小。
 *    - 当你需要修改字符串数据或需要拥有字符串的所有权时，就应该使用 `String`。
 *
 * 3. 创建 `String`
 *    - `String::new()`: 创建一个空的 `String`。
 *    - `to_string()` 方法: 在实现了 `Display` trait 的类型上调用，包括 `&str`。
 *    - `String::from()`: 从一个 `&str` 创建一个新的 `String`。
 *
 * 4. 更新 `String`
 *    - `push_str(&str)`: 将一个字符串切片附加到 `String` 的末尾。
 *    - `push(char)`: 将单个字符附加到 `String` 的末尾。
 *    - `+` 运算符 (使用 `add` trait): 可以将两个 `String` 连接起来，但这会获取第一个 `String` 的所有权。
 *      `let s3 = s1 + &s2;` // s1 在此之后被移动，不再有效。
 *    - `format!` 宏: 用于连接多个字符串，并且不会获取任何参数的所有权。这是连接字符串的推荐方式。
 *
 * 5. 内部表示：UTF-8 和索引
 *    - Rust 的 `String` 内部是基于 `Vec<u8>` 的封装，它保证了其内容始终是有效的 UTF-8 编码。
 *    - **重要**: 你不能使用整数索引来访问 `String` 中的字符！`let c = s[0];` 是无效的。
 *    - 为什么？因为 UTF-8 编码中，一个“字符”（用户看到的）可能由1到4个字节组成。
 *      例如，'é' 占2个字节，'中' 占3个字节。直接索引字节可能会得到一个无效的字符片段。
 *
 * 6. 遍历字符串
 *    - 为了正确处理 Unicode，Rust 提供了多种遍历字符串的方式。
 *    - `.chars()`: 遍历字符串中的 `char`。这是最常用的方式。
 *    - `.bytes()`: 遍历底层的 `u8` 字节。
 *
 * 7. 字符串切片 (Slicing)
 *    - 虽然不能用单个索引访问字符，但你可以获取 `String` 的一部分，即一个字符串切片 `&str`。
 *    - `let slice = &s[0..4];`
 *    - 切片操作必须发生在有效的 `char` 边界上。如果切片发生在多字节字符的中间，程序会 `panic`。
 *      因此，进行字符串切片时必须非常小心。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

fn main() {
    // 3. 创建 String
    let mut s = String::new();
    s.push_str("initial content");
    println!("s is: {}", s);

    let data = "some data";
    let s_from_data = data.to_string();
    println!("s_from_data: {}", s_from_data);

    let s_from_literal = String::from("hello world");
    println!("s_from_literal: {}", s_from_literal);

    // 4. 更新 String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str 接收 &str，不会获取所有权
    println!("s1 is now: {}, s2 is still: {}", s1, s2);

    s1.push('!');
    println!("After pushing a char: {}", s1);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    // s3 的所有权被移动，s4 的引用被使用
    let s5 = s3 + &s4;
    // println!("{}", s3); // 这会报错，s3 已被移动
    println!("Concatenated with +: {}", s5);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // format! 宏不会获取任何所有权，是连接字符串的最佳方式
    let tictactoe = format!("{}-{}-{}", tic, tac, toe);
    println!("Formatted string: {}", tictactoe);
    println!("Original strings are still valid: {}, {}, {}", tic, tac, toe);

    // 6. 遍历字符串
    let hello = "Здравствуйте"; // 俄语 "Hello"
    println!("\nIterating over '{}'", hello);

    println!("Chars:");
    for c in hello.chars() {
        print!("{} ", c);
    }
    println!();

    println!("Bytes:");
    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();

    // 7. 字符串切片
    // 这个字符串中的每个字符都占用2个字节
    let slice = &hello[0..4]; // 获取前两个字符 "Зд"
    println!("Slice [0..4] is: {}", slice);
    // let invalid_slice = &hello[0..1]; // 这会导致 panic!
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 反转字符串:
 *    编写一个函数，接收一个 `&str` 作为输入，返回一个拥有所有权的新 `String`，
 *    这个新 `String` 是输入字符串的反转版本。
 *    提示：`.chars().rev().collect::<String>()` 是一个简洁的方法。
 *
 * 2. 检查回文:
 *    编写一个函数，接收一个 `&str`，如果这个字符串是回文（正读和反读都一样，忽略大小写和空格），
 *    则返回 `true`，否则返回 `false`。
 *    例如, "A man, a plan, a canal: Panama" 应该返回 true。
 *
 */