#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let equal = |list1: &[T], list2: &[T]| -> bool {
        let mut reply = true;
        if list1.len() == list2.len() {
            for i in 0..list1.len() {
                if list1[i] != list2[i] {
                    reply = false;
                }
            }
        } else {
            reply = false;
        }
        reply
    };
    let sublist = |list1: &[T], list2: &[T]| -> bool {
        if list2.len() == 0 {
            true
        } else if list1.len() == 0 {
            false
        } else {
            list1.windows(list2.len()).any(|slice| slice == list2) // OPTIMIZATION with windows() and any()
        }
    };
    let options = (
        equal(first_list, second_list),
        sublist(first_list, second_list),
        sublist(second_list, first_list),
    );
    match options {
        (true, _, _) => Comparison::Equal,
        (false, true, false) => Comparison::Superlist,
        (false, false, true) => Comparison::Sublist,
        (_, _, _) => Comparison::Unequal,
    }
}
