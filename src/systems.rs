use crate::{
    components::{FlickerMarker, Flickered, NoFlicker, RepeatingFlicker},
    config::FlickerPluginConfig,
    events::FlickerStartEvent,
    flicker::FlickerMaterial,
};

use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    hierarchy::Children,
    query::{With, Without},
    system::{Commands, Query, Res, ResMut},
};
use bevy_sprite::MeshMaterial2d;

use bevy_asset::Assets;
use bevy_image::{Image, TextureAtlasLayout};
use bevy_log::{error, warn};
use bevy_math::{primitives::Rectangle, URect, Vec2, Vec3};
use bevy_render::mesh::{Mesh, Mesh2d};
use bevy_sprite::Sprite;
use bevy_time::Time;
use bevy_transform::components::Transform;

pub(crate) fn flicker_start(
    sprites: Query<&Sprite, Without<NoFlicker>>,
    mesh_components: Query<&Mesh2d, Without<NoFlicker>>,
    mut flicker_materials: ResMut<Assets<FlickerMaterial>>,
    mut flicker_start_events: EventReader<FlickerStartEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    flickereds: Query<&FlickerMarker>,
    config: Res<FlickerPluginConfig>,
    with_children: Query<&Children>,
    flicker_children: Query<Entity, With<Flickered>>,
) {
    for e in flicker_start_events.read() {
        if flickereds.get(e.entity).is_ok() && config.ignore_overlap() {
            // We ignore this flicker event entirely.
            continue;
        }

        // Get image handle or image handle save
        let (material, mesh) = if let Ok(sprite) = sprites.get(e.entity) {
            let image_handle = &sprite.image;
            let img = if let Some(img) = images.get(image_handle) {
                img
            } else {
                error!("Could not get image from image handle to begin flicker");
                continue;
            };

            if let Some(texture_atlas) = sprite.texture_atlas.as_ref() {
                let index = texture_atlas.index;
                if let Some(atlas) = atlas_layouts.get(&texture_atlas.layout) {
                    let curr_rect = atlas
                        .textures
                        .get(index)
                        .copied()
                        .unwrap_or(URect::new(0, 0, 0, 0));
                    let rect_size = Vec2::new(curr_rect.width() as f32, curr_rect.height() as f32);
                    let img_size = img.size().as_vec2();
                    let ratio = img_size / rect_size;
                    let offset = curr_rect.min.as_vec2() / img_size;
                    let size = rect_size / img_size;
                    let mesh_size = sprite.custom_size.unwrap_or(rect_size);
                    (
                        FlickerMaterial {
                            source_image: Some(image_handle.clone()),
                            offset,
                            size,
                            ratio,
                            color: e.color.into(),
                            ..Default::default()
                        },
                        Mesh::from(Rectangle::new(mesh_size.x, mesh_size.y)),
                    )
                } else {
                    error!(
                        "Could not get atlas to determine which part of sprite is currently active"
                    );
                    continue;
                }
            } else {
                // No texture atlas, so go with the whole image.
                let mesh_size = sprite.custom_size.unwrap_or(img.size().as_vec2());
                (
                    FlickerMaterial {
                        source_image: Some(image_handle.clone()),
                        color: e.color.into(),
                        ..Default::default()
                    },
                    Mesh::from(Rectangle::new(mesh_size.x, mesh_size.y)),
                )
            }
        } else if let Ok(mesh_handle) = mesh_components.get(e.entity) {
            if let Some(mesh) = meshes.get(&mesh_handle.0).cloned() {
                (
                    FlickerMaterial {
                        color: e.color.into(),
                        ..Default::default()
                    },
                    mesh,
                )
            } else {
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
                        if let Ok(mut entity_commands) = commands.get_entity(*child) {
                            entity_commands.despawn();
                        }
                    }
                }
            }
        }

        if let Ok(mut entity_commands) = commands.get_entity(e.entity) {
            entity_commands.with_children(|parent| {
                parent.spawn((
                    MeshMaterial2d(flicker_materials.add(material)),
                    Mesh2d(meshes.add(mesh)),
                    Transform {
                        // Translation is relative to its parent, so 1.0 guarantees it is always in
                        // front of its parent.
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    Flickered::with_secs(e.secs),
                ));
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
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
            if let Ok(mut entity_commands) = commands.get_entity(parent.get()) {
                entity_commands.remove::<FlickerMarker>();
            }
        }
    }
}

pub(crate) fn repeating_flicker_tick(
    mut repeating_flickers: Query<(Entity, &mut RepeatingFlicker)>,
    mut flicker_start_event_writer: EventWriter<FlickerStartEvent>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut repeating_flicker) in repeating_flickers.iter_mut() {
        if repeating_flicker.curr_pulse_count > 0 {
            // We still have flickers left in the current pulse.
            repeating_flicker.timer.tick(time.delta());
            if repeating_flicker.timer.just_finished() {
                // The pause has finished, flicker again
                flicker_start_event_writer.write(repeating_flicker.generate_start_event(entity));
                repeating_flicker.curr_pulse_count -= 1;
                if repeating_flicker.curr_pulse_count == 0 {
                    if let Some(count) = repeating_flicker.count.as_mut() {
                        // We have a finite count and we just finished a pulse, so decrement count and
                        // check for termination condition
                        *count -= 1;
                        if *count == 0 {
                            // We've finished flickering, remove.
                            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                                entity_commands.remove::<RepeatingFlicker>();
                            }
                        }
                    }
                }
            }
        } else {
            repeating_flicker.pulse_timer.tick(time.delta());
            if repeating_flicker.pulse_timer.just_finished() {
                // We are ready for another pulse.
                repeating_flicker.curr_pulse_count = repeating_flicker.pulse_count;
                // Also reset the flicker timer since there's likely a small amount of
                // overflow from the last tick.
                //repeating_flicker.timer.reset();
            }
        }
    }
}
