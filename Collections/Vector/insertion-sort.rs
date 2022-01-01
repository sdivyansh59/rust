#[allow(dead_code)]

use std::io;

fn main (){
    let mut v : Vec<i32> = Vec:: new();
    
    println!("Enter size of array:");
    let mut size = String::new();

    io::stdin().read_line( &mut size).expect("failed to readline");
    
    let size : u32 = size.trim().parse().expect("Please type a number!");

    for _i in 0..size {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("failed to take input");
        let  temp : i32 = temp.trim().parse().expect("please enter number only!");
        v.push(temp);
    } 

    for i in &v {
        println!("{}", i)
    }
    
    insertion_sort(&mut v);

    for i in &v {
        println!("{}", i)
    }
}

fn insertion_sort(  v : &mut Vec<i32>) {
   
  println!("insertion called");
  v.sort();

}
