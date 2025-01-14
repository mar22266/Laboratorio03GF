use crate::color::Color;
use crate::fragment::Fragment;
use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::vertex::Vertex;
use nalgebra_glm::{dot, Vec3};

pub fn _triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));

    fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, framebuffer: &mut Framebuffer) {
    let (a, b, c) = (
        v1.transformed_position,
        v2.transformed_position,
        v3.transformed_position,
    );

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    let triangle_area = edge_function(&a, &b, &c);
    let light_dir = Vec3::new(0.0, 0.0, -1.0);

   
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            
            if x >= 0 && x < framebuffer.width as i32 && y >= 0 && y < framebuffer.height as i32 {
                let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

                let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

      
                if w1 >= 0.0 && w1 <= 1.0 && w2 >= 0.0 && w2 <= 1.0 && w3 >= 0.0 && w3 <= 1.0 {
                
                    let normal = v1.transformed_normal * w1
                        + v2.transformed_normal * w2
                        + v3.transformed_normal * w3;
                    let normal = normal.normalize();

                 
                    let intensity = dot(&normal, &light_dir).max(0.0);

                    let base_color = Color::new(100, 100, 100); 
                    let lit_color = base_color * intensity;

                
                    let depth = w1 * a.z + w2 * b.z + w3 * c.z;

                
                    let index = y as usize * framebuffer.width + x as usize;

                    if depth < framebuffer.zbuffer[index] {
                        framebuffer.zbuffer[index] = depth;
                        framebuffer.point_with_color(x as usize, y as usize, lit_color);
                    }
                }
            }
        }
    }
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}
