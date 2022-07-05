use std::time::SystemTime;

pub struct Device
{
    pub device_name: String,
    pub description: String,
    pub coordinates: (f32, f32),
    pub battery_life: f32,
    pub time: SystemTime,
    pub device_ID: i32
}