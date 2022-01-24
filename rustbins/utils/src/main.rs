use clap::{Arg, App};
use std::collections::HashMap; //HashMap is a subset of standard collections
use std::env;
use std::io::prelude::*;
use std::io::{Read, Write, BufWriter};
use std::fs::File;

struct TestFile {
	filepath: String,
	pub data: String,
	pub filesize: usize,
}
// TestFile不pub是因为实际上进行io流的是hashmap,多个文件集成好后再io，所以只有hashmap的结构要暴露。
pub struct FileFlow {
	files: HashMap<String, TestFile>,
} 

pub fn write(path: &str, data: &str) -> std::io::Result<u32>{ // 返回状态码

	Ok(0)

}

fn main() -> std::io::Result<()>
{
	let mut buffer = File::open("footester.txt")?;
	let buffer_string: String;
	debug_assert_eq!(buffer.read_to_string(&buffer_string), "asdsssssasdas");
	//buffer.write_all(b"asdsssssasdas")?;
	//buffer.flush()?;
	Ok(())


    
}