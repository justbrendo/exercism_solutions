#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Checks if _first_list contains _second_list
pub fn contains<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _second_list.is_empty() {
        return true;
    }

    if _second_list.len() > _first_list.len() {
        return false;
    }

    for window in _first_list.windows(_second_list.len()) {
        if window.eq(_second_list) {
            return true;
        }
    }
    return false;
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }

    if contains(_first_list, _second_list) {
        return Comparison::Superlist;
    }

    if contains(_second_list, _first_list) {
        return Comparison::Sublist;
    }

    return Comparison::Unequal;
}
