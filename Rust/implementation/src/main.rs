use FogNetwork::{Instance::*, Nodes::FGN};
use User::{Application, ApplicationParameter};

#[allow(non_snake_case)]
mod FogNetwork;
#[allow(non_snake_case)]
mod User;
#[allow(non_snake_case)]
mod Constants;

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
        FGN::new(String::from("Test_FGN_1"))
    )?;

    new_instance.add_FGN(
        FGN::new(String::from("Test_FGN_2"))
    )?;

    let test_app = Application::new(
        String::from("test_app"),
        vec![
            ApplicationParameter::new(
                Constants::Application::ParameterType::AccessRate, 2.7),
            ApplicationParameter::new(
                Constants::Application::ParameterType::RequiredResources, 3.4),
            ApplicationParameter::new(
                Constants::Application::ParameterType::ProcessingTime, 1.2),
        ],
    );

    // let request_result1 = test_app.send_request(
    //     &mut new_instance.get_FGN(String::from("Test_FGN_1")).unwrap());
    // let request_result2 = test_app.send_request(
    //     &mut new_instance.get_FGN(String::from("Test_FGN_does_not_exist")).unwrap());
    let request_result3 = test_app.send_request(
        &mut new_instance.get_FGN(String::from("Test_FGN_2")).unwrap());

    // dbg!(&request_result1);
    // dbg!(&request_result2);
    dbg!(&request_result3);

    Ok(())
}
