use bevy::ecs::system::{EntityCommands, SystemParam};
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use bevy::prelude::*;

pub struct GltfSpawnerPlugin;

impl Plugin for GltfSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_gltf_nodes);
    }
}

#[derive(Component)]
pub struct SpawnGltfNode(pub Handle<Gltf>, pub &'static str);

fn spawn_gltf_nodes(
    mut commands: Commands,
    query: Query<(Entity, &SpawnGltfNode)>,
    gltfs: Res<Assets<Gltf>>,
    spawner: Spawner,
) {
    for (entity, SpawnGltfNode(gltf, node_name)) in query.iter() {
        let gltf = if let Some(gltf) = gltfs.get(gltf) {
            gltf
        } else {
            continue;
        };
        let gltf_node = &gltf.named_nodes[*node_name];
        let gltf_node = spawner.gltf_nodes.get(gltf_node).unwrap();
        let mut cmd = commands.entity(entity);
        cmd.remove::<SpawnGltfNode>();
        spawner.spawn_node_recursive(&gltf_node, &mut cmd);
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
    fn spawn_node_recursive(&self, gltf_node: &GltfNode, cmd: &mut EntityCommands) {
        cmd.insert(gltf_node.transform);
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
                    self.spawn_node_recursive(child_node, &mut cmd);
                }
            });
        }
    }
}
