// fn take(v: Vec<i32>) {
//     println!("We took v: {}", v[10] + v[100]);
// }

// fn cop(a: i32, b: i32) {
//     println!("{}", a + b);
// }

// fn re(v: Vec<i32>) -> Vec<i32> {
//     println!("{}", v[120] + v[111]);
//     v
// }

// fn borrow1(v: &Vec<i32>) {
//     println!("{}", (*v)[10] + (*v)[12]);
// }

// fn borrow2(v: &Vec<i32>) {
//     println!("{}", v[10] + v[11]);
// }

fn main() {
    // let x = 1;
    // let y = x;

    // {
    //     let a = 10;
    // }

    // x + a;
    // let s = String::from("String");
    // let y = &s;

    // println!("{}", s);

    // let mut v = Vec::new();

    // for i in 1..1000 {
    //     v.push(i);
    // }
    // take(v);

    // println!("Finished");

    // let a = 32;
    // let b = 45;

    // cop(a, b);

    // println!("We have a: {} and b: {}", a, b);

    // let mut v = Vec::new();
    // for i in 1..1000 {
    //     v.push(i);
    // }

    // v = re(v);

    // println!("Still own v: {} {}", v[0], v[1]);

    // borrow1(&v);
    // println!("Still own v: {} {}", v[0], v[1]);

    // borrow2(&v);
    // println!("Still own v: {} {}", v[0], v[1]);

    let v = vec![4, 5, 6, 4, 34, 5, 6, 4, 3, 3, 5, 5, 3, 23, 1];

    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
