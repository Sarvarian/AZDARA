use gdnative::prelude::*;

type Vector2D = euclid::Vector2D<u8, euclid::UnknownUnit>;
type ObCoord = [Vector2D; 4];

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
        // --- Casting --- \\
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
        let mut obcoords = Vec::<ObCoord>::with_capacity(obsticles.len() as usize);

        for o in obsticles.iter() {
            let obcoord: ObCoord = [
                Vector2D::new(o.x * 2, o.y * 2),
                Vector2D::new((o.x * 2) + 2, o.y * 2),
                Vector2D::new((o.x * 2) + 2, (o.y * 2) + 2),
                Vector2D::new(o.x * 2, (o.y * 2) + 2),
            ];
            obcoords.push(obcoord);
        }

        // --- Packing --- \\
        let mut obpacks = Vec::<Vec<ObCoord>>::with_capacity(obcoords.len());

        for o in obcoords.into_iter() {
            if let Some(o) = is_in_packs(&mut obpacks, o) {
                obpacks.push(vec![o]);
            }
        }

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

        obpacks.iter().for_each(|p| {
            p.iter().for_each(|o| {
                let mut outline = Vector2Array::new();
                for v in o.iter() {
                    outline.push(v.clone().cast::<f32>());
                }
                np.add_outline(outline);
            });
        });

        // --- Ending --- \\
        np.make_polygons_from_outlines();
        npi.set_navigation_polygon(np);
        nav2d.add_child(npi, false);
        Variant::from_object(nav2d)
    }
}

fn is_in_packs(obpacks: &mut Vec<Vec<ObCoord>>, o: ObCoord) -> Option<ObCoord> {
    for p in obpacks.iter_mut() {
        if pack_check(p, &o) {
            p.push(o);
            return None;
        }
    }
    Some(o)
}

fn pack_check(pack: &Vec<ObCoord>, io: &ObCoord) -> bool {
    for iv in io.iter() {
        for o in pack.iter() {
            for v in o.iter() {
                if iv == v {
                    return true;
                }
            }
        }
    }
    false
}
