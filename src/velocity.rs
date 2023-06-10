use bevy::prelude::*;
use rand::prelude::*;


#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

impl Velocity{
    pub fn new_3d_unit_rand() -> Self{
        let mut rng = rand::thread_rng();
        return Velocity(Vec3{
            x: (rng.gen::<f32>() - 0.5),
            y: (rng.gen::<f32>() - 0.5),
            z: (rng.gen::<f32>() - 0.5)
        }.normalize())
    }

    pub fn new_2d_unit_rand() -> Self{
        let mut rng = rand::thread_rng();
        return Velocity(Vec3{
            x: (rng.gen::<f32>() - 0.5),
            y: (rng.gen::<f32>() - 0.5),
            z: 0.0
        }.normalize())
    }

}

pub fn velocity_transform_system(
    time : Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>
){
    query.for_each_mut(|
        (mut transform,velocity)
        |{
            //moves the entity to the next step according to the veloty
            transform.translation += velocity.0 * time.delta_seconds();
            //sets the entity's rotation to face the veloty
            transform.rotation = Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize());
        }
    )
}