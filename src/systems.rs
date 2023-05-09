use crate::{
    components::{Flickered, NoFlicker, RepeatingFlicker, FlickerMarker},
    config::FlickerPluginConfig,
    events::FlickerStartEvent,
    flicker::FlickerMaterial,
};
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};

pub(crate) fn flicker_start(
    sprites: Query<(&Sprite, &Handle<Image>), Without<NoFlicker>>,
    tass: Query<(&TextureAtlasSprite, &Handle<TextureAtlas>), Without<NoFlicker>>,
    mesh_components: Query<&Mesh2dHandle, Without<NoFlicker>>,
    mut flicker_materials: ResMut<Assets<FlickerMaterial>>,
    mut flicker_start_events: EventReader<FlickerStartEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    flickereds: Query<&FlickerMarker>,
    config: Res<FlickerPluginConfig>,
    with_children: Query<&Children>,
    flicker_children: Query<Entity, With<Flickered>>,
) {
    for e in flicker_start_events.iter() {
        if flickereds.get(e.entity).is_ok() && config.ignore_overlap() {
            // We ignore this flicker event entirely.
            continue;
        }

        // Get image handle or image handle save
        let (material, mesh) = if let (Ok(handle), Ok(sprite)) = (
            sprites
                .get_component::<Handle<Image>>(e.entity)
                .cloned(),
            sprites.get_component::<Sprite>(e.entity),
        ) {
            if let Some(image) = images.get(&handle) {
                (
                    FlickerMaterial {
                        source_image: Some(handle.clone()),
                        color: e.color,
                        ..default()
                    },
                    Mesh::from(shape::Quad::new(sprite.custom_size.unwrap_or(image.size()))),
                )
            } else {
                error!("Could not get image from image handle to begin flicker");
                continue;
            }
        }
        // Get texture atlas or texture atlas save
        else if let (Ok(handle), Ok(tas)) = (
            tass.get_component::<Handle<TextureAtlas>>(e.entity)
                .cloned(),
            tass.get_component::<TextureAtlasSprite>(e.entity),
        ) {
            let index = tass
                .get_component::<TextureAtlasSprite>(e.entity)
                .unwrap()
                .index;
            if let Some((atlas, img_handle)) = atlases
                .get(&handle)
                .and_then(|atlas| Some((atlas, atlas.texture.clone())))
            {
                if let Some(img) = images.get(&img_handle) {
                    let curr_rect = atlas
                        .textures
                        .get(index)
                        .copied()
                        .unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0));
                    let rect_size = Vec2::new(curr_rect.width(), curr_rect.height());
                    let img_size = img.size();
                    let ratio = img_size / rect_size;
                    let offset = curr_rect.min / img_size;
                    let size = rect_size / img_size;
                    (
                        FlickerMaterial {
                            source_image: Some(atlas.texture.clone()),
                            offset,
                            size,
                            ratio,
                            color: e.color,
                            ..default()
                        },
                        Mesh::from(shape::Quad::new(tas.custom_size.unwrap_or(rect_size))),
                    )
                } else {
                    error!(
                        "Could not get image from image handle to begin flicker from sprite sheet"
                    );
                    continue;
                }
            } else {
                error!("Could not get atlas to determine which part of sprite is currently active");
                continue;
            }
        } else if let Ok(mesh_handle) =
            mesh_components.get_component::<Mesh2dHandle>(e.entity)
        {
            if let Some(mesh) = meshes.get(&mesh_handle.0).cloned() {
                (
                    FlickerMaterial {
                        color: e.color,
                        ..default()
                    },
                    mesh
                )
            }
            else {
                error!("Entity {:?} had an invalid mesh handle", e.entity);
                continue;
            }
        } else {
            warn!(
                "Attempted to flicker on a despawned or sprite-less entity {:?}",
                e.entity
            );
            continue;
        };


        if !config.ignore_overlap() {
            // Despawn any previous flickering children
            if let Ok(children) = with_children.get(e.entity) {
                // Iterate over the children and remove any flickers
                for child in children {
                    if flicker_children.contains(*child) {
                        if let Some(mut entity_commands) = commands.get_entity(*child) {
                            entity_commands.despawn();
                        }
                    }
                }
            }
        }

        if let Some(mut entity_commands) = commands.get_entity(e.entity) {
            entity_commands.with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    material: flicker_materials.add(material),
                    mesh: Mesh2dHandle(meshes.add(mesh)),
                    ..default()
                })
                .insert(Flickered::with_secs(e.secs));
            });
            entity_commands.insert(FlickerMarker);
            
        }
    }
}


pub(crate) fn flicker_tick(
    mut flickered: Query<(&Parent, Entity, &mut Flickered)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (parent, entity, mut flickered) in flickered.iter_mut() {
        flickered.0.tick(time.delta());
        if flickered.0.finished() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
            if let Some(mut entity_commands) = commands.get_entity(parent.get()) {
                entity_commands.remove::<FlickerMarker>();
            }
        }
    }
}


pub(crate) fn repeating_flicker_tick(
    mut repeating_flickers: Query<(Entity, &mut RepeatingFlicker)>,
    mut flicker_start_event_writer: EventWriter<FlickerStartEvent>,
    time: Res<Time>,
) {
    for (entity, mut repeating_flicker) in repeating_flickers.iter_mut() {
        repeating_flicker.timer.tick(time.delta());
        if repeating_flicker.timer.just_finished() {
            // The pause has finished, flicker again
            flicker_start_event_writer.send(repeating_flicker.generate_start_event(entity)); 
        }
    }
}

