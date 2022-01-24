use clap::{Arg, App};
use std::env;
use std::io::{Read, Write};

struct TestFile {
	filepath: String,
	pub data: String,
	pub filesize: usize,
}
// TestFile不pub是因为实际上进行io流的是hashmap,多个文件集成好后再io，所以只有hashmap的结构要暴露。
pub struct FileFlow {
	files: HashMap<String, TestFile>,
} 

fn main() 
{



    
}