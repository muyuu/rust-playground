mod prime_factorization;

fn main () {
    let num = add(1, 10);

    let is_bigger = is_bigger_than_right(100, 99);

    println!("fn add result: {}", num);
    println!("fn is_bigger_than_right: {}", is_bigger);

    let text = if is_bigger {
        "大きいよ"
    } else {
        "小さいよ"
    };

    println!("{}", text);

    let loop_result = run_loop_for_sum(2, 10);
    println!("loop result: {}.", loop_result);

    fizz_buzz(30);

    let vec = vec![1,2,3,4,5,6,7];
    let msg = find(vec, 4);
    println!("run find. {}", msg);


    let mut order = Order::new(
        1,
        "Muyuu Fujita".to_string(),
        "2021-03-03T09:00:00Z".to_string(),
        false,
    );
    println!("id: {}", order.id);
    println!("id: {}", order.pic);
    println!("id: {}", order.date);
    let (pic, accepted) = order.quick_look();
    println!("担当者: {}, 完了: {}", pic, accepted);
    order.get_accepted();
    println!("完了: {}", order.accepted);
    order.add_item(Item::new(1, "コカ・コーラ".to_string()));
    order.add_item(Item::new(2, "コーヒー".to_string()));
    println!("name: {}, id: {}.", &order.items[0].name, &order.items[0].id);
    println!("name: {}, id: {}.", &order.items[1].name, &order.items[1].id);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_bigger_than_right(left: i32, right: i32) -> bool {
    left > right
}

fn run_loop_for_sum(mut i: i32, m: i32) -> i32 {
    let result = loop {

        if i > m {
            break i;
        }

        i = i * 2;
    };

    return result;
}

fn fizz_buzz(max: i32) -> () {
    for i in 0..=max {
        if i % 15 == 0 {
            println!("fizzbuzz!");
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", i);
        }
    }
}

fn find(source: Vec<i32>, target: i32) -> String {
    for i in source.into_iter() {
        if i == target {
            return format!("found! {}", target);
        }
    }

    format!("not found")
}

struct Order {
    id: i32,
    pic: String,
    date: String,
    items: Vec<Item>,
    accepted: bool,
}

impl Order {
    fn new(id: i32, pic: String, date: String, accepted: bool) -> Order {
        Order {
            id,
            pic,
            date,
            items: vec![],
            accepted,
        }
    }

    // インスタンスメソッドは引数に self を入れる？
    fn quick_look(&self) -> (&String, &bool) {
        (&self.pic, &self.accepted)
    }

    fn get_accepted(&mut self) {
        self.accepted = true;
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

struct Item {
    id: i32,
    name: String,
}

impl Item {
    fn new(id: i32, name: String) -> Item {
        Item {
            id,
            name,
        }
    }
}
