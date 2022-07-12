// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

<<<<<<< HEAD
// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);
=======

fn main() {
    let vec0 = Vec::new();
    let vec2 = (&vec0).to_vec();
    let mut vec1 = fill_vec(vec2);
>>>>>>> 90fab5c (2022/7/17/14.12)

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
<<<<<<< HEAD
    let mut vec = vec;
=======
    let mut vec = vec;// 当这块不加上to_vec()时，这里的接收参数就是&Vec<i32>,加上之后,变长了Vec<i32>类型
>>>>>>> 90fab5c (2022/7/17/14.12)

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
