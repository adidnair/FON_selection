use crate::FogNetwork::Nodes::FGN;
use crate::Constants;
use std::fmt;

pub struct Application {
    id: String,
    parameters: Vec<ApplicationParameter>,
}

#[derive(Clone, Debug)]
pub struct ApplicationParameter {
    pub param_type: Constants::Application::ParameterType,
    pub val: f32,
}

impl ApplicationParameter {
    pub fn new(param_type: Constants::Application::ParameterType, val: f32) -> Self {
        Self { val, param_type }
    }

    // pub fn fuzzy_set_index(&self) -> Result<usize, String> {
    //     for i in 0..self.fuzzy_sets.0.len() {
    //         if self.val > self.fuzzy_sets.1[i]
    //         && self.val < self.fuzzy_sets.1[i+1] {
    //             return Ok(i);
    //         }
    //     }
    //     Err(String::from("Parameter value is out of bounds \
    //         even after normalization."))
    // }
}

impl fmt::Display for Constants::Application::ParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccessRate => {write!(f, "AccessRate")},
            Self::RequiredResources => {write!(f, "RequiredResources")},
            Self::ProcessingTime => {write!(f, "ProcessingTime")},
        }
    }
}

// TODO: Build better send/receive request methods so as to not borrow
//       FGNS as mutable
impl Application {
    pub fn new(id: String, parameters: Vec<ApplicationParameter>) -> Self {
        Self { id, parameters, }
    }

     pub fn send_request(&self, fgn: &mut FGN) -> Result<(), String> {
        fgn.get_request(self.parameters.clone())
    }

    pub fn send_requests(&self, fgns: Vec<&mut FGN>) -> Result<(), String> {
        for fgn in fgns {
            match self.send_request(fgn) {
                Err(err)    => {return Err(err)},
                Ok(_)       => {}
            }
        }
        Ok(())
    }
}
