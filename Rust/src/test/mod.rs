use bitflags;
use gdnative::prelude::*;

type Vector2D = euclid::Vector2D<u8, euclid::UnknownUnit>;
type ObCoord = [Vector2D; 4];

bitflags! {
    struct Neighbors : u8 {
        const NONE  = 0b00000000;
        const UP    = 0b00000001;
        const RIGHT = 0b00000010;
        const DOWN  = 0b00000100;
        const LEFT  = 0b00001000;
        const FULL  = Self::UP.bits | Self::RIGHT.bits | Self::DOWN.bits | Self::LEFT.bits;
    }
}

struct Obsticle {
    coords: ObCoord,
    pack: u8,
    neighbors: Neighbors,
}

impl std::ops::Deref for Obsticle {
    type Target = ObCoord;

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Test;

#[methods]
impl Test {
    fn new(_owner: &Node) -> Self {
        Test {}
    }

    #[export]
    fn get_hello(&self, _owner: &Node) -> Variant {
        Variant::from_str("Hello Rusty".to_string())
    }

    #[export]
    fn build_nav2d(
        &self,
        _owner: &Node,
        start_point: Vector2,
        end_point: Vector2,
        obsticles: Vector2Array,
    ) -> Variant {
        // --- Converting --- \\
        let mut start_point = Vector2D::new(start_point.x as u8, start_point.y as u8);
        let mut end_point = Vector2D::new(end_point.x as u8, end_point.y as u8);
        let obsticles = {
            let mut res = Vec::<_>::with_capacity(obsticles.len() as usize);
            for i in 0..obsticles.len() {
                res.push(obsticles.get(i).cast::<u8>());
            }
            res
        };

        // --- Starting --- \\
        let nav2d = gdnative::api::Navigation2D::new();
        let npi = gdnative::api::NavigationPolygonInstance::new();
        let np = gdnative::api::NavigationPolygon::new();

        // --- Transforming --- \\
        let mut obsticles = (0..obsticles.len())
            .map(|index| {
                let o = obsticles[index];
                let coords: ObCoord = [
                    Vector2D::new(o.x * 2, o.y * 2),
                    Vector2D::new((o.x * 2) + 2, o.y * 2),
                    Vector2D::new((o.x * 2) + 2, (o.y * 2) + 2),
                    Vector2D::new(o.x * 2, (o.y * 2) + 2),
                ];
                const X: Vector2D = Vector2D::new(1, 0);
                const Y: Vector2D = Vector2D::new(0, 1);
                let mut neighbors = Neighbors::NONE;
                for o2 in 0..obsticles.len() {
                    if obsticles[o2] == o + X {
                        neighbors = neighbors | Neighbors::RIGHT
                    } else if obsticles[o2] == o - X {
                        neighbors = neighbors | Neighbors::LEFT
                    } else if obsticles[o2] == o - Y {
                        neighbors = neighbors | Neighbors::UP
                    } else if obsticles[o2] == o + Y {
                        neighbors = neighbors | Neighbors::DOWN
                    }
                    if neighbors == Neighbors::FULL {
                        break;
                    }
                }
                Obsticle {
                    coords,
                    pack: index as u8,
                    neighbors,
                }
            })
            .collect::<Vec<Obsticle>>();

        // --- Packing --- \\
        let mut one_more_time = true;
        while one_more_time {
            one_more_time = false;
            (0..obsticles.len()).for_each(|o1| {
                (0..obsticles.len()).for_each(|o2| {
                    (0..obsticles[o1].len()).for_each(|v1| {
                        (0..obsticles[o1].len()).for_each(|v2| {
                            if obsticles[o1].pack != obsticles[o2].pack {
                                if obsticles[o1][v1] == obsticles[o2][v2] {
                                    obsticles[o2].pack = obsticles[o1].pack;
                                    one_more_time = true;
                                }
                            }
                        });
                    });
                });
            });
        }

        // --- Massaging --- \\
        let mut msg = String::new();
        for (i, o) in obsticles.iter().enumerate() {
            msg = format!("{}{}: {} : {:?}\n", msg, i, o.pack, o.neighbors);
        }
        godot_print!("{}", msg);

        // --- Outlining --- \\
        let mut outline = Vector2Array::new();
        start_point *= 2;
        end_point *= 2;
        let start_point = start_point.cast::<f32>();
        let end_point = end_point.cast::<f32>();
        outline.push(start_point);
        outline.push(Vector2::new(end_point.x, start_point.y));
        outline.push(end_point);
        outline.push(Vector2::new(start_point.x, end_point.y));
        np.add_outline(outline);

        obsticles.iter().for_each(|o| {
            let mut outline = Vector2Array::new();
            for v in o.iter() {
                outline.push(v.clone().cast::<f32>());
            }
            np.add_outline(outline);
        });

        // --- Ending --- \\
        np.make_polygons_from_outlines();
        npi.set_navigation_polygon(np);
        nav2d.add_child(npi, false);
        Variant::from_object(nav2d)
    }
}

struct Vertex {
    coord: Vector2D,
    obsticle: usize,
}

fn outline_renderer(obsticles: &Vec<Obsticle>) {
    let vertices = obsticles.iter().enumerate().fold(
        Vec::<Vertex>::with_capacity(obsticles.len() * 4),
        |vertices, (index, obsticle)| {
            obsticle.iter().fold(vertices, |mut vertices, vertex| {
                vertices.push(Vertex {
                    coord: vertex.clone(),
                    obsticle: index,
                });
                vertices
            })
        },
    );
}
