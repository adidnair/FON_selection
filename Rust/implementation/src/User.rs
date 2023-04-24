use crate::FogNetwork::Nodes::FGN;
use std::fmt;

pub struct Application {
    id: String,
    parameters: Vec<ApplicationParameter>,
}

#[derive(Clone, Debug)]
pub struct ApplicationParameter {
    pub param_type: ApplicationParameterType,
    pub val: f32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ApplicationParameterType {
    AccessRate,
    RequiredResources,
    ProcessingTime,
}

impl ApplicationParameter {
    pub fn new(param_type: ApplicationParameterType, val: f32) -> Self {
        Self { param_type, val, }
    }

}

impl ApplicationParameterType {
    fn get_fuzzy_set(&self) {
        match self {
            Self::AccessRate => {},
            Self::RequiredResources => {},
            Self::ProcessingTime => {},
        }
    }
}

impl fmt::Display for ApplicationParameterType {
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
