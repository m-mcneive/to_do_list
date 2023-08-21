use to_do_list::{Item, List};
fn main() {
    let a = Item::new(String::from("Apple"), false);
    let b = Item::new(String::from("Banana"), false);
    let mut l = List::new();
    l.add(a);
    l.add(b);
    l.printlist();
    // let mut list: Vec<String> = vec![];
    // loop {
    //     println!("What would you like to do? \n1. Add \n2. Remove \n3. List");

    //     let mut input = String::new();

    //     std::io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");

    //     match input.trim().to_lowercase().as_str() {
    //         "add" => add(&mut list),
    //         "remove" => delete(&mut list),
    //         "list" => print_items(&list),
    //         _ => println!("Do nothing"),
    //     };
    // }
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
    print_items(list);
    println!("Type the number of the item you want to delete");
    std::io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");
    let i: i32 = item.trim()
        .parse()
        .expect("Input not an integer");
    print!("{}[2J", 27 as char);
    list.remove((i - 1) as usize);
}

fn print_items(list: &Vec<String>) {
    print!("{}[2J", 27 as char);
    println!("Your To-Do List");
    let mut index = 1;
    for item in list{
        print!("{}. {}", index, item);
        index += 1;
    }
}
