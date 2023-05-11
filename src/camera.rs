use bevy::{prelude::*, render::camera::ScalingMode};
use super::constrained_world::PeriodicWorldBounds;

//distance outide of the camera's view that the world boundary is set at, prevents user from seeing teleport.
const WORLD_BOUNDARY_MARGIN:f32 = 0.05;

//These camera settings allow the window to resize and reshape while keeping the entitys size on screen realtivly constant.
pub fn get_camera_bundle() -> Camera2dBundle{
    
    return Camera2dBundle{
        projection: OrthographicProjection{
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

//This system continuously resizes the world bounds to fit the window
pub fn world_resize_system(
    mut world:ResMut<PeriodicWorldBounds>, 
    query:Query<&OrthographicProjection>
){
    let projection = query.single();
    world.min = projection.area.min.extend(world.min.z) - Vec3::ONE * WORLD_BOUNDARY_MARGIN;
    world.max = projection.area.max.extend(world.max.z) + Vec3::ONE * WORLD_BOUNDARY_MARGIN;
}