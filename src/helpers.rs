use bevy::prelude::*;
use std::f32::consts::PI;

pub struct HelperPlugin;  

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<YRotation>()
            .add_system(y_rot_update)
            ;
    }
}

#[derive(Resource, Debug)]
pub struct YRotation(pub Quat);
impl Default for YRotation {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Quat::default())
    }
}


fn y_rot_update(
    query_player_transform: Query<&Transform, With<crate::player::Player>>,
    mut y_rotation: ResMut<YRotation>,
){
    let player_transform = *query_player_transform.get_single().unwrap();
    let player_forward = player_transform.forward();
    let y_rot = Vec2::new(0.0,-1.0).angle_between(Vec2::new(player_forward.x, player_forward.z));
    let quat = Quat::from_axis_angle(Vec3::new(0.0,1.0,0.0), y_rot);
    //let (_, player_rot, player_pos) = player_transform.to_scale_rotation_translation();
    //let changed_player_rot = player_rot.to_axis_angle();
    //let quat: Quat = Quat::from_axis_angle(Vec3::new(0.0,1.0,0.0), get_y_rotation(changed_player_rot.0, changed_player_rot.1));
    
    println!("{:?}", y_rot);
    //y_rotation.0 = quat;
}

fn get_y_rotation(vec: Vec3, rot: f32) -> f32 {
    let new_rot = rot*vec.y.signum();
    if new_rot<0.0 {
        return new_rot+2.0*PI;
    }
    return new_rot;
}