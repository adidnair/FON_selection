use crate::FogNetwork::Nodes::FGN;

// Instance of Fog Network
pub struct Instance {
    number_of_parameters: usize,
    algo_params: Vec<Parameter>,
    fgns: Vec<FGN>,
}


// Structure of parameter used in algo
#[derive(Debug)]
pub struct Parameter {
    id: String,
    weight: f32,
    to_be_maximized: bool,
}

impl Parameter {
    // Constructor
    pub fn new(
        id: String, weight: f32, to_be_maximized: bool
    ) -> Self {
        Parameter { id,
            weight,
            to_be_maximized
        }
    }
}

impl Instance {
    // Empty Constructor
    pub fn new_empty(algo_params: Vec<Parameter>) -> Instance {
        Instance {
            number_of_parameters: algo_params.len(),
            algo_params,
            fgns: Vec::new(),
        }
    }

    // Constructor
    pub fn new(algo_params: Vec<Parameter>, fgns: Vec<FGN>) -> Instance {
        Instance {
            number_of_parameters: algo_params.len(),
            algo_params,
            fgns,
        }
    }

    // Function to add new node to network
    #[allow(non_snake_case)]
    pub fn add_FGN(&mut self, new_fgn: FGN) -> Result<() , String> {
        for fgn in self.fgns.iter() {
            if fgn.get_id() == new_fgn.get_id() {
                {
                    return Err(format!("FGN with id \"{}\" already exists.",
                        new_fgn.get_id()))
                }
            }
        }
        self.fgns.push(new_fgn);
        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn get_FGN(&mut self, fgn_id: String) -> Result<&mut FGN , String> {
        for fgn in self.fgns.iter_mut() {
            if *fgn.get_id() == fgn_id {
                return Ok(fgn);
            }
        }
        Err(format!("FGN with id \"{}\" does not exist.", fgn_id))
    }

    // TOPSIS algo implementation for FON selection
    // Returns reference to FGN selected as FON
    #[allow(non_snake_case)]
    pub fn TOPSIS(&self) -> Result<&FGN, String> {
        let mut V: Vec<Vec<f32>>;
        match self.fgns
            .iter()
            .map(|fgn| {
                self.algo_params
                    .iter()
                    .map(|fgn_prop| {
                        fgn.get_property_value(fgn_prop.id.clone())
                    })
                    .collect()
            })
            .collect() {
                Ok(matrix)  => {V = matrix;},
                Err(err)         => {return Err(err);},
        }
        let m = V.len();
        let k = V[0].len();
        let weight_sum = self.algo_params
            .iter()
            .fold(0f32, |sum, parameter| {
                sum + parameter.weight
            });
        let weights: Vec<f32> = self.algo_params.iter()
            .map(|parameter| {
                parameter.weight / weight_sum
            })
            .collect();
        // Normalize
        let mut positive_ideal_solution: Vec<f32> = self.algo_params.iter()
            .map(|x| {
                if x.to_be_maximized {
                    f32::MIN
                } else {
                    f32::MAX
                }
            })
            .collect();
        let mut negative_ideal_solution: Vec<f32> = self.algo_params.iter()
            .map(|x| {
                if x.to_be_maximized {
                    f32::MAX
                } else {
                    f32::MIN
                }
            })
            .collect();
        for j in 0..k {
            let mut denom = 0f32;
            for i in 0..m {
                denom += V[i][j].powi(2);
            }
            for i in 0..m {
                V[i][j] /= denom.sqrt();
                V[i][j] *= weights[j];
            }
            for i in 0..m {
                if self.algo_params[i].to_be_maximized {
                    positive_ideal_solution[j] = positive_ideal_solution[j]
                        .max(V[i][j]);
                    negative_ideal_solution[j] = negative_ideal_solution[j]
                        .min(V[i][j]);
                } else {
                    positive_ideal_solution[j] = positive_ideal_solution[j]
                        .min(V[i][j]);
                    negative_ideal_solution[j] = negative_ideal_solution[j]
                        .max(V[i][j]);
                }
            }
        }
        let positive_distance: Vec<f32> = V.iter()
            .map(|x| {
                (0..k as usize).into_iter()
                    .fold(0f32, |sum, j| {
                        sum + (x[j] - positive_ideal_solution[j]).powi(2)
                    })
                    .sqrt()
            })
            .collect();
        let negative_distance: Vec<f32> = V.iter()
            .map(|x| {
                (0..k as usize).into_iter()
                    .fold(0f32, |sum, j| {
                        sum + (x[j] - negative_ideal_solution[j]).powi(2)
                    })
                    .sqrt()
            })
            .collect();
        let quality: Vec<f32> = positive_distance.iter()
            .zip(negative_distance.iter())
            .map(|(p, n)| {
                n / (p + n)
            })
            .collect();
        let (FON_index, _FON_quality) = quality.iter()
            .enumerate()
            .fold((0usize, f32::MIN), |(idx_max, qual_max), (idx, qual)| {
                if &qual_max > qual {
                    (idx_max, qual_max)
                } else {
                    (idx, *qual)
                }
            });

        Ok(&self.fgns[FON_index])
    }
}
