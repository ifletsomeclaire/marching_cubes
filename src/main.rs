use bevy::{prelude::*, render::pipeline::PrimitiveTopology};
use gaiku_3d::common::{Baker, FileFormat};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::time::Instant;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(read.system())
        .add_system(update)
        .run();
}

fn read(
    commands: &mut Commands,
    mut meshies: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, -4.0, 5.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 150.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(MainCamera::default());

    // copied from gaiku example
    let now = Instant::now();
    let file = format!("{}/assets/planet.gox", env!("CARGO_MANIFEST_DIR"));
    // let file = format!("{}/assets/small_tree.gox", env!("CARGO_MANIFEST_DIR"));
    // let file = format!("{}/assets/terrain.gox", env!("CARGO_MANIFEST_DIR"));
    let chunks = gaiku_3d::formats::GoxReader::read(&file);
    let mut meshes = vec![];

    let reader_elapsed = now.elapsed().as_secs();
    let now = Instant::now();

    for chunk in chunks.iter() {
        let mesh = gaiku_3d::bakers::MarchingCubesBaker::bake(chunk);
        if let Some(mesh) = mesh {
            meshes.push((mesh, chunk.position()));
        }
    }

    let baker_elapsed = now.elapsed().as_secs();
    let now = Instant::now();

    println!(
        "<<{}>> Chunks Reader: {} Baker: {} secs Export: {} secs",
        chunks.len(),
        reader_elapsed,
        baker_elapsed,
        now.elapsed().as_secs()
    );

    let mut rng = StdRng::from_entropy();

    // mesh data from gaiku is in mint vector types
    for (mesh_data, chunk_pos) in meshes.iter() {
        let mut meshie = Mesh::new(PrimitiveTopology::TriangleList);
        meshie.set_indices(Some(bevy::render::mesh::Indices::U16(
            mesh_data.indices.to_owned(),
        )));
        let positions = mesh_data
            .vertices
            .iter()
            .map(|pos| [pos.x, pos.y, pos.z])
            .collect::<Vec<_>>();
        meshie.set_attribute("Vertex_Position", positions);

        // Normals are required for PBR pipeline, but the marching squares algorithm doesn't output them as part of the mesh
        if mesh_data.normals.len() != mesh_data.vertices.len() {
            let normals = (0..mesh_data.vertices.len())
                .map(|_| [0.0; 3])
                .collect::<Vec<_>>();
            meshie.set_attribute("Vertex_Normal", normals);
        } else {
            let normals = mesh_data
                .normals
                .iter()
                .map(|norm| [norm.x, norm.y, norm.z])
                .collect::<Vec<_>>();
            meshie.set_attribute("Vertex_Normal", normals);
        }

        // UVs are required for PBR pipeline, but the marching squares algorithm doesn't output them as part of the mesh
        if mesh_data.uv.len() != mesh_data.vertices.len() {
            let uvs = (0..mesh_data.vertices.len())
                .map(|_| [0.0; 2])
                .collect::<Vec<_>>();
            meshie.set_attribute("Vertex_Uv", uvs);
        } else {
            let uvs = mesh_data
                .uv
                .iter()
                .map(|uv| [uv.x, uv.y])
                .collect::<Vec<_>>();
            meshie.set_attribute("Vertex_Uv", uvs);
        }
        // mesh definition in gaiku also has colors and tangents???

        let handle = meshies.add(meshie);

        commands.spawn(PbrBundle {
            mesh: handle,
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                ),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(
                chunk_pos.x,
                chunk_pos.y,
                chunk_pos.z,
            )),
            ..Default::default()
        });
    }
}

fn update(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&MainCamera, &mut Transform)>) {
    for (maincamera, mut camera) in query.iter_mut() {
        if keyboard_input.pressed(maincamera.z) {
            camera.translation.z -= 0.01;
        }
        if keyboard_input.pressed(maincamera.x) {
            camera.translation.z += 0.01;
        }
        if keyboard_input.pressed(maincamera.w) {
            camera.translation.y += 0.01;
        }
        if keyboard_input.pressed(maincamera.a) {
            camera.translation.x -= 0.01;
        }
        if keyboard_input.pressed(maincamera.s) {
            camera.translation.y -= 0.01;
        }
        if keyboard_input.pressed(maincamera.d) {
            camera.translation.x += 0.01;
        }
    }
}
pub struct MainCamera {
    w: KeyCode,
    a: KeyCode,
    s: KeyCode,
    d: KeyCode,
    z: KeyCode,
    x: KeyCode,
}
impl Default for MainCamera {
    fn default() -> Self {
        Self {
            w: KeyCode::W,
            a: KeyCode::A,
            s: KeyCode::S,
            d: KeyCode::D,
            z: KeyCode::Z,
            x: KeyCode::X,
        }
    }
}
