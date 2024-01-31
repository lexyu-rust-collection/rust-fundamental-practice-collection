pub fn output() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Monday;

    match today {
        Days::Monday => println!("hates monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost Weekend"),
        Days::Saturday => println!("Weekend"),
        Days::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend : {}", today.is_weekend());
}
