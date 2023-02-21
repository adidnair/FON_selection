#![allow(dead_code, non_snake_case)]
use std::collections::HashMap;

struct Property {
    id: String,
    value: f32,
}

struct FGN {
    id: String,
    properties: HashMap<String, f32>,
}

impl FGN {
    pub fn add_property(
        &mut self, new_property: Property
    ) -> Result<() , String> {
        if self.properties.contains_key(&new_property.id) {
            Err(format!("Property with id \"{}\" already exists.", new_property.id))
        } else {
            Ok(())
        }
    }

    pub fn get_property_value(&self, property_id: String) -> Result<f32, String> {
        if let Some(x) = self.properties.get(&property_id) {
            Ok((*x).clone())
        } else {
            Err(format!("FGN \"{}\" has no property with id \"{}\"", self.id, property_id))
        }
    }
}

struct Parameter {
    id: String,
    weight: f32,
    to_be_maximized: bool,
}

struct Instance {
    number_of_parameters: usize,
    algo_params: Vec<Parameter>,
    fgns: Vec<FGN>,
}

impl Instance {
    pub fn new_empty(algo_params: Vec<Parameter>) -> Instance {
        Instance {
            number_of_parameters: algo_params.len(),
            algo_params,
            fgns: Vec::new(),
        }
    }

    pub fn new(algo_params: Vec<Parameter>, nodes: Vec<FGN>) -> Instance {
        Instance {
            number_of_parameters: algo_params.len(),
            algo_params,
            fgns: nodes,
        }
    }

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
                    positive_ideal_solution[j] = positive_ideal_solution[j].max(V[i][j]);
                    negative_ideal_solution[j] = negative_ideal_solution[j].min(V[i][j]);
                } else {
                    positive_ideal_solution[j] = positive_ideal_solution[j].min(V[i][j]);
                    negative_ideal_solution[j] = negative_ideal_solution[j].max(V[i][j]);
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

fn main() {
}
