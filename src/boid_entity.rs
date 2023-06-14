use bevy::prelude::*;
use super::velocity::*;
use super::constrained_world::*;

#[derive(Component, Default)]
pub struct BoidEntity{
    visible_distances:Vec<f32>,
    visible_velocities:Vec<Vec3>,
    visible_positions:Vec<Vec3>
}

pub fn boid_behaviour_system(
    world : Res<PeriodicWorldBounds>,
    time : Res<Time>,
    mut query : Query<(Entity,&mut BoidEntity, &mut Velocity, &Transform)>
){
    //hardcoded constants
    let max_speed = 1.0;
    let perception_distance = 0.5;
    let avoidance_distance = 0.2;
    let avoidance_multiplier = 10.0;

    let mut combinations = query.iter_combinations_mut();
    while let Some([mut ent1, mut ent2]) = combinations.fetch_next(){
        //find all entities that can see each other
        let distance = world.difference_within_periodic_bounds(&ent1.3.translation, &ent2.3.translation).length();
        if distance < perception_distance{
            ent1.1.visible_distances.push(distance);
            ent2.1.visible_distances.push(distance);
            ent1.1.visible_velocities.push(ent2.2.0.clone());
            ent2.1.visible_velocities.push(ent1.2.0.clone());
            ent1.1.visible_positions.push(ent2.3.translation.clone());
            ent2.1.visible_positions.push(ent1.3.translation.clone());
        }
    }

    query.for_each_mut(|mut component_bundle|{
        //calculate and apply acceleration based on visible entities
        let mut acceleration = Vec3::ZERO;

        let other_velocities = &component_bundle.1.visible_velocities;
        let other_distances = &component_bundle.1.visible_distances;
        let other_positions = &component_bundle.1.visible_positions;

        let n = other_velocities.len();

        let mut average_position = Vec3::ZERO;
        let mut average_velocity = Vec3::ZERO;

        for i in 0..n{
            let velocity = other_velocities.get(i).expect("visible vector length mismatch");
            let distance = other_distances.get(i).expect("visible vector length mismatch");
            let position = other_positions.get(i).expect("visible vector length mismatch");
            //collect average data for alignment and cohesion steps
            average_position += *position;
            average_velocity += *velocity;

            //avoidance
            if distance < &avoidance_distance{
                let direction_vector = world.difference_within_periodic_bounds(position, &component_bundle.3.translation).normalize_or_zero();
                let strength = ((avoidance_distance - distance)/avoidance_distance) * avoidance_multiplier;
                acceleration -= direction_vector * strength;
            }
        }
        //prep average data
        average_position /= n as f32;
        average_velocity /= n as f32;

        


        //apply the calculated acceleration
        component_bundle.2.0 += acceleration * time.delta_seconds();
        //limit speed
        component_bundle.2.0 = component_bundle.2.0.normalize() * max_speed;


        component_bundle.1.visible_distances.clear();
        component_bundle.1.visible_velocities.clear();
        component_bundle.1.visible_positions.clear();
    });

}