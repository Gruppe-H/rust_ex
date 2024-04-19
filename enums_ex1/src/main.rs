// Define an enums 'TrafficLight' to represent the states of a traffic light
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // Method to return the time (in secounds) a light should be displayed based on its color
    fn display_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 5,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 5,
        }
    }

    // Function to simulate the changing of lights and print which light is on at any time
    fn simulate(&mut self) {
        println!("Traffic Light Simulation: ");
        loop {
            println!("{:?} light is on for {} secounds", self, self.display_time());
            std::thread::sleep(std::time::Duration::from_secs(self.display_time() as u64));
            *self = match *self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green => TrafficLight::Yellow,
            };
            }
        }
    }
     fn main() {
        let mut light = TrafficLight::Red;
        light.simulate();
     }
