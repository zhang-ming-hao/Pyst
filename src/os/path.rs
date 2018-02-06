use std::path::*;

/// 判断路径是否为一个文件  
/// # 参数：  
/// * p：str类型的路径
///   
/// # 返回值：  
/// 是文件返回true，否则返回false
pub fn IsFile(p: &str) -> bool
{
    let path = Path::new(p);

    path.is_file()
}

/// 判断路径是否为一个目录  
/// # 参数：  
/// * p：str类型的路径   
///
/// # 返回值：  
/// 是文件返回true，否则返回false
pub fn IsDir(p: &str) -> bool
{
    let path = Path::new(p);

    path.is_dir()
}

/// 从路径中分享出文件或文件夹名
/// # 参数：  
/// * p：str类型的路径   
///
/// # 返回值：  
/// 成功返回文件或文件夹名，否则返回空字符串
pub fn BaseName(p: &str) -> String
{
    let path = Path::new(p);
    match path.file_name()
    {
        None => String::from(""),
        Some(s) => 
        {
            match s.to_str()
            {
                None => String::from(""),
                Some(st) => String::from(st),
            }            
        },
    }
}

/*
pub fn Split(p: &str) -> String
{

}
*/

/// 将多个路径拼接成一个 
/// # 参数：  
/// 要拼接的多个字符串  
///
/// # 返回值：  
/// join成功返回join后的路径，失败返回空字符串
#[macro_export]
macro_rules! PathJoin
{
    ($($x:expr),*) => {
        {
            let mut p = std::path::PathBuf::new();
            $(p.push($x);)*

            match p.to_str() {
                Some(s) => String::from(s),
                None => String::from(""),
            }
        }
    };
}