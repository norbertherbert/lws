
fn t(a: &[u8]) -> [u8; 4] {

    // let mut v= a.to_vec();
    // v.push(0);
    // v.try_into().unwrap()


    let mut dest = [0; 4];
    dest[..3].copy_from_slice(a);
    dest
}

fn main() {
    let a = [1_u8, 1, 1, 1];
    dbg!(t(&a[..3]));
}