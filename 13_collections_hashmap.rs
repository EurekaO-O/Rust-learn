// 13_collections_hashmap.rs
// 核心内容：介绍键值对集合HashMap<K, V>的创建、插入、访问和更新。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 哈希映射（Hash Map），类型写作 `HashMap<K, V>`，存储了从“键”（Keys）类型 `K`
 * 到“值”（Values）类型 `V` 的映射关系。
 * 它通过一个“哈希函数”（hashing function）来决定如何在内存中存放键和值。
 *
 * 1. 创建 HashMap
 *    - `HashMap::new()`: 创建一个新的、空的 `HashMap`。
 *    - `use std::collections::HashMap;`: `HashMap` 不在预导入（prelude）中，所以需要手动导入。
 *
 * 2. 插入数据
 *    - 使用 `insert(key, value)` 方法来添加或更新一个键值对。
 *
 * 3. 所有权
 *    - 对于像 `i32` 这样实现了 `Copy` trait 的类型，它们的值会被复制到 `HashMap` 中。
 *    - 对于像 `String` 这样拥有所有权的类型，它们的值（和所有权）会被移动到 `HashMap` 中。
 *      一旦被移动，你就不能再使用原来的变量了。
 *
 * 4. 访问数据
 *    - 使用 `get(&key)` 方法来获取与键对应的值。
 *    - `get` 方法返回一个 `Option<&V>`，因为请求的键可能不存在。
 *    - 如果键存在，返回 `Some(&value)`；如果不存在，返回 `None`。
 *
 * 5. 遍历 HashMap
 *    - 使用 `for` 循环可以遍历 `HashMap` 中的所有键值对。
 *    - 遍历的顺序是任意的，不保证与插入的顺序相同。
 *
 * 6. 更新 HashMap 中的值
 *    处理更新时有几种常见的情况：
 *    a) 覆盖旧值:
 *       - 如果你用相同的键 `insert` 两次，第二次的值会覆盖第一次的值。
 *
 *    b) 仅在键不存在时插入:
 *       - `entry(key)` 方法会返回一个名为 `Entry` 的枚举，它代表一个键是否存在。
 *       - `or_insert(value)` 方法：如果键已存在，`or_insert` 返回该键对应值的可变引用；
 *         如果键不存在，它会先插入给定的 `value`，然后返回这个新值的可变引用。
 *       - 这是一种非常方便的模式，用于确保一个键总是有值。
 *
 *    c) 基于旧值来更新值:
 *       - 结合 `entry` 和 `or_insert`，我们可以获取一个值的可变引用，然后直接修改它。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 1. 需要手动导入 HashMap
use std::collections::HashMap;

fn main() {
    // 创建一个新的 HashMap，键是 String，值是 i32
    let mut scores = HashMap::new();

    // 2. 插入数据
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 3. 所有权演示
    let team_name = String::from("Red");
    let team_score = 100;
    scores.insert(team_name, team_score);
    // `team_name` 的所有权被移动到了 scores 中，不能再使用
    // println!("{}", team_name); // 这会报错！
    // `team_score` 是 i32，是 Copy 类型，所以可以继续使用
    println!("Team score variable is still valid: {}", team_score);

    // 4. 访问数据
    let blue_team_score = scores.get(&String::from("Blue"));
    match blue_team_score {
        Some(score) => println!("Blue team score is: {}", score),
        None => println!("Blue team not found."),
    }

    // 5. 遍历 HashMap
    println!("\nAll scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 6. 更新 HashMap
    // a) 覆盖旧值
    println!("\nUpdating Yellow's score...");
    scores.insert(String::from("Yellow"), 75);
    println!("{:?}", scores);

    // b) 仅在键不存在时插入
    println!("\nUsing entry().or_insert()...");
    // "Green" 不存在，所以会插入 30
    scores.entry(String::from("Green")).or_insert(30);
    // "Blue" 已存在，所以 or_insert 不会做任何事
    scores.entry(String::from("Blue")).or_insert(1000);
    println!("{:?}", scores);

    // c) 基于旧值来更新值
    let text = "hello world wonderful world";
    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        // `entry(word.to_string()).or_insert(0)` 返回一个 &mut i32
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1; // 使用解引用操作符 `*` 来修改这个值
    }
    println!("\nWord counts: {:?}", word_counts);
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 公司部门管理:
 *    使用 `HashMap` 和 `Vec`，创建一个文本界面来允许用户向公司的某个部门添加员工。
 *    例如，用户可以输入 "Add Sally to Engineering" 或 "Add Amir to Sales"。
 *    然后，用户应该能够输入一个部门名称，程序会打印出该部门所有员工的列表，
 *    并按字母顺序排序。
 *
 * 2. (来自 `11_collections_vector.rs` 的挑战) 使用 HashMap 计算众数:
 *    现在你已经学习了 `HashMap`，请重新完成之前 `Vec` 那一课的挑战：
 *    给定一个整数 `Vec`，编写一个函数返回众数（出现次数最多的值）。
 *    使用 `HashMap` 来记录每个数字出现的次数，会使这个问题变得简单很多。
 *
 */