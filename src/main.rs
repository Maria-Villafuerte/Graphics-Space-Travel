use nalgebra_glm::{Vec3, Vec4, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::{f32::consts::PI, time::Instant};

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod solar_system;

use solar_system::SolarSystem;
use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
    cloud_noise: FastNoiseLite, 
    band_noise: FastNoiseLite, 
    current_shader: u8,
}

fn create_noise(current_shader: u8) -> FastNoiseLite {
    match current_shader {
        1 => create_earth_noise(),
        2 => create_mars_noise(),
        3 => create_mercury_noise(),
        4 => FastNoiseLite::new(),
        5 => create_jupiter_noise(),
        6 => create_urano_noise(), 
        8 => create_moon_noise(),
        9 => FastNoiseLite::new(),
        _ => create_earth_noise(),  
    }
}

fn create_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_lacunarity(Some(3.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.5)); 
    noise
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(40);  
    noise.set_noise_type(Some(NoiseType::Perlin)); 
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(2));
    noise.set_fractal_lacunarity(Some(3.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.01));
    noise
}

fn create_mars_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1234);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(1.5)); 
    noise
}

fn create_moon_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(2));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(3.0));  
    noise
}

fn create_mercury_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(1.0));
    noise.set_frequency(Some(5.0));  
    noise
}

fn create_jupiter_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(5678);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::DomainWarpProgressive));
    noise.set_fractal_octaves(Some(6));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(2.0));
    noise
}

fn create_jupiter_band_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(7890);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_frequency(Some(1.0));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise
}

fn create_urano_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(2021);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.4));
    noise.set_frequency(Some(0.2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(camera_distance: f32, window_width: f32, window_height: f32) -> Mat4 {
    let fov = 60.0 * PI / 180.0;
    let aspect_ratio = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = camera_distance * 3.0;
    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn gaussian_blur(buffer: &mut [u32], width: usize, height: usize, kernel_size: usize, sigma: f32) {
    let gaussian_kernel = create_gaussian_kernel(kernel_size, sigma);
    let kernel_sum: f32 = gaussian_kernel.iter().map(|&x| x as f32).sum();

    for y in 0..height {
        let mut temp_row = vec![0u32; width];
        for x in 0..width {
            let mut filtered_pixel = 0f32;
            for k in 0..gaussian_kernel.len() {
                let sample_x = x as i32 + k as i32 - (gaussian_kernel.len() / 2) as i32;
                if sample_x >= 0 && sample_x < width as i32 {
                    filtered_pixel += buffer[sample_x as usize + y * width] as f32 * gaussian_kernel[k] as f32;
                }
            }
            temp_row[x] = (filtered_pixel / kernel_sum).round() as u32;
        }
        buffer[y * width..(y + 1) * width].copy_from_slice(&temp_row);
    }

    for x in 0..width {
        let mut temp_col = vec![0u32; height];
        for y in 0..height {
            let mut filtered_pixel = 0f32;
            for k in 0..gaussian_kernel.len() {
                let sample_y = y as i32 + k as i32 - (gaussian_kernel.len() / 2) as i32;
                if sample_y >= 0 && sample_y < height as i32 {
                    filtered_pixel += buffer[x + sample_y as usize * width] as f32 * gaussian_kernel[k] as f32;
                }
            }
            temp_col[y] = (filtered_pixel / kernel_sum).round() as u32;
        }
        for y in 0..height {
            buffer[x + y * width] = temp_col[y];
        }
    }
}

fn create_gaussian_kernel(size: usize, sigma: f32) -> Vec<u32> {
    let mut kernel = vec![0u32; size];
    let mean = (size as f32 - 1.0) / 2.0;
    let coefficient = 1.0 / (2.0 * std::f32::consts::PI * sigma * sigma).sqrt();

    for x in 0..size {
        let exp_numerator = -((x as f32 - mean) * (x as f32 - mean)) / (2.0 * sigma * sigma);
        let exp_value = (-exp_numerator).exp();
        kernel[x] = (coefficient * exp_value * 255.0) as u32;
    }

    kernel
}

fn apply_bloom(original: &mut [u32], bloom: &[u32], width: usize, height: usize) {
    for i in 0..original.len() {
        let original_color = original[i];
        let bloom_intensity = bloom[i];
        if bloom_intensity > 0 {
            original[i] = blend_bloom(original_color, bloom_intensity);
        }
    }
}

fn blend_bloom(base_color: u32, bloom_intensity: u32) -> u32 {
    let bloom_strength = 0.8;
    let max_bloom_effect = 1.2;

    let r = ((base_color >> 16) & 0xFF) as f32;
    let g = ((base_color >> 8) & 0xFF) as f32;
    let b = (base_color & 0xFF) as f32;
    let bloom = bloom_intensity as f32 * bloom_strength;

    let new_r = ((r + bloom).min(255.0 * max_bloom_effect)).min(255.0) as u32;
    let new_g = ((g + bloom).min(255.0 * max_bloom_effect)).min(255.0) as u32;
    let new_b = ((b + bloom).min(255.0 * max_bloom_effect)).min(255.0) as u32;

    (new_r << 16) | (new_g << 8) | new_b
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], time: u32) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let (shaded_color, emission) = fragment_shader(&fragment, &uniforms, time);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth, emission);
        }
    }
}

fn world_to_screen(point: Vec3, uniforms: &Uniforms) -> Vec3 {
    let pos = Vec4::new(point.x, point.y, point.z, 1.0);
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * pos;
    let w = transformed[3];
    let ndc = Vec4::new(
        transformed[0] / w,
        transformed[1] / w,
        transformed[2] / w,
        1.0
    );
    let screen = uniforms.viewport_matrix * ndc;
    Vec3::new(screen[0], screen[1], screen[2])
}

fn main() {
    let mut last_bloom_update = 0;
    let bloom_update_interval = 5;
    let system_radius = 20.0;
    let camera_distance = system_radius * 2.5;
    let camera_height = system_radius * 1.0;
    
    let window_width = 680;
    let window_height = 800;
    let framebuffer_width = window_width; 
    let framebuffer_height = window_height;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Solar System - Use WASD to move, Mouse to look",
        window_width,
        window_height,
        WindowOptions::default(),
    )
        .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000); // Fondo negro para el espacio

    // Inicializar la cámara en una posición elevada y alejada
    let mut camera = Camera::new(
        Vec3::new(camera_distance, camera_height, camera_distance),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let moon = Obj::load("assets/models/moon.obj").expect("Failed to load obj");
    let ring_obj = Obj::load("assets/models/ring.obj").expect("Failed to load ring model");
    let spaceship = Obj::load("assets/models/Navesita.obj").expect("Failed to load spaceship");

    let vertex_arrays = obj.get_vertex_array(); 
    let ring_vertex_array = ring_obj.get_vertex_array();
    let spaceship_vertex_array = spaceship.get_vertex_array();

    let mut last_frame_time = Instant::now();
    let mut time = 0;

    let projection_matrix = create_perspective_matrix(camera_distance, window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
    let mut uniforms = Uniforms { 
        model_matrix: Mat4::identity(), 
        view_matrix: Mat4::identity(), 
        projection_matrix, 
        viewport_matrix, 
        time: 0, 
        noise: create_noise(1),
        cloud_noise: create_cloud_noise(),
        band_noise: create_jupiter_band_noise(), 
        current_shader: 1,
    };

    let mut solar_system = SolarSystem::new();

    // Tracking del mouse
    let mut last_mouse_pos: Option<(f32, f32)> = None;
    window.set_cursor_visibility(false);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let delta_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();
        
        // Actualizar el sistema solar con la cámara
        solar_system.update(delta_time, &mut camera);
        
        // Manejar input
        handle_input(&window, &mut camera, &mut solar_system);
        
        // Manejar movimiento del mouse
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if let Some((last_x, last_y)) = last_mouse_pos {
                let delta_x = x - last_x;
                let delta_y = y - last_y;
                camera.handle_mouse_movement(delta_x, delta_y, 0.003);
            }
            last_mouse_pos = Some((x, y));
        }

        // Manejar scroll del mouse
        if let Some(scroll) = window.get_scroll_wheel() {
            camera.handle_mouse_scroll(scroll.1 * 0.1);
        }
        if uniforms.current_shader == 7 && time - last_bloom_update >= bloom_update_interval {
            gaussian_blur(&mut framebuffer.emissive_buffer, framebuffer.width, framebuffer.height, 10, 2.0); // Reduced kernel size
            apply_bloom(&mut framebuffer.buffer, &framebuffer.emissive_buffer, framebuffer.width, framebuffer.height);
            last_bloom_update = time;
        }

        framebuffer.clear();

        // Renderizar órbitas
        for body in &solar_system.bodies {
            if !body.orbit_points.is_empty() {
                for point in &body.orbit_points {
                    let screen_pos = world_to_screen(*point, &uniforms);
                    if screen_pos.x >= 0.0 && screen_pos.x < framebuffer_width as f32 
                       && screen_pos.y >= 0.0 && screen_pos.y < framebuffer_height as f32 {
                        framebuffer.set_current_color(0x444444);
                        framebuffer.point(screen_pos.x as usize, screen_pos.y as usize, screen_pos.z, 0);
                    }
                }
            }
        }
        
        // Renderizar cuerpos celestes
        for (i, body) in solar_system.bodies.iter().enumerate() {
            uniforms.current_shader = body.shader_id;
            uniforms.model_matrix = create_model_matrix(
                body.position,
                body.scale,
                Vec3::new(0.0, body.rotation, 0.0)
            );
            uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
            
            render(&mut framebuffer, &uniforms, &vertex_arrays, time as u32);
            
            // Renderizar anillos de Saturno
            if i == 5 {
                uniforms.current_shader = 9;
                let ring_scale = body.scale * 1.5;
                let ring_matrix = Mat4::new_scaling(ring_scale) * uniforms.model_matrix;
                uniforms.model_matrix = ring_matrix;
                render(&mut framebuffer, &uniforms, &ring_vertex_array, time as u32);
            }
        }

        // Renderizar nave espacial
        uniforms.current_shader = 8; // Shader específico para la nave
        uniforms.model_matrix = create_model_matrix(
            solar_system.spaceship_position,
            0.02, // Escala de la nave
            solar_system.spaceship_rotation
        );
        render(&mut framebuffer, &uniforms, &spaceship_vertex_array, time as u32);

        // Efectos de post-procesamiento para el sol
        if uniforms.current_shader == 7 {
            gaussian_blur(&mut framebuffer.emissive_buffer, framebuffer.width, framebuffer.height, 20, 2.5);
            apply_bloom(&mut framebuffer.buffer, &framebuffer.emissive_buffer, framebuffer.width, framebuffer.height);
        }

        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        time += 1;
    }
}

fn handle_input(window: &Window, camera: &mut Camera, solar_system: &mut SolarSystem) {
    let movement_speed = 0.5;
    
    // Movimiento básico
    if window.is_key_down(Key::W) {
        let new_pos = camera.eye + camera.get_forward() * movement_speed;
        if !solar_system.check_collision(&new_pos) {
            camera.eye = new_pos;
            camera.center += camera.get_forward() * movement_speed;
        }
    }
    if window.is_key_down(Key::S) {
        let new_pos = camera.eye - camera.get_forward() * movement_speed;
        if !solar_system.check_collision(&new_pos) {
            camera.eye = new_pos;
            camera.center -= camera.get_forward() * movement_speed;
        }
    }
    if window.is_key_down(Key::A) {
        let new_pos = camera.eye - camera.get_right() * movement_speed;
        if !solar_system.check_collision(&new_pos) {
            camera.eye = new_pos;
            camera.center -= camera.get_right() * movement_speed;
        }
    }
    if window.is_key_down(Key::D) {
        let new_pos = camera.eye + camera.get_right() * movement_speed;
        if !solar_system.check_collision(&new_pos) {
            camera.eye = new_pos;
            camera.center += camera.get_right() * movement_speed;
        }
    }

    // Warping a planetas
    if window.is_key_down(Key::Key1) { solar_system.warp_to_planet(0); }
    if window.is_key_down(Key::Key2) { solar_system.warp_to_planet(1); }
    if window.is_key_down(Key::Key3) { solar_system.warp_to_planet(2); }
    if window.is_key_down(Key::Key4) { solar_system.warp_to_planet(3); }
    if window.is_key_down(Key::Key5) { solar_system.warp_to_planet(4); }

    // Vista de pájaro
    if window.is_key_down(Key::B) {
        solar_system.toggle_bird_eye_view();
    }
}