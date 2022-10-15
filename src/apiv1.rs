use serde::Deserialize;
use sqlite::State;
use crate::appconfig;
use rocket::serde::json::Json;

#[derive(Deserialize)]
pub struct User {
    name: String,
    function: String,
}

/*
The url is build up as /<version>/<suite>/<function>
therefore /v1/test/sayhi/<name>/<age> means version1, test suite and function sayhi
*/
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


#[post("/v1/test/create", format = "json", data = "<user>")]
pub fn create(user: Json<User>) -> String{
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

#[post("/v1/test/delete", format = "json", data = "<user>")]
pub fn delete(user: Json<User>) -> String{
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
        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/50").expect("Woops").text().unwrap();
        assert!(resp.contains("Oh dear Ray, you are considered an old man aka a boomer"));

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/49").expect("Woops").text().unwrap();
        assert!(resp.contains("Hello Ray, you are wise and ripe but not yet old"));

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/30").expect("Woops").text().unwrap();
        assert!(resp.contains("Hello Ray, you are wise and ripe but not yet old"));

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/29").expect("Woops").text().unwrap();
        assert!(resp.contains("Hi Ray, I see you are an inexperienced noob"));

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/79").expect("Woops").text().unwrap();
        assert!(resp.contains("Oh dear Ray, you are considered an old man aka a boomer"));

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/sayhi/Ray/80").expect("Woops").text().unwrap();
        assert!(resp.contains("dinosaur"));
    }

    #[test]
    fn test_query(){
        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/query/Raymond").expect("Woops").text().unwrap();
        assert!(resp.contains("Raymond"));
        assert!(resp.contains("Developer"));
        assert!(resp.contains("CEO"));

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/query/Rene").expect("Woops").text().unwrap();
        assert!(resp.contains("Rene"));
        assert!(resp.contains("Developer"));
        assert!(!resp.contains("CEO"));  

        let resp = reqwest::blocking::get("https://api.phonax.com:8000/api/v1/test/query/NONE_EXISTING").expect("Woops").text().unwrap();
        assert!(resp.contains("No records found"));
    }

    #[test]
    fn test_save(){
        let content: &str = "{ \"name\": \"test_suite\", \"function\": \"Developer\" }";

        let client = reqwest::blocking::Client::new();
        let t = client
            .post("https://api.phonax.com:8000/api/v1/test/create")
            .header("Content-Type", "application/json")
            .body(&*content)
            .send().unwrap().text().unwrap();
        assert!(t.contains("SUCCESS"));

        let t = client
            .post("https://api.phonax.com:8000/api/v1/test/delete")
            .header("Content-Type", "application/json")
            .body(&*content)
            .send().unwrap().text().unwrap();
        assert!(t.contains("SUCCESS"));
    }

}