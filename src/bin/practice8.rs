fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_powered_nums = nums
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect::<Vec<i32>>();

    println!("Even powered nums: {:?}", even_powered_nums);

    let sum = even_powered_nums.iter().sum::<i32>();
    println!("Sum: {:?}", sum);

    let has_big = nums.iter().any(|x| *x > 9);

    println!("Has big: {}", has_big);

    let all_positive = nums.iter().all(|x| x.is_positive());

    println!("All positive: {}", all_positive);

    // .iter() only read data, not modify
    // .iter_mut() read and modify data
    // .into_iter() take the ownership
}
