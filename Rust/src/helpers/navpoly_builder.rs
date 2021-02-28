use gdnative::core_types::{Vector2, Vector2Array};
use outline_rendering::outline_renderer;
use std::collections::HashMap;

pub type InputCoordinatePrecision = u16;
pub type CoordinatePrecision = u32;
pub type OffsetPrecision = u8;
pub type InputVector2D = euclid::Vector2D<InputCoordinatePrecision, euclid::UnknownUnit>;
pub type Vector2D = euclid::Vector2D<CoordinatePrecision, euclid::UnknownUnit>;
pub type Outline = gdnative::core_types::TypedArray<euclid::Vector2D<f32, euclid::UnknownUnit>>;

pub fn build(
    start_point: InputVector2D,
    end_point: InputVector2D,
    obsticles: Vec<InputVector2D>,
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
    let start_point: Vector2D = start_point.cast();
    let mut end_point: Vector2D = end_point.cast();

    // --- Hashing --- \\
    let map_cap = obsticles.len();
    let mut obsticles =
        obsticles
            .into_iter()
            .fold(HashMap::with_capacity(map_cap), |mut map, o| {
                let o: Vector2D =
                    Vector2D::new(1 as CoordinatePrecision, 1 as CoordinatePrecision) + o.cast();
                end_point.x = end_point.x.max(o.x);
                end_point.y = end_point.y.max(o.y);
                map.insert(
                    o,
                    vec![
                        VERTICES[0](&o, offset),
                        VERTICES[1](&o, offset),
                        VERTICES[2](&o, offset),
                        VERTICES[3](&o, offset),
                    ],
                );
                map
            });

    // --- Building Borders --- \\
    for i in start_point.x..=end_point.x {
        let o = Vector2D::new(i, start_point.y);
        obsticles.insert(
            o,
            vec![
                VERTICES[0](&o, offset),
                VERTICES[1](&o, offset),
                VERTICES[2](&o, offset),
                VERTICES[3](&o, offset),
            ],
        );
        let o = Vector2D::new(i, end_point.y);
        obsticles.insert(
            o,
            vec![
                VERTICES[0](&o, offset),
                VERTICES[1](&o, offset),
                VERTICES[2](&o, offset),
                VERTICES[3](&o, offset),
            ],
        );
    }
    for i in start_point.y..=end_point.y {
        let o = Vector2D::new(start_point.x, i);
        obsticles.insert(
            o,
            vec![
                VERTICES[0](&o, offset),
                VERTICES[1](&o, offset),
                VERTICES[2](&o, offset),
                VERTICES[3](&o, offset),
            ],
        );
        let o = Vector2D::new(end_point.x, i);
        obsticles.insert(
            o,
            vec![
                VERTICES[0](&o, offset),
                VERTICES[1](&o, offset),
                VERTICES[2](&o, offset),
                VERTICES[3](&o, offset),
            ],
        );
    }

    // --- Rendering Outlines --- \\
    while let Ok(outline) = outline_renderer(&mut obsticles, offset) {
        navpoly.add_outline(outline);
    }

    let offset2d: Vector2 = Vector2::new(offset as f32, offset as f32);
    let start_point: Vector2 = start_point.cast() - offset2d;
    let end_point: Vector2 = (end_point.cast() * offset as f32) + (offset2d * 2f32);
    let mut outline = Vector2Array::new();
    outline.push(start_point);
    outline.push(Vector2::new(end_point.x, start_point.y));
    outline.push(end_point);
    outline.push(Vector2::new(start_point.x, end_point.y));
    navpoly.add_outline(outline);

    // --- Ending --- \\
    navpoly.make_polygons_from_outlines();
    navpoly
}

const VERTICES: [fn(&Vector2D, u8) -> Vector2D; 4] = [
    // vertex0,
    |obsticle: &Vector2D, offset: OffsetPrecision| -> Vector2D {
        Vector2D::new(
            obsticle.x * offset as CoordinatePrecision,
            (obsticle.y * offset as CoordinatePrecision) + offset as CoordinatePrecision,
        )
    },
    // vertex1,
    |obsticle: &Vector2D, offset: OffsetPrecision| -> Vector2D {
        Vector2D::new(
            obsticle.x * offset as CoordinatePrecision,
            obsticle.y * offset as CoordinatePrecision,
        )
    },
    // vertex2,
    |obsticle: &Vector2D, offset: OffsetPrecision| -> Vector2D {
        Vector2D::new(
            (obsticle.x * offset as CoordinatePrecision) + offset as CoordinatePrecision,
            obsticle.y * offset as CoordinatePrecision,
        )
    },
    // vertex3,
    |obsticle: &Vector2D, offset: OffsetPrecision| -> Vector2D {
        Vector2D::new(
            (obsticle.x * offset as CoordinatePrecision) + offset as CoordinatePrecision,
            (obsticle.y * offset as CoordinatePrecision) + offset as CoordinatePrecision,
        )
    },
];

mod outline_rendering {
    use super::{HashMap, OffsetPrecision, Outline, Vector2D, VERTICES};

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
            let mut margin_x = 0.1f32;
            let mut margin_y = 0.1f32;
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
                    + euclid::Vector2D::<f32, euclid::UnknownUnit>::new(margin_x, margin_y),
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
        for (o, vertices) in obsticles.iter() {
            for index in 0..4u8 {
                if !obsticles.contains_key(&NEIGHBORS[index as usize](o)) {
                    for v in vertices.into_iter() {
                        if v == &VERTICES[index as usize](o, offset) {
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
        if let Some(vertices) = obsticles.get(obsticle) {
            for (index, v) in vertices.iter().enumerate() {
                if &VERTICES[(direction / 2) as usize](obsticle, offset) == v {
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
        for i in 1..3u8 {
            let neighbor = NEIGHBORS[((direction + i) % 8) as usize](&obsticle);
            if obsticles.contains_key(&neighbor) {
                return (neighbor, (direction + (i * 2) + 4) % 8);
            }
        }

        (obsticle, (direction + 2) % 8)
    }

    const NEIGHBORS: [fn(&Vector2D) -> Vector2D; 8] = [
        // left_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x - 1, obsticle.y) },
        // left_up_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x - 1, obsticle.y - 1) },
        // up_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x, obsticle.y - 1) },
        // up_right_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x + 1, obsticle.y - 1) },
        // right_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x + 1, obsticle.y) },
        // right_down_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x + 1, obsticle.y + 1) },
        // down_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x, obsticle.y + 1) },
        // down_left_neighbor,
        |obsticle: &Vector2D| -> Vector2D { Vector2D::new(obsticle.x - 1, obsticle.y + 1) },
    ];
}
