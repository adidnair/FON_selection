use crate::FogNetwork::Nodes::ApplicationParameterTemplate;
use std::collections::HashMap;

use FogNetwork::{Instance::*, Nodes::FGN};
use User::{Application, ApplicationParameterType, ApplicationParameter};

#[allow(non_snake_case)]
mod FogNetwork;
#[allow(non_snake_case)]
mod User;

fn main() -> Result<(), String> {
    let mut new_instance = Instance::new_empty(
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

    new_instance.add_FGN(
        FGN::new(
            String::from("Test_FGN_1"),
            HashMap::from([
                (ApplicationParameterType::AccessRate, ApplicationParameterTemplate::new(0.0, 4.0)),
                (ApplicationParameterType::RequiredResources, ApplicationParameterTemplate::new(0.0, 4.0)),
            ]),
        )
    )?;

    new_instance.add_FGN(
        FGN::new(
            String::from("Test_FGN_2"),
            HashMap::from([
                (ApplicationParameterType::AccessRate, ApplicationParameterTemplate::new(0.0, 10.0)),
                (ApplicationParameterType::RequiredResources, ApplicationParameterTemplate::new(0.0, 10.0)),
            ]),
        )
    )?;

    let test_app = Application::new(
        String::from("test_app"),
        vec![
            ApplicationParameter::new(
                ApplicationParameterType::AccessRate, 2.7),
            ApplicationParameter::new(
                ApplicationParameterType::RequiredResources, 5.3),
        ],
    );

    let request_result = test_app.send_request(
        &mut new_instance.get_FGN(String::from("Test_FGN_1")).unwrap());

    dbg!(&request_result);

    Ok(())
}
