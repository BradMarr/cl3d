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

    pub fn draw_line(
        &mut self,
        mut position1: (usize, usize),
        mut position2: (usize, usize),
        pixel: char,
    ) {
        let pos1 = (position1.0 as f32, position1.1 as f32);
        let pos2 = (position2.0 as f32, position2.1 as f32);
        let mut slope = (pos2.1 - pos1.1) / (pos2.0 - pos1.0);
        let mut swapped = false;

        if position1.0 > position2.0 {
            std::mem::swap(&mut position1, &mut position2);
        }

        if slope.abs() > 1.0 {
            std::mem::swap(&mut position1.0, &mut position1.1);
            std::mem::swap(&mut position2.0, &mut position2.1);
            swapped = true;
            slope = 1.0 / slope;
        }

        fn point_slope(x: usize, slope: f32, position1: (usize, usize)) -> usize {
            let y = slope * (x - position1.0) as f32 + position1.1 as f32;
            let y = y as usize;
            return y;
        }

        for mut x in position1.0..position2.0 + 1 {
            let mut y = point_slope(x, slope, position1);
            if swapped {
                std::mem::swap(&mut x, &mut y);
            }
            self.draw_pixel((x, y), pixel);
        }
    }
    pub fn draw_shape(&mut self, pixel: char) {
        fn project(fov: f32, z: f32, pos: f32) -> f32 {
            let screen_z = (fov * 0.5).to_radians().tan(); //technically the reciprocal
            let in_2d = pos / (z * screen_z);
            return in_2d;
        }

        let mut projected_vertices: Vec<(usize, usize)> = vec![];
        for vertice in self.obj.vertices.iter() {
            let mut x = project(self.fov, vertice[2], vertice[0]);
            x *= self.resolution.0 as f32;
            x += self.resolution.0 as f32 * 0.5;
            let mut y = project(self.fov, vertice[2], vertice[1]);
            y *= self.resolution.1 as f32 * -1.0;
            y += self.resolution.1 as f32 * 0.5;
            projected_vertices.push((x as usize, y as usize));
        }

        let obj = self.obj.clone();

        for face in obj.faces.iter() {
            for (loop_index, vertice_index) in face.iter().enumerate() {
                let start_index = face[0].saturating_sub(1);
                let current_index = vertice_index.saturating_sub(1);
                if loop_index >= face.len() - 1 {
                    if start_index < projected_vertices.len() && current_index < projected_vertices.len() {
                        self.draw_line(
                            projected_vertices[start_index],
                            projected_vertices[current_index],
                            pixel,
                        );
                    }
                } else {
                    let next_index = face[loop_index + 1].saturating_sub(1);
                    if current_index < projected_vertices.len() && next_index < projected_vertices.len() {
                        self.draw_line(
                            projected_vertices[current_index],
                            projected_vertices[next_index],
                            pixel,
                        );
                    }
                }
            }
        }
    }
}
