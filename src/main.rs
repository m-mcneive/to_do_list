fn main() {
    let mut list: Vec<String> = vec![];
    loop {
        println!("What would you like to do? \nAdd \nRemove \nList");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "add" => add(&mut list),
            "delete" => delete(&mut list),
            "list" => printItems(&list),
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

fn delete(list: &mut Vec<String>) {
    let mut item = String::new();

    std::io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");

    list.retain(|x| *x != item);
}

fn printItems(list: &Vec<String>) {
    let mut index = 1;
    for item in list{
        print!("{}. {}", index, item);
        index += 1;
    }
}
