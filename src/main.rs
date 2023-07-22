fn main() {
    let mut list: Vec<String> = vec![];
    loop {
        println!("What would you like to do? \nAdd \nRemove \nCheck");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "add" => add(&mut list),
            "delete" => println!("Can't delete yet"),
            "check" => println!("Can't check yet"),
            _ => println!("Do nothing"),
        };
    }
}

fn add(list: &mut Vec<String>) {
    let mut item = String::new();

    std::io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");

    list.push(item);

}
