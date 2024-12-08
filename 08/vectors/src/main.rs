fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut vp = vec![100, 32, 57];
    for i in &mut vp {
        *i += 50;
        println!("{i}");
    }
}
