use crate::obj::{parse_obj, Obj};

pub struct Screen {
    buffer: Vec<Vec<char>>,
    resolution: (usize, usize),
    fov: f32,
    obj: Obj,
}

impl Screen {
    pub fn new(resolution: (usize, usize), fov: f32, path: &str) -> Screen {
        let internal_buffer = vec!['_'; resolution.0];
        let buffer = vec![internal_buffer; resolution.1];

        let obj = parse_obj(path);

        return Screen {
            buffer,
            resolution,
            fov,
            obj,
        };
    }

    pub fn print_frame(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for x_index in 0..self.buffer.len() {
            for y_index in 0..self.buffer[0].len() {
                print!("{}", self.buffer[x_index][y_index]);
            }
            print!("\n");
        }
        use std::io::Write;

        std::io::stdout().flush().unwrap();
    }

    pub fn draw_pixel(&mut self, position: (usize, usize), pixel: char) {
        if position.0 > 0 && position.1 > 0 {
            self.buffer[position.1 - 1][position.0 - 1] = pixel;
        }
    }

    fn point_slope(x: usize, slope: f32, position1: (usize, usize)) -> usize {
        let y = slope * (x - position1.0) as f32 + position1.1 as f32;
        let y = y as usize;
        return y;
    }
    pub fn draw_line(
        &mut self,
        mut position1: (usize, usize),
        mut position2: (usize, usize),
        pixel: char,
    ) {
        let mut steep = false;
    
        if (position2.1 as isize - position1.1 as isize).abs() > (position2.0 as isize - position1.0 as isize).abs() {
            std::mem::swap(&mut position1.0, &mut position1.1);
            std::mem::swap(&mut position2.0, &mut position2.1);
            steep = true;
        }
    
        if position1.0 > position2.0 {
            std::mem::swap(&mut position1, &mut position2);
        }
    
        let pos1 = (position1.0 as f32, position1.1 as f32);
        let pos2 = (position2.0 as f32, position2.1 as f32);
    
        let slope = if pos2.0 - pos1.0 != 0.0 {
            (pos2.1 - pos1.1) / (pos2.0 - pos1.0)
        } else {
            1.0
        };
    
        for x in position1.0..=position2.0 {
            let y = Self::point_slope(x, slope, position1);
            if steep {
                self.draw_pixel((y, x), pixel);
            } else {
                self.draw_pixel((x, y), pixel);
            }
        }
    }

    fn project(fov: f32, z: f32, pos: f32) -> f32 {
        let screen_z = (fov * 0.5).to_radians().tan(); //technically the reciprocal
        let in_2d = pos / (z * screen_z);
        return in_2d;
    }

    pub fn draw_shape(&mut self, pixel: char) {
        let mut projected_vertices: Vec<(usize, usize)> = vec![];
        for vertice in self.obj.vertices.iter() {
            let mut x = Self::project(self.fov, vertice[2], vertice[0]);
            x *= self.resolution.0 as f32;
            x += self.resolution.0 as f32 * 0.5;
            let mut y = Self::project(self.fov, vertice[2], vertice[1]);
            y *= self.resolution.1 as f32 * -1.0;
            y += self.resolution.1 as f32 * 0.5;
            projected_vertices.push((x.round() as usize, y.round() as usize));
        }

        let obj = self.obj.clone();
        for face in obj.faces.iter() {
            let vertices_count = face.len();
            for i in 0..vertices_count {
                let start_index = face[i].saturating_sub(1);
                let end_index = face[(i + 1) % vertices_count].saturating_sub(1);
    
                if start_index < projected_vertices.len() && end_index < projected_vertices.len() {
                    self.draw_line(
                        projected_vertices[start_index],
                        projected_vertices[end_index],
                        pixel,
                    );
                }
            }
        }
    }
    pub fn print_vertices(&self) {
        for (i, vertice) in self.obj.vertices.iter().enumerate() {
            let mut x = Self::project(self.fov, vertice[2], vertice[0]);
            x *= self.resolution.0 as f32;
            x += self.resolution.0 as f32 * 0.5;
            let mut y = Self::project(self.fov, vertice[2], vertice[1]);
            y *= self.resolution.1 as f32 * -1.0;
            y += self.resolution.1 as f32 * 0.5;
            println!("{}: ({}, {})", i + 1, x.round(), y.round());
        }
    }
}
