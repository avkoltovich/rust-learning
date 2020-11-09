struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct User {
    login: String,
    email: String,
    age: u32,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let mut alex = User {
        login: String::from("avkoltovich"),
        email: String::from("avkoltovich@gmail.com"),
        age: 33,
        active: true,
    };

    alex.age = 34;

    let white = Color(256, 256, 256);

    let red = white.0;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Площадь, расчитанная через метод: {}",
        rect1.area()
     )
}

fn create_user(login: String, email: String) -> User {
    User {
        login,
        email,
        active: true,
        age: 40
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
