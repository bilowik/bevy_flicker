## Bevy Flicker

This plugin facilitates creating a brief overlay/mix of a specific color over a sprite or mesh.

To trigger a flicker, you can send a FlickerStartEvent, which will contain the parameters
that dictate the color, length, and strength of the flicker. 

Included is also a RepeatingFlicker component that will send a FlickerStartEvent on an interval.

This also works on textures with alpha, the overlay takes into account the alpha of the 
underlying texture and will adjust the overlay alpha so that it's intensity is proportional between
different underlying alpha values. So an underlying 0.2 alpha value will reduce the alpha of the overlay by
80%. For alpha values of 0, the overlay's alpha will also be 0.  
See [this alpha example](https://github.com/bilowik/bevy_flicker/tree/main/examples/alpha_flicker.rs) for 
a visual of this effect, you will notice that the underlying color is still looks consistent across the
sprite. 

See more, complete examples (some shown below) [here](https://github.com/bilowik/bevy_flicker/tree/main/examples)

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
___

| `cargo run --example mesh_flicker` | 
|:--:|
|![5b-1OxIy](https://github.com/bilowik/bevy_flicker/assets/43679332/e0024971-57d9-4300-ba9d-f931f5212f75)|
___

| `cargo run --example alpha_flicker` |
|:--:|
|![lzwT7Hl-](https://github.com/bilowik/bevy_flicker/assets/43679332/1afffb11-541c-4d66-8108-621ec38f430e)|


