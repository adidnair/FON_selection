use implementation::FogNetwork::{Instance::*, Nodes::{FGN, AvailableFCN, StatusMetricParameter}};
use implementation::User::{Application, ExpectationMetricParameter};
use implementation::Constants;

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

    let _ = test_fgn.application_placement();
    let _ = test_fgn.application_placement_with_lookup();

    // for _ in 0..100_000 {
    //     let _ = test_fgn.application_placement();
    // }
    // println!("{}", mapping);

    // // ERU
    // println!("[");
    // for i in 2..=10 {
    //     println!("    [");
    //     for j in 1..=8 {
    //         print!("        [");
    //         for k in (30..=120).step_by(10) {
    //             let mut i = i as f32;
    //             let mut j = j as f32;
    //             let mut k = k as f32;
    //             i = 2.0 * (i - Constants::ERU::ROE_PARAM_TEMPLATES[0].min)
    //                 / (Constants::ERU::ROE_PARAM_TEMPLATES[0].max - Constants::ERU::ROE_PARAM_TEMPLATES[0].min) -1.0;
    //             j = 2.0 * (j - Constants::ERU::ROE_PARAM_TEMPLATES[1].min)
    //                 / (Constants::ERU::ROE_PARAM_TEMPLATES[1].max - Constants::ERU::ROE_PARAM_TEMPLATES[1].min) -1.0;
    //             k = 2.0 * (k - Constants::ERU::ROE_PARAM_TEMPLATES[2].min)
    //                 / (Constants::ERU::ROE_PARAM_TEMPLATES[2].max - Constants::ERU::ROE_PARAM_TEMPLATES[2].min) -1.0;
    //
    //             // dbg!(i, j, k);
    //             let a = Constants::ERU::ProcessingTimeMembershipFunction(i);
    //             let b = Constants::ERU::AccessRateMembershipFunction(j);
    //             let c = Constants::ERU::RequiredResourcesMembershipFunction(k);
    //
    //             let mut ROE_numerator: f32 = 0.0;
    //             let mut ROE_denominator: f32 = 0.0;
    //
    //             for p in 0..3 {
    //                 for q in 0..3 {
    //                     for r in 0..3 {
    //                         let u = f32::max(a[p], f32::max(b[q], c[r]));
    //
    //                         ROE_numerator += u
    //                         * Constants::ERU::ROE_SINGLETON_VALUES[Constants::ERU::ROE_FUZZY_RULES[r][p][q]];
    //                         ROE_denominator += u;
    //                     }
    //                 }
    //             }
    //             print!("{:.2}, ", ROE_numerator / ROE_denominator);
    //         }
    //         println!("],");
    //     }
    //     println!("    ],");
    // }
    // println!("],");

    // // CSU
    // println!("[");
    // for i in (100..=600).step_by(100) {
    //     println!("    [");
    //     for j in 1..=10 {
    //         print!("        [");
    //         for k in (10..=70).step_by(10) {
    //             let mut i = i as f32;
    //             let mut j = j as f32;
    //             let mut k = k as f32;
    //             i = 2.0 * (i - Constants::CSU::CSS_PARAM_TEMPLATES[0].min)
    //                 / (Constants::CSU::CSS_PARAM_TEMPLATES[0].max - Constants::CSU::CSS_PARAM_TEMPLATES[0].min) -1.0;
    //             j = 2.0 * (j - Constants::CSU::CSS_PARAM_TEMPLATES[1].min)
    //                 / (Constants::CSU::CSS_PARAM_TEMPLATES[1].max - Constants::CSU::CSS_PARAM_TEMPLATES[1].min) -1.0;
    //             k = 2.0 * (k - Constants::CSU::CSS_PARAM_TEMPLATES[2].min)
    //                 / (Constants::CSU::CSS_PARAM_TEMPLATES[2].max - Constants::CSU::CSS_PARAM_TEMPLATES[2].min) -1.0;
    //
    //             // dbg!(i, j, k);
    //             let a = Constants::CSU::RoundTripTimeMembershipFunction(i);
    //             let b = Constants::CSU::ResourceAvailablityMembershipFunction(j);
    //             let c = Constants::CSU::ProcessingSpeedMembershipFunction(k);
    //
    //             let mut CSS_numerator: f32 = 0.0;
    //             let mut CSS_denominator: f32 = 0.0;
    //
    //             for p in 0..3 {
    //                 for q in 0..3 {
    //                     for r in 0..3 {
    //                         let u = f32::min(a[p], f32::min(b[q], c[r]));
    //
    //                         CSS_numerator += u
    //                         * Constants::CSU::CSS_SINGLETON_VALUES[Constants::CSU::CSS_FUZZY_RULES[r][p][q]];
    //                         CSS_denominator += u;
    //                     }
    //                 }
    //             }
    //             print!("{:.2}, ", CSS_numerator / CSS_denominator);
    //         }
    //         println!("],");
    //     }
    //     println!("    ],");
    // }
    // println!("],");

    Ok(())
}
