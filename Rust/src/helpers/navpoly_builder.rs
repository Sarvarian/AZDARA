use gdnative::core_types::{Vector2, Vector2Array};
use outline_rendering::outline_renderer;
use std::collections::HashMap;

type CoordinatePrecision = u8;
type VertexCoordinatePrecision = f32;
type OffsetPrecision = u8;
type Vector2D = euclid::Vector2D<CoordinatePrecision, euclid::UnknownUnit>;
type Outline = gdnative::core_types::TypedArray<euclid::Vector2D<f32, euclid::UnknownUnit>>;

pub fn build(
    mut start_point: Vector2D,
    mut end_point: Vector2D,
    obsticles: Vec<Vector2D>,
    offset: OffsetPrecision,
) -> gdnative::Ref<gdnative::api::NavigationPolygon, gdnative::prelude::Unique> {
    // --- Starting --- \\
    let navpoly = gdnative::api::NavigationPolygon::new();
    let offset = if offset < 2 {
        2
    } else if offset % 2 == 1 {
        offset - 1
    } else {
        offset
    };

    // --- Sorting --- \\
    // obsticles.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    // obsticles.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    // --- Hashing --- \\
    let map_cap = obsticles.len();
    let mut obsticles =
        obsticles
            .into_iter()
            .fold(HashMap::with_capacity(map_cap), |mut map, o| {
                map.insert(
                    o,
                    vec![
                        vertex0(&o, offset),
                        vertex1(&o, offset),
                        vertex2(&o, offset),
                        vertex3(&o, offset),
                    ],
                );
                map
            });

    while let Ok(outline) = outline_renderer(&mut obsticles, offset) {
        navpoly.add_outline(outline);
    }

    let mut outline = Vector2Array::new();
    start_point *= offset;
    end_point *= offset;
    let start_point = start_point.cast::<f32>();
    let end_point = end_point.cast::<f32>();
    outline.push(start_point);
    outline.push(Vector2::new(end_point.x, start_point.y));
    outline.push(end_point);
    outline.push(Vector2::new(start_point.x, end_point.y));
    navpoly.add_outline(outline);

    // --- Ending --- \\
    navpoly.make_polygons_from_outlines();
    navpoly
}

fn vertex0(obsticle: &Vector2D, offset: OffsetPrecision) -> Vector2D {
    Vector2D::new(obsticle.x * offset, (obsticle.y * offset) + offset)
}

fn vertex1(obsticle: &Vector2D, offset: OffsetPrecision) -> Vector2D {
    Vector2D::new(obsticle.x * offset, obsticle.y * offset)
}

fn vertex2(obsticle: &Vector2D, offset: OffsetPrecision) -> Vector2D {
    Vector2D::new((obsticle.x * offset) + offset, obsticle.y * offset)
}

fn vertex3(obsticle: &Vector2D, offset: OffsetPrecision) -> Vector2D {
    Vector2D::new(
        (obsticle.x * offset) + offset,
        (obsticle.y * offset) + offset,
    )
}

mod outline_rendering {
    use super::{
        vertex0, vertex1, vertex2, vertex3, HashMap, OffsetPrecision, Outline, Vector2D,
        VertexCoordinatePrecision,
    };

    /// pub const LEFT: u8 = 0u8;
    /// pub const LETF_UP: u8 = 1u8;
    /// pub const UP: u8 = 2u8;
    /// pub const UP_RIGHT: u8 = 3u8;
    /// pub const RIGHT: u8 = 4u8;
    /// pub const RIGHT_DOWN: u8 = 5u8;
    /// pub const DOWN: u8 = 6u8;
    /// pub const DOWN_LEFT: u8 = 7u8;

    pub fn outline_renderer(
        obsticles: &mut HashMap<Vector2D, Vec<Vector2D>>,
        offset: OffsetPrecision,
    ) -> Result<Outline, ()> {
        let (desirable_obsticle, mut direction) = find_desirable_obsticle(obsticles, offset)?;
        let mut o = desirable_obsticle.clone();
        let mut outline = Outline::new();
        while let Ok(v_index) = direction_vertex_index(&obsticles, &o, direction, offset) {
            let mut margin_x = 0.1 as VertexCoordinatePrecision;
            let mut margin_y = 0.1 as VertexCoordinatePrecision;
            {
                if direction == 0 {
                    margin_x *= -1.;
                    margin_y *= -1.;
                } else if direction == 2 {
                    margin_y *= -1.;
                } else if direction == 6 {
                    margin_x *= -1.;
                }
            }
            outline.push(
                (obsticles.get_mut(&o).ok_or(())?.remove(v_index).cast())
                    + euclid::Vector2D::<VertexCoordinatePrecision, euclid::UnknownUnit>::new(
                        margin_x, margin_y,
                    ),
            );

            // (o, direction) = next_obsticle_and_direction(obsticles, o, direction);

            let (next_o, next_dir) = next_obsticle_and_direction(obsticles, o, direction);
            o = next_o;
            direction = next_dir;
        }
        Ok(outline)
    }

    fn find_desirable_obsticle(
        obsticles: &mut HashMap<Vector2D, Vec<Vector2D>>,
        offset: OffsetPrecision,
    ) -> Result<(&Vector2D, u8), ()> {
        type NFP = fn(&Vector2D) -> Vector2D;
        let neighbors: [NFP; 4] = [left_neighbor, up_neighbor, right_neighbor, down_neighbor];
        type VDFP = fn(&Vector2D, u8) -> Vector2D;
        let vertex_direction: [VDFP; 4] = [vertex0, vertex1, vertex2, vertex3];
        for (o, vertices) in obsticles.iter() {
            for index in 0..4u8 {
                if !obsticles.contains_key(&neighbors[index as usize](o)) {
                    for v in vertices.into_iter() {
                        if v == &vertex_direction[index as usize](o, offset) {
                            return Ok((o, (index * 2) % 8));
                        }
                    }
                }
            }
        }
        Err(())
    }

    fn direction_vertex_index(
        obsticles: &HashMap<Vector2D, Vec<Vector2D>>,
        obsticle: &Vector2D,
        direction: u8,
        offset: OffsetPrecision,
    ) -> Result<usize, ()> {
        type FP = fn(&Vector2D, u8) -> Vector2D;
        let vertex_direction: [FP; 4] = [vertex0, vertex1, vertex2, vertex3];

        if let Some(vertices) = obsticles.get(obsticle) {
            for (index, v) in vertices.iter().enumerate() {
                if &vertex_direction[(direction / 2) as usize](obsticle, offset) == v {
                    return Ok(index);
                }
            }
        }

        Err(())
    }

    fn next_obsticle_and_direction(
        obsticles: &HashMap<Vector2D, Vec<Vector2D>>,
        obsticle: Vector2D,
        direction: u8,
    ) -> (Vector2D, u8) {
        type NFP = fn(&Vector2D) -> Vector2D;
        let neighbors: [NFP; 8] = [
            left_neighbor,
            left_up_neighbor,
            up_neighbor,
            up_right_neighbor,
            right_neighbor,
            right_down_neighbor,
            down_neighbor,
            down_left_neighbor,
        ];

        for i in 1..3u8 {
            let neighbor = neighbors[((direction + i) % 8) as usize](&obsticle);
            if obsticles.contains_key(&neighbor) {
                return (neighbor, (direction + (i * 2) + 4) % 8);
            }
        }

        (obsticle, (direction + 2) % 8)
    }

    pub fn left_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x - 1, obsticle.y)
    }

    pub fn left_up_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x - 1, obsticle.y - 1)
    }

    pub fn up_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x, obsticle.y - 1)
    }

    pub fn up_right_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x + 1, obsticle.y - 1)
    }

    pub fn right_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x + 1, obsticle.y)
    }

    pub fn right_down_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x + 1, obsticle.y + 1)
    }

    pub fn down_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x, obsticle.y + 1)
    }

    pub fn down_left_neighbor(obsticle: &Vector2D) -> Vector2D {
        Vector2D::new(obsticle.x - 1, obsticle.y + 1)
    }
}
