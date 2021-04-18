mod class4;

use class4::question1::{Question1};
use class4::question2::sum;
use crate::class4::question3::{Square, Shape, Rectangle, Circle};

fn lesson_four() {
    // question 1
    let q_one = Question1{};
    q_one.run();

    // question 2
    let nums: [u32; 4] = [1, 2, 3, 4];
    println!("sum without overflow: {:?}", sum(&nums));
    let nums: [u32; 4] = [1, 2, 3, u32::MAX];
    println!("sum with overflow: {:?}", sum(&nums));

    // question 3
    assert_eq!(Square::new(2.0).area(), 4.0);
    assert_eq!(Rectangle::new(2.0, 3.0).area(), 6.0);
    println!("area of circle is {}", Circle::new(2.0).area());
}

fn lesson_five() {
    // question 1
    let q_one = Question1{};
    q_one.run();

    // question 2
    let nums: [u32; 4] = [1, 2, 3, 4];
    println!("sum without overflow: {:?}", sum(&nums));
    let nums: [u32; 4] = [1, 2, 3, u32::MAX];
    println!("sum with overflow: {:?}", sum(&nums));

    // question 3
    assert_eq!(Square::new(2.0).area(), 4.0);
    assert_eq!(Rectangle::new(2.0, 3.0).area(), 6.0);
    println!("area of circle is {}", Circle::new(2.0).area());
}


fn main() {
    lesson_four();
}