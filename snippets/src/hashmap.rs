#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn options_test() {
        let teams = vec![String::from("Blue"), String::from("Red")];
        let scores = vec![22, 38];

        let final_score:HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
        println!("final_score: {:?}", final_score);

        let mut score = HashMap::new();
        score.insert(String::from("Blue"), 22);
        let s = score.entry(String::from("Red")).or_insert(38);
        println!("e: {:?}", &s);
        println!("score: {:?}", score);
    }
}
