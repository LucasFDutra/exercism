#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.len() == 0 {
        return Comparison::Sublist;
    } else if second_list.len() == 0 {
        return Comparison::Superlist;
    } else if first_list.len() < second_list.len() {
        let len_first = first_list.len();
        let n_chunks = second_list.len() - len_first + 1;
        for i in 0..n_chunks {
            if &second_list[i..i + len_first] == first_list {
                return Comparison::Sublist;
            }
        }
    } else if first_list.len() > second_list.len() {
        let len_second = second_list.len();
        let n_chunks = first_list.len() - len_second + 1;
        for i in 0..n_chunks {
            if &first_list[i..i + len_second] == second_list {
                return Comparison::Superlist;
            }
        }
    }

    return Comparison::Unequal;
}
