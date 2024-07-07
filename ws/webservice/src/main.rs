use std::num::ParseIntError;

fn main() {
    let result = square("ok");
    println!("{:?}", result);
}


fn square(val: &str) -> Result<i32, ParseIntError> {

    // 另外一种处理错误的方式：使用 ? 运算符
    // 在函数中使用 ? 运算符，该运算符尝试从 Result 中获取值：
    // 成功，拿到 Ok 变体中的值
    // 失败：接收 Error，并终止函数执行，并把错误传播到调用该函数的函数

    let num = val.parse::<i32>()?;

    Ok(num.pow(2))


    // match val.parse::<i32>() {
    //     Ok(num) => Ok(num.pow(2)),
    //     Err(e) => Err(e)
    // }
}