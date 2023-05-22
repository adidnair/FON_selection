use FogNetwork::{Instance::*, Nodes::{FGN, AvailableFCN, StatusMetricParameter}};
use User::{Application, ExpectationMetricParameter};

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
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::AccessRate, 2.7),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::RequiredResources, 3.4),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::ProcessingTime, 1.2),
        ],
    );

    // let request_result1 = test_app.send_request(
    //     &mut new_instance.get_FGN(String::from("Test_FGN_1")).unwrap());
    // let request_result2 = test_app.send_request(
    //     &mut new_instance.get_FGN(String::from("Test_FGN_does_not_exist")).unwrap());
    let test_fgn = new_instance.get_FGN(String::from("Test_FGN_2")).unwrap();
    let _request_result3 = test_app.send_placement_request(
        test_fgn);

    // dbg!(&request_result1);
    // dbg!(&request_result2);
    // dbg!(&request_result3);
    //

    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn1"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 1.2),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 3.1),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 2.1),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn2"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 3.2),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 0.7),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 1.4),
            ]
        },
    );

    let _ = test_fgn.application_placement();

    Ok(())
}
