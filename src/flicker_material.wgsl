#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

struct FragmentInput {
    #import bevy_sprite::mesh2d_vertex_output
};

struct FlickerMaterial {
    offset: vec2<f32>,
    size: vec2<f32>,
    ratio: vec2<f32>,
    color: vec4<f32>,
}

@group(1) @binding(2)
var<uniform> flicker_material: FlickerMaterial;

//let ZERO: vec2<f32> = vec2<f32>(0.0, 0.0);

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let old_range = 1.0;
    let new_range = flicker_material.size;
    let uv = ((in.uv * new_range) + flicker_material.offset);
    let color = textureSample(texture, our_sampler, uv);

    // Return the flicker color except keep the alpha consistent with the underlying texture
    return vec4<f32>(flicker_material.color.r, flicker_material.color.g, flicker_material.color.b, min(flicker_material.color.a, color.a));
}
