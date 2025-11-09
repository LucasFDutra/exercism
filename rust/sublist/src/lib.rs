#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist(smaller: &[i32], larger: &[i32]) -> bool {
    if smaller.len() == 0 {
        return true;
    }
    for chunk in larger.windows(smaller.len()) {
        if chunk == smaller {
            return true;
        }
    }
    return false;
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.len() < second_list.len() {
        if is_sublist(first_list, second_list) {
            return Comparison::Sublist;
        }
    } else if first_list.len() > second_list.len() {
        if is_sublist(second_list, first_list) {
            return Comparison::Superlist;
        }
    }

    return Comparison::Unequal;
}
