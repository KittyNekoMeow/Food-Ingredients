use::std::io;

//Matching food to ingredients
fn food_match(food: &str) -> Vec<&str> {
    match food {
        "cake" => ["milk", "egg", "flour"].to_vec(),
        _ => ["Didn't put in allowed food"].to_vec()
    }
}
//TODO: Change .to_vec to somthing prettier

fn main() {

    println!("Check the ingredients need for cake");

    //Input is a string
    let mut input = String::new();

    //Get's input
    io::stdin().read_line(&mut input).unwrap();

    //Cleaning input
    let check_food1: String = input.trim().parse().unwrap();

    //Converting input to str
    let check_food2 = check_food1.as_str();

    //Calling the ingredients for food
    let ingredients = food_match(check_food2);

    //Printing ingredients
    println!("The ingredients for {} ", input);
    print!("are {:?}", ingredients);

}