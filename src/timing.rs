use specs::prelude::*;

use crate::{
    resources::*,
};

#[derive(SystemData)]
pub struct TimingData<'a> {
    dt: WriteExpect<'a, DeltaTime>,
}

pub struct Timing;

impl<'a> System<'a> for Timing {
    type SystemData = TimingData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.dt.update_dt();
    }
}

