// 06_flow_control.rs
// 核心内容：涵盖if-else表达式、多种循环（loop, while, for）的用法。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 流程控制结构允许你根据条件来决定是否执行某些代码，或者多次执行某些代码。
 *
 * 1. if-else 表达式
 *    - `if` 表达式允许你根据一个条件来分支代码。
 *    - 条件必须是 `bool` 类型。Rust不会像某些语言那样自动将非布尔类型转换为布尔类型。
 *    - `if` 后面的代码块与一个条件关联，`else` 后的代码块则在条件为 `false` 时执行。
 *    - `else if` 可以用来处理多个互斥的条件。
 *    - 因为 `if` 是一个表达式，所以你可以用它来给 `let` 语句赋值。在这种情况下，
 *      `if` 和 `else` 块的返回值类型必须相同。
 *
 * 2. 循环 (Loops)
 *    Rust提供了三种循环结构：`loop`, `while`, 和 `for`。
 *
 *    a) `loop` - 无限循环
 *       - `loop` 关键字会创建一个无限循环，它会一直重复执行，直到你显式地告诉它停止。
 *       - 通常使用 `break` 关键字来退出循环。
 *       - `loop` 循环也可以是一个表达式，可以从 `break` 语句返回值。
 *
 *    b) `while` - 条件循环
 *       - `while` 循环会在每次循环开始时评估一个布尔条件。
 *       - 只要条件为 `true`，循环体就会一直执行。
 *       - 当条件变为 `false` 时，循环停止。
 *
 *    c) `for` - 集合遍历循环
 *       - `for` 循环用于遍历一个集合（比如数组或范围）中的每个元素。
 *       - 这是Rust中最常用、最安全的循环结构，因为它避免了手动管理索引可能导致的
 *         “差一错误”（off-by-one errors）和数组越界访问。
 *       - 我们可以使用 `(1..4)` 这样的语法来创建一个范围（Range），它不包含上界（即1, 2, 3）。
 *       - 使用 `.rev()` 方法可以反转一个范围。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

fn main() {
    // 1. if-else 表达式
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 在 let 语句中使用 if 表达式
    let condition = true;
    let value = if condition { 5 } else { 6 }; // if 和 else 块的返回值类型必须是 i32
    println!("The value from the if expression is: {}", value);

    // 2. loop 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 退出循环，并从 break 返回值 20
        }
    };
    println!("The result from the loop is: {}", result);

    // 3. while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 4. for 循环
    let a = [1,2,3,4]; // **修正点**: 初始化数组

    // 使用 for 遍历数组元素，更安全简洁
    for element in a {
        println!("the value is: {}", element);
    }

    // 使用 for 和范围 (Range)
    // `(1..4)` 生成 1, 2, 3
    // `.rev()` 将其反转为 3, 2, 1
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF AGAIN!!!");

    // 练习1：
    fibonacci_sequence(10);

    // 练习2：
    print_christmas_lyrics();
}
// 练习1：
fn fibonacci_sequence(n: u32){

    if n <= 0{
        println!("请输入一个大于 0 的数");
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 0..n{
        print!("{} ",a);
        let next = a + b;
        a = b;
        b = next;
    }
    println!()
}

// 练习2：
fn print_christmas_lyrics() {
    // 礼物数组，索引 0 对应第一天，索引 1 对应第二天，以此类推
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    // 天数数组，用于打印 "first", "second" 等
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];
    println!("--- The Twelve Days of Christmas ---");
    // 外层循环：遍历每一天 (从 0 到 11 对应第一到第十二天)
    for day_index in 0..12 {
        println!("\n[Verse {}]", day_index + 1);
        println!("On the {} day of Christmas,", days[day_index]);
        println!("My true love sent to me");
        // 内层循环：倒序打印从当天到第一天的所有礼物
        // (day_index..=0).rev() 是错误的，应该是 (0..=day_index).rev()
        for gift_index in (0..=day_index).rev() {
            // 如果是第一天 (day_index > 0) 并且是最后一个礼物 (gift_index == 0)，
            // 在礼物前加上 "And"
            if day_index > 0 && gift_index == 0 {
                print!("And ");
            }
            
            println!("{}", gifts[gift_index]);
        }
    }
}
/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 斐波那契数列:
 *    编写一个程序，使用循环（`loop`, `while`, 或 `for` 都可以）来生成并打印斐波那契数列的前n个数字。
 *    斐波那契数列的规则是：前两个数是0和1，从第三个数开始，每个数都是前两个数的和。
 *    (0, 1, 1, 2, 3, 5, 8, ...)
 *
 * 2. "The Twelve Days of Christmas" 歌词打印:
 *    使用循环（嵌套循环可能会有帮助）来打印出经典圣诞歌曲 "The Twelve Days of Christmas" 的全部歌词。
 *    你需要一个外层循环来控制天数（从第一天到第十二天），和一个内层循环来打印每天收到的礼物。
 *
 */