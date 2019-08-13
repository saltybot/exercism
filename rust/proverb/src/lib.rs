pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = "".to_string();

    if list.len() > 0 {
        let mut word = list[0];
        for i in 1..list.len() {
            proverb.push_str(
                &format!("For want of a {} the {} was lost.\n", word, list[i]).to_string(),
            );
            word = list[i];
        }
        proverb.push_str(&format!("And all for the want of a {}.", list[0]).to_string());
    }

    proverb
}
