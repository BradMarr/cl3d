#[derive(Clone)]
pub struct Obj {
    pub vertices: Vec<Vec<f32>>,
    pub faces: Vec<Vec<usize>>,
}

pub fn parse_obj(path: &str) -> Obj {
    let mut obj = Obj {
        vertices: vec![],
        faces: vec![],
    };

    for line in std::fs::read_to_string(path).unwrap().lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("v") => {
                let vertice = tokens
                    .map(|i| i.parse().expect("error parsing obj"))
                    .collect::<Vec<f32>>();
                obj.vertices.push(vertice);
            }
            Some("f") => {
                let face = tokens
                    .map(|i| i.parse().expect("error parsing obj"))
                    .collect::<Vec<usize>>();
                obj.faces.push(face);
            }
            _ => {}
        }
    }
    return obj;
}
