// Define a struct `Player` to represent a handball player
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)] // Implement necessary traits for comparison and printing
struct Player {
    name: String,      // Name of the player
    position: String,  // Position of the player
    shirt_number: u32, // Shirt number of the player
}

impl Player {
    // Constructor method to create a new player instance
    fn new(name: String, position: String, shirt_number: u32) -> Self {
        Player { name, position, shirt_number }
    }
}

// Define a struct `Team` to represent a handball team
struct Team {
    players: Vec<Player>, // A vector of players in the team

}

impl Team{
    // Constructor method to create a new team instance
    fn new() -> Self {
        Team {
            players: Vec::new(),
        }
    }

    // Method to add a player to the team
    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    // Method to find a player by name
    fn find_player_by_name(&self, name: &str) -> Option<&Player> {
        self.players.iter().find(|player| player.name == name)
    }

    // Method to find a player by shirt number
    fn find_player_by_shirt_number(&self, shirt_number: u32) -> Option<&Player> {
        self.players.iter().find(|player| player.shirt_number == shirt_number)
    }

    // Method to sort the players in the team (based on their names)
    fn sort_players(&mut self) {
        self.players.sort();
    }

}

fn main() {
    let mut team = Team::new(); // Create a new team instance

    //create some player instances and add them to the team
    let player1 = Player::new(String::from("John Doe"), String::from("Goalkeeper"), 1);
    let player2 = Player::new(String::from("Jane Doe"), String::from("Left Wing"), 2);
    let player3 = Player::new(String::from("Brian Smith"), String::from("Right Wing"), 3);

    team.add_player(player1);
    team.add_player(player2);
    team.add_player(player3);

    //print the teams players before sorting
    println!("Team players: {:?}", team.players);

    //Find a player by name and print result
    if let Some(player) = team.find_player_by_name("John Doe") {
        println!("Found player by name: {:?}", player);
    } else {
        println!("Player not found");
    }

    // Find a player by shirt number and print result
    if let Some(player) = team.find_player_by_shirt_number(2) {
        println!("Found player by shirt number: {:?}", player);
    } else {
        println!("Player not found");
    }

    // Sort the players in the team and print the result
    team.sort_players();
    println!("Team players after sorting: {:?}", team.players);
}