use bevy::prelude::Component;
#[derive(Component)]
pub struct Health{
    max_health : f32,
    health : f32,
    regen_rate : f32
}

#[derive(Component)]
pub struct Energy{
    max_energy : f32,
    energy : f32,
    regen_rate : f32
}