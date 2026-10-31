fn first_even(nums: &[i32]) -> Option<i32> {
    nums.iter().find(|x| *x % 2 == 0).copied()
}

fn main() {
    let arr = [1, 3, 23, 5];

    match first_even(&arr) {
        Some(num) => println!("First even number is {}", num),
        None => println!("No even number found"),
    }
}
