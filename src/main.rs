fn main() {
    let mut list: Vec<String> = vec![];
    loop {
        println!("What would you like to do? \n1. Add \n2. Remove \n3. List");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "add" => add(&mut list),
            "remove" => delete(&mut list),
            "list" => printItems(&list),
            _ => println!("Do nothing"),
        };
    }
}

fn add(list: &mut Vec<String>) {
    let mut item = String::new();
    println!("Type an item to add...");
    std::io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");

    print!("{}[2J", 27 as char);
    list.push(item);

}

fn delete(list: &mut Vec<String>) {
    let mut item = String::new();
    println!("Type an item to delete");
    std::io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");

    print!("{}[2J", 27 as char);
    list.retain(|x| *x != item);
}

fn printItems(list: &Vec<String>) {
    print!("{}[2J", 27 as char);
    println!("Your To-Do List");
    let mut index = 1;
    for item in list{
        print!("{}. {}", index, item);
        index += 1;
    }
}
