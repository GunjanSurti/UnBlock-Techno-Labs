fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;

    println!("{:#?}", r);
    println!("{:#?}", address);

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3); 
    // from safe library
    /* split_at_mut => Divides one mutable slice into two at an index.
    can take generic too
    The first will contain all indices from [0, mid) (excluding the index mid itself) and the second will contain all indices from [mid, len) (excluding the index len itself).
    ******************Panics*****************
    Panics if "mid > len".
    */

    println!("{:#?}", a);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
