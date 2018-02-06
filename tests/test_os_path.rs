#![allow(non_snake_case)]

#[macro_use]
extern crate pyst;
use pyst::os::path;

#[test]
fn IsFile_NotFile()
{
    assert_eq!(path::IsFile("C:\\Windows"), false);
}

#[test]
fn IsFile_OK()
{
    assert_eq!(path::IsFile("C:\\Windows\\win.ini"), true);
}

#[test]
fn IsDir_OK()
{
    assert_eq!(path::IsDir("C:\\Windows"), true);
}

#[test]
fn IsFile_NotDir()
{
    assert_eq!(path::IsDir("C:\\Windows\\win.ini"), false);
}


#[test]
fn Join_OK()
{
    assert_eq!(PathJoin!("c:\\", "windows", "system32") , "c:\\windows\\system32")
}

#[test]
fn BaseName_OK()
{
    assert_eq!(path::BaseName("C:\\Windows\\win.ini"), "win.ini");
}

#[test]
fn BaseName_NoFile()
{
    assert_eq!(path::BaseName("C:\\"), "");
}