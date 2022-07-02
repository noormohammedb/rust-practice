struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("foo"),
        email: String::from("foo@email.com"),
        sign_in_count: 9,
        active: true,
    };
    println!("user1.username: {}", user1.username);

    let user1_name = user1.username;

    user1.username = String::from("bar");
    println!("user1.username: {}", user1.username);
    println!("user1.email: {}", user1.email);

    let user2 = build_user(String::from("koo"), String::from("koo@emial.com"));

    println!("user2.username: {}", user2.username);
    println!("user2.email: {}", user2.email);

    let user3 = User {
        username: String::from("lorem"),
        email: String::from("lorem@email.com"),
        ..user2
    };

    println!("user3.username: {}", user3.username);
    println!("user3.email: {}", user3.email);
    println!("user3.active: {}", user3.active);

    // tuple structs without named fields

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let wid1 = 40;
    let higt1 = 20;
    println!("fn Area is {}", area_fn(wid1, higt1));

    let rect = (wid1, higt1);
    println!("fn_tuple Area is {}", area_fn_tuple(rect));

    let my_rect_st = Rectangle {
        width: wid1,
        height: higt1,
    };
    println!("fn_struct Area is {}", area_fn_struct(&my_rect_st));
    println!("area in struct Area is {}", my_rect_st.area());

    println!("my_rect_st: {:#?}", my_rect_st);

    let rect1 = Rectangle {
        width: 30,
        height: 15,
    };
    let rect2 = Rectangle {
        width: 90,
        height: 40,
    };

    println!(
        "my_rect_st can hold rect1 : {}",
        my_rect_st.can_hold(&rect1)
    );

    println!(
        "my_rect_st can hold rect2 : {}",
        my_rect_st.can_hold(&rect2)
    );

    let my_square = Rectangle::square(5);
    println!("my_square : {:#?}", my_square)
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn area_fn(wt: u32, ht: u32) -> u32 {
    wt * ht
}

fn area_fn_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_fn_struct(rect_data: &Rectangle) -> u32 {
    rect_data.width * rect_data.height
}
