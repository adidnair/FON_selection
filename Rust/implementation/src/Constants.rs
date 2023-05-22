pub mod ERU {
    use crate::FogNetwork::Nodes::ExpectationMetricParameterTemplate;

    pub const ROE_FUZZY_OUTPUT_SET: [&'static str; 3] = ["High", "Medium", "Low"];

    pub const ROE_FUZZY_RULES: [[[usize; ROE_FUZZY_OUTPUT_SET.len()];
                                            ROE_FUZZY_OUTPUT_SET.len()];
                                            ROE_FUZZY_OUTPUT_SET.len()] = [
        [
            [1, 1, 0],
            [1, 0, 0],
            [0, 0, 0],
        ],
        [
            [1, 1, 1],
            [1, 1, 0],
            [1, 0, 0],
        ],
        [
            [2, 1, 1],
            [1, 1, 1],
            [1, 1, 0],
        ],
    ];

    pub const ROE_EXPECTED_PARAMS: [super::Application::ExpectationMetricParameterType; 3] = [
        super::Application::ExpectationMetricParameterType::ProcessingTime,
        super::Application::ExpectationMetricParameterType::AccessRate,
        super::Application::ExpectationMetricParameterType::RequiredResources,
    ];

    pub const ROE_PARAM_TEMPLATES: [ExpectationMetricParameterTemplate; ROE_EXPECTED_PARAMS.len()] = [
        ExpectationMetricParameterTemplate{min: 2.0, max: 10.0},
        ExpectationMetricParameterTemplate{min: 1.0, max: 8.0},
        ExpectationMetricParameterTemplate{min: 30.0, max: 120.0},
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
            // Small
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Regular
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Large
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
            // Stringent
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Moderate
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Flexible
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

pub mod CSU {
    use crate::FogNetwork::Nodes::StatusMetricParameterTemplate;

    pub const CSS_FUZZY_OUTPUT_SET: [&'static str; 3] = ["High", "Medial", "Lower"];

    pub const CSS_FUZZY_RULES: [[[usize; CSS_FUZZY_OUTPUT_SET.len()];
                                            CSS_FUZZY_OUTPUT_SET.len()];
                                            CSS_FUZZY_OUTPUT_SET.len()] = [
        [
            [2, 1, 1],
            [2, 2, 1],
            [2, 2, 2],
        ],
        [
            [1, 1, 1],
            [2, 1, 1],
            [2, 2, 1],
        ],
        [
            [1, 1, 0],
            [1, 1, 1],
            [2, 1, 1],
        ],
    ];

    pub const CSS_EXPECTED_PARAMS: [super::FCN::StatusMetricParameterType; 3] = [
        super::FCN::StatusMetricParameterType::RoundTripTime,
        super::FCN::StatusMetricParameterType::ResourceAvailability,
        super::FCN::StatusMetricParameterType::ProcessingSpeed,
    ];

    pub const CSS_PARAM_TEMPLATES: [StatusMetricParameterTemplate; CSS_EXPECTED_PARAMS.len()] = [
        StatusMetricParameterTemplate{min: 100.0, max: 600.0},
        StatusMetricParameterTemplate{min: 1.0, max: 10.0},
        StatusMetricParameterTemplate{min: 10.0, max: 70.0},
    ];

    pub const CSS_FUZZY_SETS: [[&'static str; 3]; CSS_EXPECTED_PARAMS.len()] = [
        // RoundTripTime
        ["Short", "Typical", "Lengthy"],
        // ResourceAvailablity
        ["Poor", "Standard", "Rich"],
        // ProcessingSpeed
        ["Least", "Average", "Intense"],
    ];

    pub fn RoundTripTimeMembershipFunction(val: f32) -> [f32; CSS_FUZZY_SETS[0].len()] {
        [
            // Short
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Typical
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Lengthy
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= 0.0 {0.0}
                else if val < 0.8 {val * (1.0/0.8)}
                else if val <= 1.0 {1.0}
                else {panic!("out of range.")}
            },
        ]
    }

    pub fn ResourceAvailablityMembershipFunction(val: f32) -> [f32; CSS_FUZZY_SETS[0].len()] {
        [
            // Poor
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Standard
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Rich
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= 0.0 {0.0}
                else if val < 0.8 {val * (1.0/0.8)}
                else if val <= 1.0 {1.0}
                else {panic!("out of range.")}
            },
        ]
    }

    pub fn ProcessingSpeedMembershipFunction(val: f32) -> [f32; CSS_FUZZY_SETS[0].len()] {
        [
            // Least
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {1.0}
                else if val < 0.0 {-val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Average
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= -0.8 {0.0}
                else if val < 0.0 {val * (1.0/0.8) + 1.0}
                else if val < 0.8 {1.0 - val * (1.0/0.8)}
                else if val <= 1.0 {0.0}
                else {panic!("out of range.")}
            },
            // Intense
            {
                if val < -1.0 {panic!("out of range.")}
                else if val <= 0.0 {0.0}
                else if val < 0.8 {val * (1.0/0.8)}
                else if val <= 1.0 {1.0}
                else {panic!("out of range.")}
            },
        ]
    }

    pub const CSS_SINGLETON_VALUES: [f32; CSS_FUZZY_OUTPUT_SET.len()] = [
        10.0, 5.0, 2.0
    ];

}

pub mod Application {
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub enum ExpectationMetricParameterType {
        AccessRate,
        RequiredResources,
        ProcessingTime,
    }

    use std::fmt;
    impl fmt::Display for ExpectationMetricParameterType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::AccessRate => {write!(f, "AccessRate")},
                Self::RequiredResources => {write!(f, "RequiredResources")},
                Self::ProcessingTime => {write!(f, "ProcessingTime")},
            }
        }
    }
}

pub mod FCN {
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub enum StatusMetricParameterType {
        RoundTripTime,
        ResourceAvailability,
        ProcessingSpeed,
    }

    use std::fmt;
    impl fmt::Display for StatusMetricParameterType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::RoundTripTime => {write!(f, "RoundTripTime")},
                Self::ResourceAvailability => {write!(f, "ResourceAvailability")},
                Self::ProcessingSpeed => {write!(f, "ProcessingSpeed")},
            }
        }
    }
}



