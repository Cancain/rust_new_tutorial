fn main() {
    //i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    // let mut x: u32 = 5;
    // x = 5;
    
    let a = 1 + 20;

    let s = 30 - 20;
    let m = 5 * 10;
    let d = 4/6; 
    let r = 49 % 2;

    //bool: true/false
    let c = 'c';
    let c: char = 'z';

    //tuples
    let t : (i32, f64, char) = (42, 6.12, 'j');
    let (z, y, x) = t;
    let (_, _ , x) = t;

    println!("{} {} {}", t.0, t.1, t.2);
    let ab = [1,2,3,4,5,6,7,8];
    let ab1 = ab[0];
}
