use gdnative::prelude::*;

use crate::helpers::navpoly_builder::{
    build, InputCoordinatePrecision, InputVector2D, OffsetPrecision,
};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Test {
    start_point: InputVector2D,
    end_point: InputVector2D,
    obsticles: Vec<InputVector2D>,
    offset: OffsetPrecision,
}

#[methods]
impl Test {
    fn new(_owner: &Node) -> Self {
        Test {
            start_point: InputVector2D::new(
                0 as InputCoordinatePrecision,
                0 as InputCoordinatePrecision,
            ),
            end_point: InputVector2D::new(
                0 as InputCoordinatePrecision,
                0 as InputCoordinatePrecision,
            ),
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
        self.start_point = InputVector2D::new(
            start_point.x.clamp(
                InputCoordinatePrecision::MIN as f32,
                InputCoordinatePrecision::MAX as f32,
            ) as InputCoordinatePrecision,
            start_point.y.clamp(
                InputCoordinatePrecision::MIN as f32,
                InputCoordinatePrecision::MAX as f32,
            ) as InputCoordinatePrecision,
        );
        self.end_point = InputVector2D::new(
            end_point.x.clamp(
                InputCoordinatePrecision::MIN as f32,
                InputCoordinatePrecision::MAX as f32,
            ) as InputCoordinatePrecision,
            end_point.y.clamp(
                InputCoordinatePrecision::MIN as f32,
                InputCoordinatePrecision::MAX as f32,
            ) as InputCoordinatePrecision,
        );
        self.obsticles = {
            let mut res = Vec::<InputVector2D>::with_capacity(obsticles.len() as usize);
            for i in 0..obsticles.len() {
                res.push(
                    obsticles
                        .get(i)
                        .clamp(
                            InputVector2D::new(
                                InputCoordinatePrecision::MIN,
                                InputCoordinatePrecision::MIN,
                            )
                            .cast(),
                            InputVector2D::new(
                                InputCoordinatePrecision::MAX,
                                InputCoordinatePrecision::MAX,
                            )
                            .cast(),
                        )
                        .cast(),
                );
            }
            res
        };
        self.offset = offset;
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
