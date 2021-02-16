use gdnative::{api::*, thread_access::Unique, Ref};

pub struct Nav2D(Vec<Ref<Navigation2D, Unique>>);

impl Nav2D {
    pub fn new() -> Self {
        Nav2D(Vec::<Ref<Navigation2D, Unique>>::with_capacity(1))
    }

    // pub fn new_nav2d(&mut self) -> usize {
    //     let navigation_2d = Navigation2D::new();
    //     let navigation_polygon_instance = NavigationPolygonInstance::new();
    //     let navigation_polygon = NavigationPolygon::new();

    //     navigation_polygon_instance
    //         .as_ref()
    //         .set_navigation_polygon(navigation_polygon);

    //     navigation_2d
    //         .as_ref()
    //         .add_child(navigation_polygon_instance, false);

    //     self.0.push(navigation_2d);
    //     self.len()
    // }
}

impl std::ops::Deref for Nav2D {
    type Target = Vec<Ref<Navigation2D, Unique>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Nav2D {
    fn drop(&mut self) {
        for _i in (0..self.len()).rev() {
            if let Some(navigation_2d) = self.0.pop().take() {
                navigation_2d.free();
            }
        }
    }
}
