extern crate mysql as my;
extern crate rouille;
use std::io;

struct db {
    name:String,
}
struct tb {
    name:String,
}

struct student {
    id:u32,
    name:String,
    Year:u32,
}

fn main (){

let mut  optbuild = my::OptsBuilder::new();

optbuild.ip_or_hostname(Some("localhost"))
	.user(Some("kluzynick"))
	.pass(Some("kluzypass"));

let mut optcon = optbuild;
let pool = my::Pool::new(optcon).unwrap();

let databases: Vec<db> = pool.prep_exec("SHOW DATABASES",())
	.map(|result|{
		result.map(|x| x.unwrap()).map(|row|{
		let name = my::from_row(row);
		db{name:name}
		}).collect()}).unwrap();


let num_db = databases.len();
let mut i=0;
let mut db_sel=String::new();

println!("Please select a database by entering the corresponding number");
while i<num_db {
	println!(r"{})  {}",i,databases[i].name);
	i=i+1;
}

io::stdin().read_line(&mut db_sel)
	.expect("Failed to read line");
let db_sel:usize = db_sel.trim().parse()
	.expect("Please type a number");
println!("You selected the {} database, now select a table",databases[db_sel].name);

let cmd = "USE ".to_owned() + &databases[db_sel].name;
let mut con = pool.get_conn().unwrap();
con.query(cmd).unwrap();

let tables: Vec<tb> = con.prep_exec("SHOW TABLES",())
	.map(|result|{
		result.map(|x| x.unwrap()).map(|row|{
		let name = my::from_row(row);
		tb{name:name}
		}).collect()}).unwrap();

let num_tb = tables.len();
let mut i=0;
let mut tb_sel=String::new();

println!("Please select a table by entering the corresponding number");
while i<num_tb {
	println!(r"{})  {}",i,tables[i].name);
	i=i+1;
}

io::stdin().read_line(&mut tb_sel)
	.expect("Failed to read line");
let tb_sel:usize = tb_sel.trim().parse()
	.expect("Please type a number");
println!("You selected the {} table",tables[tb_sel].name);
		
		
}
