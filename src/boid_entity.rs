use bevy::prelude::*;
use super::velocity::*;
use super::constrained_world::*;



#[derive(Component, Default)]
pub struct BoidEntity{
    visable_entities : Vec<Entity>
}

pub fn boid_behaviour_system(
    world : Res<PeriodicWorldBounds>,
    mut query : Query<(Entity,&mut BoidEntity, &Velocity, &Transform)>
){
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut ent1, mut ent2]) = combinations.fetch_next(){
        
    }
}