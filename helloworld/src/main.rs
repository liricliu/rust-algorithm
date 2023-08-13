use scanf::scanf;

fn main() {
    let mut a:i32=0;
    let mut b:i32=0;
    if scanf!("{} {}",a,b).is_ok(){
        print!("{}\n",a+b);
    }
}
