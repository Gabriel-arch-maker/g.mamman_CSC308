struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        self.radius.powi(2) * 22.0/7.0
    }
}

fn main() {

    let circle = Circle {radius: 20.0};
    println!("{}", circle.area());

}
