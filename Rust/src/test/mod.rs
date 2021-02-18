use gdnative::prelude::*;

use crate::helpers::navpoly_builder::{build, CoordinatePrecision, OffsetPrecision, Vector2D};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Test {
    start_point: Vector2D,
    end_point: Vector2D,
    obsticles: Vec<Vector2D>,
    offset: OffsetPrecision,
}

#[methods]
impl Test {
    fn new(_owner: &Node) -> Self {
        Test {
            start_point: Vector2D::new(0 as CoordinatePrecision, 0 as CoordinatePrecision),
            end_point: Vector2D::new(0 as CoordinatePrecision, 0 as CoordinatePrecision),
            obsticles: vec![],
            offset: 2 as OffsetPrecision,
        }
    }

    #[export]
    fn get_hello(&self, _owner: &Node) -> Variant {
        Variant::from_str("Hello Rusty".to_string())
    }

    #[export]
    fn set_obsticles(
        &mut self,
        _owner: &Node,
        start_point: Vector2,
        end_point: Vector2,
        obsticles: Vector2Array,
        offset: OffsetPrecision,
    ) {
        // --- Converting --- \\
        self.start_point = Vector2D::new(
            start_point.x.clamp(
                CoordinatePrecision::MIN as f32,
                CoordinatePrecision::MAX as f32,
            ) as CoordinatePrecision,
            start_point.y.clamp(
                CoordinatePrecision::MIN as f32,
                CoordinatePrecision::MAX as f32,
            ) as CoordinatePrecision,
        );
        self.end_point = Vector2D::new(
            end_point.x.clamp(
                CoordinatePrecision::MIN as f32,
                CoordinatePrecision::MAX as f32,
            ) as CoordinatePrecision,
            end_point.y.clamp(
                CoordinatePrecision::MIN as f32,
                CoordinatePrecision::MAX as f32,
            ) as CoordinatePrecision,
        );
        self.obsticles = {
            let mut res = Vec::<Vector2D>::with_capacity(obsticles.len() as usize);
            for i in 0..obsticles.len() {
                res.push(
                    obsticles
                        .get(i)
                        .clamp(
                            Vector2D::new(CoordinatePrecision::MIN, CoordinatePrecision::MIN)
                                .cast(),
                            Vector2D::new(CoordinatePrecision::MAX, CoordinatePrecision::MAX)
                                .cast(),
                        )
                        .cast(),
                );
            }
            res
        };
        self.offset = offset.clamp(OffsetPrecision::MIN, OffsetPrecision::MAX);
    }

    #[export]
    fn build_navpoly(&self, _owner: &Node) -> Variant {
        let np = build(
            self.start_point,
            self.end_point,
            self.obsticles.clone(),
            self.offset,
        );

        Variant::from_object(np)
    }
}
