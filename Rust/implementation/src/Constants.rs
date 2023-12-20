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


    pub const ROE_LOOK_UP: [[[f32; 10]; 8]; 9] = [
        [
            [6.42, 6.41, 6.34, 6.27, 6.19, 6.08, 5.94, 5.79, 5.65, 5.63, ],
            [6.45, 6.44, 6.34, 6.27, 6.19, 6.14, 5.99, 5.84, 5.70, 5.69, ],
            [6.54, 6.53, 6.39, 6.28, 6.28, 6.24, 6.16, 6.02, 5.89, 5.88, ],
            [6.64, 6.62, 6.52, 6.44, 6.38, 6.34, 6.26, 6.19, 6.08, 6.06, ],
            [6.78, 6.75, 6.57, 6.49, 6.44, 6.40, 6.31, 6.24, 6.20, 6.20, ],
            [6.97, 6.94, 6.67, 6.47, 6.53, 6.50, 6.35, 6.30, 6.30, 6.30, ],
            [7.15, 7.13, 6.95, 6.80, 6.65, 6.60, 6.52, 6.45, 6.39, 6.39, ],
            [7.21, 7.20, 7.05, 6.90, 6.76, 6.65, 6.57, 6.50, 6.43, 6.42, ],
        ],
        [
            [6.44, 6.42, 6.34, 6.27, 6.19, 6.12, 5.97, 5.82, 5.68, 5.66, ],
            [6.50, 6.48, 6.37, 6.30, 6.22, 6.18, 6.03, 5.87, 5.73, 5.71, ],
            [6.59, 6.57, 6.42, 6.31, 6.31, 6.29, 6.21, 6.06, 5.92, 5.90, ],
            [6.69, 6.67, 6.56, 6.48, 6.41, 6.39, 6.31, 6.23, 6.11, 6.09, ],
            [6.81, 6.79, 6.61, 6.53, 6.47, 6.43, 6.34, 6.27, 6.22, 6.20, ],
            [7.00, 6.98, 6.71, 6.51, 6.57, 6.53, 6.38, 6.33, 6.31, 6.30, ],
            [7.19, 7.17, 7.00, 6.85, 6.69, 6.64, 6.56, 6.48, 6.41, 6.39, ],
            [7.22, 7.22, 7.09, 6.94, 6.79, 6.70, 6.62, 6.55, 6.46, 6.44, ],
        ],
        [
            [6.52, 6.51, 6.37, 6.27, 6.26, 6.22, 6.13, 5.97, 5.84, 5.83, ],
            [6.60, 6.58, 6.42, 6.32, 6.31, 6.30, 6.20, 6.04, 5.89, 5.88, ],
            [6.85, 6.83, 6.64, 6.50, 6.47, 6.47, 6.48, 6.29, 6.08, 6.06, ],
            [6.87, 6.85, 6.70, 6.60, 6.50, 6.50, 6.51, 6.41, 6.23, 6.21, ],
            [6.96, 6.94, 6.75, 6.65, 6.56, 6.52, 6.51, 6.41, 6.27, 6.26, ],
            [7.20, 7.18, 6.95, 6.73, 6.68, 6.64, 6.58, 6.47, 6.32, 6.30, ],
            [7.26, 7.25, 7.18, 7.05, 6.85, 6.80, 6.82, 6.69, 6.48, 6.47, ],
            [7.27, 7.27, 7.25, 7.13, 6.94, 6.86, 6.88, 6.76, 6.54, 6.52, ],
        ],
        [
            [6.60, 6.59, 6.45, 6.37, 6.34, 6.30, 6.23, 6.15, 6.01, 5.99, ],
            [6.69, 6.67, 6.52, 6.43, 6.39, 6.39, 6.31, 6.22, 6.06, 6.05, ],
            [6.92, 6.91, 6.73, 6.60, 6.53, 6.53, 6.56, 6.44, 6.21, 6.19, ],
            [7.00, 6.99, 6.85, 6.68, 6.56, 6.56, 6.56, 6.50, 6.31, 6.29, ],
            [7.10, 7.08, 6.90, 6.74, 6.61, 6.57, 6.56, 6.50, 6.36, 6.34, ],
            [7.26, 7.24, 7.03, 6.86, 6.80, 6.76, 6.70, 6.59, 6.42, 6.41, ],
            [7.31, 7.30, 7.23, 7.15, 6.99, 6.94, 6.92, 6.79, 6.57, 6.55, ],
            [7.32, 7.32, 7.31, 7.23, 7.07, 7.00, 6.98, 6.85, 6.62, 6.60, ],
        ],
        [
            [6.68, 6.68, 6.60, 6.53, 6.46, 6.38, 6.31, 6.24, 6.17, 6.16, ],
            [6.73, 6.72, 6.63, 6.55, 6.48, 6.43, 6.36, 6.28, 6.20, 6.19, ],
            [6.88, 6.87, 6.75, 6.64, 6.54, 6.50, 6.52, 6.42, 6.29, 6.28, ],
            [7.03, 7.02, 6.89, 6.74, 6.60, 6.56, 6.52, 6.48, 6.39, 6.37, ],
            [7.15, 7.14, 6.97, 6.82, 6.69, 6.63, 6.57, 6.53, 6.47, 6.47, ],
            [7.25, 7.23, 7.06, 6.92, 6.88, 6.82, 6.73, 6.64, 6.57, 6.56, ],
            [7.34, 7.32, 7.23, 7.15, 7.08, 7.01, 6.89, 6.77, 6.66, 6.66, ],
            [7.37, 7.36, 7.29, 7.21, 7.14, 7.05, 6.93, 6.81, 6.70, 6.68, ],
        ],
        [
            [6.85, 6.82, 6.55, 6.46, 6.47, 6.44, 6.32, 6.24, 6.24, 6.24, ],
            [6.91, 6.89, 6.61, 6.52, 6.53, 6.49, 6.37, 6.29, 6.25, 6.24, ],
            [7.10, 7.08, 6.84, 6.70, 6.62, 6.58, 6.56, 6.45, 6.30, 6.28, ],
            [7.19, 7.18, 6.97, 6.80, 6.71, 6.67, 6.60, 6.55, 6.43, 6.42, ],
            [7.22, 7.21, 7.00, 6.83, 6.76, 6.73, 6.65, 6.60, 6.48, 6.46, ],
            [7.32, 7.30, 7.09, 6.93, 6.91, 6.88, 6.81, 6.69, 6.51, 6.50, ],
            [7.42, 7.40, 7.29, 7.21, 7.14, 7.12, 7.07, 6.94, 6.77, 6.75, ],
            [7.45, 7.44, 7.36, 7.28, 7.22, 7.19, 7.15, 7.03, 6.86, 6.85, ],
        ],
        [
            [7.01, 6.99, 6.72, 6.53, 6.56, 6.52, 6.39, 6.33, 6.32, 6.32, ],
            [7.08, 7.06, 6.80, 6.60, 6.62, 6.58, 6.45, 6.38, 6.34, 6.32, ],
            [7.27, 7.25, 7.02, 6.85, 6.75, 6.71, 6.69, 6.53, 6.37, 6.35, ],
            [7.28, 7.26, 7.07, 6.95, 6.89, 6.85, 6.76, 6.64, 6.52, 6.50, ],
            [7.31, 7.29, 7.10, 6.99, 6.94, 6.91, 6.82, 6.70, 6.57, 6.55, ],
            [7.40, 7.39, 7.18, 7.05, 7.02, 6.99, 6.96, 6.82, 6.62, 6.60, ],
            [7.50, 7.49, 7.38, 7.30, 7.23, 7.21, 7.21, 7.12, 6.94, 6.92, ],
            [7.53, 7.53, 7.45, 7.37, 7.30, 7.28, 7.29, 7.20, 7.03, 7.01, ],
        ],
        [
            [7.18, 7.15, 6.99, 6.85, 6.70, 6.62, 6.55, 6.47, 6.41, 6.40, ],
            [7.22, 7.21, 7.07, 6.92, 6.76, 6.69, 6.62, 6.54, 6.43, 6.42, ],
            [7.28, 7.26, 7.23, 7.13, 6.92, 6.86, 6.91, 6.75, 6.53, 6.51, ],
            [7.34, 7.32, 7.26, 7.18, 7.08, 7.02, 6.92, 6.80, 6.63, 6.61, ],
            [7.40, 7.38, 7.29, 7.21, 7.14, 7.11, 7.01, 6.88, 6.74, 6.72, ],
            [7.49, 7.48, 7.39, 7.30, 7.24, 7.21, 7.23, 7.11, 6.93, 6.91, ],
            [7.59, 7.57, 7.49, 7.41, 7.34, 7.30, 7.26, 7.21, 7.12, 7.10, ],
            [7.62, 7.61, 7.54, 7.46, 7.39, 7.34, 7.30, 7.26, 7.19, 7.18, ],
        ],
        [
            [7.21, 7.20, 7.05, 6.90, 6.76, 6.65, 6.57, 6.50, 6.43, 6.42, ],
            [7.23, 7.23, 7.11, 6.96, 6.82, 6.73, 6.66, 6.58, 6.47, 6.45, ],
            [7.28, 7.28, 7.27, 7.18, 6.97, 6.90, 6.94, 6.79, 6.56, 6.54, ],
            [7.34, 7.34, 7.30, 7.22, 7.13, 7.05, 6.96, 6.83, 6.66, 6.64, ],
            [7.42, 7.41, 7.33, 7.25, 7.18, 7.16, 7.05, 6.93, 6.79, 6.78, ],
            [7.51, 7.50, 7.42, 7.34, 7.28, 7.25, 7.27, 7.15, 6.98, 6.97, ],
            [7.60, 7.60, 7.52, 7.45, 7.37, 7.34, 7.30, 7.25, 7.17, 7.15, ],
            [7.63, 7.62, 7.55, 7.48, 7.40, 7.35, 7.30, 7.26, 7.21, 7.21, ],
        ],
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

    pub const CSS_LOOK_UP: [[[f32; 7]; 10]; 6] = [
        [
            [2.00, 2.50, 3.75, 5.00, 5.00, 5.00, 5.00, ],
            [2.08, 2.63, 3.82, 5.00, 5.00, 5.00, 5.00, ],
            [2.92, 3.44, 4.22, 5.00, 5.00, 5.00, 5.00, ],
            [3.75, 4.06, 4.32, 5.00, 5.00, 5.00, 5.00, ],
            [4.58, 4.67, 4.67, 5.00, 5.00, 5.00, 5.00, ],
            [5.00, 5.00, 5.00, 5.00, 5.54, 5.54, 5.69, ],
            [5.00, 5.00, 5.00, 5.00, 6.14, 6.56, 7.08, ],
            [5.00, 5.00, 5.00, 5.00, 6.29, 7.60, 8.47, ],
            [5.00, 5.00, 5.00, 5.00, 6.97, 8.95, 9.86, ],
            [5.00, 5.00, 5.00, 5.00, 7.08, 9.17, 10.00, ],
        ],
        [
            [2.00, 2.38, 3.17, 4.25, 4.50, 4.62, 5.00, ],
            [2.08, 2.52, 3.24, 4.29, 4.53, 4.65, 5.00, ],
            [2.61, 3.12, 3.66, 4.50, 4.71, 4.77, 5.00, ],
            [3.17, 3.50, 3.76, 4.50, 4.74, 4.77, 5.00, ],
            [3.76, 3.94, 4.03, 4.67, 4.80, 4.78, 5.00, ],
            [4.41, 4.60, 4.64, 5.00, 5.34, 5.37, 5.54, ],
            [4.50, 4.65, 4.74, 5.00, 5.74, 5.96, 6.39, ],
            [4.50, 4.65, 4.71, 5.00, 5.80, 6.60, 7.31, ],
            [4.92, 4.94, 4.95, 5.00, 6.29, 7.60, 8.55, ],
            [5.00, 5.00, 5.00, 5.00, 6.39, 7.81, 8.75, ],
        ],
        [
            [2.00, 2.38, 2.50, 2.75, 3.83, 4.62, 5.00, ],
            [2.08, 2.52, 2.62, 2.87, 3.91, 4.65, 5.00, ],
            [2.50, 3.04, 3.21, 3.61, 4.33, 4.77, 5.00, ],
            [2.50, 3.04, 3.41, 4.17, 4.56, 4.77, 5.00, ],
            [2.59, 3.15, 3.78, 4.67, 4.80, 4.78, 5.00, ],
            [3.24, 3.81, 4.39, 5.00, 5.34, 5.37, 5.54, ],
            [3.83, 4.19, 4.56, 5.00, 5.44, 5.58, 5.83, ],
            [4.39, 4.58, 4.65, 5.00, 5.48, 5.58, 5.83, ],
            [4.92, 4.94, 4.95, 5.00, 5.78, 5.87, 6.18, ],
            [5.00, 5.00, 5.00, 5.00, 5.83, 5.94, 6.25, ],
        ],
        [
            [2.00, 2.00, 2.00, 2.00, 2.83, 3.69, 4.25, ],
            [2.00, 2.06, 2.05, 2.08, 2.93, 3.73, 4.29, ],
            [2.00, 2.23, 2.35, 2.61, 3.47, 3.96, 4.50, ],
            [2.00, 2.23, 2.62, 3.17, 3.76, 3.96, 4.50, ],
            [2.00, 2.26, 2.85, 3.76, 4.03, 4.07, 4.67, ],
            [2.33, 2.93, 3.46, 4.41, 4.64, 4.74, 5.00, ],
            [2.83, 3.27, 3.76, 4.50, 4.74, 4.77, 5.00, ],
            [3.39, 3.65, 3.79, 4.50, 4.71, 4.77, 5.00, ],
            [4.13, 4.31, 4.38, 4.92, 4.95, 4.94, 5.00, ],
            [4.25, 4.44, 4.50, 5.00, 5.00, 5.00, 5.00, ],
        ],
        [
            [2.00, 2.00, 2.00, 2.00, 2.50, 2.56, 2.75, ],
            [2.00, 2.06, 2.05, 2.08, 2.62, 2.69, 2.87, ],
            [2.00, 2.23, 2.29, 2.50, 3.21, 3.35, 3.61, ],
            [2.00, 2.23, 2.26, 2.50, 3.24, 3.73, 4.17, ],
            [2.00, 2.26, 2.36, 2.59, 3.54, 4.07, 4.67, ],
            [2.33, 2.93, 2.97, 3.24, 4.15, 4.74, 5.00, ],
            [2.50, 3.04, 3.24, 3.83, 4.38, 4.77, 5.00, ],
            [2.50, 3.04, 3.53, 4.39, 4.65, 4.77, 5.00, ],
            [2.71, 3.27, 4.07, 4.92, 4.95, 4.94, 5.00, ],
            [2.75, 3.31, 4.17, 5.00, 5.00, 5.00, 5.00, ],
        ],
        [
            [2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00, ],
            [2.00, 2.00, 2.00, 2.00, 2.08, 2.08, 2.08, ],
            [2.00, 2.00, 2.00, 2.00, 2.57, 2.69, 2.92, ],
            [2.00, 2.00, 2.00, 2.00, 2.68, 3.31, 3.75, ],
            [2.00, 2.00, 2.00, 2.00, 2.98, 3.96, 4.58, ],
            [2.00, 2.33, 2.33, 2.42, 3.63, 4.61, 5.00, ],
            [2.00, 2.38, 2.68, 3.25, 4.05, 4.62, 5.00, ],
            [2.00, 2.38, 3.09, 4.08, 4.43, 4.62, 5.00, ],
            [2.00, 2.47, 3.66, 4.92, 4.92, 4.92, 5.00, ],
            [2.00, 2.50, 3.75, 5.00, 5.00, 5.00, 5.00, ],
        ],
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



