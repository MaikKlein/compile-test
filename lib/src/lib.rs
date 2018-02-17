#[macro_use]
extern crate crunchy;
use std::path::Path;

macro_rules! long_ct {
    () => {
        unroll! {
            for i in 0..1024 {
                for _ in 0..i {
                    let v: Vec<u32> = vec![i as u32; 1024];
                    println!("{:?}", v);
                }
            }
        }
    }

}
pub fn test<P: AsRef<Path>>(p: P) {
    long_ct!();
    println!("{:?}", p.as_ref());
}

pub fn test1_impl(p: &Path) {
    long_ct!();
    println!("{:?}", p);
}

pub fn test1<P: AsRef<Path>>(p: P) {
    test1_impl(p.as_ref());
}

pub fn test2<P: AsRef<Path>>(p: P) {
    pub fn test2_impl(p: &Path) {
        long_ct!();
        println!("{:?}", p);
    }

    test2_impl(p.as_ref());
}
