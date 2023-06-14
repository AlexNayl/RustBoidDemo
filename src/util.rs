use bevy::prelude::*;
use rand::*;

pub fn new_2d_unit_rand() -> Vec2{
    let mut rng = rand::thread_rng();
    return Vec2{
        x: (rng.gen::<f32>() - 0.5),
        y: (rng.gen::<f32>() - 0.5)
    }.normalize()
}