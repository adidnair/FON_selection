use std::collections::HashMap;
use crate::Constants;
use crate::User::ExpectationMetricParameter;

const NONE_APP_PARAM_WORKAROUND: Option<ExpectationMetricParameter> = None;
const NONE_STAT_PARAM_WORKAROUND: Option<StatusMetricParameter> = None;
// const VEC_ARRAY_WORKAROUND: Vec<usize> = Vec::new();

// Structure of property of FGN
pub struct FGNProperty {
    id: String,
    value: f32,
}

pub struct ExpectationMetricParameterTemplate {
    pub min: f32,
    pub max: f32,
}

pub struct StatusMetricParameterTemplate {
    pub min: f32,
    pub max: f32,
}

pub struct ERU {
    parameters: [Option<ExpectationMetricParameter>; Constants::ERU::ROE_EXPECTED_PARAMS.len()],
    // ROE_fuzzy_rules: [[[usize; 3]; 3]; 3],
}

pub struct CSU {
    parameters: [Option<StatusMetricParameter>; Constants::ERU::ROE_EXPECTED_PARAMS.len()],
    // CSS_fuzzy_rules: [[[usize; 3]; 3]; 3],
}

#[derive(Clone, Debug)]
pub struct StatusMetricParameter {
    pub param_type: Constants::FCN::StatusMetricParameterType,
    pub val: f32,
}

impl StatusMetricParameter {
    pub fn new(param_type: Constants::FCN::StatusMetricParameterType, val: f32) -> Self {
        Self { val, param_type }
    }
}

pub struct ApplicationRequest {
    pub id: String,
    pub params: Vec<ExpectationMetricParameter>
}

pub struct AvailableFCN {
    pub id: String,
    pub params: Vec<StatusMetricParameter>
}

// Structure of FGN
pub struct FGN {
    id: String,
    properties: HashMap<String, f32>,
    // Expectation Rating Unit
    application_requests: Vec<ApplicationRequest>,
    available_fcns: Vec<AvailableFCN>,
    ERU: ERU,
    CSU: CSU,
}

impl FGN {
    pub fn new(id: String) -> Self {
        Self {
            id,
            properties: HashMap::new(),
            application_requests: Vec::new(),
            available_fcns: Vec::new(),
            ERU: ERU {
                parameters: [NONE_APP_PARAM_WORKAROUND; Constants::ERU::ROE_EXPECTED_PARAMS.len()],
                // ROE_fuzzy_rules: Constants::ERU::ROE_FUZZY_RULES,
            },
            CSU: CSU {
                parameters: [NONE_STAT_PARAM_WORKAROUND; Constants::ERU::ROE_EXPECTED_PARAMS.len()],
                // CSS_fuzzy_rules: Constants::CSU::CSS_FUZZY_RULES,
            },
        }
    }

    // Add new property to FGN
    pub fn add_property(
        &mut self, new_property: FGNProperty
    ) -> Result<() , String> {
        if self.properties.contains_key(&new_property.id) {
            Err(format!("Property with id \"{}\" already exists.",
                new_property.id))
        } else {
            self.properties.insert(new_property.id, new_property.value);
            Ok(())
        }
    }

    // Get value of property of FGN
    pub fn get_property_value(
        &self, property_id: String
    ) -> Result<f32, String> {
        if let Some(x) = self.properties.get(&property_id) {
            Ok((*x).clone())
        } else {
            Err(format!("FGN \"{}\" has no property with id \"{}\"",
                self.id, property_id))
        }
    }

    // Get id of FGN
    pub fn get_id(&self) -> &String {
        &self.id
    }


    pub fn get_application_request(&mut self, app_request: ApplicationRequest) -> Result<(), String> {
        
        self.application_requests.push(app_request);
        Ok(())
    }

    pub fn add_available_fcn(&mut self, new_fcn: AvailableFCN) -> Result<(), String> {
        self.available_fcns.push(new_fcn);
        Ok(())
    }

    pub fn application_placement(&mut self) -> Result<String, String> {
        if self.application_requests.len() >= self.available_fcns.len() {
            return Err(String::from("Not enough FCNs to map applications to."));
        }

        let mut ROEs: Vec<f32> = self.application_requests.iter()
            .map(|app| {
                self.ERU.ROE(app.params.clone()).unwrap()
            }).collect();
        let mut CSSs: Vec<f32> = self.available_fcns.iter()
            .map(|fcn| {
                self.CSU.CSS(fcn.params.clone()).unwrap()
            }).collect();

        let mut ROE_indices: Vec<usize> = (0..ROEs.len()).collect();
        let mut CSS_indices: Vec<usize> = (0..CSSs.len()).collect();

        for i in 0..ROE_indices.len() {
            let mut j = i;
            while j > 0 && ROEs[j-1] < ROEs[j] {
                ROEs.swap(j, j-1);
                ROE_indices.swap(j, j-1);
                j -= 1;
            }
        }

        for i in 0..CSS_indices.len() {
            let mut j = i;
            while j > 0 && CSSs[j-1] < CSSs[j] {
                CSSs.swap(j, j-1);
                CSS_indices.swap(j, j-1);
                j -= 1;
            }
        }

        let mappings = ROE_indices.iter().enumerate().map(|(i, &roe)| {
            self.application_requests[roe].id.clone() + &String::from(" - ")
                + &self.available_fcns[CSS_indices[i]].id
        }).collect::<Vec<String>>().join("\n");

        Ok(String::from(
            String::from("Mapping: \n") + &mappings
        ))
    }

    pub fn application_placement_with_lookup(&mut self) -> Result<String, String> {
        if self.application_requests.len() >= self.available_fcns.len() {
            return Err(String::from("Not enough FCNs to map applications to."));
        }

        let mut ROEs: Vec<f32> = self.application_requests.iter()
            .map(|app| {
                self.ERU.ROE_lookup(app.params.clone()).unwrap()
            }).collect();
        let mut CSSs: Vec<f32> = self.available_fcns.iter()
            .map(|fcn| {
                self.CSU.CSS_lookup(fcn.params.clone()).unwrap()
            }).collect();

        let mut ROE_indices: Vec<usize> = (0..ROEs.len()).collect();
        let mut CSS_indices: Vec<usize> = (0..CSSs.len()).collect();

        for i in 0..ROE_indices.len() {
            let mut j = i;
            while j > 0 && ROEs[j-1] < ROEs[j] {
                ROEs.swap(j, j-1);
                ROE_indices.swap(j, j-1);
                j -= 1;
            }
        }

        for i in 0..CSS_indices.len() {
            let mut j = i;
            while j > 0 && CSSs[j-1] < CSSs[j] {
                CSSs.swap(j, j-1);
                CSS_indices.swap(j, j-1);
                j -= 1;
            }
        }

        let mappings = ROE_indices.iter().enumerate().map(|(i, &roe)| {
            self.application_requests[roe].id.clone() + &String::from(" - ")
                + &self.available_fcns[CSS_indices[i]].id
        }).collect::<Vec<String>>().join("\n");

        Ok(String::from(
            String::from("Mapping: \n") + &mappings
        ))
    }
}

impl ERU {
    fn get_membership_values(&self) -> [
        [f32; Constants::ERU::ROE_FUZZY_SETS[0].len()];
        Constants::ERU::ROE_FUZZY_SETS.len()
    ] {
        [
            Constants::ERU::ProcessingTimeMembershipFunction(self.parameters[0].as_ref().unwrap().val),
            Constants::ERU::AccessRateMembershipFunction(self.parameters[1].as_ref().unwrap().val),
            Constants::ERU::RequiredResourcesMembershipFunction(self.parameters[2].as_ref().unwrap().val),
        ]
    }


    pub fn ROE(&mut self, expectation_params: Vec<ExpectationMetricParameter>) -> Result<f32, String> {
        // filter parameters
        for (i, param) in expectation_params.iter().enumerate() {
            for expected_type in Constants::ERU::ROE_EXPECTED_PARAMS {
                if param.param_type == expected_type {
                    if param.val >= Constants::ERU::ROE_PARAM_TEMPLATES[i].min
                    && param.val <= Constants::ERU::ROE_PARAM_TEMPLATES[i].max {
                        self.parameters[i] = Some(param.clone());
                    } else {
                        println!("Value of parameter \"{} ({:?})\" is out of the bounds \
                            defined by the Fog Network. Omitting...", param.param_type, param.val);
                    }
                    continue;
                }
            }
        }

        for param in &self.parameters {
            if param.is_none() {
                return Err(String::from("All required parameters were not supplied."));
            }
        }
        // normalize parameters
        for (i, param) in self.parameters.iter_mut().enumerate() {
            if let Some(ref mut raw_param) = param {
                raw_param.val = 2.0 * (raw_param.val - Constants::ERU::ROE_PARAM_TEMPLATES[i].min)
                    / (Constants::ERU::ROE_PARAM_TEMPLATES[i].max - Constants::ERU::ROE_PARAM_TEMPLATES[i].min) - 1.0;
            }
        }

        // println!("({:?}, {:?}, {:?})", self.parameters[0].as_ref().unwrap().val, self.parameters[1].as_ref().unwrap().val, self.parameters[2].as_ref().unwrap().val);

        // 1. calculate membership function for each value of each parameter
        let membership_values = self.get_membership_values();

        let mut ROE_numerator: f32 = 0.0;
        let mut ROE_denominator: f32 = 0.0;

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let fuzzy_output_membership_value = f32::max(
                        membership_values[0][i],
                        f32::max(
                            membership_values[1][j],
                            membership_values[2][k]
                        )
                    );
                    ROE_numerator += fuzzy_output_membership_value
                    * Constants::ERU::ROE_SINGLETON_VALUES[Constants::ERU::ROE_FUZZY_RULES[k][i][j]];
                    ROE_denominator += fuzzy_output_membership_value;
                }
            }
        }

        Ok(ROE_numerator / ROE_denominator)
    }

    pub fn ROE_lookup(&mut self, expectation_params: Vec<ExpectationMetricParameter>) -> Result<f32, String> {
        // filter parameters
        for (i, param) in expectation_params.iter().enumerate() {
            for expected_type in Constants::ERU::ROE_EXPECTED_PARAMS {
                if param.param_type == expected_type {
                    if param.val >= Constants::ERU::ROE_PARAM_TEMPLATES[i].min
                    && param.val <= Constants::ERU::ROE_PARAM_TEMPLATES[i].max {
                        self.parameters[i] = Some(param.clone());
                    } else {
                        println!("Value of parameter \"{} ({:?})\" is out of the bounds \
                            defined by the Fog Network. Omitting...", param.param_type, param.val);
                    }
                    continue;
                }
            }
        }

        for param in &self.parameters {
            if param.is_none() {
                return Err(String::from("All required parameters were not supplied."));
            }
        }

        Ok(Constants::ERU::ROE_LOOK_UP
           [{
               let val = self.parameters[0].as_ref().unwrap().val;
               if val < 2.5 {0}
               else if val < 3.5 {1}
               else if val < 4.5 {2}
               else if val < 5.5 {3}
               else if val < 6.5 {4}
               else if val < 7.5 {5}
               else if val < 8.5 {6}
               else if val < 9.5 {7}
               else{8}
           }]
           [{
               let val = self.parameters[1].as_ref().unwrap().val;
               if val < 1.5 {0}
               else if val < 2.5 {1}
               else if val < 3.5 {2}
               else if val < 4.5 {3}
               else if val < 5.5 {4}
               else if val < 6.5 {5}
               else if val < 7.5 {6}
               else{7}
           }]
           [{
               let val = self.parameters[2].as_ref().unwrap().val;
               if val < 35.0 {0}
               else if val < 45.0 {1}
               else if val < 55.0 {2}
               else if val < 65.0 {3}
               else if val < 75.0 {4}
               else if val < 85.0 {5}
               else if val < 95.0 {6}
               else if val < 105.0 {7}
               else if val < 115.0 {8}
               else{9}
           }]
       )
    }
}

impl CSU {
    fn get_membership_values(&self) -> [
        [f32; Constants::CSU::CSS_FUZZY_SETS[0].len()];
        Constants::CSU::CSS_FUZZY_SETS.len()
    ] {
        [
            Constants::CSU::RoundTripTimeMembershipFunction(self.parameters[0].as_ref().unwrap().val),
            Constants::CSU::ResourceAvailablityMembershipFunction(self.parameters[1].as_ref().unwrap().val),
            Constants::CSU::ProcessingSpeedMembershipFunction(self.parameters[2].as_ref().unwrap().val),
        ]
    }

    pub fn CSS(&mut self, status_params: Vec<StatusMetricParameter>) -> Result<f32, String> {
        // filter parameters
        for (i, param) in status_params.iter().enumerate() {
            for expected_type in Constants::CSU::CSS_EXPECTED_PARAMS {
                if param.param_type == expected_type {
                    if param.val >= Constants::CSU::CSS_PARAM_TEMPLATES[i].min
                    && param.val <= Constants::CSU::CSS_PARAM_TEMPLATES[i].max {
                        self.parameters[i] = Some(param.clone());
                    } else {
                        println!("Value of parameter \"{}\" is out of the bounds \
                            defined by the Fog Network. Omitting...", param.param_type);
                    }
                    continue;
                }
            }
        }

        for param in &self.parameters {
            if param.is_none() {
                return Err(String::from("All required parameters were not supplied."));
            }
        }
        // normalize parameters
        for (i, param) in self.parameters.iter_mut().enumerate() {
            if let Some(ref mut raw_param) = param {
                raw_param.val = 2.0 * (raw_param.val - Constants::CSU::CSS_PARAM_TEMPLATES[i].min)
                    / (Constants::CSU::CSS_PARAM_TEMPLATES[i].max - Constants::CSU::CSS_PARAM_TEMPLATES[i].min) - 1.0;
            }
        }

        // 1. calculate membership function for each value of each parameter
        let membership_values = self.get_membership_values();

        let mut CSS_numerator: f32 = 0.0;
        let mut CSS_denominator: f32 = 0.0;

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let fuzzy_output_membership_value = f32::min(
                        membership_values[0][i],
                        f32::min(
                            membership_values[1][j],
                            membership_values[2][k]
                        )
                    );
                    CSS_numerator += fuzzy_output_membership_value
                    * Constants::CSU::CSS_SINGLETON_VALUES[Constants::CSU::CSS_FUZZY_RULES[k][i][j]];
                    CSS_denominator += fuzzy_output_membership_value;
                }
            }
        }

        Ok(CSS_numerator/CSS_denominator)
    }

    pub fn CSS_lookup(&mut self, status_params: Vec<StatusMetricParameter>) -> Result<f32, String> {
        // filter parameters
        for (i, param) in status_params.iter().enumerate() {
            for expected_type in Constants::CSU::CSS_EXPECTED_PARAMS {
                if param.param_type == expected_type {
                    if param.val >= Constants::CSU::CSS_PARAM_TEMPLATES[i].min
                    && param.val <= Constants::CSU::CSS_PARAM_TEMPLATES[i].max {
                        self.parameters[i] = Some(param.clone());
                    } else {
                        println!("Value of parameter \"{}\" is out of the bounds \
                            defined by the Fog Network. Omitting...", param.param_type);
                    }
                    continue;
                }
            }
        }

        for param in &self.parameters {
            if param.is_none() {
                return Err(String::from("All required parameters were not supplied."));
            }
        }

        Ok(Constants::CSU::CSS_LOOK_UP
           [{
               let val = self.parameters[0].as_ref().unwrap().val;
               if val < 150.0 {0}
               else if val < 250.0 {1}
               else if val < 350.0 {2}
               else if val < 450.0 {3}
               else if val < 550.0 {4}
               else {6}
           }]
           [{
               let val = self.parameters[1].as_ref().unwrap().val;
               if val < 1.5 {0}
               else if val < 2.5 {1}
               else if val < 3.5 {2}
               else if val < 4.5 {3}
               else if val < 5.5 {4}
               else if val < 6.5 {5}
               else if val < 7.5 {6}
               else if val < 8.5 {7}
               else if val < 9.5 {8}
               else {9}
           }]
           [{
               let val = self.parameters[2].as_ref().unwrap().val;
               if val < 15.0 {0}
               else if val < 25.0 {1}
               else if val < 35.0 {2}
               else if val < 45.0 {3}
               else if val < 55.0 {4}
               else if val < 65.0 {5}
               else {6}
           }]
       )
    }
}
