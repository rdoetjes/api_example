use sqlite::State;

#[get("/sayhi/<name>/<age>")]
pub fn test(name: String, age: u8) -> String{

    match age {
        age if age < 30 => format!("Hi {}, I see you are an inexperienced noob", name),
        age if age >=30 && age < 50  => format!("Hello {}, you are wise and ripe and not yet old", name),
        age if age >= 50 && age <80 => format!("Oh dear {}, you are considered an old man aka a boomer", name),
        age if age >= 80 => format!("Yoh people! {} here, is so old that he walked with the dinosaurs!", name),
        _ => "".to_string(),
    }
}

#[get("/query/<name>")]
pub fn query(name: String) -> String{
    let conn = sqlite::open("./test.db").expect("Whoops database not found");
    let mut result: String = "".to_string();

    let mut statement = conn.prepare("SELECT * FROM test where name = ?1").unwrap().bind(1, &*name).unwrap();
    while let State::Row = statement.next().unwrap() {
        result += "Name: ";
        result += statement.read::<String>(0).unwrap().as_str();
        result += " ";
        result += "Description: ";
        result += statement.read::<String>(1).unwrap().as_str();
        result += "\r\n";
    }
    
    result
}
