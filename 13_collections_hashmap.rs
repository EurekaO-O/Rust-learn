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

use std::collections::HashMap;
use std::io;//导入需要用户输入的包
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

    // 练习1：
    // 创建一个新的、可变的 HashMap。
    // Key 的类型是 String（部门名），Value 的类型是 Vec<String>（该部门的员工列表）
    let mut departments: HashMap<String,Vec<String>> = HashMap::new();
    println!("Welcome to Company System!");
    println!("plz enter order like (Add xxx to xxx,List xxx,List All,Quit)");
    
    loop{

        // 创建一个可变的空字符串，用来存放用户输入的内容
        let mut input = String::new();
        // 读取一行用户输入数据
        // &mut input 表示我们把 input 的可变引用传给 read_line，这样它就能修改 input 的内容
        // .expect() 是一个简单的错误处理方式，如果读取失败，程序会崩溃并显示后面的消息
        io::stdin().read_line(&mut input).expect("读取用户输入失败");

        // .trim() 会去掉输入字符串首尾的空白字符（比如换行符）
        // .split_whitespace() 会用空白字符（空格、制表符等）把字符串分割成一个一个的单词
        // .collect() 把这些单词收集到一个 Vec<&str> 类型的动态数组中
        let words: Vec<&str> =  input.trim().split_whitespace().collect();

        // 使用 match 语句来解析用户输入的命令
        // 这是 Rust 中非常强大和常见的模式匹配功能
        match words.as_slice(){
            // 模式1：匹配 "Add <xxx> to <xxx>" 格式的命令
            ["Add",name,"to",department] => {
                println!("正在添加{}到{}部门...",name,department);

                // 处理添加逻辑
                // 1. .entry(department.to_string()): 检查 'department' 这个键是否存在。
                //    .to_string() 是因为 department 是 &str 类型，而我们的 key 是 String 类型。
                // 2. .or_insert(Vec::new()): 如果键不存在，就插入一个新的空 Vec 作为值。
                // 3. 无论键是本来就存在还是刚刚插入的，.entry().or_insert() 都会返回一个指向 Vec 的可变引用。
                // 4. .push(name.to_string()): 最后，调用 Vec 的 push 方法，把员工名字加进去。
                departments.entry(department.to_string()).or_insert(Vec::new()).push(name.to_string());
                println!("添加成功！")
            }
        
            // 模式三：匹配 "List All" 命令
            ["List","All"] => {
                println!("公司所有部门及员工列表：");
                // 为了保证每次输出的顺序一致，我们先收集所有的部门名并排序
                let mut sorted_departments: Vec<_> = departments.keys().collect();
                sorted_departments.sort();
                // 遍历
                for department in sorted_departments {
                    // departments[department] 是获取部门对应员工列表的简写
                    // 这里我们确定 key 肯定存在，所以可以直接用
                    let mut employees = departments[department].clone();
                    employees.sort();
                    println!("\n ## {} ##",department);
                    for employee in employees{
                        println!("- {}",employee);
                    }
                }
            }
            
            // 模式二：匹配 "List <xxx>" 格式的命令
            ["List",department] => {
                println!("{}部门的员工列表:",department);

                // 查询方法.get()
                match departments.get(*department){
                    // Some(employees) 表示我们成功找到了部门，employees 是对员工列表 Vec 的引用
                    Some(employees) => {
                        // 创建一个克隆，因为我们不想直接修改原始数据，只是为了排序打印
                        let mut sorted_employees = employees.clone();
                        // 对员工字母排序
                        sorted_employees.sort();
                        // 遍历
                        for employee in sorted_employees {
                            println!("- {}",employee);
                        }
                    }
                    None => {
                        println!("未找到'{}'部门",department);
                    }
                }
            }

            // 模式四：匹配 "Quit" 命令
            ["Quit"] => {
                println!("Thanks,Bye!");
                break;
            }
            // 默认模式：如果用户输入的命令不匹配以上任何一种格式
            _ => {
                println!("无效命令。有效格式: 'Add <name> to <department>', 'List <department>', 'List All', 'Quit'");
            }
        }
    }
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