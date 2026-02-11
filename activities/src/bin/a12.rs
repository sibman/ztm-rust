// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#[derive(Debug)]
enum BoxColor {
    Red,
    Green,
    Blue,
}
struct ShipingBox {
    color: BoxColor,
    dimensions: (u32, u32, u32),
    weight: u32,
}
impl ShipingBox {
    fn new(color: BoxColor, dimensions: (u32, u32, u32), weight: u32) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }
    fn print_characteristics(&self) {
        println!("Color: {:?}", self.color);
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {:?}", self.weight);
    }
}
fn main() {
    let box1 = ShipingBox::new(BoxColor::Red, (10, 20, 30), 5);
    box1.print_characteristics();
    let box2 = ShipingBox::new(BoxColor::Green, (20, 30, 40), 10);
    box2.print_characteristics();
    let box3 = ShipingBox::new(BoxColor::Blue, (30, 40, 50), 15);
    box3.print_characteristics();
}
