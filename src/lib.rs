
pub struct Item {
    pub name: String,
    pub done: bool,
}

pub struct List {
    pub list: Vec<Item>,
}

impl Item {
    pub fn new(name: String, done: bool) -> Item {
        Item{ name, done } 
    }
    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

impl List {
    pub fn new() -> List{
        let list: Vec<Item> = Vec::new();
        List { list }
        
    }
    pub fn add(&mut self, item: Item) {
        self.list.push(item);
    }

    pub fn get_items<'a>(&self) -> &Vec<Item> {
        &self.list
    }
    pub fn printlist(&self) {
        let mut count = 1;
        for entry in &self.list {
            println!("{}. {}", count, entry.name);
            count += 1;
        }
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        let a = Item::new(String::from("apple"), false);
        let b = Item::new(String::from("banana"), false);
        let mut l = List::new();
        l.add(a);
        l.add(b);

    }
}