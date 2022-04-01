#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len(); // I actually don't know if this is more efficient but I did it anyways
    let second_len = _second_list.len();

    if first_len == 0 && second_len == 0 {
        return Comparison::Equal;
    } else if first_len == 0 {
        return Comparison::Sublist;
    } else if second_len == 0 {
        return Comparison::Superlist;
    }

    let big_list: &[T]; // our variables so I don't have to deal with figuring out sizes later
    let small_list: &[T];
    let is_sub: bool; // if true, list A might be a sublist of B, if false, list A might be a superlist of B

    if first_len >= second_len {
        big_list = _first_list;
        small_list = _second_list;
        is_sub = false;
    } else {
        big_list = _second_list;
        small_list = _first_list;
        is_sub = true;
    }

    match (big_list, small_list) {
        (big, small) if big.len() == small.len() => {
            if big == small {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        } // match arm 1
        (big, small) => {
            for list in big.windows(small.len()) {
                if list == small && is_sub {
                    return Comparison::Sublist;
                } else if list == small {
                    return Comparison::Superlist;
                }
            }
            Comparison::Unequal
        } // match arm 2
    }
}
