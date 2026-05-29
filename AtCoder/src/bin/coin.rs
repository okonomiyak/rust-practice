use proconio::input;

fn main(){
    input! {a:u16,b:u16,c:u16,x:u16,}let mut time_amount = 0;for i in 0..=a{for j in 0..=b{for k in 0..=c{if 500*i+100*j+50*k == x{time_amount += 1;}}}}println!("{}",time_amount);
}