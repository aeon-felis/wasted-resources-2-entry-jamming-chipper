use bevy::ecs::system::{EntityCommands, SystemParam};
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy_rapier2d::prelude::*;

pub struct GltfSpawnerPlugin;

impl Plugin for GltfSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GltfNodeAddedEvent>();
        app.add_system_to_stage(CoreStage::PostUpdate, spawn_gltf_nodes);
        app.add_system_to_stage(CoreStage::PostUpdate, spawn_colliders);
    }
}

#[derive(Component)]
pub struct SpawnGltfNode(pub Handle<Gltf>, pub &'static str);

pub struct GltfNodeAddedEvent(pub Entity);

#[derive(Component)]
pub struct SpawnCollider {
    pub gltf: Handle<Gltf>,
    pub node_name: &'static str,
    pub material: ColliderMaterial,
}

fn spawn_gltf_nodes(
    mut commands: Commands,
    query: Query<(Entity, &SpawnGltfNode, Option<&Transform>)>,
    gltfs: Res<Assets<Gltf>>,
    spawner: Spawner,
    mut event_writer: EventWriter<GltfNodeAddedEvent>,
) {
    for (entity, SpawnGltfNode(gltf, node_name), orig_transform) in query.iter() {
        let gltf = if let Some(gltf) = gltfs.get(gltf) {
            gltf
        } else {
            continue;
        };
        let gltf_node = &gltf.named_nodes[*node_name];
        let gltf_node = spawner.gltf_nodes.get(gltf_node).unwrap();
        let mut cmd = commands.entity(entity);
        cmd.remove::<SpawnGltfNode>();
        spawner.spawn_node_recursive(gltf_node, &mut cmd, orig_transform);
        event_writer.send(GltfNodeAddedEvent(entity));
    }
}

#[derive(SystemParam)]
struct Spawner<'w, 's> {
    gltf_nodes: Res<'w, Assets<GltfNode>>,
    gltf_meshes: Res<'w, Assets<GltfMesh>>,
    #[system_param(ignore)]
    marker: std::marker::PhantomData<&'s usize>,
}

impl<'w, 's> Spawner<'w, 's> {
    fn spawn_node_recursive(
        &self,
        gltf_node: &GltfNode,
        cmd: &mut EntityCommands,
        orig_transform: Option<&Transform>,
    ) {
        if let Some(orig_transform) = orig_transform {
            cmd.insert(*orig_transform * gltf_node.transform);
        } else {
            cmd.insert(gltf_node.transform);
        }
        if let Some(mesh) = &gltf_node.mesh {
            let mesh = self.gltf_meshes.get(mesh).unwrap();
            cmd.with_children(|commands| {
                for primitive in mesh.primitives.iter() {
                    commands.spawn_bundle(PbrBundle {
                        mesh: primitive.mesh.clone(),
                        material: primitive.material.clone().unwrap_or_default(),
                        ..Default::default()
                    });
                }
            });
        }
        if !gltf_node.children.is_empty() {
            cmd.with_children(|commands| {
                for child_node in gltf_node.children.iter() {
                    let mut cmd = commands.spawn();
                    cmd.insert(GlobalTransform::identity());
                    self.spawn_node_recursive(child_node, &mut cmd, None);
                }
            });
        }
    }
}

fn spawn_colliders(
    mut commands: Commands,
    query: Query<(Entity, &SpawnCollider)>,
    gltfs: Res<Assets<Gltf>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (
        entity,
        SpawnCollider {
            gltf,
            node_name,
            material,
        },
    ) in query.iter()
    {
        let gltf = if let Some(gltf) = gltfs.get(gltf) {
            gltf
        } else {
            continue;
        };
        let gltf_node = &gltf.named_nodes[*node_name];
        let gltf_node = gltf_nodes.get(gltf_node).unwrap();
        assert!(
            gltf_node.children.is_empty(),
            "Collider node {:?} must not have children",
            node_name
        );
        let mut cmd = commands.entity(entity);
        cmd.remove::<SpawnCollider>();
        let mesh = gltf_node
            .mesh
            .as_ref()
            .expect("Collider node must have a mesh");
        let mesh = gltf_meshes.get(mesh).unwrap();
        let mut it = mesh.primitives.iter();
        let primitive = it.next().expect("Collider node has no primitives");
        assert!(
            it.next().is_none(),
            "Collider node has more than on primitive"
        );
        assert!(
            primitive.material.is_none(),
            "Collider node {:?} must not have materials",
            node_name
        );
        let mesh = meshes.get(&primitive.mesh).unwrap();
        let tri_mesh = TriMesh::new(
            {
                if let VertexAttributeValues::Float32x3(vertices) =
                    mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
                {
                    vertices.iter().map(|&[x, y, _]| point![x, y]).collect()
                } else {
                    panic!("Not Float32x3")
                }
            },
            {
                let mut triangles = Vec::new();
                match mesh.indices().unwrap() {
                    Indices::U16(_indices) => {}
                    Indices::U32(indices) => {
                        for i in 0..indices.len() {
                            if i % 3 == 0 {
                                triangles.push([indices[i], indices[i + 1], indices[i + 2]]);
                            }
                        }
                    }
                }
                triangles
            },
        );
        cmd.insert_bundle(ColliderBundle {
            shape: SharedShape::new(tri_mesh).into(),
            material: (*material).into(),
            mass_properties: ColliderMassProps::Density(0.0).into(),
            ..Default::default()
        });
    }
}
