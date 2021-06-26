use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Label)]
pub struct AlertMessage {
    time: f32,
    mm: f32,
}

#[methods]
impl AlertMessage {
    fn new(_owner: &Label) -> Self {
        AlertMessage {
            time: 5f32,
            mm: 1f32,
        }
    }

    #[export]
    fn _process(&mut self, owner: &Label, delta: f32) {
        if self.time < 0f32 {
            owner.queue_free();
        } else if self.time < self.mm {
            owner.set_modulate(Color {
                r: 1f32,
                g: 1f32,
                b: 1f32,
                a: (self.time / self.mm),
            })
        }
        self.time -= delta;
    }
}
