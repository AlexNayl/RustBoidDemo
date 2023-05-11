use bevy::{prelude::*};

#[derive(Resource)]
pub struct PeriodicWorldBounds{
    pub min:Vec3,
    pub max:Vec3
}

impl PeriodicWorldBounds{
    pub fn get_size(&self) -> Vec3{
        //The size of the world
        return self.max - self.min;
    }

    pub fn difference_within_periodic_bounds(&self,a:&Vec3, b:&Vec3) -> Vec3{
        //Calculates the vector difference taking into acound the periodic bounds
        let mut result = *a - *b;
        let size = self.get_size();
        let half_size = size / 2.0;
        
        for i in 0..3{
            //for each axis
            if result[i] > half_size[i]{
                result[i] -= size[i];
            }else if result[i] < -half_size[i]{
                result[i] += size[i];
            }
        };

        return result;
    }


}

pub fn periodic_boundary_system(world:Res<PeriodicWorldBounds>, mut query:Query<&mut Transform>){
    //This system restricts all trasnforms to within the world limits via repearting boundary conditions
    query.for_each_mut(|mut transform|{
        let size = world.get_size();
        let position = &mut transform.translation;

        for i in 0..3{
            //for each axis
            while position[i] < world.min[i]{
                position[i] += size[i];
            }
            while position[i] > world.max[i]{
                position[i] -= size[i];
            }
        }
    })
}


