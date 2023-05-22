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

    let test_fgn = new_instance.get_FGN(String::from("Test_FGN_2")).unwrap();

    let _ = Application::new(
        String::from("app1"),
        vec![
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::AccessRate, 2.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::RequiredResources, 2.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::ProcessingTime, 120.0),
        ],
    ).send_placement_request(test_fgn);
    let _ = Application::new(
        String::from("app2"),
        vec![
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::AccessRate, 5.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::RequiredResources, 5.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::ProcessingTime, 70.0),
        ],
    ).send_placement_request(test_fgn);
    let _ = Application::new(
        String::from("app3"),
        vec![
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::AccessRate, 3.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::RequiredResources, 3.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::ProcessingTime, 90.0),
        ],
    ).send_placement_request(test_fgn);
    let _ = Application::new(
        String::from("app4"),
        vec![
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::AccessRate, 7.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::RequiredResources, 8.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::ProcessingTime, 60.0),
        ],
    ).send_placement_request(test_fgn);
    let _ = Application::new(
        String::from("app5"),
        vec![
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::AccessRate, 8.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::RequiredResources, 3.0),
            ExpectationMetricParameter::new(
                Constants::Application::ExpectationMetricParameterType::ProcessingTime, 50.0),
        ],
    ).send_placement_request(test_fgn);

    // let request_result1 = test_app.send_request(
    //     &mut new_instance.get_FGN(String::from("Test_FGN_1")).unwrap());
    // let request_result2 = test_app.send_request(
    //     &mut new_instance.get_FGN(String::from("Test_FGN_does_not_exist")).unwrap());
    // let _request_result3 = test_app.send_placement_request(
    //     test_fgn);

    // dbg!(&request_result1);
    // dbg!(&request_result2);
    // dbg!(&request_result3);
    //

    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn1"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 100.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 3.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 20.0),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn2"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 100.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 2.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 20.0),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn3"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 200.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 4.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 40.0),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn4"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 300.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 5.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 30.0),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn5"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 400.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 6.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 50.0),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn6"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 500.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 8.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 70.0),
            ]
        },
    );
    let _ = test_fgn.add_available_fcn(
        AvailableFCN {
            id: String::from("fcn7"),
            params: vec![
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::RoundTripTime , 500.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ResourceAvailability , 6.0),
                StatusMetricParameter::new(
                    Constants::FCN::StatusMetricParameterType::ProcessingSpeed , 60.0),
            ]
        },
    );

    let mapping = test_fgn.application_placement().unwrap();
    println!("{}", mapping);

    Ok(())
}
