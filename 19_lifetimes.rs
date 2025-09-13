// 19_lifetimes.rs
// 核心内容：[Rust核心] 讲解生命周期，解决悬垂引用问题，确保所有引用都有效。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 生命周期的主要目标是防止悬垂引用（dangling references）。
 * 悬垂引用是指一个引用指向了已经被释放的内存。
 *
 * 1. 借用检查器 (Borrow Checker)
 *    - Rust 编译器有一个“借用检查器”，它会比较作用域来确定所有的借用是否有效。
 *    - 在大多数情况下，借用检查器可以隐式地、自动地完成这项工作。
 *    - 但在某些情况下，当引用的生命周期关系不明确时（例如，一个函数返回一个引用），
 *      借用检查器就无法推断出引用的有效性，此时就需要我们手动标注生命周期。
 *
 * 2. 生命周期注解语法
 *    - 生命周期注解并不会改变任何引用的存活时间。
 *    - 相反，它们描述了多个引用生命周期之间的关系，以便借用检查器可以进行分析。
 *    - 生命周期注解以撇号 `'` 开头，后面跟着一个小写字母，通常是 `'a`, `'b` 等。
 *    - `&i32`        // 一个普通的引用
 *    - `&'a i32`     // 一个带有显式生命周期 'a 的引用
 *    - `&'a mut i32` // 一个带有显式生命周期的可变引用
 *
 * 3. 在函数签名中使用生命周期
 *    - 这是生命周期注解最常见的场景。
 *    - 我们在函数名后的尖括号 `<>` 中声明生命周期参数，就像泛型一样。
 *    - 然后在函数参数和返回值中使用这些生命周期。
 *    - `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }`
 *      - `'a` 是我们声明的生命周期参数。
 *      - `x: &'a str`: 表示 `x` 是一个生命周期为 `'a` 的字符串切片引用。
 *      - `y: &'a str`: 表示 `y` 也是一个生命周期为 `'a` 的字符串切片引用。
 *      - `-> &'a str`: 表示函数返回的字符串切片引用的生命周期也是 `'a`。
 *
 *    - 这个签名告诉编译器：返回的引用的生命周期，与传入的两个引用 `x` 和 `y` 中
 *      **较短**的那个生命周期相关联。
 *    - 这意味着返回的引用，其存活时间不会超过 `x` 和 `y` 中任何一个的存活时间，
 *      从而保证了它永远不会变成悬垂引用。
 *
 * 4. 生命周期省略规则 (Lifetime Elision Rules)
 *    - 实际上，你之前写的很多函数都有生命周期，但你不需要手动标注它们。
 *    - 这是因为 Rust 编译器内置了一套“生命周期省略规则”，如果你的代码符合这些规则，
 *      编译器就可以自动推断生命周期，你就不需要写 `'a` 了。
 *    - 规则1: 每个作为输入的引用，都会得到一个自己独立的生命周期参数。
 *    - 规则2: 如果只有一个输入生命周期参数，那么这个生命周期会被赋给所有输出的生命周期参数。
 *    - 规则3: 如果有多个输入生命周期参数，但其中一个是 `&self` 或 `&mut self`（即这是一个方法），
 *      那么 `self` 的生命周期会被赋给所有输出的生命周期参数。
 *
 * 5. 在结构体定义中使用生命周期
 *    - 当结构体的一个字段是引用类型时，你必须在结构体定义中添加生命周期注解。
 *    - `struct ImportantExcerpt<'a> { part: &'a str, }`
 *    - 这表示 `ImportantExcerpt` 的实例不能比它字段 `part` 所引用的字符串切片活得更长。
 *
 * 6. 静态生命周期 (`'static`)
 *    - `'static` 是一个特殊的生命周期，它表示引用可以在整个程序的持续时间内都有效。
 *    - 所有的字符串字面量都拥有 `'static` 生命周期，因为它们直接存储在程序的二进制文件中。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 3. 在函数签名中使用生命周期
// 这个函数接收两个字符串切片，并返回较长的那一个
// `'a` 告诉编译器，返回的引用至少和 x、y 活得一样长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 5. 在结构体定义中使用生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法中的生命周期
impl<'a> ImportantExcerpt<'a> {
    // 根据省略规则3，`&self` 的生命周期被赋给了返回值，所以不需要显式标注
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz"; // string2 是 &'static str

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 演示生命周期的作用
    let string3 = String::from("long string is long");
    let result2;
    {
        let string4 = String::from("xyz");
        // result2 的生命周期被约束为 string4 和 string3 中较短的那个，即 string4
        result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string inside inner scope is {}", result2);
    } // string4 在这里被 drop
    // 如果取消下面这行注释，会编译失败，因为 result2 的生命周期已经结束
    // println!("The longest string outside inner scope is {}", result2);

    // 使用带生命周期的结构体
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The important excerpt is: {}", i.part);

    // 6. 静态生命周期
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // 练习1：
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "Today's longest string");
    println!("The longest string is {}", result);

    // 练习2：
    // 创建一个拥有自己数据所有权的 String
    let my_string = String::from("hello world of rust");
    // 创建 Text 实例，它的 content 字段借用了 my_string 的数据
    let text_instance = Text {
        content: my_string.as_str(),
    };
    // 调用方法
    let first = text_instance.first_word();
    // 打印结果
    println!("The original content is: '{}'", text_instance.content);
    println!("The first word is: '{}'", first);
}
// 练习1：
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 练习2：
// 1. 定义结构体 Text，它包含一个生命周期为 'a 的字符串切片引用
struct Text<'a> {
    content: &'a str,
}
// 2. 为 Text 实现方法
impl<'a> Text<'a> {
    // 方法返回一个字符串切片，它的生命周期也是 'a
    // &self 的生命周期是独立的，但因为返回值来源于 self.content，
    // 所以返回值的生命周期必须是 'a。
    fn first_word(&self) -> &'a str {
        // 将内容按空格分割
        let bytes = self.content.as_bytes();
        // 遍历字节，找到第一个空格
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // 返回从开头到第一个空格的切片
                return &self.content[0..i];
            }
        }
        // 如果没有空格，整个内容就是第一个单词
        self.content
    }
}
/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 修复悬垂引用:
 *    下面的 `longest_with_an_announcement` 函数有一个问题，它混合了不同的生命周期。
 *    请尝试理解编译器的错误信息，并思考如何修复它。
 *    提示：返回值的生命周期应该依赖于哪个输入参数？
 *
 *    use std::fmt::Display;
 *
 *    fn longest_with_an_announcement<'a, T>(
 *        x: &'a str,
 *        y: &'a str,
 *        ann: T,
 *    ) -> &'a str
 *    where
 *        T: Display,
 *    {
 *        println!("Announcement! {}", ann);
 *        if x.len() > y.len() {
 *            x
 *        } else {
 *            y
 *        }
 *    }
 *
 * 2. 结构体和引用:
 *    创建一个结构体 `Text<'a>`，它有一个字段 `content`，类型是 `&'a str`。
 *    为它实现一个方法 `first_word(&self) -> &'a str`，该方法返回 `content` 中的第一个单词。
 *    在 `main` 函数中创建一个 `Text` 实例并调用 `first_word` 方法。
 *    思考一下，为什么 `first_word` 的返回值生命周期必须是 `'a`？
 *
 */