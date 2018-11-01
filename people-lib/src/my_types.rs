extern crate mysql as my;
extern crate database_lib as dbl;
use my_types::dbl::interface::Entry;
use my_types::dbl::interface::Key;
use my_types::dbl::interface::Table;

#[derive(Debug)]
pub struct mysql_table{
	//names are based on the mysql names
	tb_name:String,
	db_name: String,
	key_name: String,
	pool:my::Pool, //The pool that the user is connected to at the time. Use open_mysql(...) to get a Pool
	field: Vec<String>, //List of the fields in the tables, excludes key field
	
}


impl <E:Entry+ 'static>Table<E> for mysql_table{
	// functions for insert and lookup
	// These functions insert/lookup from the mysql database, not a local table
	
	//Example of a working lookup, but with predifined stuct names
	/*
		.map(|result|{
		result.map(|x| x.unwrap()).map(|row|{
		let name = my::from_row(row);
		db{name:name}
		}).collect()}).unwrap();	
	*/
	//Defines what type the key is
	type Key = ();

	//Searches the tables for a key
	fn lookup(&self, key: Box<dyn Key<E>>) -> Option<E>{
		//Send "SELECT * WHERE key_name = Key
		
		let mut con = self.pool.get_conn().unwrap();
		let vec_result: Vec<Vec<my::Value>> = con.prep_exec("SELECT * WHERE ? = ?" ,(&self.key_name,&key.downcast_ref::<mysql_table_key>().unwrap().id))
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{
					//Iterates through each row
					let vec_data = my::Row::unwrap(row); //Returns a vector of my::Values for each row
					vec_data //The order of the vector depends on the order inside the mySQL database
					
					//Other option that returns a vector of strings
					//Entry::from_vec_string(this_result.into_iter().map(|value| String::from_value(value)).collect())
				}).collect()
			}).unwrap();
		let this_result =&vec_result[0]; //Saves the desired entry to a seperate vec
		let end_result = Entry::from_mysql(this_result);
		
		Some(end_result)
	}

	//Inserts a new row into the table
	//Check QueryResult.last_insert_id
	fn insert(&mut self, entry: E) -> Box<dyn Key<E>>{
		//Use auto increment
		//Send "INSERT E INTO tb_name
		//SEND "LAST_INSERT_ID()" to get key to send back
		//Don't forget to send "USE self.db_name"

		
		let mut values :String = String::new();
		//let mut data ="";
		let mut entry_string = entry.to_vec_string();
		let mut i=0;
		while i < self.field.len(){
			values = values.to_owned() + ", ?" ;
			//data   = &(data.to_owned()   + ", ?");
			i=i+1;
		}
		let cmd = &("INSERT INTO ".to_string() + &self.tb_name +
			" (" + &self.key_name + &values + 
			") VALUES (NULL" + &values + ")");
		
		println!("{}",cmd);
		let mut con = self.pool.get_conn().unwrap();
		con.prep_exec(cmd,self.field.append(&mut entry_string)).unwrap();
		//Get last entry in that table
		let this_key: Vec<mysql_table_key> = con.prep_exec("SELECT LAST_INSERT_ID()",())
			.map(|result|{
				result.map(|x| x.unwrap()).map(|row|{
				let id = my::from_row(row);
				mysql_table_key{id:id}
				}).collect()
			}).unwrap();
			
		Box::new(mysql_table_key {
            id: this_key[0].id
        })
			
			

	}
}

#[derive(Debug)]
pub struct mysql_table_key{
	id: usize
}

impl <E:Entry + 'static> Key<E> for mysql_table_key{
	fn same_as(&self, other: Box<dyn Key<E>>) -> bool {
		/*
        if let Some(other_key) = other.downcast_ref::<mysql_table_key>() {
            self.id == other_key.id
        } else {
            false
        }
		*/
		true
    }
}
