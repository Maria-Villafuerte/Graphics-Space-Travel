use nalgebra_glm::{dot, mat4_to_mat3, normalize, Mat3, Vec2, Vec3, Vec4};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;


pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}


pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  match uniforms.current_shader {
      // Different Earth-like planets
      1 => tropical_earth_shader(fragment, uniforms, time),    // Lush tropical version
      3 => frozen_earth_shader(fragment, uniforms, time),      // Ice age version
      5 => desert_earth_shader(fragment, uniforms, time),      // Desert world
      2 => ocean_earth_shader(fragment, uniforms, time),       // Water world
      7 => jungle_earth_shader(fragment, uniforms, time),      // Dense jungle world
      4 => volcanic_earth_shader(fragment, uniforms, time),    // Volcanic active Earth
      6 => ancient_earth_shader(fragment, uniforms, time),     // Primordial Earth
      _ => (Color::new(0, 0, 0), 0),
  }
}

fn tropical_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x,
      fragment.vertex_position.y
  );
  
  // Vibrant tropical colors
  let water_color_1 = Color::from_float(0.0, 0.5, 0.8);     // Bright turquoise
  let water_color_2 = Color::from_float(0.1, 0.6, 0.9);     // Light blue
  let land_color_1 = Color::from_float(0.2, 0.8, 0.3);      // Bright green
  let land_color_2 = Color::from_float(0.8, 0.8, 0.2);      // Sandy yellow
  let cloud_color = Color::from_float(1.0, 1.0, 1.0);       // White clouds

  let land_threshold = 0.4;

  let base_color = if noise_value > land_threshold {
      land_color_1.lerp(&land_color_2, (noise_value - land_threshold) / (1.0 - land_threshold))
  } else {
      water_color_1.lerp(&water_color_2, noise_value / land_threshold)
  };

  // Enhanced atmospheric effects
  let light_position = Vec3::new(1.0, 1.0, 2.0);
  let light_dir = normalize(&(light_position - fragment.vertex_position));
  let normal = normalize(&fragment.normal);
  let diffuse = dot(&normal, &light_dir).max(0.0);

  (base_color * (0.2 + 0.8 * diffuse), 0)
}

fn frozen_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x * 2.0,
      fragment.vertex_position.y * 2.0
  );

  // Ice world colors
  let ice_color_1 = Color::from_float(0.8, 0.9, 0.95);      // White ice
  let ice_color_2 = Color::from_float(0.7, 0.8, 0.9);       // Blue ice
  let water_color = Color::from_float(0.2, 0.3, 0.5);       // Dark cold water
  
  let ice_threshold = 0.3;
  
  let base_color = if noise_value > ice_threshold {
      ice_color_1.lerp(&ice_color_2, (noise_value - ice_threshold) / (1.0 - ice_threshold))
  } else {
      water_color
  };

  // Crystalline reflection effect
  let light_dir = normalize(&Vec3::new(1.0, 1.0, 1.0));
  let normal = normalize(&fragment.normal);
  let diffuse = dot(&normal, &light_dir).max(0.0).powf(1.5);

  (base_color * (0.3 + 0.7 * diffuse), 0)
}

fn desert_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x * 3.0,
      fragment.vertex_position.y * 3.0
  );

  // Desert world colors
  let sand_color_1 = Color::from_float(0.9, 0.8, 0.5);      // Light sand
  let sand_color_2 = Color::from_float(0.8, 0.6, 0.3);      // Dark sand
  let rock_color = Color::from_float(0.5, 0.4, 0.3);        // Rocky outcrops
  
  let terrain_threshold = 0.5;
  
  let base_color = if noise_value > terrain_threshold {
      sand_color_1.lerp(&sand_color_2, (noise_value - terrain_threshold) / (1.0 - terrain_threshold))
  } else {
      rock_color.lerp(&sand_color_2, noise_value / terrain_threshold)
  };

  // Strong sun lighting
  let light_dir = normalize(&Vec3::new(0.0, 1.0, 0.5));
  let normal = normalize(&fragment.normal);
  let diffuse = dot(&normal, &light_dir).max(0.0);

  (base_color * (0.4 + 0.6 * diffuse), 0)
}

fn ocean_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x + time as f32 * 0.001,
      fragment.vertex_position.y + time as f32 * 0.001
  );

  // Ocean world colors with deep trenches
  let shallow_water = Color::from_float(0.0, 0.5, 0.8);     // Shallow water
  let deep_water = Color::from_float(0.0, 0.2, 0.5);        // Deep water
  let trench_water = Color::from_float(0.0, 0.1, 0.3);      // Oceanic trenches

  let depth = (noise_value + 1.0) / 2.0;
  
  let base_color = if depth > 0.7 {
      shallow_water.lerp(&deep_water, (depth - 0.7) / 0.3)
  } else {
      deep_water.lerp(&trench_water, depth / 0.7)
  };

  // Wave and reflection effects
  let wave_effect = (time as f32 * 0.01 + noise_value * 10.0).sin() * 0.1;
  let light_dir = normalize(&Vec3::new(1.0, 1.0, 1.0));
  let normal = normalize(&fragment.normal);
  let diffuse = (dot(&normal, &light_dir) + wave_effect).max(0.0);

  (base_color * (0.2 + 0.8 * diffuse), 0)
}

fn jungle_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x * 4.0,
      fragment.vertex_position.y * 4.0
  );

  // Dense vegetation colors
  let canopy_color_1 = Color::from_float(0.1, 0.6, 0.2);    // Light canopy
  let canopy_color_2 = Color::from_float(0.0, 0.4, 0.1);    // Dark canopy
  let water_color = Color::from_float(0.2, 0.3, 0.1);       // Murky water

  let vegetation_threshold = 0.2;
  
  let base_color = if noise_value > vegetation_threshold {
      canopy_color_1.lerp(&canopy_color_2, (noise_value - vegetation_threshold) / (1.0 - vegetation_threshold))
  } else {
      water_color
  };

  // Humid atmosphere effect
  let light_dir = normalize(&Vec3::new(1.0, 1.0, 0.5));
  let normal = normalize(&fragment.normal);
  let diffuse = dot(&normal, &light_dir).max(0.0);
  let humidity = (time as f32 * 0.001).sin() * 0.1;

  (base_color * (0.3 + 0.7 * diffuse + humidity), 0)
}

fn volcanic_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x * 2.0,
      fragment.vertex_position.y * 2.0
  );

  // Volcanic terrain colors
  let lava_color = Color::from_float(0.9, 0.3, 0.1);        // Bright lava
  let rock_color_1 = Color::from_float(0.3, 0.2, 0.2);      // Dark rock
  let rock_color_2 = Color::from_float(0.5, 0.3, 0.2);      // Light rock

  let lava_threshold = 0.7;
  
  let base_color = if noise_value > lava_threshold {
      lava_color
  } else {
      rock_color_1.lerp(&rock_color_2, noise_value / lava_threshold)
  };

  // Lava glow effect
  let glow = if noise_value > lava_threshold {
      50 // Emission value for lava
  } else {
      0
  };

  // Hot atmosphere effect
  let light_dir = normalize(&Vec3::new(0.0, 1.0, 1.0));
  let normal = normalize(&fragment.normal);
  let diffuse = dot(&normal, &light_dir).max(0.0);
  let heat_distortion = (time as f32 * 0.002 + noise_value * 5.0).sin() * 0.1;

  (base_color * (0.2 + 0.8 * (diffuse + heat_distortion)), glow)
}

fn ancient_earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
  let noise_value = uniforms.noise.get_noise_2d(
      fragment.vertex_position.x * 1.5,
      fragment.vertex_position.y * 1.5
  );

  // Primordial Earth colors
  let lava_color = Color::from_float(0.8, 0.4, 0.1);        // Cooling lava
  let rock_color = Color::from_float(0.4, 0.3, 0.3);        // Basalt
  let steam_color = Color::from_float(0.7, 0.7, 0.7);       // Steam clouds

  let terrain_threshold = 0.5;
  let steam_threshold = 0.8;
  
  let base_color = if noise_value > steam_threshold {
      steam_color
  } else if noise_value > terrain_threshold {
      lava_color.lerp(&rock_color, (noise_value - terrain_threshold) / (steam_threshold - terrain_threshold))
  } else {
      rock_color
  };

  // Atmospheric chaos effect
  let light_dir = normalize(&Vec3::new(1.0, 0.5, 0.0));
  let normal = normalize(&fragment.normal);
  let diffuse = dot(&normal, &light_dir).max(0.0);
  let chaos = (time as f32 * 0.003 + noise_value * 3.0).sin() * 0.2;

  let glow = if noise_value > terrain_threshold { 20 } else { 0 };

  (base_color * (0.3 + 0.7 * (diffuse + chaos)), glow)
}