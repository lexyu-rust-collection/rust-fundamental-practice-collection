pub fn output() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    /*
    match me.position {
        Position::Manager => println!("you're a manager"),
        Position::Supervisor => println!("you're a supervisor"),
        Position::Worker => println!("you're a worker"),
    };
     */
    println!("{:?}", me);
    print_employee(me);
    print_employee(me);
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}
