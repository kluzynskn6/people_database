extern crate mysql as my;
extern crate rouille;


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

}
