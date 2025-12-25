// TODO: Fix the compiler error in this function.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    //let mut vec: Vec<i32> = Vec::new();

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![1, 2, 3];
    let vec_new = fill_vec(vec0);
    println!("new_vec is {:?}", vec_new);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
