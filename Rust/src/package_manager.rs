use gdnative::prelude::*;

type Owner = Object;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct PackageManager;

#[methods]
impl PackageManager {
    fn new(_owner: &Owner) -> Self {
        PackageManager
    }
}
