use bevy::{
    prelude::*,
    sprite::Mesh2dHandle,
};
use crate::{
    events::{FlickerStartEvent, FlickerEndEvent},
    components::{NoFlicker, Flickered, ImageSave, TextureAtlasSave},
    flicker::FlickerMaterial,
};

pub fn flicker_start(
    query: Query<AnyOf<((&Handle<Image>, &Sprite), (&Handle<TextureAtlas>, &TextureAtlasSprite))>, Without<NoFlicker>>,
    mut flicker_materials: ResMut<Assets<FlickerMaterial>>,
    mut flicker_start_events: EventReader<FlickerStartEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    for e in flicker_start_events.iter() {
    
        let (material, mesh_size) = if let (Ok(handle), Ok(sprite)) = (query.get_component::<Handle<Image>>(e.0), query.get_component::<Sprite>(e.0)) {
            if let Some(image) = images.get(handle) {
                commands.entity(e.0)
                    .insert(ImageSave(handle.clone()))
                    .remove::<Handle<Image>>();
                info!("Flicker image size: {:?}", sprite.custom_size.unwrap_or(image.size()));
                (FlickerMaterial { source_image: handle.clone(), ..default() }, sprite.custom_size.unwrap_or(image.size()))
            }
            else {
                error!("Could not get image from image handle to begin flicker");
                continue;
            }
        }
        else if let (Ok(handle), Ok(tas)) = (query.get_component::<Handle<TextureAtlas>>(e.0), query.get_component::<TextureAtlasSprite>(e.0)) {
            let index = query.get_component::<TextureAtlasSprite>(e.0).unwrap().index;
            if let Some((atlas, img_handle)) = atlases.get(handle).and_then(|atlas| Some((atlas, atlas.texture.clone()))) {
                if let Some(img) = images.get(&img_handle) {
                    let curr_rect = atlas.textures.get(index).copied().unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0));
                    let rect_size = Vec2::new(curr_rect.width(), curr_rect.height());
                    let img_size = img.size();
                    let ratio = img_size / rect_size;
                    let offset = curr_rect.min / img_size;
                    let size = rect_size / img_size;
                    if let Some(mut entity_commands) = commands.get_entity(e.0) {
                        entity_commands.insert(TextureAtlasSave(handle.clone()))
                            .remove::<Handle<TextureAtlas>>();
                        (FlickerMaterial {
                            source_image: atlas.texture.clone(),
                            offset,
                            size,
                            ratio,
                            ..default()
                        },
                        tas.custom_size.unwrap_or(rect_size))
                    }
                    else {
                        error!("Entity {:?} no longer exists.", e.0);
                        continue;
                    }
                }
                else {
                    error!("Could not get image from image handle to begin flicker from sprite sheet");
                    continue;
                }
            }
            else {
                error!("Could not get atlas to determine which part of sprite is currently active");
                continue
            }
        }
        else {
            warn!("Attempted to flicker on a despawned or sprite-less entity {:?}", e.0);
            continue
        };
        
        if let Some(mut entity_commands) = commands.get_entity(e.0) {
            entity_commands.insert((
                flicker_materials.add(material), 
                Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(mesh_size)))),
                Flickered::default(),
            ));
        }

    }
}

pub fn flicker_end(
    query: Query<(&Handle<FlickerMaterial>, AnyOf<(&ImageSave, &TextureAtlasSave)>), Without<NoFlicker>>,
    mut flicker_end_events: EventReader<FlickerEndEvent>,
    mut commands: Commands,
) {
    for e in flicker_end_events.iter() {
        if let Ok(_) = query.get_component::<Handle<FlickerMaterial>>(e.0) {
            if let Ok(ImageSave(handle)) = query.get_component::<ImageSave>(e.0) {
                
                if let Some(mut entity_commands) = commands.get_entity(e.0) {
                    entity_commands.insert(handle.clone())
                        .remove::<Handle<FlickerMaterial>>()
                        .remove::<ImageSave>()
                        .remove::<Flickered>();
                }
            }
            else if let Ok(TextureAtlasSave(texture_atlas_save)) = query.get_component::<TextureAtlasSave>(e.0) {
                if let Some(mut entity_commands) = commands.get_entity(e.0) {
                    entity_commands.insert(texture_atlas_save.clone())
                        .remove::<Flickered>()
                        .remove::<TextureAtlasSave>()
                        .remove::<Handle<FlickerMaterial>>();
                }
            }
        }
        else {
            #[cfg(feature = "warnings")]
            warn!("Tried ending flicker for an invalid entity: {:?}", e.0);
        }
    }
}


pub fn flicker_tick(
    mut flickered: Query<(Entity, &mut Flickered)>,
    mut flicker_end_events: EventWriter<FlickerEndEvent>,
    time: Res<Time>,
) {
    for (entity, mut flickered) in flickered.iter_mut() {
        flickered.0.tick(time.delta());
        if flickered.0.finished() {
            flicker_end_events.send(FlickerEndEvent(entity));
        }
    }
}
