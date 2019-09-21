//use std::mem;

fn main() {
    // let t = (1, 'a', false);
    // let f = (2, t);
    // let ff = (2, (1, 'a', false)); //same thing as above
    // println!("{} {} {}", t.0, t.1, t.2);
    // println!("{:#?}", ff); 
    // let t = (1,2,3,4,5,6,7,8,9,10,11,12,13,14);

    // println!("{:?}", t);
    
    // let xs: [i32; 5] = [4,5,6,7,8];
    // println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs))
    // let ys = &xs[2..4];
    // println!("{} {}", ys[0], ys[1]);

    // println!("{:?} {:?}", ys, xs);
    
    // let s = "String".to_string();
    // let ss = String::from("String");

    // let slice = &ss[2..4];
    // println!("{}", slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let s = h + &w;
    println!("{}", s);
}
