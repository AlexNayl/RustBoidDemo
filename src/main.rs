use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use boid_entity::BoidEntity;

mod velocity;
mod constrained_world;
mod camera;
mod boid_entity;
mod util;

const NUM_BOID_ENTITIES: usize = 10;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_system(velocity::velocity_transform_system)
    .add_system(constrained_world::periodic_boundary_system)
    .add_system(camera::world_resize_system)
    .add_system(boid_entity::boid_behaviour_system)
    .add_startup_system(startup_system)
    .insert_resource(constrained_world::PeriodicWorldBounds{min: Vec3::ZERO, max: Vec3::ONE})
    .run();
}

fn startup_system(
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>
    ){
        commands.spawn(camera::get_camera_bundle());
        for _ in 0..NUM_BOID_ENTITIES{
            commands.spawn((MaterialMesh2dBundle{
            mesh: meshes.add(shape::RegularPolygon::new(0.1, 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(util::new_2d_unit_rand().extend(0.0)*10.0).with_scale(Vec3 { x: 0.5, y: 1.0, z: 1.0 }),
        ..default()
        },
        velocity::Velocity::new_2d_unit_rand(),
        BoidEntity::default()
    ));
        }
        



}