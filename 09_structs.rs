// 09_structs.rs
// 核心内容：定义和实例化结构体，使用字段，以及为结构体实现方法（impl）。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 结构体（Struct）是一种自定义数据类型，允许你将多个相关的值打包成一个有意义的组合。
 * 如果你熟悉面向对象编程，结构体就像一个对象的“数据属性”部分。
 *
 * 1. 定义结构体
 *    - 我们使用 `struct` 关键字，后跟结构体的名字，然后是一个包含所有字段（fields）的花括号 `{}`。
 *    - 在结构体定义中，我们为每个字段指定名字和类型。
 *    - 命名规范：结构体类型名使用驼峰式命名法（CamelCase）。
 *
 * 2. 实例化结构体
 *    - 为了使用一个结构体，我们需要创建它的一个“实例”（instance）。
 *    - 实例化时，我们使用 `StructName { ... }` 语法，并为每个字段提供具体的值。
 *    - 字段的顺序在实例化时不必与定义时相同。
 *
 * 3. 访问和修改字段
 *    - 我们可以使用点号 `.` 来访问实例的字段值。
 *    - 如果实例是可变的（`mut`），我们也可以使用点号来修改字段的值。
 *
 * 4. 字段初始化简写 (Field Init Shorthand)
 *    - 当你的函数参数或变量名与结构体的字段名完全相同时，可以使用一种方便的简写语法，
 *      而无需重复 `field: field`。
 *
 * 5. 结构体更新语法 (Struct Update Syntax)
 *    - 当你想基于一个旧实例来创建一个新实例，并且只改变其中少数几个字段时，
 *      可以使用 `..` 语法。这会将你没有显式设置的字段从旧实例中自动复制过来。
 *    - 注意：这个语法会转移所有权（Move），就像赋值一样。如果旧实例的某个字段是 `String` 这种
 *      非 `Copy` 类型，那么在使用更新语法后，旧实例将不再可用。
 *
 * 6. 元组结构体 (Tuple Structs)
 *    - 有时，你想给一个元组整体命名，但又不想为每个字段命名。这时可以使用元组结构体。
 *    - 它们使用 `struct` 关键字，后跟名字和元组的类型定义。
 *    - 例如: `struct Color(i32, i32, i32);`
 *
 * 7. 为结构体实现方法 (impl)
 *    - 方法（Methods）与函数类似，但它们在结构体（或枚举、trait）的上下文中被定义。
 *    - 第一个参数总是 `self`，它代表调用该方法的结构体实例。
 *    - 我们使用 `impl` 关键字（implementation的缩写）来为结构体定义方法。
 *    - `&self`：表示该方法借用了实例的不可变引用。
 *    - `&mut self`：表示该方法借用了实例的可变引用。
 *    - `self`：表示该方法获取了实例的所有权。
 *
 * 8. 关联函数 (Associated Functions)
 *    - 在 `impl` 块中，我们也可以定义不以 `self` 作为第一个参数的函数。这些被称为“关联函数”。
 *    - 它们通常用作构造函数（constructor），用于返回一个新的结构体实例。
 *    - 我们使用 `::` 语法来调用关联函数，例如 `User::new(...)`。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 1. 定义一个 User 结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 7. 为 User 结构体实现方法
impl User {
    // 这是一个方法，它不可变地借用 User 实例
    fn describe(&self) -> String {
        format!(
            "User: {}, Email: {}, Active: {}, Sign-ins: {}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }

    // 这是一个方法，它可变地借用 User 实例
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    // 8. 这是一个关联函数，通常用作构造函数
    fn new(username: String, email: String) -> User {
        User {
            active: true,
            username, // 使用字段初始化简写
            email,    // 使用字段初始化简写
            sign_in_count: 1,
        }
    }
}

// 6. 定义一个元组结构体
struct Color(u8, u8, u8);
struct Point(f64, f64);

fn main() {
    // 2. 实例化一个 User 结构体
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 3. 访问和修改字段
    println!("User1's email is: {}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("User1's new email is: {}", user1.email);

    // 调用方法
    println!("{}", user1.describe());
    user1.increment_sign_in_count();
    println!("After incrementing: {}", user1.describe());

    // 使用关联函数创建新用户
    let user2 = User::new(
        String::from("user2"),
        String::from("user2@example.com"),
    );
    println!("Newly created user: {}", user2.describe());

    // 5. 使用结构体更新语法
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2 // 将 user2 剩余的字段值赋给 user3
    };
    println!("User3 created from user2: {}", user3.describe());
    // 注意：此时 user2.username 的所有权已经移动到 user3.username
    // 所以 user2 不能再被完整地使用了。
    // println!("{}", user2.describe()); // 这会报错！

    // 实例化元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("The color is RGB({}, {}, {})", black.0, black.1, black.2);
    println!("The point is at ({}, {})", origin.0, origin.1);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // 使用 {} 占位符，它会调用我们实现的 Display trait
    println!("Here is the rectangle: {}", rect);
    // 也可以通过 .to_string() 方法将它转换为字符串，
    // 因为所有实现了 Display 的类型都会自动获得 ToString trait。
    let s = rect.to_string();
    println!("The rectangle as a string: {}", s);
}


// 练习1：
struct Rectangle{
    width: u32,
    height: u32
}
// 18_Traits练习2的实现
impl fmt::Display for Rectangle{
    // 方法签名完全匹配 trait 定义
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用 write! 宏来将格式化后的字符串写入到 f (formatter) 中。
        // 这个宏的用法和 println! 非常相似。
        // 它返回一个 fmt::Result，如果写入成功，则为 Ok(())，如果失败则为 Err。
        write!(f, "Rectangle (width: {}, height: {})", self.width, self.height)
    }
}
impl Rectangle {

    // 练习2：
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self,r1:Rectangle) -> bool{
        self.height >= r1.height && self.width >= r1.width
    }

    // 练习3：
    fn square(size: u32)->Rectangle{
        Rectangle { width: (size), height: (size) }
    }
}
/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 创建一个 `Rectangle` 结构体:
 *    定义一个名为 `Rectangle` 的结构体，它有两个字段：`width` 和 `height`，类型都是 `u32`。
 *
 * 2. 为 `Rectangle` 实现方法:
 *    - 在一个 `impl` 块中，为 `Rectangle` 实现一个名为 `area` 的方法，
 *      它计算并返回矩形的面积 (`width * height`)。
 *    - 再实现一个名为 `can_hold` 的方法，它接收另一个 `Rectangle` 的引用作为参数，
 *      如果当前的矩形可以完全容纳下另一个矩形（即当前矩形的宽和高都大于等于另一个），
 *      则返回 `true`，否则返回 `false`。
 *
 * 3. (可选) 添加一个关联函数:
 *    为 `Rectangle` 添加一个名为 `square` 的关联函数，它接收一个 `u32` 类型的边长 `size`，
 *    并返回一个宽和高都等于 `size` 的 `Rectangle` 实例。
 *
 */