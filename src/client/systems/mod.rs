mod render;
mod input;
mod movement;

pub mod prelude {
    pub use super::render::*;
    pub use super::input::*;
    pub use super::movement::*;

    use shipyard::{Workload, IntoWorkload};

    pub struct Workloads;

    impl Workloads {
        pub fn events() -> Workload {
            (InputSystem::control_player).into_workload()
        }
    
        pub fn update() -> Workload {
            (MovementSystem::update).into_workload()
        }
    
        pub fn draw() -> Workload {
            (RenderSystem::draw_entities).into_sequential_workload()
        }
    }
}

use shipyard::*;



