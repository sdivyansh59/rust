
fn main(){
   let mut v = vec![0];
   v.push(1);
   v.push(2);
   v.push(3);

//    let b = v.get(0);
   println!("Vector is {:?} and Length is {}", v, v.len());
   println!("Vector is {:?}", &v[1]);
//    println!("Vector is {:?}", v.get(1));  //error 
    let c = v.pop();
    println!("Last pop element is {:?} ",c);



}