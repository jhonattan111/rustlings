fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut _vec = vec.clone();

    _vec.push(88);

    return _vec;
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let mut vec0: Vec<i32> = vec![22, 44, 66];

        let mut vec1: Vec<i32> = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
