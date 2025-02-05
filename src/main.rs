use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                decorations: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let idnh_icon = asset_server.load("textures/idnh_icon.png");

    let mut mesh = Mesh::from(Rectangle::default());

    let vertex_colors: Vec<[f32; 4]> = vec![
        LinearRgba::RED.to_f32_array(),
        LinearRgba::GREEN.to_f32_array(),
        LinearRgba::BLUE.to_f32_array(),
        LinearRgba::WHITE.to_f32_array(),
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle = meshes.add(mesh);

    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_translation(Vec3::new(-96., 0., 0.)).with_scale(Vec3::splat(128.)),
    ));

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(idnh_icon)),
        Transform::from_translation(Vec3::new(96., 0., 0.)).with_scale(Vec3::splat(128.)),
    ));

}
