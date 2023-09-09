//! A very basic implementation of a character controller for a dynamic rigid body.
//! Supports directional movement and jumping.
//!
//! Bevy XPBD does not have a built-in character controller yet, so you will have to implement
//! the logic yourself.
//!
//! For a kinematic character controller, see the `basic_kinematic_character` example.

use std::f32::consts::PI;

use bevy::{prelude::*, utils::hashbrown::raw::Global};
use bevy_xpbd_3d::{math::*, prelude::*, PhysicsSchedule, PhysicsStepSet};
use crate::game_const::*;

pub struct PlayerPlugin;  

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.init_resource::<JumpStrength>()
            .add_systems(Startup, setup)
            .add_systems(PhysicsSchedule, movement.before(PhysicsStepSet::BroadPhase))
            ;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerJump {
    dir: Direction
}

pub enum Direction {
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 0.005, 8.0),
    ));

    // Wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XZY, 0.0, PI/2.0, 0.0)).with_translation(Vec3::new(4.0,4.0,0.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 0.005, 8.0),
    ));

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 1.0,
                height: 0.2,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        },
        RigidBody::Dynamic,
        Position(Vector::Y * 1.0),
        Collider::cylinder(0.2, 1.0),
        // Prevent the player from falling over
        //LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        // Cast the player shape downwards to detect when the player is grounded
        
        
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        Player,
    )).with_children(|parent| {
        parent.spawn(
            (
                PlayerJump { dir: Direction::Down },
                ShapeCaster::new(
                    Collider::cylinder(0.1, 0.95),
                    Vector::NEG_Y * 0.05,
                    Quaternion::default(),
                    Vector::NEG_Y,
                ).with_ignore_origin_penetration(true) // Don't count player's collider
                .with_max_time_of_impact(0.2)
                .with_max_hits(1),
            )
        );
        parent.spawn(
            (
                PlayerJump { dir: Direction::Up },
                ShapeCaster::new(
                    Collider::cylinder(0.1, 0.95),
                    Vector::Y * 0.05,
                    Quaternion::default(),
                    Vector::Y,
                ).with_ignore_origin_penetration(true) // Don't count player's collider
                .with_max_time_of_impact(0.2)
                .with_max_hits(1),
            )
        );
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut LinearVelocity>,
    mut jump_query: Query<(&ShapeHits, &PlayerJump)>,
    query_player_transform: Query<&GlobalTransform, With<Player>>,
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
) {
    for (mut linear_velocity) in &mut players {
        // Directional movement
        let mut velocity_change = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            velocity_change.z -= 1.2;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            velocity_change.z += 1.2;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            velocity_change.x -= 1.2;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            velocity_change.x += 1.2;
        }

        // Jump if space pressed and the player is close enough to the ground
        for (ground_hits, player_jump) in jump_query.iter() {
            if keyboard_input.just_pressed(KeyCode::Space) && !ground_hits.is_empty() {
                match player_jump.dir {
                    Direction::Down => linear_velocity.y += 8.0,
                    Direction::Up => linear_velocity.y += 8.0,
                };
            }
        }

        let sens_x = 0.5;
        let sens_y = 0.5;


        // movement
        //for ev in motion_evr.iter() {
            //camera_look.0.x -= ev.delta.x*sens_x*SENS_X;
            //camera_look.0.y -= ev.delta.y*sens_y*SENS_Y;

            //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        //}

        //camera_look.0.y = camera_look.0.y.clamp(-1.1, 0.40);
        
        //let camera_dirx = Quat::from_axis_angle(Vec3::Y, camera_look.0.x);
        //velocity_change = camera_dirx*velocity_change;

        linear_velocity.x += velocity_change.x;
        linear_velocity.z += velocity_change.z;

        // Slow player down on the x and y axes
        linear_velocity.x *= 0.8;
        linear_velocity.z *= 0.8;
    }
}


fn camera_follow(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {

}