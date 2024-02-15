fn five() -> i32 {//->后申明返回值类型，返回值为函数体最后一个表达式的值
    5//使用return关键字和指定值，可以从函数中提前返回
}//这个函数等同于let x = 5;

fn main() {
    let x = five();
    println!("The value of x is:{x}")
}
