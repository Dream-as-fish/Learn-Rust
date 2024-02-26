fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut count = 3;//用于确定循环次数
    let number = 10;//第n位的大小


    while count <= number {
        let sum = a + b;
        a = b;
        b = sum;
        count += 1;
    }
    println!("{b}");
}
