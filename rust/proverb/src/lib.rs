pub fn build_proverb(list: &[&str]) -> String {
    // For want of a nail the shoe was los
    // For want of a shoe the horse was lost.
    // For want of a horse the rider was lost.
    // For want of a rider the message was lost.
    // For want of a message the battle was lost.
    // For want of a battle the kingdom was lost.
    // And all for the want of a nail.
    let mut proverb = String::new();
    for i in 0..list.len().saturating_sub(1) {
        proverb += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
    }

    if !list.is_empty() {
        proverb += &format!("And all for the want of a {}.", list[0]);
    }
    return proverb;
}
