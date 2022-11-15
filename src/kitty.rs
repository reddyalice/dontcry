use bevy::prelude::Component;


pub struct Construct{
    base_health : f32,
    base_energy : f32,
    base_hregen_rate : f32,
    base_eregen_rate : f32,
}





#[derive(Component)]
pub struct Kitty{
    nickname : &'static str,
    race : Construct
}



