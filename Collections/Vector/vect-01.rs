
#[allow(dead_code)]

//run rust code
// rustc filename.rs
//  ./filename
fn main(){


// Declaring using type annotation declare empty vector 
let v: Vec<i32> = Vec::new();

// Method-2 using vec! macro
// let v2 = vec![1,2,3,4];
// no need to define type annotation here

// Method-3 most of time we will use it

let mut v3 = Vec::new();
v3.push(10);
v3.push(20);
v3.push(30);
// here rust will automatically detect v3 of type i32



// Print
for i in &v3 {
    println!("{}",i);
}
// mutable
for i in &mut v3 {
    *i +=2;
    println!("{}",i);
}

}