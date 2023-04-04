
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

const FULL_BLACK: Color = Color::rgb(0.0,0.0,0.0);
const BLACK: Color = Color::rgb(0.2,0.2,0.2);
const BLUE_DARK: Color = Color::rgb(0.0,0.0,0.5);
const BLUE_LIGHT: Color = Color::rgb(0.0,0.0,1.0);
const RED_LIGHT: Color = Color::rgb(1.0,0.0,0.0);
const GREEN_LIGHT: Color = Color::rgb(0.0,1.0,0.0);
const GREEN_DARK: Color = Color::rgb(0.0,0.5,0.0);

#[derive(Component)]
pub struct Sticky;


macro_rules! m_spawn_cuboid_wall_x {
    ($vec2:expr, $vec3:expr, $commands:expr, $meshes:expr, $materials:expr, $color:expr) => {
        $commands.spawn(PbrBundle {
            mesh: $meshes.add(shape::Box::new($vec2.x*CUBOID_SIZE, $vec2.y*CUBOID_SIZE, CUBOID_DEPTH).into()),
            material: $materials.add($color.into()),
            transform: Transform::from_translation(CUBOID_SIZE*$vec3),
            ..default()
        })
        .insert(RigidBody::Fixed)
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
        .insert(RigidBody::Fixed)
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
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid($vec2.x*CUBOID_SIZE/2.0, $floor_size*CUBOID_DEPTH/2.0, $vec2.y*CUBOID_SIZE/2.0));
    };
}

macro_rules! m_spawn_cuboid {
    ($size:expr, $pos:expr, $commands:expr, $meshes:expr, $materials:expr, $color:expr) => {
        $commands.spawn(PbrBundle {
            mesh: $meshes.add(shape::Box::new($size.x*CUBOID_SIZE, $size.y*CUBOID_SIZE, $size.z*CUBOID_SIZE).into()),
            material: $materials.add($color.into()),
            transform: Transform::from_translation(CUBOID_SIZE*$pos + CUBOID_DEPTH*Vec3::new(0.0,0.5*$size.y,0.0)),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid($size.x*CUBOID_SIZE/2.0, $size.y*CUBOID_SIZE/2.0, $size.z*CUBOID_SIZE/2.0));
    };
}

macro_rules! m_spawn_sticky_field {
    ($size:expr, $pos:expr, $commands:expr, $meshes:expr, $materials:expr, $color:expr) => {
        $commands.spawn((Sticky,PbrBundle {
            mesh: $meshes.add(shape::Box::new($size.x*CUBOID_SIZE, $size.y*CUBOID_SIZE, $size.z*CUBOID_SIZE).into()),
            material: $materials.add($color.into()),
            transform: Transform::from_translation(CUBOID_SIZE*$pos + CUBOID_DEPTH*Vec3::new(0.0,0.5,0.0)),
            ..default()
        }))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid($size.x*CUBOID_SIZE/2.0, $size.y*CUBOID_SIZE/2.0, $size.z*CUBOID_SIZE/2.0))
        .insert(ActiveEvents::COLLISION_EVENTS);
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
    m_spawn_cuboid_floor!(Vec2::new(5.0,35.0),Vec3::new(0.0,0.0,-15.0), commands, meshes, materials, BLACK, 10.0);
    // main four walls
    //m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,-2.5), commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,2.5), commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(35.0,3.0), Vec3::new(-2.5,0.0,-15.0), commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(35.0,3.0), Vec3::new(2.5,0.0,-15.0), commands, meshes, materials, BLUE_DARK);



    // small square parkour course
    m_spawn_cuboid_floor!(Vec2::new(0.5,0.5),Vec3::new(2.0,0.3,0.0), commands, meshes, materials, RED_LIGHT, 1.0);
    m_spawn_cuboid_floor!(Vec2::new(0.5,0.5),Vec3::new(2.0,0.6,0.7), commands, meshes, materials, GREEN_LIGHT, 1.0);
    m_spawn_cuboid_floor!(Vec2::new(0.5,0.5),Vec3::new(2.0,0.9,1.4), commands, meshes, materials, RED_LIGHT, 1.0);


    const SPEC_CUBE: f32 = 0.2;
    m_spawn_cuboid!(Vec3::new(4.9,SPEC_CUBE,0.4), Vec3::new(0.0,SPEC_CUBE/2.0+0.11,-25.0), commands, meshes, materials, BLUE_DARK);


    // Second part
    const SECOND_ROOM_OFFSET: Vec3 = Vec3::new(0.0,1.5,5.0);

    // main floor
    m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,0.0,0.0)+SECOND_ROOM_OFFSET, commands, meshes, materials, BLACK, 10.0);
    // main four walls
    //m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,-2.5)+SECOND_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,2.5)+SECOND_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(-2.5,0.0,0.0)+SECOND_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(2.5,0.0,0.0)+SECOND_ROOM_OFFSET, commands, meshes, materials, BLUE_DARK);

    // Third part
    const THIRD_ROOM_OFFSET: Vec3 = Vec3::new(0.0,0.0,-5.0);

    // main floor
    //m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,0.0,0.0)+THIRD_ROOM_OFFSET, commands, meshes, materials, BLACK, 10.0);
    // main four walls
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,-27.5)+THIRD_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    //m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,2.5)+THIRD_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(-2.5,0.0,0.0)+THIRD_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(2.5,0.0,0.0)+THIRD_ROOM_OFFSET, commands, meshes, materials, BLUE_DARK);

    // Fourth part
    const FOURTH_ROOM_OFFSET: Vec3 = Vec3::new(5.0,1.5,0.0);

    // main floor
    m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,0.0,0.0)+FOURTH_ROOM_OFFSET, commands, meshes, materials, BLACK, 10.0);
    // main four walls
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,-2.5)+FOURTH_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,2.5)+FOURTH_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    //m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(-2.5,0.0,0.0)+FOURTH_ROOM_OFFSET, commands, meshes, materials, BLUE_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(2.5,0.0,0.0)+FOURTH_ROOM_OFFSET, commands, meshes, materials, BLUE_DARK);

    // Fifth part
    const FIFTH_ROOM_OFFSET: Vec3 = Vec3::new(-5.0,1.5,0.0);

    // main floor
    m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,-0.1,0.0)+FIFTH_ROOM_OFFSET, commands, meshes, materials, BLACK, 10.0);
    // main four walls
    m_spawn_cuboid_wall_x!(Vec2::new(4.97,3.0), Vec3::new(0.015,0.0,-2.5)+FIFTH_ROOM_OFFSET, commands, meshes, materials, GREEN_LIGHT);
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,2.5)+FIFTH_ROOM_OFFSET, commands, meshes, materials, GREEN_LIGHT);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(-2.5,0.0,0.0)+FIFTH_ROOM_OFFSET, commands, meshes, materials, GREEN_LIGHT);
    //m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(2.5,0.0,0.0)+FIFTH_ROOM_OFFSET, commands, meshes, materials, GREEN_LIGHT);
    
    // Secret room
    const SECRET_ROOM_OFFSET: Vec3 = Vec3::new(-4.96,0.0,-5.0);
    let secret_room_offset = SECRET_ROOM_OFFSET+FIFTH_ROOM_OFFSET;
    // main floor
    m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,0.0,0.0)+secret_room_offset, commands, meshes, materials, FULL_BLACK, 1.0);
    // main four walls
    m_spawn_cuboid_wall_x!(Vec2::new(5.0,3.0), Vec3::new(0.0,0.0,-2.5)+secret_room_offset, commands, meshes, materials, FULL_BLACK);
    m_spawn_cuboid_wall_x!(Vec2::new(4.98,3.0), Vec3::new(-0.02,0.0,2.5)+secret_room_offset, commands, meshes, materials, FULL_BLACK);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(-2.5,0.0,0.0)+secret_room_offset, commands, meshes, materials, FULL_BLACK);
    m_spawn_cuboid_wall_z!(Vec2::new(5.0,3.0), Vec3::new(2.5,0.0,0.0)+secret_room_offset, commands, meshes, materials, FULL_BLACK);
    // roof
    m_spawn_cuboid_floor!(Vec2::new(5.0,5.0),Vec3::new(0.0,1.3,0.0)+secret_room_offset, commands, meshes, materials, FULL_BLACK, 1.0);



    const CUBE_HEIGHT: f32 = 0.4;
    m_spawn_cuboid!(Vec3::new(0.8,CUBE_HEIGHT,0.8), Vec3::new(-0.4,CUBE_HEIGHT/2.0,-0.4)+FIFTH_ROOM_OFFSET, commands, meshes, materials, BLUE_DARK);
    m_spawn_cuboid!(Vec3::new(0.4,CUBE_HEIGHT,0.8), Vec3::new(0.22,CUBE_HEIGHT/2.0,0.0)+FIFTH_ROOM_OFFSET, commands, meshes, materials, BLUE_DARK);
    m_spawn_cuboid!(Vec3::new(1.6,CUBE_HEIGHT,0.4), Vec3::new(0.0,CUBE_HEIGHT/2.0,-1.02)+FIFTH_ROOM_OFFSET, commands, meshes, materials, BLUE_DARK);

    // sticky field
    const FIELD_HEIGHT: f32 = 0.01;
    m_spawn_sticky_field!(Vec3::new(0.6,FIELD_HEIGHT,0.6), Vec3::new(1.0,0.5*FIELD_HEIGHT,-2.0)+FIFTH_ROOM_OFFSET, commands, meshes, materials, GREEN_DARK);

}