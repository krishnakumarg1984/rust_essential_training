/* YOUR CODE GOES HERE */
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // latitude, longitude
}

impl Location {
    fn display(&self) {
        match self {
            Self::Unknown => println!("Location is unknown"),
            Self::Anonymous => println!("Location is anonymous"),
            Self::Known(lat, lon) => println!("Location is ({lat},{lon})"),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608_295, -80.604_177);
    address.display();
}
