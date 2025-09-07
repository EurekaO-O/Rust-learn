// 17_generics.rs
// 核心内容：讲解泛型，如何将其用于函数、结构体和枚举中，以减少代码重复。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 泛型（Generics）是一种抽象的方式，用于处理重复的代码。它允许我们编写不指定具体类型的函数或数据结构，
 * 而将具体类型留到调用或实例化时再确定。
 *
 * 1. 为什么需要泛型？- 消除重复
 *    - 假设你需要一个函数来找出一组数字中的最大值，然后又需要一个函数来找出一组字符中的最大值。
 *      这两个函数的逻辑是完全相同的，只有处理的数据类型不同。
 *    - 使用泛型，我们可以只编写一个 `largest` 函数，它可以处理任何实现了特定行为（比如比较大小）的类型。
 *
 * 2. 在函数定义中使用泛型
 *    - 我们在函数名后的尖括号 `<>` 中声明泛型类型参数。
 *    - 泛型类型参数的命名通常是简短的大写字母，例如 `T` (代表 Type)。
 *    - 声明后，我们就可以在函数参数和函数体中使用这个泛型 `T` 作为具体的类型。
 *    - 例如: `fn largest<T>(list: &[T]) -> &T { ... }`
 *
 * 3. Trait 约束 (Trait Bounds) - 限制泛型类型
 *    - 上面的 `largest` 函数实际上无法编译。因为 `>` 比较运算符并不是对所有类型都有效的。
 *    - 为了让函数能编译，我们需要告诉编译器，泛型 `T` 必须是“可比较的”。
 *    - 我们通过“Trait 约束”来实现这一点。Trait 定义了类型可以拥有的行为（下一课的主题）。
 *    - `std::cmp::PartialOrd` trait 适用于可以比较大小的类型。
 *    - `Copy` trait 适用于可以按位复制的类型。
 *    - 语法: `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { ... }`
 *      这表示 `T` 必须同时实现 `PartialOrd` 和 `Copy` 两个 trait。
 *
 * 4. 在结构体定义中使用泛型
 *    - 结构体也可以使用泛型，从而在它的字段中持有不同类型的值。
 *    - 例如，我们可以定义一个 `Point<T>` 结构体，它的 `x` 和 `y` 字段都是 `T` 类型。
 *      这样我们就可以创建 `Point<i32>` 或 `Point<f64>`。
 *    - 结构体也可以有多个泛型参数，例如 `Point<T, U>`。
 *
 * 5. 在枚举定义中使用泛型
 *    - 我们其实已经见过泛型枚举了！`Option<T>` 和 `Result<T, E>` 都是泛型枚举。
 *    - `Option<T>` 的 `Some` 变体持有一个 `T` 类型的值。
 *    - `Result<T, E>` 的 `Ok` 变体持有 `T` 类型的值，`Err` 变体持有 `E` 类型的值。
 *
 * 6. 在方法定义中使用泛型
 *    - 我们可以在 `impl` 块中为泛型结构体或枚举实现方法。
 *    - `impl<T> Point<T> { ... }`
 *    - 我们甚至可以为特定的具体类型实现方法，例如：
 *      `impl Point<f32> { ... }` 这样，只有 `Point<f32>` 类型的实例才会有这些方法。
 *
 * 7. 性能
 *    - Rust 中的泛型是“零成本抽象”（zero-cost abstraction）。
 *    - 编译器通过“单态化”（monomorphization）过程，在编译时将泛型代码替换为具体的类型代码。
 *    - 这意味着使用泛型不会有任何运行时性能损失，你既获得了代码的灵活性，又保持了与手写具体类型代码相同的速度。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 2. 在函数中使用泛型，并带有 Trait 约束
// 这个函数可以找到任何实现了 PartialOrd (可比较) 和 Copy (可复制) trait 的类型的切片中的最大值
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list;
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 4. 在结构体中使用泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 6. 在方法定义中使用泛型
impl<T, U> Point<T, U> {
    // 这个方法适用于任何 Point<T, U>
    fn x(&self) -> &T {
        &self.x
    }

    // 这个方法会消耗 Point，并返回一个新的、包含不同类型的 Point
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 只为 Point<f32, f32> 实现的方法
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // 使用泛型函数 largest
    let number_list = vec!;
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!;
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 使用泛型结构体 Point
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed_point = Point { x: 5, y: 4.0 }; // T 是 i32, U 是 f64

    println!("integer_point.x = {}", integer_point.x());
    println!("float_point distance from origin: {}", float_point.distance_from_origin());
    // 下面这行会报错，因为 distance_from_origin 只为 Point<f32, f32> 定义
    // println!("integer_point distance: {}", integer_point.distance_from_origin());

    let p3 = integer_point.mixup(float_point);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 泛型 `Pair` 结构体:
 *    创建一个名为 `Pair<T>` 的泛型结构体，它有两个字段 `first` 和 `second`，类型都是 `T`。
 *    为它实现一个 `new` 关联函数来创建实例。
 *    再为它实现一个 `cmp_display` 方法，该方法需要 `T` 同时实现 `Display` (可打印) 和 `PartialOrd` (可比较) trait。
 *    `cmp_display` 方法会比较 `first` 和 `second`，并打印出较大的那个。
 *
 * 2. 泛型 `filter` 函数:
 *    编写一个泛型函数 `filter`，它接收一个 `&[T]` 切片和一个闭包（`F: Fn(&T) -> bool`）作为参数。
 *    这个函数应该返回一个新的 `Vec<T>`，其中只包含满足闭包条件的元素。
 *    确保 `T` 实现了 `Clone` trait，因为你需要克隆元素到新的 `Vec` 中。
 *    (我们还没有正式学习闭包，但你可以把它看作一个可以捕获环境的匿名函数。
 *    例如 `|&x| x > 5` 就是一个判断数字是否大于5的闭包。)
 *
 */