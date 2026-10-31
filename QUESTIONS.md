# Round 1 Questions

## 練習 1：基本 function

寫 `fn f_to_c(f: f64) -> f64`，將華氏轉攝氏，喺 `main` 印出 `f_to_c(98.6)`。

提示：最後一行唔加分號就係 return value，好似 Ruby 嘅隱式 return。

## 練習 2：Option

寫 `fn first_even(nums: &[i32]) -> Option<i32>`，搵出第一個雙數。喺 `main` 用 `match` 處理 `Some` 同 `None`。

提示：可以用 `for n in nums`，或者 `nums.iter().find(...)`。

## 練習 3：Ownership（修 compile error）

```rust
fn main() {
    let name = String::from("Ferris");
    greet(name);
    println!("{}", name);
}

fn greet(n: String) {
    println!("Hi {}", n);
}
```

用兩種方法修好：(a) 用 `&` 借用，(b) 用 `.clone()`。再諗下邊種好啲，點解。

## 練習 4：enum + match

定義：

```rust
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}
```

寫 `fn area(s: &Shape) -> f64`。之後試吓喺 enum 加個 `Triangle`，睇編譯器點提你。

## 練習 5：Result 同 `?`

寫 `fn parse_age(s: &str) -> Result<u32, std::num::ParseIntError>`，用 `s.parse::<u32>()` 同 `?`。喺 `main` 試 `"25"` 同 `"abc"`。

---

# Round 2 Questions

Files live in `src/bin/practiceN.rs`. Run with `cargo run --bin practiceN`.
Each file has its own `main` and prints visible output.

## practice6: `struct` + `impl`

Struct 好似 TS 嘅 `interface` 加 class 嘅 method 合體，但 data 同 behavior 分開寫：`struct` 放 field，`impl` 放 method。

- 定義 `struct Rectangle { width: f64, height: f64 }`
- `impl` 入面寫：
  - `fn new(width: f64, height: f64) -> Rectangle`，呢個係 associated function，好似 Rails 嘅 `self.build`，冇 `self` 參數
  - `fn area(&self) -> f64`
  - `fn scale(&mut self, factor: f64)`，會改自己
- `main` 入面 new 一個，print area，scale 之後再 print 一次

**要諗**：點解 `area` 用 `&self`，`scale` 用 `&mut self`？個 `let` 要加 `mut` 先得，點解？

## practice7: `trait`

Trait 即係 TS 嘅 `interface`，但可以幫自己嘅 type 加實作。

- 定義 `trait Describe { fn describe(&self) -> String; }`
- 幫 practice6 嘅 `Rectangle` 同一個新 struct `Circle { radius: f64 }` 都 `impl Describe`
- 寫一個 `fn print_it(item: &impl Describe)`，用兩種 type 都叫一次

**要諗**：如果想擺 Rectangle 同 Circle 入同一個 `Vec`，`impl Describe` 夠唔夠？（你會撞到牆，睇下個 error 講乜。呢個問題留到 round 3 先解，今次撞完記低個 error 就夠。）

## practice8: Iterators 基礎

Iterator 即係 JS 嘅 `.map().filter()` chain，但係 lazy，要 `collect()` 先真正行。

用 `let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];`：

1. 揀出偶數，平方，`collect` 做 `Vec<i32>`
2. 用 `.sum::<i32>()` 計總和
3. 用 `.iter().any(...)` 同 `.all(...)` 各試一次

**要諗**：`.iter()`、`.into_iter()`、`.iter_mut()` 有咩分別？同 ownership 有咩關係？

## practice9: Struct + Iterator 夾埋用

- 定義 `struct Student { name: String, score: u32 }`
- 整一個 `Vec<Student>`（最少 4 個）
- 用 iterator 做：
  1. 揀出 score >= 60 嘅人，`collect` 成 `Vec<String>` 嘅名
  2. 計平均分（記得 `f64` 轉換）
  3. 搵最高分嗰個（提示：`max_by_key`，個結果係 `Option`，你識處理）

## practice10: 迷你 CLI 項目（熱身）

寫個 todo list 喺記憶體度（暫時唔使存檔）：

- `struct Todo { id: u32, title: String, done: bool }`
- `struct TodoList { items: Vec<Todo> }`，有 `add`、`complete(id) -> Result<(), String>`、`list`
- `main` 入面行一串操作：add 三個、complete 一個、complete 一個唔存在嘅 id（要見到 Err 被處理）、最後 print 晒

**重點**：`complete` 要用 `iter_mut().find(...)` 加 `ok_or(...)?` 或者 `match`，將 round 1 嘅 `Option`/`Result` 同今輪嘅嘢串埋。

## Reminders

- 卡住就 `.clone()`，唔好為咗靚而浪費時間。
- Compiler error 先自己睇晒，睇唔明先貼嚟，我逐行拆。
- 做完一個就 save 低叫我 review，唔使等晒五個。
