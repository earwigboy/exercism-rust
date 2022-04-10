#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|w| w == first_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_list_length = _first_list.len();
    let second_list_length = _second_list.len();

    if first_list_length == second_list_length && _first_list == _second_list {
        Comparison::Equal
    } else if first_list_length < second_list_length && is_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if second_list_length < first_list_length && is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
