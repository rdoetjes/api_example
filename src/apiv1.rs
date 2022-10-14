use sqlite::State;
use crate::appconfig;


#[get("/v1/test/sayhi/<name>/<age>")]
pub fn test(name: String, age: u8) -> String{

    match age {
        age if age < 30 => format!("Hi {}, I see you are an inexperienced noob", name),
        age if age >=30 && age < 50  => format!("Hello {}, you are wise and ripe but not yet old", name),
        age if age >= 50 && age <80 => format!("Oh dear {}, you are considered an old man aka a boomer", name),
        age if age >= 80 => format!("Yoh people! {} here, is so old that he walked with the dinosaurs!", name),
        _ => "".to_string(),
    }
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

    #[test]
    fn test_sayhi(){
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

}