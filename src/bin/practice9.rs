struct Student {
    name: String,
    score: u32,
}

fn main() {
    let list = [
        Student {
            name: String::from("John"),
            score: 80,
        },
        Student {
            name: String::from("Jane"),
            score: 90,
        },
        Student {
            name: String::from("Jim"),
            score: 70,
        },
        Student {
            name: String::from("Jill"),
            score: 60,
        },
    ];

    let pass_names: Vec<String> = list
        .iter()
        .filter(|x| x.score >= 60)
        .map(|x| x.name.clone())
        .collect();

    println!("Pass Names: {:?}", pass_names);

    let avg_score: f64 = list.iter().map(|x| x.score as f64).sum::<f64>() / list.len() as f64;

    println!("Average Score: {:?}", avg_score);

    let best_student = list.iter().max_by_key(|x| x.score);

    if let Some(x) = best_student {
        println!("Best student: {}", x.name);
    }
}
