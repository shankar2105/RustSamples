extern crate sqlite3;
pub use sqlite3::*;
//pub use sqlite3::cursor::*;
//use std::collections::hash::map::HashMap;
static STR: &'static str = "Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a
galley of type and scrambled it to make a type specimen book.";

fn prepare_vec_u8 (arr: &[u8]) -> Vec<u8> {
	let mut vec = Vec::new();
	for i in 0..arr.len() {
		vec.push(arr[i]);
	}
	vec
}
fn main() {
//	let str: String = "Maidsafe Sample Text".to_string();
	let mut sql = sqlite3::open("./db").unwrap();
	sql.exec("create table if not exists stud (blob varchar(255))").unwrap();
	let mut query = sql.prepare("insert into stud(blob) values(?)", &None).unwrap();
	query.bind_params(&[types::BindArg::Blob(prepare_vec_u8(&STR.as_bytes()))]);
	query.step();
//	sql.exec("insert into stud values('2', 'adam')").unwrap();
//	sql.exec("insert into stud values('3', 'smith')").unwrap();
	let mut result = sql.prepare("select * from stud", &None).unwrap();
//	result.clear_bindings();
//	let mut local: &str = "";
//	let mut r = result.step_row().unwrap().unwrap();
//	match r.get("blob").unwrap() {
//			&types::BindArg::Integer(ref p) => println!("Data :: {:?}", p),
//			_ => println!("Don't have Daniel's number."),
//		}
	loop {
		println!("------------------------");
		if result.step() == types::ResultCode::SQLITE_DONE{
			break;
		}
		println!("Result :: {:?}", result.get_blob(0).unwrap());
	}
}
