#[derive(Debug, Copy, Clone)]
enum Light {
    RED(usize),
    YELLOW(usize),
    GREEN(usize)
}

trait TrafficLight {
    fn get_time(&self) -> usize;
}

impl TrafficLight for Light {
    fn get_time(&self) -> usize {
        match self {
            Light::RED(time) => *time,
            Light::YELLOW(time) => *time,
            Light::GREEN(time) => *time,
        }
    }
}

pub struct Question1 {}

impl Question1 {
    pub fn run(&self) {
        let mut g = Light::GREEN(10);
        println!("green time is {:?}", g.get_time());

        g = Light::RED(10);
        println!("red time is {:?}", g.get_time());

        g = Light::YELLOW(10);
        println!("yellow time is {:?}", g.get_time());
    }
}