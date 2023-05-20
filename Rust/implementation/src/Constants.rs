pub mod ERU {
    use crate::FogNetwork::Nodes::ApplicationParameterTemplate;

    pub const ROE_FUZZY_OUTPUT_SET: [&'static str; 3] = ["High", "Medium", "Low"];

    pub const ROE_FUZZY_RULES: [[[usize; ROE_FUZZY_OUTPUT_SET.len()];
                                            ROE_FUZZY_OUTPUT_SET.len()];
                                            ROE_FUZZY_OUTPUT_SET.len()] = [
        [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 1],
        ],
        [
            [0, 0, 1],
            [0, 1, 1],
            [1, 1, 1],
        ],
        [
            [0, 1, 1],
            [1, 1, 1],
            [1, 1, 2],
        ],
    ];

    pub const ROE_EXPECTED_PARAMS: [super::Application::ParameterType; 3] = [
        super::Application::ParameterType::ProcessingTime,
        super::Application::ParameterType::AccessRate,
        super::Application::ParameterType::RequiredResources,
    ];

    pub const ROE_PARAM_TEMPLATES: [ApplicationParameterTemplate; ROE_EXPECTED_PARAMS.len()] = [
        ApplicationParameterTemplate{min: 0.0, max: 4.0},
        ApplicationParameterTemplate{min: 0.0, max: 4.0},
        ApplicationParameterTemplate{min: 0.0, max: 4.0},
    ];

    pub const ROE_FUZZY_SETS: [[&'static str; 3]; ROE_EXPECTED_PARAMS.len()] = [
        ["Slow", "Normal", "Fast"],
        ["Small", "Regular", "Large"],
        ["Stringent", "Moderate", "Flexible"],
    ];

    pub fn AccessRateMembershipFunction(val: f32) -> [f32; ROE_FUZZY_SETS[0].len()] {
        [
            // Slow
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Normal
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Fast
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= 0.0 {0.0}
                else if val < 0.8 {val * (1.0/0.8)}
                else if val <= 1.0 {1.0}
                else {panic!("out of range.")}
            },
        ]
    }

    pub fn RequiredResourcesMembershipFunction(val: f32) -> [f32; ROE_FUZZY_SETS[0].len()] {
        [
            // Slow
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Normal
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Fast
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= 0.0 {0.0}
                else if val < 0.8 {val * (1.0/0.8)}
                else if val <= 1.0 {1.0}
                else {panic!("out of range.")}
            },
        ]
    }

    pub fn ProcessingTimeMembershipFunction(val: f32) -> [f32; ROE_FUZZY_SETS[0].len()] {
        [
            // Slow
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Normal
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Fast
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= 0.0 {0.0}
                else if val < 0.8 {val * (1.0/0.8)}
                else if val <= 1.0 {1.0}
                else {panic!("out of range.")}
            },
        ]
    }

    pub const ROE_SINGLETON_VALUES: [f32; ROE_FUZZY_OUTPUT_SET.len()] = [
        10.0, 5.0, 2.0
    ];

}

pub mod Application {
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub enum ParameterType {
        AccessRate,
        RequiredResources,
        ProcessingTime,
    }
}
