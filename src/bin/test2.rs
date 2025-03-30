fn main() {
    let mut x = 1;
    let y = &mut x;
    drop(y);
    let z = &mut x;
}
