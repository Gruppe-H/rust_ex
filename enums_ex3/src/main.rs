// Define an enum `Color` to represent primary and secondary colors
#[derive(Debug)]
enum Color {
    Primary(PrimaryColor),
    Secondary(SecondaryColor),
}

// Implement Debug trait for PrimaryColor
impl std::fmt::Debug for PrimaryColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimaryColor::Red(_, _, _) => write!(f, "Red"),
            PrimaryColor::Blue(_, _, _) => write!(f, "Blue"),
            PrimaryColor::Yellow(_, _, _) => write!(f, "Yellow"),
        }
    }
}
// Define an enum `PrimaryColor` to represent primary colors (Red, Blue, Yellow)
enum PrimaryColor {
    Red(u8, u8, u8),    // RGB values for Red
    Blue(u8, u8, u8),   // RGB values for Blue
    Yellow(u8, u8, u8), // RGB values for Yellow
}

// Implement Debug trait for SecondaryColor
impl std::fmt::Debug for SecondaryColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecondaryColor::Orange(_, _, _) => write!(f, "Orange"),
            SecondaryColor::Purple(_, _, _) => write!(f, "Purple"),
            SecondaryColor::Green(_, _, _) => write!(f, "Green"),
        }
    }
}

// Define an enum `SecondaryColor` to represent secondary colors (Orange, Purple, Green)
enum SecondaryColor {
    Orange(u8, u8, u8),  // RGB values for Orange
    Purple(u8, u8, u8),  // RGB values for Purple
    Green(u8, u8, u8),   // RGB values for Green
}

impl Color {
    // Method to mix two primary colors and create a secondary color
    fn mix(primary1: PrimaryColor, primary2: PrimaryColor) -> Option<Color> {
        match (primary1, primary2) {
            // Mixing Red and Blue to create Purple
            (PrimaryColor::Red(r1, g1, b1), PrimaryColor::Blue(r2, g2, b2))
            | (PrimaryColor::Blue(r1, g1, b1), PrimaryColor::Red(r2, g2, b2)) => {
                Some(Color::Secondary(SecondaryColor::Purple(
                    (r1 + r2) / 2,
                    (g1 + g2) / 2,
                    (b1 + b2) / 2,
                )))
            }
            // Mixing Red and Yellow to create Orange
            (PrimaryColor::Red(r1, g1, b1), PrimaryColor::Yellow(r2, g2, b2))
            | (PrimaryColor::Yellow(r1, g1, b1), PrimaryColor::Red(r2, g2, b2)) => {
                Some(Color::Secondary(SecondaryColor::Orange(
                    (r1 + r2) / 2,
                    (g1 + g2) / 2,
                    (b1 + b2) / 2,
                )))
            }
            // Mixing Blue and Yellow to create Green
            (PrimaryColor::Blue(r1, g1, b1), PrimaryColor::Yellow(r2, g2, b2))
            | (PrimaryColor::Yellow(r1, g1, b1), PrimaryColor::Blue(r2, g2, b2)) => {
                Some(Color::Secondary(SecondaryColor::Green(
                    (r1 + r2) / 2,
                    (g1 + g2) / 2,
                    (b1 + b2) / 2,
                )))
            }
            _ => None, // No secondary color can be created with the given primary colors
        }
    }
}

fn main() {
    // Example: Mixing primary colors to create secondary colors
    let primary1 = PrimaryColor::Red(255, 0, 0);
    let primary2 = PrimaryColor::Blue(0, 0, 255);
    if let Some(secondary) = Color::mix(primary1, primary2) {
        match secondary {
            Color::Secondary(sec_color) => {
                match sec_color {
                    SecondaryColor::Purple(r, g, b) => {
                        println!("Mixed color: Purple (R: {}, G: {}, B: {})", r, g, b);
                    }
                    _ => println!("Mixed color: {:?}", sec_color),
                }
            }
            _ => println!("No secondary color can be created with the given primary colors"),
        }
    } else {
        println!("No secondary color can be created with the given primary colors");
    }
}
