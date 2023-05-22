use crate::FogNetwork::Nodes::{FGN, ApplicationRequest};
use crate::Constants;

pub struct Application {
    id: String,
    parameters: Vec<ExpectationMetricParameter>,
}

#[derive(Clone, Debug)]
pub struct ExpectationMetricParameter {
    pub param_type: Constants::Application::ExpectationMetricParameterType,
    pub val: f32,
}

impl ExpectationMetricParameter {
    pub fn new(param_type: Constants::Application::ExpectationMetricParameterType, val: f32) -> Self {
        Self { val, param_type }
    }
}

// TODO: Build better send/receive request methods so as to not borrow
//       FGNS as mutable
impl Application {
    pub fn new(id: String, parameters: Vec<ExpectationMetricParameter>) -> Self {
        Self { id, parameters, }
    }

     pub fn send_placement_request(&self, fgn: &mut FGN) -> Result<(), String> {
        fgn.get_application_request(ApplicationRequest {
            id: self.id.clone(), params: self.parameters.clone()
        })
    }

    pub fn send_placement_requests(&self, fgns: Vec<&mut FGN>) -> Result<(), String> {
        for fgn in fgns {
            match self.send_placement_request(fgn) {
                Err(err)    => {return Err(err)},
                Ok(_)       => {}
            }
        }
        Ok(())
    }
}
