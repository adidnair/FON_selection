use FogNetwork::Instance::*;

#[allow(non_snake_case)]
mod FogNetwork;
mod User;

fn main() {
    let new_instance = Instance::new_empty(
        vec![
           Parameter::new(
                String::from("RoundTripTime"),
                1.0f32,
                false,
            ),
            Parameter::new(
                String::from("ResourceAvailability"),
                1.0f32,
                true,
            ),
            Parameter::new(
                String::from("ProcessingSpeed"),
                1.0f32,
                true,
            ),
        ],
    );
    let param = Parameter {
                String::from("RoundTripTime"),
                1.0f32,
                false,
    };
}
