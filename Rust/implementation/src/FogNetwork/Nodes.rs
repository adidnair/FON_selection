use std::collections::HashMap;

use crate::User::{ApplicationParameter, ApplicationParameterType};

// Structure of property of FGN
pub struct FGNProperty {
    id: String,
    value: f32,
}

pub struct ApplicationParameterTemplate {
    min: f32,
    max: f32,
}

impl ApplicationParameterTemplate {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

pub struct ERU {
    parameters: Vec<ApplicationParameter>,
    // fuzzified_parameters = 
}

// Structure of FGN
pub struct FGN {
    id: String,
    properties: HashMap<String, f32>,
    app_param_templates: HashMap<ApplicationParameterType,
                                 ApplicationParameterTemplate>,
    // Expectation Rating Unit
    ERU: ERU,
}

impl FGN {
    pub fn new(
        id: String, app_param_templates: HashMap<ApplicationParameterType,
                                                 ApplicationParameterTemplate>
    ) -> Self {
        Self {
            id,
            properties: HashMap::new(),
            app_param_templates,
            ERU: ERU { parameters: Vec::new() },
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

    pub fn get_request(&mut self, params: Vec<ApplicationParameter>) -> Result<(), String> {
        // filter parameters
        self.ERU.parameters = params;
        println!("Param values before filtering: ");
        dbg!(&self.ERU.parameters);
        self.ERU.parameters.retain(|param| {
            let exists = self.app_param_templates.contains_key(&param.param_type);
            if !exists {
                println!("Parameter \"{}\" is not recognized by the \
                    Fog Network. Omitting...", param.param_type);
            }
            let template = self.app_param_templates.get(&param.param_type).unwrap();
            let in_bounds = param.val >= template.min && param.val <= template.max;
            if !in_bounds {
                println!("Value of parameter \"{}\" is out of the bounds \
                    defined by the Fog Network. Omitting...", param.param_type);
            }
            exists && in_bounds
        });

        println!("Param values before normalization: ");
        dbg!(&self.ERU.parameters);
        // normalize parameters
        for param in self.ERU.parameters.iter_mut() {
            let template = self.app_param_templates.get(&param.param_type).unwrap();
            param.val = 2.0f32 * (param.val - template.min)
                / (template.max - template.min);
        }
        println!("Param values after normalization: ");
        dbg!(&self.ERU.parameters);

        // fuzzification
        // TODO: implement membership function


        // fuzzy inference
        // defuzzification
        Ok(())
    }
}
