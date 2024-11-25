use nalgebra_glm::{Vec3, dot, Vec2};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;

// In triangle.rs
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    // Early frustum culling
    if (a.x < 0.0 && b.x < 0.0 && c.x < 0.0) || 
       (a.x > 800.0 && b.x > 800.0 && c.x > 800.0) ||
       (a.y < 0.0 && b.y < 0.0 && c.y < 0.0) || 
       (a.y > 600.0 && b.y > 600.0 && c.y > 600.0) {
        return fragments;
    }

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    let triangle_area = edge_function(&a, &b, &c);

    // Skip if triangle is too small
    if triangle_area.abs() < 0.1 {
        return fragments;
    }

    let light_dir = Vec3::new(0.0, 0.0, 1.0);

    let mut point = Vec3::new(min_x as f32 + 0.5, min_y as f32 + 0.5, 0.0);
    
    for y in min_y..=max_y {
        point.x = min_x as f32 + 0.5;
        
        for x in min_x..=max_x {
            // Calculate barycentric coordinates
            let w1 = edge_function(&b, &c, &point) / triangle_area;
            let w2 = edge_function(&c, &a, &point) / triangle_area;
            let w3 = edge_function(&a, &b, &point) / triangle_area;

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                // Properly interpolate all vertex attributes
                let normal = (v1.transformed_normal * w1 + 
                            v2.transformed_normal * w2 + 
                            v3.transformed_normal * w3).normalize();
                
                let depth = a.z * w1 + b.z * w2 + c.z * w3;
                let vertex_position = v1.position * w1 + v2.position * w2 + v3.position * w3;

                // Calculate lighting intensity properly
                let intensity = dot(&normal, &light_dir).max(0.0);

                // Interpolate texture coordinates directly since they're Vec2
                let uv = v1.tex_coords * w1 + v2.tex_coords * w2 + v3.tex_coords * w3;

                fragments.push(Fragment::new(
                    Vec2::new(x as f32, y as f32),
                    v1.color,
                    depth,
                    normal,
                    intensity,
                    vertex_position,
                    Some(uv)  // Wrap in Some since Fragment expects Option<Vec2>
                ));
            }
            point.x += 1.0;
        }
        point.y += 1.0;
    }

    fragments
}
fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}