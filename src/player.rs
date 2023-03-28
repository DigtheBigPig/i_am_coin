
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;  

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<JumpStrength>()
            .add_startup_system(create_player)
            .add_system(move_player)
            ;
    }
}

const SPAWN_POINT: Vec3 = Vec3::new(0.0,5.0,0.0);



#[derive(Component)]
pub struct Player;



fn create_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    // Cylinder
    commands.spawn((Player, PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder{radius: 1.0, height: 0.2, resolution: 32, ..default()})),
        material: debug_material.clone(),//materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(SPAWN_POINT),
        ..default()
    }))
    // Add rigidbody
    .insert(RigidBody::Dynamic)
    // Add colider
    .insert(Collider::cylinder(0.1, 1.0))
    // Add gravity
    .insert(GravityScale(1.0))
    // Outside impulse/forces(for movement)
    .insert(ExternalImpulse {
        impulse: Vec3::ZERO,
        torque_impulse: Vec3::ZERO,
    })
    .insert(ExternalForce {
        force: Vec3::ZERO,
        torque: Vec3::ZERO,
    })
    // Damping for air friction and such
    .insert(Damping { linear_damping: 0.5, angular_damping: 10.0 });
}

const SPEED: f32 = 35.0;
const ROT_SPEED: f32 = 10.0;
const BASE_JUMP_STRNGTH: f32 = 7.5;
pub const MAX_JUMP_TIME_LENGTH: f32 = 1.0;




#[derive(Resource, Debug)]
pub struct JumpStrength(pub f32);
impl Default for JumpStrength {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(0.0)
    }
}


fn move_player(
    time: Res<Time>,
    mut jump_strength: ResMut<JumpStrength>,
    mut player_transform: Query<&mut Transform,With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ext_impulses: Query<&mut ExternalImpulse, With<Player>>,
    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let player_pos = player_transform.get_single().unwrap().translation;
    let player_rot = player_transform.get_single().unwrap().rotation;
    let speed = SPEED; //*time.delta_seconds();
    let rot_speed = ROT_SPEED;

    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = player_rot*Vec3::new(0.0, 0.0, 0.0);
        ext_force.torque = player_rot*Vec3::new(0.0, 0.0, 0.0);
    }
    // movement
    /*if keyboard_input.pressed(KeyCode::A) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = player_rot*Vec3::new(-speed, 0.0, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = player_rot*Vec3::new(speed, 0.0, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::W) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = player_rot*Vec3::new(0.0, 0.0, -speed);
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = player_rot*Vec3::new(0.0, 0.0, speed);
        }
    }*/



    // rotation
    if keyboard_input.pressed(KeyCode::J) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.torque = player_rot*Vec3::new(0.0, rot_speed, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::K) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.torque = player_rot*Vec3::new(0.0, -rot_speed, 0.0);
        }
    }


    // JUMP
    if keyboard_input.pressed(KeyCode::Space) {
        jump_strength.0 += time.delta_seconds();
        jump_strength.0 = jump_strength.0.clamp(0.0, MAX_JUMP_TIME_LENGTH);
    }
    if keyboard_input.just_released(KeyCode::Space) {
        let grounded:bool = cast_ray(rapier_context, player_pos, player_rot);
        if grounded {
            for mut ext_impulse in ext_impulses.iter_mut() {
                ext_impulse.impulse = Vec3::new(0.0, jump_strength.0*BASE_JUMP_STRNGTH, jump_strength.0*BASE_JUMP_STRNGTH);
                ext_impulse.torque_impulse = Vec3::new(jump_strength.0*rot_speed ,0.0,0.0);
            }
        }
        jump_strength.0 = 0.0;
    }


    // RESET
    if keyboard_input.pressed(KeyCode::R) {
        player_transform.get_single_mut().unwrap().translation = Vec3::ZERO + SPAWN_POINT;
        player_transform.get_single_mut().unwrap().rotation = Quat::IDENTITY;
    }

}

fn cast_ray(rapier_context: Res<RapierContext> , pos: Vec3, dir: Quat) -> bool {
    let ray_pos = pos-dir*Vec3::new(0.0,0.2,0.0);
    let ray_pos_neg = pos+dir*Vec3::new(0.0,0.2,0.0);
    let ray_dir = dir*Vec3::new(0.0, -1.0, 0.0);
    let max_toi = 0.1;
    let solid = true;

    if let Some((_,_)) = rapier_context.cast_ray(
        ray_pos, ray_dir, max_toi, solid, default()
    ) {
        return true;
    }
    if let Some((_,_)) = rapier_context.cast_ray(
        ray_pos_neg, -ray_dir, max_toi, solid, default()
    ) {
        return true;
    }
    return false;
    
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}