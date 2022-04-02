#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Clone>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut big: Vec<T> = vec![];
    let mut small: Vec<T> = vec![];

    if _first_list.len() >= _second_list.len() {
        big = _first_list.to_vec();
        small = _second_list.to_vec();
    } else {
        big = _second_list.to_vec();
        small = _first_list.to_vec();
    }

    unimplemented!("Not Yet")
}
