#[macro_export]
macro_rules! comp_from_config {
    ($comp_type:ty) => {
        ron::from_str::<$comp_type>(
            &fs::read_to_string(
                "config/".to_owned() + &stringify!($comp_type).to_lowercase() + ".ron",
            )
            .unwrap(),
        )
        .expect("Failed to load player.ron")
    };
    ($comp_type:ty,$file_name:expr) => {
        ron::from_str::<$comp_type>(&fs::read_to_string(($file_name)).unwrap())
            .expect("Failed to load player.ron")
    };
}