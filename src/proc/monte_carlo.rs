use rand::prelude::*;

pub fn integrate_zero_one(
    fxn: Box<dyn Fn(f32) -> f32>, num: i32) -> f32 
{
    let dx: f32 = 1 as f32/num as f32;
    (0..num).into_iter()
        .fold(0.0, |mut sumn, val| -> f32 {
            sumn += dx * &fxn((val as i32 * dx as i32) as f32); sumn
        }
    )
}

pub fn integrate_monte_carlo(
    a: f32, b: f32, num: i32,
    fxn: impl Fn(f32) -> f32) -> f32 
{
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0, 1);
    let sum: f32 = (0..num).into_iter()
        .fold(0.0, move |mut sumn, _| -> f32 {
            sumn += &fxn(num as f32 * (b - a) + a); sumn
        }
    );
    (b - a) / num as f32 * sum
}
