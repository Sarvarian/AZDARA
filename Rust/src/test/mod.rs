use gdnative::prelude::*;

type Vector2D = euclid::Vector2D<u8, euclid::UnknownUnit>;

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
    fn build_navpoly(
        &self,
        _owner: &Node,
        start_point: Vector2,
        end_point: Vector2,
        obsticles: Vector2Array,
    ) -> Variant {
        // --- Converting --- \\
        let start_point = Vector2D::new(start_point.x as u8, start_point.y as u8);
        let end_point = Vector2D::new(end_point.x as u8, end_point.y as u8);
        let obsticles = {
            let mut res = Vec::<_>::with_capacity(obsticles.len() as usize);
            for i in 0..obsticles.len() {
                res.push(obsticles.get(i).cast::<u8>());
            }
            res
        };

        let np = crate::helpers::navpoly_builder::build(start_point, end_point, obsticles, 2);

        Variant::from_object(np)
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
        let start_point = Vector2D::new(start_point.x as u8, start_point.y as u8);
        let end_point = Vector2D::new(end_point.x as u8, end_point.y as u8);
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
        let np = crate::helpers::navpoly_builder::build(start_point, end_point, obsticles, 2);

        // --- Ending --- \\
        npi.set_navigation_polygon(np);
        nav2d.add_child(npi, false);
        Variant::from_object(nav2d)
    }
}
