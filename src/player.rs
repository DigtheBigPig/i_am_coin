
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
            //.add_system(move_player)
            ;
    }
}

const SPAWN_POINT: Vec3 = Vec3::new(0.0,5.0,0.0);

#[derive(Default, Debug)]
pub enum PlayerSide {
    #[default]
    Normal,
    Sticky,
    Slippey,
}

#[derive(Component)]
pub struct PlayerParent;


#[derive(Component, Default)]
pub struct Player{
    pub head: PlayerSide,
    pub tail: PlayerSide,
}


#[derive(Component)]
pub struct PlayerCamera;

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //Debug
    mut ui_data: ResMut<crate::debug2::UiData>,
) {

    /*let camera_id = commands.spawn((PlayerCamera, Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })).id();*/

    let material_image: Handle<Image> = asset_server.load("coin_test1.PNG");

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(material_image/*images.add(uv_debug_texture())*/),
        //base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    

    // Cylinder
    let player_id = commands.spawn((Player::default(), PbrBundle {
        //mesh: meshes.add(Mesh::from(shape::Cylinder{radius: 1.0, height: 0.2, resolution: 12, ..default()})),
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0,0.2,1.0))),
        material: debug_material.clone(),//materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(SPAWN_POINT),
        ..default()
    }))
    // Add rigidbody
    .insert(RigidBody::Dynamic)
    // Add colider
    .insert(Collider::cuboid(0.5, 0.1, 0.5))
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
    .insert(Friction::coefficient(0.7))
    // Damping for air friction and such
    .insert(Damping { linear_damping: 0.5, angular_damping: 1.0 })
    .insert(Ccd::enabled()).id();

    /*let player_parent = commands.spawn((PlayerParent, SpatialBundle{
        transform: Transform::from_translation(SPAWN_POINT),
        ..default()
    })).push_children(&[camera_id, player_id]).id();*/


    //Debug
    ui_data.entity = Some(player_id);

}

const SPEED: f32 = 35.0;
const ROT_SPEED: f32 = 10.0;
const BASE_JUMP_STRNGTH: f32 = 6.0;
const BASE_FLIP_STRNGTH: f32 = 20.0;
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

/*fn update_parent_pos(
    player_transform: Query<&GlobalTransform,With<Player>>,
    mut parent_transform: Query<&mut Transform,With<PlayerParent>>,
) {
    match parent_transform.get_single_mut() {
        Err(_) => panic!("Not one player parent"),
        Ok(mut parent_pos) => {
            match player_transform.get_single() {
                Err(_) => panic!("Not one player"),
                Ok(player_pos) => {
                    *parent_pos = Transform::from_translation(player_pos.translation());
                }
            }
        }
    }
}*/


fn move_player(
    time: Res<Time>,
    mut jump_strength: ResMut<JumpStrength>,
    mut player_transform: Query<&mut Transform,With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ext_impulses: Query<&mut ExternalImpulse, With<Player>>,
    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
    rapier_context: Res<RapierContext>,
    y_rotation: Res<crate::helpers::YRotation>,
) {
    let player_pos = player_transform.get_single().unwrap().translation;
    //let player_rot = player_transform.get_single().unwrap().rotation;
    let player_rot = y_rotation.camera_dir;
    let speed = SPEED; //*time.delta_seconds();
    let rot_speed = ROT_SPEED;

    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = player_rot*Vec3::new(0.0, 0.0, 0.0);
        ext_force.torque = player_rot*Vec3::new(0.0, 0.0, 0.0);
    }
    // movement
    if keyboard_input.pressed(KeyCode::J) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = player_rot*Vec3::new(-speed, 0.0, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::K) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = player_rot*Vec3::new(speed, 0.0, 0.0);
        }
    }
    /*if keyboard_input.pressed(KeyCode::W) {
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
    if keyboard_input.pressed(KeyCode::A) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.torque = player_rot*Vec3::new(0.0, crate::helpers::bool_posneg(y_rotation.heads)*rot_speed, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.torque = player_rot*Vec3::new(0.0, -crate::helpers::bool_posneg(y_rotation.heads)*rot_speed, 0.0);
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
                ext_impulse.impulse = y_rotation.quat*Vec3::new(0.0, jump_strength.0*BASE_JUMP_STRNGTH, 2.0*jump_strength.0*BASE_JUMP_STRNGTH);
                ext_impulse.torque_impulse = y_rotation.quat*Vec3::new(-jump_strength.0*BASE_FLIP_STRNGTH ,0.0,0.0);
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

pub fn cast_ray(rapier_context: Res<RapierContext> , pos: Vec3, dir: Quat) -> bool {
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