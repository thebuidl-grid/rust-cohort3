// tuple struct

#[derive(Debug)]
struct TupleStruct(i32, String, &'static str, u8); // custom data type of tuple struct

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
    id: u32,
    address: String,
    gender: Gender,
}

#[derive(Debug, Clone)]
enum Gender {
    Male,
    Female,
    NotToSay,
    Others(String),
}

impl Gender{
    fn gender(&self)->String{
        match self {
            Gender::Male =>{
                String::from("he is a male child")
            }
            Gender::Female => {
                String::from("shes a princess")
            }
            Gender::NotToSay =>{
                String::from("confuse human")
            }
            Gender::Others(x)=>{
                let m = format!("hmmmmm!!!! {}",x);
                m
            }
        }
    }
}


impl User {
    fn new(name: String, age: u8, address: String, id: u32, gender: Gender) -> Self {
        let user = User {
            name,
            address,
            age,
            id,
            gender,
        };

        user
    }

    fn debug(&self) {
        println!("user: {:?}", self);
    }

    fn mut_name(& mut self,name: String){
        self.name = name;
        // self.address = "".to_string();
        // self.age = 55
    }
}

pub fn group() {
    let string = String::from("ruuyrururruu");

    let _data = TupleStruct(6, "tuesday".to_string(), "sample", 8);
    let mut user1 = User::new(
        "uche".to_string(),
        44,
        "The buidl".to_string(),
        54,
        Gender::Male,
    );
    let user2 = User::new(
        "uche".to_string(),
        44,
        "The buidl".to_string(),
        54,
        Gender::Male,
    );
    let user3 = User::new(
        "uche".to_string(),
        44,
        "The buidl".to_string(),
        54,
        Gender::Male,
    );
    let user4 = User::new(
        "uche".to_string(),
        44,
        "The buidl".to_string(),
        54,
        Gender::Male,
    );

    user1.debug();
    user1.mut_name("john".to_string());
    user1.debug();

    let m = Gender::Male.gender();

    println!("{}",m);
    // let gender = Gender::NotToSay(false);
}






