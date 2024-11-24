
use nalgebra_glm::{Vec3, Mat4, perspective};
use std::f32::consts::PI;
use crate::camera::Camera;

pub struct CelestialBody {
    pub position: Vec3,
    pub rotation: f32,
    pub orbital_radius: f32,
    pub orbital_speed: f32,
    pub rotation_speed: f32,
    pub scale: f32,
    pub shader_id: u8,
    pub orbit_points: Vec<Vec3>,  // Puntos para renderizar la órbita
    pub collision_radius: f32,    // Radio de colisión
}

pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    pub spaceship_position: Vec3,
    pub spaceship_rotation: Vec3,
    time: f32,
    pub bird_eye_view: bool,
    pub warp_target: Option<usize>,
    pub warp_animation: f32,
}

impl SolarSystem {
    pub fn new() -> Self {
        let mut bodies = Vec::new();
        
        // Sol (centro del sistema) con mayor escala y emisión
        bodies.push(CelestialBody {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            orbital_radius: 0.0,
            orbital_speed: 0.0,
            rotation_speed: 0.01,
            scale: 3.0,
            shader_id: 7,
            orbit_points: Vec::new(),
            collision_radius: 3.5,
        });

        // Planetas con órbitas y colisiones
        let planet_configs = [
            (4.0, 0.8, 0.4, 3, 0.5),   // Mercurio
            (7.0, 0.5, 0.8, 1, 1.0),   // Tierra
            (10.0, 0.3, 0.6, 2, 0.7),  // Marte
            (15.0, 0.15, 1.5, 5, 1.8), // Júpiter
            (20.0, 0.1, 1.3, 4, 1.5),  // Saturno
        ];

        for (orbital_radius, orbital_speed, scale, shader_id, collision_scale) in planet_configs.iter() {
            let mut orbit_points = Vec::new();
            for i in 0..360 {
                let angle = i as f32 * PI / 180.0;
                let x = orbital_radius * angle.cos();
                let z = orbital_radius * angle.sin();
                orbit_points.push(Vec3::new(x, 0.0, z));
            }

            bodies.push(CelestialBody {
                position: Vec3::new(*orbital_radius, 0.0, 0.0),
                rotation: 0.0,
                orbital_radius: *orbital_radius,
                orbital_speed: *orbital_speed,
                rotation_speed: 0.02,
                scale: *scale,
                shader_id: *shader_id,
                orbit_points,
                collision_radius: scale * collision_scale,
            });
        }

        SolarSystem {
            bodies,
            spaceship_position: Vec3::new(25.0, 5.0, 25.0),
            spaceship_rotation: Vec3::new(0.0, 0.0, 0.0),
            time: 0.0,
            bird_eye_view: false,
            warp_target: None,
            warp_animation: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, camera: &mut Camera) {
        self.time += delta_time;
        
        // Actualizar cuerpos celestes
        for body in &mut self.bodies {
            body.rotation += body.rotation_speed * delta_time;
            
            if body.orbital_radius > 0.0 {
                let angle = self.time * body.orbital_speed;
                body.position.x = body.orbital_radius * angle.cos();
                body.position.z = body.orbital_radius * angle.sin();
            }
        }

        // Manejar warping
        if let Some(target) = self.warp_target {
            self.warp_animation += delta_time * 2.0;
            if self.warp_animation >= 1.0 {
                camera.eye = self.bodies[target].position + Vec3::new(5.0, 2.0, 5.0);
                camera.center = self.bodies[target].position;
                self.warp_target = None;
                self.warp_animation = 0.0;
            }
        }

        // Actualizar vista de pájaro
        if self.bird_eye_view {
            camera.eye = Vec3::new(0.0, 50.0, 0.0);
            camera.center = Vec3::new(0.0, 0.0, 0.0);
        }

        // Actualizar posición de la nave espacial
        self.spaceship_position = camera.eye + camera.get_forward() * 2.0;
        self.spaceship_rotation = camera.get_rotation();
    }

    pub fn check_collision(&self, new_position: &Vec3) -> bool {
        for body in &self.bodies {
            let distance = (body.position - new_position).magnitude();
            if distance < body.collision_radius {
                return true;
            }
        }
        false
    }

    pub fn warp_to_planet(&mut self, planet_index: usize) {
        if planet_index < self.bodies.len() {
            self.warp_target = Some(planet_index);
            self.warp_animation = 0.0;
        }
    }

    pub fn toggle_bird_eye_view(&mut self) {
        self.bird_eye_view = !self.bird_eye_view;
    }
}