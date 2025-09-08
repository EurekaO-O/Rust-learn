// 11_collections_vector.rs
// 核心内容：介绍动态数组Vec<T>的创建、添加、读取、遍历和修改。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 与数组（Array）不同，数组的长度是固定的。而动态数组（Vector），类型写作 `Vec<T>`，
 * 允许你存储一个可变数量的、相同类型的值。`T` 是泛型，代表 `Vec` 中存储的元素类型。
 *
 * 1. 创建 Vector
 *    - `Vec::new()`: 创建一个新的、空的 `Vec`。
 *    - `vec![]` 宏: 一个方便的宏，可以让你在创建 `Vec` 的同时提供初始值。
 *
 * 2. 存储与内存
 *    - `Vec` 的数据存储在堆（Heap）上，这意味着它的大小可以在运行时增长或缩小。
 *    - 当 `Vec` 本身离开作用域时，它所拥有的所有内容也会被清理掉，这得益于所有权系统。
 *
 * 3. 添加元素
 *    - 使用 `push()` 方法可以在 `Vec` 的末尾添加一个新元素。
 *    - 为了能够 `push`，`Vec` 必须是可变的（`mut`）。
 *
 * 4. 读取元素
 *    有两种主要方式来访问 `Vec` 中的元素：
 *    a) 索引语法 `[]`:
 *       - `let third = &v[2];`
 *       - 这种方式简单直接，但如果索引越界，程序会 `panic`。
 *    b) `get()` 方法:
 *       - `let third = v.get(2);`
 *       - `get()` 方法返回一个 `Option<&T>`。如果索引越界，它会返回 `None`，而不会 `panic`。
 *       - 这使得 `get()` 在你不确定索引是否有效时，是更安全、更健壮的选择。
 *
 * 5. 所有权和借用规则在 `Vec` 中的体现
 *    - 这是一个非常重要的细节：你不能在持有对 `Vec` 中某个元素的不可变引用的同时，
 *      尝试向 `Vec` 中添加新元素（这需要一个可变引用）。
 *    - `let mut v = vec![1, 2, 3];`
 *    - `let first = &v[0];`
 *    - `v.push(4); // 这里会报错！`
 *    - `println!("The first element is: {}", first);`
 *    - 为什么？因为 `push` 可能会导致 `Vec` 在堆上重新分配内存以获得更多空间。
 *      如果发生重分配，`Vec` 的所有元素可能会被移动到新的内存地址，
 *      这就会使我们之前获取的 `first` 引用变成一个悬垂引用，指向无效的内存。
 *      Rust的借用检查器在编译时就防止了这种情况的发生。
 *
 * 6. 遍历 `Vec` 中的元素
 *    - 使用 `for` 循环是遍历 `Vec` 中所有元素的最佳方式。
 *    - `for i in &v`: 创建不可变引用来遍历。
 *    - `for i in &mut v`: 创建可变引用来遍历，以便在循环中修改元素。
 *
 * 7. 使用枚举来存储多种类型
 *    - `Vec` 只能存储相同类型的元素。但有时我们需要存储不同类型的东西。
 *    - 解决方案是定义一个枚举，该枚举的变体可以包含不同的数据类型。
 *      然后，我们可以创建一个存储该枚举类型实例的 `Vec`。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

use std::collections::HashMap;

fn main() {
    // 1. 创建 Vector
    // 创建一个空的 Vec<i32>
    let mut v: Vec<i32> = Vec::new();

    // 3. 添加元素
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Initial vector v: {:?}", v);

    // 使用 vec! 宏创建并初始化
    let v2: Vec<_> = vec![1,23,45];
    println!("v2 created with macro: {:?}", v2);

    // 4. 读取元素
    // 使用索引
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 使用 get 方法
    match v.get(2) {
        Some(third_val) => println!("The third element is {}", third_val),
        None => println!("There is no third element."),
    }

    // get 方法处理越界情况
    let does_not_exist = v.get(100);
    println!("Getting index 100: {:?}", does_not_exist); // 打印 None

    // 5. 借用规则演示
    // let mut v_borrow = vec!;
    // let first = &v_borrow; // 不可变借用
    // v_borrow.push(6); // 可变借用 - 这里会报错！
    // println!("The first element is: {}", first);

    // 6. 遍历 Vector
    let v_iter = vec![1,2,3];
    println!("\nIterating over v_iter:");
    for i in &v_iter {
        println!("- {}", i);
    }

    // 遍历并修改元素
    let mut v_mut_iter = vec![1,2,3,64];
    println!("\nIterating and modifying v_mut_iter:");
    for i in &mut v_mut_iter {
        *i += 50; // `*` 是解引用操作符，用于获取引用指向的值
    }
    // 打印修改后的 vector
    println!("Modified v_mut_iter:");
    for i in &v_mut_iter {
        println!("- {}", i);
    }

    // 7. 使用枚举存储多种类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("\nRow with multiple types: {:?}", row);

    // 练习1： 
    let list1 = vec![5, 1, 2, 5, 3, 5, 2];
    println!("List 1: {:?}", list1);
    match calculate_median(&list1) {
        Some(median) => println!("  Median is: {}", median), // 输出：3
        None => println!("  No median found."),
    }
    // 练习2：
    match calculate_mode(&list1) {
        Some(mode) => println!("  Mode is: {}", mode),   // 输出：5
        None => println!("  No mode found."),
    }
}

fn calculate_median(numbers: &[i32]) -> Option<f64>{
    if numbers.is_empty(){
        return None;
    }

    let mut sorted_numbers = numbers.to_vec();

    sorted_numbers.sort_unstable();
    let len = sorted_numbers.len();
    let mid_index = len / 2;

    if len % 2 == 0 {
        let mid1 = sorted_numbers[mid_index -1] as f64;
        let mid2 = sorted_numbers[mid_index] as f64;
        Some((mid1 + mid2) / 2.0)
    }else{
        Some(sorted_numbers[mid_index] as f64)
    }

}
// 思路：用hashmap记录所有元素的出现次数，出现次数最多的元素即为众数
fn calculate_mode(numbers: &[i32]) -> Option<i32>{
    if numbers.is_empty(){
        return None;
    }

    let mut counts = HashMap::new();

    for &num in numbers{
        //entry(num)检查num是否为map中的键
        //or_insert 如果不存在，插入0，并且返回该值的可变引用
        *counts.entry(num).or_insert(0)+=1;
    }
    // 现在我们需要找到计数值最大的那个条目。
    // `counts.iter()` 创建一个迭代器。
    // `.max_by_key(|&(_, count)| count)` 找到一个条目，其 count (值) 是最大的。
    // `max_by_key` 返回一个 Option，因为 HashMap 可能为空（尽管我们已经处理了空列表）。
    // `map(|(&num, _)| num)` 如果找到了最大条目，就提取出它的键（num），并返回它。
    let mode = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num);
    
    mode

}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 计算中位数和众数:
 *    给定一个整数 `Vec`，编写两个函数：一个返回中位数（排序后中间位置的值），
 *    另一个返回众数（出现次数最多的值；提示：使用HashMap会更容易，但也可以尝试只用Vec解决）。
 *
 * 2. 字符串转换成 Pig Latin:
 *    编写一个函数，它接收一个字符串切片 `&str`，并将其中的每个单词转换成 Pig Latin 形式。
 *    规则是：如果单词以元音开头，则在末尾加上 "-hay"。如果以辅音开头，则将第一个辅音
 *    移到末尾，并加上 "-ay"。
 *    例如, "first" -> "irst-fay", "apple" -> "apple-hay"。
 *    函数应该返回一个新的 `String`。
 *
 */