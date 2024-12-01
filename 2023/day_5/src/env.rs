

pub fn get_paths_by_mod<'a>(mode: String) -> [&'a str; 7] {
    if mode == String::from("test") {
        return [
            "./input/calibration/map_seed_soil.txt",
            "./input/calibration/map_soil_fertilizer.txt",
            "./input/calibration/map_fertilizer_water.txt",
            "./input/calibration/map_water_light.txt",
            "./input/calibration/map_light_temperature.txt",
            "./input/calibration/map_temperature_humidity.txt",
            "./input/calibration/map_humidity_location.txt",
        ];
    } else {
        return [
            "./input/data/map_seed_soil.txt",
            "./input/data/map_soil_fertilizer.txt",
            "./input/data/map_fertilizer_water.txt",
            "./input/data/map_water_light.txt",
            "./input/data/map_light_temperature.txt",
            "./input/data/map_temperature_humidity.txt",
            "./input/data/map_humidity_location.txt",
        ];
    }
}