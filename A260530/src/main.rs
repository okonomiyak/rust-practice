use proconio::input;

#[derive(Debug)]
enum Category {
    Vegetable,
    Drink,
    Meat
}
#[derive(Debug)]
struct Item {
    name: String,
    number: i32,
    category: Category
}
impl Category {
    fn label(&self) -> &str{
        match self {
            Category::Vegetable => "野菜",
            Category::Drink => "飲み物",
            Category::Meat => "肉",
        }
    }
    
}
fn main() {
    let mut inventory:Vec<Item> = Vec::new();
    inventory.push(Item { name: String::from("apple"), number: 10000, category: Category::Vegetable });
    inventory.push(Item { name: String::from("coke"), number: 5000, category: Category::Drink });
    inventory.push(Item { name: String::from("poke"), number: 500, category: Category::Meat });
    loop{
        println!("商品管理システムへようこそ");
        input!{
            in_number:i32,
        };
        match in_number {
            1 => inventorry_push(&mut inventory),
            2 => println!("カテゴリーは1:Vegetable,2:Drink,3:Meat"),
            3 => {for i in &inventory {println!("商品名は{}です。個数は{}です。種類は{}です。",i.name,i.number,i.category.label())}},
            _ => {println!("終了します");break;},

        }
    }
}

fn inventorry_push(arry: &mut Vec<Item>) {
    println!("商品の追加を行います");
    println!("名前,個数,種類の順番に入力してください");
    let mut category;
    input! {
        name:String,
        number:i32,
        category_number:i32
    }
    match category_number {
        1 => category = {Category::Vegetable;println!("Vegetable");},
        2 => category = {Category::Drink;println!("Drink");},
        3 => category = {Category::Meat;println!("Meat");},
        _ => println!("無効な入力"),
    }
    arry.push(Item { name: String::from("apple"), number: 10000, category: Category::Vegetable });
}