extern crate mysql_async as my;
extern crate rouille;
extern crate tokio;
extern crate futures;

use futures::Future;
use my::prelude::*;

#[macro_use]
#[derive(Debug)]

struct student {
    id:u32,
    name:String,
    Year:u32,
}

pub fn run<F,T,U>(future:F) -> Result<T,U>
where
	F:Future<Item = T, Error =U>+Send+'static,
	T:Send+'static,
	U:Send+'static,
{
	let mut runtime = tokio::runtime::Runtime::new().unwrap();
	let result = runtime.block_on(future);
	runtime.shutdown_on_idle().wait().unwrap();
	result
}

fn main (){

let mut  optbuild = my::OptsBuilder::new();

optbuild.ip_or_hostname("localhost")
	.user(Some("kluzynick"))
	.pass(Some("kluzypass"))
	.db_name(Some("People"));

let mut optcon = optbuild;
let pool = my::Pool::new(optcon);
let future = pool.get_conn().and_then(|conn|{

conn.prep_exec("SELECT * FROM Students", ())
}).and_then(|result|{
    result.map_and_drop(|row|{
      let (id, name, Year) = my::from_row(row);
      student {id:id,name:name,Year:Year,}})
}).and_then(|(_,student)|{
    pool.disconnect().map(|_| student)});

let selected_students = run(future).unwrap();

println!("Student 1 name:  {}",selected_students[0].name);

}
