use std::f32::consts::PI;

pub fn output() {
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area : {}", rec.area());
    println!("Circ Area : {}", circ.area());
}

trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Rectangle {
    length: f32,
    width: f32,
}

#[derive(Debug)]
struct Circle {
    length: f32,
    width: f32,
}

impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Self {
        return Rectangle { length, width };
    }
    fn area(&self) -> f32 {
        return self.length * self.width;
    }
}

impl Shape for Circle {
    fn new(length: f32, width: f32) -> Self {
        return Circle { length, width };
    }
    fn area(&self) -> f32 {
        return (self.length / 2.0).powf(2.0) * PI;
    }
}
