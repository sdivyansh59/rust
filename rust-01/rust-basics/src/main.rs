fn main() {
    println!("Hello, world!");

    
    let  tup: (i32, f64, u8) = (50, 50.251, 5);
    // destructure tup value
    let (x,y,z) = tup;

    println!("Tuple value is {} {} and {}", x,y,z);
    println!("We can acces tuple like this as well tup.0= {} , 
        tup.1= {} and tup.2= {}",tup.0, tup.1, tup.2);

    println!("***** Array *****");
    let arr = [1,2,3,4,5];  // arrray of 5 size
    for val in arr {
        println!("value ={}", val );
    }

    for x in 0..arr.len() {
        println!("index ={} and value = {}", x, arr[x]);
    }

}



// we can run any file using 
// rustc filename.rs
//  ./filename