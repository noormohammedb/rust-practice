trait Pilot {
    fn fly();
}

trait Wizard {
    fn fly();
}

struct Human;

impl Human {
    fn fly() {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly() {
        println!("This is your caption speaking.");
    }
}

impl Wizard for Human {
    fn fly() {
        println!("Up!");
    }
}

pub fn run() {
    // let person = Human;
    // person.fly();
    // Pilot::fly(&person);
    // Wizard::fly(&person);

    Human::fly();
    <Human as Wizard>::fly();
    <Human as Pilot>::fly();
}
