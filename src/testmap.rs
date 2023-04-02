
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct TestMapPlugin;  

impl Plugin for TestMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(testmap_setup)
            ;
    }
}

const CUBOID_DEPTH: f32 = 0.2;
const CUBOID_SIZE: f32 = 10.0;


const FLOOR_WIDTH: f32 = 10.0;
const FLOOR_LENGTH: f32 = 50.0;

const BLACK: Color = Color::rgb(0.2,0.2,0.2);
const BLUE_DARK: Color = Color::rgb(0.0,0.0,0.5);
const BLUE_LIGHT: Color = Color::rgb(0.0,0.0,1.0);
const RED_LIGHT: Color = Color::rgb(1.0,0.0,0.0);
const GREEN_LIGHT: Color = Color::rgb(0.0,1.0,0.0);



macro_rules! m_spawn_cuboid_wall_x {
    ($vec2:expr, $vec3:expr, $commands:expr, $meshes:expr, $materials:expr, $color:expr) => {
        $commands.spawn(PbrBundle {
            mesh: $meshes.add(shape::Box::new($vec2.x*CUBOID_SIZE, $vec2.y*CUBOID_SIZE, CUBOID_DEPTH).into()),
            material: $materials.add($color.into()),
            transform: Transform::from_translation(CUBOID_SIZE*$vec3),
            ..default()
        })
        .insert(Collider::cuboid($vec2.x*CUBOID_SIZE/2.0, $vec2.y*CUBOID_SIZE/2.0, CUBOID_DEPTH/2.0));
    };
}

macro_rules! m_spawn_cuboid_wall_z {
    ($vec2:expr, $vec3:expr, $commands:expr, $meshes:expr, $materials:expr, $color:expr) => {
        $commands.spawn(PbrBundle {
            mesh: $meshes.add(shape::Box::new(CUBOID_DEPTH, $vec2.y*CUBOID_SIZE, $vec2.x*CUBOID_SIZE).into()),
            material: $materials.add($color.into()),
            transform: Transform::from_translation(CUBOID_SIZE*$vec3),
            ..default()
        })
        .insert(Collider::cuboid(CUBOID_DEPTH/2.0, $vec2.y*CUBOID_SIZE/2.0, $vec2.x*CUBOID_SIZE/2.0));
    };
}

macro_rules! m_spawn_cuboid_floor {
    ($vec2:expr, $vec3:expr, $commands:expr, $meshes:expr, $materials:expr, $color:expr, $floor_size:expr) => {
        $commands.spawn(PbrBundle {
            mesh: $meshes.add(shape::Box::new($vec2.x*CUBOID_SIZE, $floor_size*CUBOID_DEPTH, $vec2.y*CUBOID_SIZE).into()),
            material: $materials.add($color.into()),
            transform: Transform::from_translation(CUBOID_SIZE*$vec3),
            ..default()
        })
        .insert(Collider::cuboid($vec2.x*CUBOID_SIZE/2.0, $floor_size*CUBOID_DEPTH/2.0, $vec2.y*CUBOID_SIZE/2.0));
    };
}

fn testmap_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


    // main floor
    m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,0.0,0.0), commands, meshes, materials, BLACK, 3.0);
    // main four walls
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,-2.5), commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,2.5), commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(-2.5,0.0,0.0), commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(2.5,0.0,0.0), commands, meshes, materials, BLUE_DARK);



    // small square parkour course
    m_spawn_cuboid_floor!(Vec2::new(0.5,0.5),Vec3::new(2.0,0.3,0.0), commands, meshes, materials, RED_LIGHT, 1.0);
    m_spawn_cuboid_floor!(Vec2::new(0.5,0.5),Vec3::new(2.0,0.6,0.7), commands, meshes, materials, GREEN_LIGHT, 1.0);
    m_spawn_cuboid_floor!(Vec2::new(0.5,0.5),Vec3::new(2.0,0.9,1.4), commands, meshes, materials, RED_LIGHT, 1.0);
}