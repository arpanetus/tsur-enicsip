#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
    for light in lights.iter_mut() {
        if light.alias == alias {
            light.brightness = value;
        }
    }
}


#[test]
fn test_change_brightness() {
    let mut lights = vec![
        Light::new("living_room"),
        Light::new("bedroom"),
        Light::new("rest_room"),
    ];
    
    for light in &lights {
        assert_eq!(light.brightness, 0);
    }

    change_brightness(&mut lights, "living_room", 200);
    
    for light in lights {
        if light.alias == "living_room" {
            assert_eq!(light.brightness, 200);
        } else {
            assert_eq!(light.brightness, 0);
        }
    }
}