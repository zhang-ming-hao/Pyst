/// 仿照Python的os.path模块编写rust的库
/// Rust的标准库中，关于路径的操作都封装在Path结构体中，不是很方便，实际上Path操作就是对字符串的操作
/// 本库中的所有的函数和宏的参数都是字符串，返回的也尽量是字符串，有些Option或Result
pub mod path;

/*
pub fn ListDir(p: &str) -> Vec<str>
{
    
}*/