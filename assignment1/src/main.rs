struct Sharman{
    name: String,
    health: String,
    experience: String,

}

struct SharmanCollection {
    sharmans: Vec<Sharman>,
}

impl SharmanCollection {
    fn new() -> Self {
        SharmanCollection {
            sharmans: Vec::new(),
        }
    }

    fn add_sharman(&mut self, sharman: Sharman) {
        self.sharmans.push(sharman);
    }

    fn find_sharman(&self, name: &str) -> Option<&Sharman> {
        self.sharmans.iter().find(|sharman| sharman.name == name)
    }
}

fn main() {

    let mut sharmen_Coll = SharmanCollection::new();

    // 1. Library Management System
    // implement a struct "sharmenColl" that holds a collection og Sharmans
    let shaman1 = Sharman{
        name: String::from("Brian Smith"),
        health: String::from("In good health"),
        experience: String::from("Experienced"),
    };
    sharmen_Coll.add_sharman(shaman1);

    let shaman2 = Sharman{
        name: String::from("John Doe"),
        health: String::from("In very good health"),
        experience: String::from("Unexperienced"),
    };
    sharmen_Coll.add_sharman(shaman2);

    let shaman3 = Sharman{
        name: String::from("Jane Doe"),
        health: String::from("In bad health"),
        experience: String::from("Proffesional"),
    };
    sharmen_Coll.add_sharman(shaman3);

    // create methods to add new sharmans to the collection and to find a sharman by name
    if let Some(found_sharman) = sharmen_Coll.find_sharman("John Doe") {
        println!("Found shaman: {}", found_sharman.name);
    } else {
        println!("Shaman not found");
    }


}