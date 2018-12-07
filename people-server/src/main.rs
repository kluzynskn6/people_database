#[macro_use]
extern crate rouille;
extern crate people_lib;
extern crate database_lib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rpassword;

use database_lib::my_types;
use database_lib::interface::Table;
use people_lib::user;
use rouille::Request;
use rouille::Response;
use std::io;
use std::marker::PhantomData;


#[derive(Deserialize)]
struct UpdateJson {
	key:i32,
	first_name: String,
	last_name: String,
	email: String,	
	banner_id: i32,
}
#[derive(Deserialize)]
struct GetJson {
	key:i32
}
#[derive(Deserialize)]
struct InsertJson {
	first_name: String,
	last_name: 	String,
	email: 		String,	
	banner_id: 	i32,
	
}
#[derive(Deserialize)]
struct RemoveJson {
	key:i32	
}


fn main() {
	//Temporary code to make it compile. Replace later
	println!("enter username: ");
	let mut user = String::new();
	io::stdin().read_line(&mut user).expect("Failed to read line");
	user = user.trim().to_string();
	println!("{}'s password: ",user);
	let pass = rpassword::read_password().unwrap().trim().to_string();
	let pool = my_types::open_mysql(user,pass).unwrap();//Open mySQL
	
	let user_table: my_types::MysqlTable<user::User>= my_types::MysqlTable {
		tb_name: "User".to_string(),
		db_name: "dbTest".to_string(),
		key_name: "userID".to_string(),
		pool:pool, 
		phantom: PhantomData,
	};	
	//End of temporary code
    rouille::start_server("localhost:8000", move |request| {
        handler(request, &user_table)
    });
}

fn handler(request: &Request, my_table: &my_types::MysqlTable<user::User>) -> Response {
    router!(request,
            (POST) ["/api/v1/users/update"] => {
				//Update existing user
				//Needs: JSON with key and user data
				//Returns: Success or Fail as plain text
                let buf = update_user(my_table, request);
				match &buf{
					Ok(_) => Response::text("Successfully updated user"),
					Err(_) => Response::text("Failed to update user")
				}
            },
            (GET) ["/api/v1/users"] => {
				//Get one user with key or all users (preffereably with limit, although not yet implemented)
				//Needs: JSON with Key (0 for all), limit (optional, not yet implemented)
				//Returns: JSON file with all of the user data
				
				//Currently can only return one user until the query builder is created
                let buf = get_user(my_table, request);
				match &buf{
					Ok(_) => Response::json(&buf.unwrap()),//unwraps after checking it's okay
					Err(_) => Response::text("Failed to get user")
				}					
            },
            (GET) ["/api/v1/users/create"] => {
				//Creates a new user in the database
				//Needs: JSON with user data
				//Returns: JSON with the key for that user
                let buf = create_user(my_table, request);
				match &buf{
					Ok(_) => Response::json(&buf.unwrap()),//unwraps after checking it's okay
					Err(_) => Response::text("Failed to create user from given data")
				}	
            },
            (GET) ["/api/v1/users/delete"] => {
				//Delete existing user from the database
				//Needs: JSON with key
				//Returns: Success or Fail as plain text
                Response::text("501: Not implemented").with_status_code(501)
            },
            _ => {
                Response::text("404: Not found").with_status_code(404)
            }
           )
}
fn update_user(user_table: &my_types::MysqlTable<user::User>, req: &Request) -> Result<(),String>{

	//Get body from request as a struct from JSON
	
	let data_test: Result<UpdateJson, rouille::input::json::JsonError> = rouille::input::json_input(&req);
	let data : UpdateJson;
	match &data_test{
		Ok(_) =>data = data_test.unwrap(),
		Err(_) => return Err("Could not parse JSON".to_string()),
	}
	//Create a user based on data
	let new_values: user::User = user::User{
		first_name: data.first_name,
		last_name : data.last_name,
		email : data.email,
		banner_id : data.banner_id.to_string(),	
	};
	let key = my_types::MysqlTableKey{
		id: data.key,
		valid : true,
	};
	
	let buf = user_table.update(key,new_values);
	match &buf {
		Ok(_) => Ok(()),
		Err(string) => Err(string.to_string()),
	}
}
fn get_user(user_table: &my_types::MysqlTable<user::User>, req: &Request) -> Result<Vec<user::User>,String>{
	//Get the data from the JSON included in the body of the request
	let data_test: Result<GetJson, rouille::input::json::JsonError> = rouille::input::json_input(&req);
	let data : GetJson;
	match &data_test{
		Ok(_) =>data = data_test.unwrap(),
		Err(_) => return Err("Could not parse JSON".to_string()),
	}
	if data.key == 0{
		//Get all, will be implemented with query
		Err("Not implemented yet!".to_string())
	}else{
		//Get the user with the key
		let my_key = my_types::MysqlTableKey{
			id:data.key,
			valid:true,
		};
		let this_user = user_table.lookup(my_key);
		match this_user {
			Some(_) => Ok(vec![this_user.unwrap()]), //unwraps after checking it's okay
			None => Err("No user assoiated with the provided key".to_string()),
}}}

fn create_user(user_table: &my_types::MysqlTable<user::User>, req: &Request) -> Result<my_types::MysqlTableKey,String>{
	//Get data from the JSON in the request
	let data_test: Result<InsertJson, rouille::input::json::JsonError> = rouille::input::json_input(&req);
	let data : InsertJson;
	match &data_test{
		Ok(_) =>data = data_test.unwrap(),
		Err(_) => return Err("Could not parse JSON".to_string()),
	}
	let this_user = user::User{
		first_name:	data.first_name,
	    last_name:	data.last_name,
	    email:		data.email,		
        banner_id:	data.banner_id.to_string(),
	};
	let buf = user_table.insert(this_user);
	
	match &buf.valid{
		true => Ok(buf),
		false => Err("Could not insert user with the given data".to_string())
	}	
}







