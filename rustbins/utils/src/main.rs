use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::DemoFile;

fn main() {

	//TODO 从命令行获取文件参数
	let path = "asd".to_owned();
	let contents = "written words".to_owned();
	let testDemoFile: DemoFile = DemoFile::new(
		&path.clone(), &contents.clone());

	}