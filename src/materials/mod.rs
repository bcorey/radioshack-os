use bevy::{
    mesh::{
        Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, VertexAttributeValues,
        VertexFormat,
    },
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
    },
    scene::SceneInstanceReady,
    shader::ShaderRef,
};

pub struct EdgeMaterialPlugin;

impl Plugin for EdgeMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<EdgeMaterial>::default())
            .add_observer(on_scene_ready_apply_edge_material);
    }
}

/// Component to apply edge material to a GLTF scene's meshes.
/// Add this to the same entity as `SceneRoot` and the material will be applied
/// automatically when the scene finishes loading.
///
/// # Example
/// ```ignore
/// commands.spawn((
///     SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/my_model.glb"))),
///     EdgeMaterialGltf {
///         color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
///         line_thickness: 2.0,
///     },
/// ));
/// ```
#[derive(Component, Clone)]
pub struct EdgeMaterialGltf {
    pub color: LinearRgba,
    pub line_thickness: f32,
}

impl Default for EdgeMaterialGltf {
    fn default() -> Self {
        Self {
            color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            line_thickness: 1.0,
        }
    }
}

/// Observer that applies edge material to all meshes in a GLTF scene when it's ready.
pub fn on_scene_ready_apply_edge_material(
    trigger: On<SceneInstanceReady>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut edge_materials: ResMut<Assets<EdgeMaterial>>,
    edge_gltf_query: Query<&EdgeMaterialGltf>,
    children_query: Query<&Children>,
    mesh_query: Query<(Entity, &Mesh3d)>,
) {
    let entity = trigger.event().entity;
    let Ok(settings) = edge_gltf_query.get(entity) else {
        return;
    };

    // Iterate over all descendants and apply edge material to meshes
    for descendant in children_query.iter_descendants(entity) {
        if let Ok((mesh_entity, mesh_handle)) = mesh_query.get(descendant) {
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                // Check if already processed
                if mesh.attribute(ATTRIBUTE_BARYCENTRIC).is_none() {
                    prepare_mesh_for_edge_rendering(mesh);
                }

                // Replace standard material with edge material
                commands
                    .entity(mesh_entity)
                    .remove::<MeshMaterial3d<StandardMaterial>>()
                    .insert(MeshMaterial3d(edge_materials.add(EdgeMaterial {
                        color: settings.color,
                        line_thickness: settings.line_thickness,
                    })));
            }
        }
    }
}

/// Custom vertex attribute for barycentric coordinates
pub const ATTRIBUTE_BARYCENTRIC: MeshVertexAttribute =
    MeshVertexAttribute::new("Barycentric", 988540917, VertexFormat::Float32x3);

/// Uniform data sent to the GPU shader
#[derive(Clone, Debug, ShaderType)]
pub struct EdgeMaterialUniform {
    pub color: Vec4,
    /// x = line_thickness, yzw = padding
    pub settings: Vec4,
}

/// A material that renders mesh edges/wireframe with configurable color and line thickness.
///
/// **Important:** The mesh must have barycentric coordinates. Use `prepare_mesh_for_edge_rendering`
/// to add them to your mesh before using this material.
///
/// # Example
/// ```ignore
/// // Prepare the mesh
/// let mut mesh = Cuboid::default().mesh().build();
/// prepare_mesh_for_edge_rendering(&mut mesh);
///
/// commands.spawn((
///     Mesh3d(meshes.add(mesh)),
///     MeshMaterial3d(materials.add(EdgeMaterial {
///         color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
///         line_thickness: 2.0,
///     })),
/// ));
/// ```
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[uniform(0, EdgeMaterialUniform)]
pub struct EdgeMaterial {
    /// The color of the edge lines
    pub color: LinearRgba,
    /// The thickness of the edge lines (default: 1.0)
    pub line_thickness: f32,
}

impl From<&EdgeMaterial> for EdgeMaterialUniform {
    fn from(material: &EdgeMaterial) -> Self {
        Self {
            color: Vec4::new(
                material.color.red,
                material.color.green,
                material.color.blue,
                material.color.alpha,
            ),
            settings: Vec4::new(material.line_thickness, 0.0, 0.0, 0.0),
        }
    }
}

impl Default for EdgeMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            line_thickness: 1.0,
        }
    }
}

impl Material for EdgeMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/edge_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/edge_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Mask(0.5)
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_BARYCENTRIC.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

/// Prepares a mesh for edge rendering by adding barycentric coordinates.
///
/// This function duplicates vertices so each triangle has unique vertices with
/// barycentric coordinates (1,0,0), (0,1,0), and (0,0,1) for each corner.
///
/// Call this on your mesh before adding it to the asset server.
pub fn prepare_mesh_for_edge_rendering(mesh: &mut Mesh) {
    let Some(indices) = mesh.indices() else {
        return;
    };

    let indices: Vec<u32> = match indices {
        Indices::U16(v) => v.iter().map(|i| *i as u32).collect(),
        Indices::U32(v) => v.clone(),
    };

    // Get existing vertex attributes
    let positions: Vec<[f32; 3]> = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attr| match attr {
            VertexAttributeValues::Float32x3(v) => Some(v.clone()),
            _ => None,
        })
        .unwrap_or_default();

    let normals: Option<Vec<[f32; 3]>> =
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            .and_then(|attr| match attr {
                VertexAttributeValues::Float32x3(v) => Some(v.clone()),
                _ => None,
            });

    let uvs: Option<Vec<[f32; 2]>> =
        mesh.attribute(Mesh::ATTRIBUTE_UV_0)
            .and_then(|attr| match attr {
                VertexAttributeValues::Float32x2(v) => Some(v.clone()),
                _ => None,
            });

    // Create new vertex data with duplicated vertices for each triangle
    let triangle_count = indices.len() / 3;
    let new_vertex_count = triangle_count * 3;

    let mut new_positions = Vec::with_capacity(new_vertex_count);
    let mut new_normals = Vec::with_capacity(new_vertex_count);
    let mut new_uvs = Vec::with_capacity(new_vertex_count);
    let mut barycentrics = Vec::with_capacity(new_vertex_count);
    let mut new_indices = Vec::with_capacity(indices.len());

    let barycentric_coords = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    for tri in 0..triangle_count {
        for corner in 0..3 {
            let old_idx = indices[tri * 3 + corner] as usize;

            new_positions.push(positions[old_idx]);

            if let Some(ref n) = normals {
                new_normals.push(n[old_idx]);
            }

            if let Some(ref u) = uvs {
                new_uvs.push(u[old_idx]);
            }

            barycentrics.push(barycentric_coords[corner]);
            new_indices.push((tri * 3 + corner) as u32);
        }
    }

    // Replace mesh data
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);

    if !new_normals.is_empty() {
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
    }

    if !new_uvs.is_empty() {
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, new_uvs);
    }

    mesh.insert_attribute(ATTRIBUTE_BARYCENTRIC, barycentrics);
    mesh.insert_indices(Indices::U32(new_indices));
}
