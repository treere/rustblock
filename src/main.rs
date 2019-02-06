extern crate amethyst;

use amethyst::{
    prelude::*,
    assets::Loader,
    renderer::{
        Projection,
        Camera,
        DrawFlat,
        PosTex,
        MeshHandle,
        Material,
        MaterialDefaults,
    },
    utils::application_root_dir,
    core::{
        TransformBundle,
        Transform,
        nalgebra::{Vector2, Vector3}
    },
};

struct Level;

impl SimpleState for Level {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);
        initialize_pad(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    transform.set_z(1.0);
    world
        .create_entity()
        .with(
            Camera::from(
                Projection::orthographic(0.0, 640., 0.0, 480., )))
        .with(transform)
        .build();
}

fn initialize_pad(world: &mut World) {
    let pad_mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, 100.0, 100.0),
    );

    let pad_material = create_colour_material(world, [0., 0., 1., 1.]);

    let mut trans = Transform::default();
    trans.set_xyz(200., 200., 0.);

    world.create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .build();
}

/// Converts a vector of vertices into a mesh.
fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Generates six vertices forming a rectangle.
fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: Vector3::new(left, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
    ]
}

/// Creates a solid material of the specified colour.
fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();
    let config = format!("{}/resources/display_config.ron", app_root);

    let game_data = GameDataBuilder::default()
        .with_basic_renderer(config, DrawFlat::<PosTex>::new(), true)?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::build("./", Level)?
        .build(game_data)?;

    game.run();

    Ok(())
}
