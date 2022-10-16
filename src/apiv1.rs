use rocket::serde::{Deserialize, Serialize};
use sqlite::State;
use crate::appconfig;
use rocket::serde::json::Json;


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    name: String,
    function: String,
}
impl Default for User {
    fn default () -> User {
        User{
            name: "".to_owned(),
            function: "".to_owned(),
        }
    }
}


/// sayhi gives back a demotivational quote unless your age is between 30 and 49
///
/// sayhi, is there to (de)motivate you, what is life without a good quote???
/// It returns a string based on age ranges
///
/// Usage:  
/// ```rust
/// println!(sayhi("Ray", 49));
/// ```
/// 
#[get("/v1/test/sayhi/<name>/<age>")]
pub fn sayhi(name: String, age: u8) -> String{
    match age {
        age if age < 30 => format!("Hi {}, I see you are an inexperienced noob", name),
        age if age >=30 && age < 50  => format!("Hello {}, you are wise and ripe but not yet old", name),
        age if age >= 50 && age <80 => format!("Oh dear {}, you are considered an old man aka a boomer", name),
        age if age >= 80 => format!("Yoh people! {} here, is so old that he walked with the dinosaurs!", name),
        _ => "".to_string(),
    }
}

/// create saves user into database
///
/// create takes a User struct and saves it to the database
/// It returns "SUCCESS" or an "Error message"
///
/// Usage:  
/// ```rust
/// create(user)
/// ```
/// 
pub fn create(user: User) -> String{
    let conn =  sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists

    let result: String = "SUCCESS".to_string();
    let _statement = match conn.execute(format!("INSERT INTO test values ('{}', '{}')", &*user.name, &*user.function) ){
        Ok(statement) => statement,
        Err(e) => { 
            return format!("Problem running query: {:?}", e)
        },
    };

    result
}

/// Converts a Json&ltUser^gt object into a User struct
///
/// You do not want to deal with Json&ltUser&gt objects in your logic but with just the stand structures
/// this makes code 
/// - more readable
/// - avoids code duplication, as you do this for every crud method
/// - allows abstraction between web objects and final models
/// 
/// Usage:  
/// ```rust
/// create(fill_user_with_userjson(&user))
/// ```
/// 
fn fill_user_with_userjson(user: &Json<User>) -> User {
    let mut t = User::default();
    t.name = user.name.to_owned();
    t.function = user.function.to_owned();
    t
}

/// web_create eventually saves a Json&ltuser&gt into database using create()
///
/// web_create takes in a serialized Json&ltUser&gt object than converts it to a User struct and saves it to the database
/// It returns "SUCCESS" or an "Error message"
///
/// Usage:  
/// ```rust
/// web_create(user)
/// ```
/// 
#[post("/v1/test/create", format = "json", data = "<user>")]
pub fn web_create(user: Json<User>) -> String{
    create(fill_user_with_userjson(&user))
}


/// delete(user: User) deletes macthing name records from the database
///
/// delete takes a User struct and deletes the matching record(s) from the database matching all the attributes in the user struct
/// It returns "SUCCESS" or an "Error message"
///
/// Usage:  
/// ```rust
/// delete(user)
/// ```
/// 
pub fn delete(user: User) -> String {
    let conn =  sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists

    let result: String = "SUCCESS".to_string();
    let _statement = match conn.execute(format!("DELETE FROM test where name='{}' and desc='{}'", &*user.name, &*user.function) ){
        Ok(statement) => statement,
        Err(e) => { 
            return format!("Problem running query: {:?}", e)
        },
    };
    result
}

/// web_delete eventually saves user into database using create()
///
/// web_delete takes in a serialized Json&ltUser&gt object than converts it to a User struct and deletes the record(s) that 
/// match all the criteria of the User struct
/// It returns "SUCCESS" or an "Error message"
///
/// Usage:  
/// ```rust
///web_delete(user)
/// ```
/// 
#[post("/v1/test/delete", format = "json", data = "<user>")]
pub fn web_delete(user: Json<User>) -> String{
    delete(fill_user_with_userjson(&user))
}

/// query(name: String) searches for a name in the database and returns those records in string format
///
/// query(name: String) searches for all records where column value 'name' is equal to name 
/// and returns all the records in string format that looks like:
/// Name: Raymond Description: CEO
/// Name: Raymond Description: Developer
///
/// Usage: 
/// ```rust
/// query("Raymond".to_string())
/// ```
#[get("/v1/test/query/<name>")]
pub fn query(name: String) -> String {
    appconfig::check_dbfile(appconfig::DATABASE_FILE);

    let conn =  sqlite::open(appconfig::DATABASE_FILE).expect("Database not readable!"); //we can unwrap we checked the file exists

    let mut result: String = "".to_string();

    let statement = match conn.prepare("SELECT * FROM test where name = ?1") {
        Ok(statement) => statement,
        Err(e) => { 
            return format!("Problem running query: {:?}", e)
        },
    };
    
    let mut t = match statement.bind(1, &*name) {
        Ok(statement) => statement,
        Err(e) => { 
            return format!("Problem binding params: {:?}", e)
        },
    };

    while let State::Row = t.next().unwrap() {
        result += "Name: ";
        result += t.read::<String>(0).unwrap().as_str();
        result += " ";
        result += "Description: ";
        result += t.read::<String>(1).unwrap().as_str();
        result += "\r\n";
    }

    if result == "" {
        result += "No records found";
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sayhi(){
        //unit test using the pub funciton without the https layer
        let s = sayhi("Ray".to_string(), 49);
        assert!(s.contains("but not yet old"));

        /*
        These are technically integration or system tests as they actually connect to the service that needs to run.
        In order for these tests to succeed with the self signed certificate with the cn=api.phonax.com, I created a hosts file entry
        in /etc/hosts 
        127.0.0.1	api.phonax.com
        Without this you would get a Warning: tls handshake with 127.0.0.1:xxxxxx failed: tls handshake eof error because 
        Reqwest at standard validates the hostname (as it should!!!)

        Deployment of certificates are easily doable in CD part where we can put down the generated certificate and key
        in the predefined locations as described in the Rocket.toml and these can (and should) only be readable by the NPA user
        that runs the service.
        And this all can easily be part of the Docker creation process.
        */ 

        //integration tests
        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/50").expect("Woops").text().unwrap();
        // assert!(resp.contains("Oh dear Ray, you are considered an old man aka a boomer"));

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/49").expect("Woops").text().unwrap();
        // assert!(resp.contains("Hello Ray, you are wise and ripe but not yet old"));

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/30").expect("Woops").text().unwrap();
        // assert!(resp.contains("Hello Ray, you are wise and ripe but not yet old"));

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/29").expect("Woops").text().unwrap();
        // assert!(resp.contains("Hi Ray, I see you are an inexperienced noob"));

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/79").expect("Woops").text().unwrap();
        // assert!(resp.contains("Oh dear Ray, you are considered an old man aka a boomer"));

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/80").expect("Woops").text().unwrap();
        // assert!(resp.contains("dinosaur"));
    }

    #[test]
    fn test_query(){
        let s = query("Raymond".to_owned());
        assert!(s.contains("Raymond"));
        assert!(s.contains("Developer"));
        assert!(s.contains("CEO"));

        /*
        These are technically integration or system tests as they actually connect to the service that needs to run.
        In order for these tests to succeed with the self signed certificate with the cn=api.phonax.com, I created a hosts file entry
        in /etc/hosts 
        127.0.0.1	api.phonax.com
        Without this you would get a Warning: tls handshake with 127.0.0.1:xxxxxx failed: tls handshake eof error because 
        Reqwest at standard validates the hostname (as it should!!!)

        Deployment of certificates are easily doable in CD part where we can put down the generated certificate and key
        in the predefined locations as described in the Rocket.toml and these can (and should) only be readable by the NPA user
        that runs the service.
        And this all can easily be part of the Docker creation process.
        */ 

        //integration tests
        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/query/Raymond").expect("Woops").text().unwrap();
        // assert!(resp.contains("Raymond"));
        // assert!(resp.contains("Developer"));
        // assert!(resp.contains("CEO"));

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/query/Rene").expect("Woops").text().unwrap();
        // assert!(resp.contains("Rene"));
        // assert!(resp.contains("Developer"));
        // assert!(!resp.contains("CEO"));  

        // let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/query/NONE_EXISTING").expect("Woops").text().unwrap();
        // assert!(resp.contains("No records found"));
    }

    #[test]
    fn test_save(){

        let user = User{
            name: "test_suite".to_owned(),
            function: "Developer".to_owned(),

        };
        
        let t = create(user);
        assert!(t.contains("SUCCESS"));

        let user = User{
            name: "test_suite".to_owned(),
            function: "Developer".to_owned(),

        };
        let t = delete(user);
        println!("{}", t);
        assert!(t.contains("SUCCESS"));

        /*
        These are technically integration or system tests as they actually connect to the service that needs to run.
        In order for these tests to succeed with the self signed certificate with the cn=api.phonax.com, I created a hosts file entry
        in /etc/hosts 
        127.0.0.1	api.phonax.com
        Without this you would get a Warning: tls handshake with 127.0.0.1:xxxxxx failed: tls handshake eof error because 
        Reqwest at standard validates the hostname (as it should!!!)

        Deployment of certificates are easily doable in CD part where we can put down the generated certificate and key
        in the predefined locations as described in the Rocket.toml and these can (and should) only be readable by the NPA user
        that runs the service.
        And this all can easily be part of the Docker creation process.
        */ 

        // //integration tests
        // let content: &str = "{ \"name\": \"test_suite\", \"function\": \"Developer\" }";

        // let client = reqwest::blocking::Client::new();
        // let t = client
        //     .post("https://api.phonax.com:8000/api/v1/test/create")
        //     .header("Content-Type", "application/json")
        //     .body(&*content)
        //     .send().unwrap().text().unwrap();
        // assert!(t.contains("SUCCESS"));

        // let t = client
        //     .post("https://api.phonax.com:8000/api/v1/test/delete")
        //     .header("Content-Type", "application/json")
        //     .body(&*content)
        //     .send().unwrap().text().unwrap();
        // assert!(t.contains("SUCCESS"));
    }

}