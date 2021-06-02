use gdnative::prelude::*;

type Owner = Reference;

#[derive(NativeClass)]
#[inherit(Owner)]
pub struct PackageManager;

#[methods]
impl PackageManager {
    fn new(_owner: &Owner) -> Self {
        PackageManager
    }
}
