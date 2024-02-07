use std::collections::HashMap;

pub fn output() {
    let mut heros = HashMap::new();
    heros.insert("Superman", "Kent");
    heros.insert("Batman", "Wayne");
    heros.insert("Flash", "Allen");

    for (k, v) in heros.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length : {}", heros.len());

    if heros.contains_key(&"Batman") {
        let the_batman = heros.get(&"Batman");
        match the_batman {
            Some(x) => println!("Is Bat"),
            None => println!("Not Bat"),
        }
    }
}
