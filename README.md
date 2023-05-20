## Bevy Flicker

This plugin facilitates creating a brief overlay/mix of a specific color over a sprite or mesh.

To trigger a flicker, you can send a FlickerStartEvent, which will contain the parameters
that dictate the color, length, and strength of the flicker. 

Included is also a RepeatingFlicker component that will send a FlickerStartEvent on an interval.

This also works on textures with alpha, the overlay takes into account the alpha of the 
underlying texture and will take the lowest alpha between the two. So if a texture has pixels
with an alpha of 0.0 and the overlay color has an alpha of 0.3, then the overlay above those pixels 
will also have an alpha of 0.0.

See more, complete examples [here](https://github.com/bilowik/bevy_flicker/tree/main/examples)

```rust
use bevy_flicker::prelude::*;

fn tick(query: Query<Entity>, mut event_writer: EventWriter<FlickerStartEvent>) {
    for e in query.iter() {
        event_writer.send(
            FlickerStartEvent::builder(e)
                .with_secs(0.5)
                .with_color(Color::rgba(0.0, 0.0, 1.0, 0.2))
                .build(),
        );
    }
}


```

### Visual Examples
| `cargo run --example sprite_flicker` | 
|:--:|
|![svuL4w_7](https://github.com/bilowik/bevy_flicker/assets/43679332/c259cfbb-a146-4d40-b7e3-90fa3b80d1a7)|

| `cargo run --example mesh_flicker` | 
|:--:|
|![5b-1OxIy](https://github.com/bilowik/bevy_flicker/assets/43679332/e0024971-57d9-4300-ba9d-f931f5212f75)|

