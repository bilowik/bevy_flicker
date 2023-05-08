use crate::{
    components::{Flickered, ImageSave, MeshColorSave, NoFlicker, TextureAtlasSave, RepeatingFlicker},
    config::FlickerPluginConfig,
    events::{FlickerEndEvent, FlickerStartEvent},
    flicker::FlickerMaterial,
};
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};

pub(crate) fn flicker_start(
    sprites: Query<(AnyOf<(&Handle<Image>, &ImageSave)>, &Sprite), Without<NoFlicker>>,
    tass: Query<
        (
            AnyOf<(&Handle<TextureAtlas>, &TextureAtlasSave)>,
            &TextureAtlasSprite,
        ),
        Without<NoFlicker>,
    >,
    mesh_with_color_materials: Query<
        (
            &Mesh2dHandle,
            &Handle<ColorMaterial>,
            Option<&MeshColorSave>,
        ),
        Without<NoFlicker>,
    >,
    mut flicker_materials: ResMut<Assets<FlickerMaterial>>,
    mut flicker_start_events: EventReader<FlickerStartEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    flickereds: Query<&Flickered>,
    config: Res<FlickerPluginConfig>,
) {
    for e in flicker_start_events.iter() {
        if flickereds.get(e.entity).is_ok() && config.ignore_overlap() {
            // We ignore this flicker event entirely.
            continue;
        }

        // Get image handle or image handle save
        let (material, mesh_size) = if let (Ok(handle), Ok(sprite)) = (
            sprites
                .get_component::<Handle<Image>>(e.entity)
                .cloned()
                .or(sprites
                    .get_component::<ImageSave>(e.entity)
                    .map(|img_save| img_save.0.clone())),
            sprites.get_component::<Sprite>(e.entity),
        ) {
            if let Some(image) = images.get(&handle) {
                (
                    FlickerMaterial {
                        source_image: Some(handle.clone()),
                        color: e.color,
                        mix_scalar: e.mix_scalar,
                        ..default()
                    },
                    sprite.custom_size.unwrap_or(image.size()),
                )
            } else {
                error!("Could not get image from image handle to begin flicker");
                continue;
            }
        }
        // Get texture atlas or texture atlas save
        else if let (Ok(handle), Ok(tas)) = (
            tass.get_component::<Handle<TextureAtlas>>(e.entity)
                .cloned()
                .or(tass
                    .get_component::<TextureAtlasSave>(e.entity)
                    .map(|s| s.0.clone())),
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
                    if let Some(mut entity_commands) = commands.get_entity(e.entity) {
                        (
                            FlickerMaterial {
                                source_image: Some(atlas.texture.clone()),
                                offset,
                                size,
                                ratio,
                                color: e.color,
                                mix_scalar: e.mix_scalar,
                                ..default()
                            },
                            tas.custom_size.unwrap_or(rect_size),
                        )
                    } else {
                        error!("Entity {:?} no longer exists.", e.entity);
                        continue;
                    }
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
            mesh_with_color_materials.get_component::<Mesh2dHandle>(e.entity)
        {
            if let Some(mesh) = meshes.get(&mesh_handle.0).cloned() {
                if let Some(mut entity_commands) = commands.get_entity(e.entity) {
                    entity_commands.with_children(|parent| {
                        parent.spawn(MaterialMesh2dBundle {
                            material: flicker_materials.add(FlickerMaterial {
                                color: e.color,
                                ..default()
                            }),
                            mesh: meshes.add(mesh).into(),
                            ..default()
                        })
                        .insert(Flickered::with_secs(e.secs));
                    });
                }
            }
            
            continue;
        } else {
            warn!(
                "Attempted to flicker on a despawned or sprite-less entity {:?}",
                e.entity
            );
            continue;
        };

        if let Some(mut entity_commands) = commands.get_entity(e.entity) {
            entity_commands.with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    material: flicker_materials.add(material),
                    mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(mesh_size)))),
                    ..default()
                })
                .insert(Flickered::with_secs(e.secs));
            });
            
        }
    }
}

pub(crate) fn flicker_end(
    query: Query<
        AnyOf<(
            &Handle<FlickerMaterial>,
            AnyOf<(&ImageSave, &TextureAtlasSave)>,
            &MeshColorSave,
        )>,
    >,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut flicker_end_events: EventReader<FlickerEndEvent>,
    mut commands: Commands,
) {
    for e in flicker_end_events.iter() {
        if let Ok(_) = query.get_component::<Handle<FlickerMaterial>>(e.entity) {
            if let Ok(ImageSave(handle)) = query.get_component::<ImageSave>(e.entity) {
                if let Some(mut entity_commands) = commands.get_entity(e.entity) {
                    entity_commands
                        .insert(handle.clone())
                        .remove::<Handle<FlickerMaterial>>()
                        .remove::<ImageSave>()
                        .remove::<Flickered>();
                }
            } else if let Ok(TextureAtlasSave(texture_atlas_save)) =
                query.get_component::<TextureAtlasSave>(e.entity)
            {
                if let Some(mut entity_commands) = commands.get_entity(e.entity) {
                    entity_commands
                        .insert(texture_atlas_save.clone())
                        .remove::<Flickered>()
                        .remove::<TextureAtlasSave>()
                        .remove::<Handle<FlickerMaterial>>();
                }
            }
        } else if let Ok(MeshColorSave(orig_color)) = query.get_component::<MeshColorSave>(e.entity)
        {
            if let Some(mut entity_commands) = commands.get_entity(e.entity) {
                entity_commands
                    .remove::<Flickered>()
                    .remove::<MeshColorSave>()
                    .remove::<Handle<ColorMaterial>>()
                    .insert(color_materials.add(ColorMaterial {
                        color: *orig_color,
                        texture: None,
                    }));
            }
        } else {
            #[cfg(feature = "warnings")]
            warn!("Tried ending flicker for an invalid entity: {:?}", e.entity);
        }
    }
}

pub(crate) fn flicker_tick(
    mut flickered: Query<(Entity, &mut Flickered)>,
    mut flicker_end_events: EventWriter<FlickerEndEvent>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut flickered) in flickered.iter_mut() {
        flickered.0.tick(time.delta());
        if flickered.0.finished() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
           // flicker_end_events.send(FlickerEndEvent { entity });
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

