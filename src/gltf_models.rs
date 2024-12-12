use bevy::prelude::*;

/*
An auxilliary gltf model loading plugin --- not really related or coupled 

*/


pub fn gltf_models_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            add_gltf_model_scenes_on_added,
            add_gltf_model_scenes_on_load,
        )
            .chain(),
    );
}

#[derive(Reflect, Component)]
pub struct AddGltfModelComponent(pub Handle<Gltf>);

fn add_gltf_model_scenes_on_added(
    mut commands: Commands,

    entity_query: Query<(Entity, &AddGltfModelComponent), Added<AddGltfModelComponent>>,

    gltf_assets: Res<Assets<Gltf>>,
) {
    for (entity, add_gltf_model_comp) in entity_query.iter() {
        if let Some(gltf) = gltf_assets.get(&add_gltf_model_comp.0) {
            let Some(gltf_scene) = gltf.scenes.first() else {
                continue;
            };

            commands
                .entity(entity)
                .try_insert(SceneRoot(gltf_scene.clone()))
                .remove::<AddGltfModelComponent>();
        }
    }
}

fn add_gltf_model_scenes_on_load(
    mut commands: Commands,
    mut asset_ready_event: EventReader<AssetEvent<Gltf>>,
    gltf_assets: Res<Assets<Gltf>>,

    entity_query: Query<(Entity, &AddGltfModelComponent)>,
) {
    for evt in asset_ready_event.read() {
        match evt {
            AssetEvent::LoadedWithDependencies { id } => {
                let Some(loaded_gltf_asset) = gltf_assets.get(*id) else {
                    continue;
                };

                for (entity, add_gltf_comp) in entity_query.iter() {
                    if *id == add_gltf_comp.0.id() {
                        let Some(gltf_scene) = loaded_gltf_asset.scenes.first() else {
                            continue;
                        };

                        commands
                            .entity(entity)
                            .try_insert(SceneRoot(gltf_scene.clone()))
                            .remove::<AddGltfModelComponent>();
                    }
                }
            }

            _ => {}
        }
    }
}