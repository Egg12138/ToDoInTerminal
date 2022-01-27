//#![allow(dead_code, unused_imports)] //直接砍掉整个程序的警告
#![cfg_attr(debug_assertions, 
	allow(
		dead_code, 
		unused_imports, 
		unused_variables, 
		unused_mut
	))] //仅开发阶段禁用warnings
use std::fs::{File, write, read};
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::str::{from_utf8};
#[derive(Debug)]
pub struct DemoFile {
	pub path: String,
	pub contents: String,
	pub sizeof: usize,
}
// TODO文件大小
impl DemoFile {
	pub fn new(p: &str, buf: &str) -> DemoFile {
		// 最终决定在new这里就先检查文件的有效性
		let file = File::open(&p).unwrap_or_else(| err | {
			if err.kind() == ErrorKind::NotFound {
				println!("Not such a file or directory: {}", &p);
				if true {
				File::create(&p).unwrap_or_else( | err | {
					panic!("[{:?}]\n Failed to create the file : {}.", err, &p);
				})
				} else {
					panic!("Cannot find file\n[{:?}]\n EXIT[255]", err);
				}
			} else {
				panic!("[{:?}]\n Failed to open the file : {}.\n", err, &p);
			}
		});
		DemoFile {
			path: String::from(p),
			contents: String::from(buf),
			sizeof: 128,
		}	
	}

}

pub trait BasicTrait { 
	fn open_self(&self) -> Result<(), Error>;
	fn read_all(&self) -> Result<String, Error>;
	fn write_one(&mut self, contents: &str) -> Result<usize, Error>;
	fn write_all_contents(&mut self, contents: &str) -> Result<(), Error>;
}

impl BasicTrait for DemoFile {
	fn open_self(&self) ->  Result<(), Error> {
		let f = File::open(&self.path).unwrap_or_else(|err| {
			if err.kind() == ErrorKind::NotFound {
				println!("No such file or directory: {}", &self.path);
				if true {
				File::create(&self.path).unwrap_or_else(|err| {
					panic!("[{:?}]\n Failed to create the file {}", err, &self.path);
				})
			} else {
				panic!("Cannot find file\n[{:?}]\n EXIT[255]", err);
			}
			} else {
				panic!("[{:?}]\n Failed to open the file {}", err, &self.path);
			}
		});
		//OpenOptions::new().read(true).open(self.path.as_ref())
		//println!("Opened {:?}", f)
		Ok(())
	}

	fn read_all(&self) -> Result<String, Error> {
		let read_buf = read(&self.path).unwrap();//Return Result<Vec<u8>>
		let read_buf_string: String = String::from_utf8(read_buf).unwrap();
		println!("read: \n {:?}", read_buf_string);
		Ok(read_buf_string)
	}
	fn write_one(&mut self, contents: &str) -> Result<usize, Error> {
		println!("String is written");
		Ok(1)
	}

	fn write_all_contents(&mut self, mut contents: &str) -> Result<(), Error> {
		while !contents.is_empty() {
			match self.write_one(contents) {
				Ok(0) => return Err(Error::new(ErrorKind::WriteZero,
											"Writing into file failed. [Empty Buffer]")),	
				Ok(other_status) => contents = &contents[other_status..],
				Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
				Err(e) => return Err(e),
				
			}
		}
		Ok(())
	}
}  


pub struct FilesFlow {
	pub filesizez: usize,
}


#[cfg(test)]
mod test {
	#[test]
	fn demofile() {
		use crate::DemoFile;
		let path: &str = "D:/sourcecodes/c/automation/db.txt";
		let buffer: &str = "This message is sent from a unit test\n good luck!";
		let TestFileStruct: DemoFile = DemoFile::new(path, buffer);


}
}