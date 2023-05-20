use std::collections::HashMap;
use crate::Constants;
use crate::User::ApplicationParameter;

const NONE_APP_PARAM_WORKAROUND: Option<ApplicationParameter> = None;
const VEC_ARRAY_WORKAROUND: Vec<usize> = Vec::new();

// Structure of property of FGN
pub struct FGNProperty {
    id: String,
    value: f32,
}

pub struct ApplicationParameterTemplate {
    pub min: f32,
    pub max: f32,
}

pub struct ERU {
    parameters: [Option<ApplicationParameter>; Constants::ERU::ROE_EXPECTED_PARAMS.len()],
    ROE_fuzzy_rules: [[[usize; 3]; 3]; 3],
}

// Structure of FGN
pub struct FGN {
    id: String,
    properties: HashMap<String, f32>,
    // Expectation Rating Unit
    ERU: ERU,
}

impl FGN {
    pub fn new(id: String) -> Self {
        Self {
            id,
            properties: HashMap::new(),
            ERU: ERU {
                parameters: [NONE_APP_PARAM_WORKAROUND; Constants::ERU::ROE_EXPECTED_PARAMS.len()],
                ROE_fuzzy_rules: Constants::ERU::ROE_FUZZY_RULES,
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

    pub fn get_request(&mut self, params: Vec<ApplicationParameter>) -> Result<f32, String> {
        // filter parameters
        for (i, param) in params.iter().enumerate() {
            for expected_type in Constants::ERU::ROE_EXPECTED_PARAMS {
                if param.param_type == expected_type {
                    if param.val >= Constants::ERU::ROE_PARAM_TEMPLATES[i].min
                    && param.val <= Constants::ERU::ROE_PARAM_TEMPLATES[i].max {
                        self.ERU.parameters[i] = Some(param.clone());
                    } else {
                        println!("Value of parameter \"{}\" is out of the bounds \
                            defined by the Fog Network. Omitting...", param.param_type);
                    }
                    continue;
                }
                // println!("Parameter of type \"{}\" not required. Omitting...", param.param_type);
            }
        }

        self.ERU.ROE()
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
            Constants::ERU::RequiredResourcesMembershipFunction(self.parameters[3].as_ref().unwrap().val),
        ]
    }


    pub fn ROE(&mut self) -> Result<f32, String> {
        for param in &self.parameters {
            if param.is_none() {
                return Err(String::from("All required parameters were not supplied."));
            }
        }
        println!("Param values before normalization: ");
        dbg!(&self.parameters);
        // normalize parameters
        for (i, param) in self.parameters.iter_mut().enumerate() {
            if let Some(ref mut raw_param) = param {
                raw_param.val = 2.0 * (raw_param.val - Constants::ERU::ROE_PARAM_TEMPLATES[i].min)
                    / (Constants::ERU::ROE_PARAM_TEMPLATES[i].max - Constants::ERU::ROE_PARAM_TEMPLATES[i].min) - 1.0;
            }
        }
        println!("Param values after normalization: ");
        dbg!(&self.parameters);

        // 1. calculate membership function for each value of each parameter
        let membership_values = self.get_membership_values();

        let mut positive_indices = [VEC_ARRAY_WORKAROUND; Constants::ERU::ROE_FUZZY_SETS.len()];

        for (i, arr) in membership_values.iter().enumerate() {
            for (j, &val) in arr.iter().enumerate() {
                if val != 0.0 {
                    positive_indices[i].push(j);
                }
            }
        }
        let positive_indices = positive_indices;

        let mut ROE_numerator: f32 = 0.0;
        let mut ROE_denominator: f32 = 0.0;

        for &i in &positive_indices[0] {
            for &j in &positive_indices[1] {
                for &k in &positive_indices[2] {
                    let fuzzy_output_membership_value = f32::max(
                        membership_values[0][i],
                        f32::max(
                            membership_values[1][j],
                            membership_values[2][k]
                        )
                    );
                    ROE_numerator += fuzzy_output_membership_value
                    * Constants::ERU::ROE_SINGLETON_VALUES[Constants::ERU::ROE_FUZZY_RULES[i][j][k]];
                    ROE_denominator += fuzzy_output_membership_value;
                }
            }
        }

        Ok(ROE_numerator/ROE_denominator)
    }
}
