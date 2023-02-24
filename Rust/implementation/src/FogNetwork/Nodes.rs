use std::collections::HashMap;

struct Property {
    id: String,
    value: f32,
}

pub struct FGN {
    id: String,
    properties: HashMap<String, f32>,
}

impl FGN {
    pub fn add_property(
        &mut self, new_property: Property
    ) -> Result<() , String> {
        if self.properties.contains_key(&new_property.id) {
            Err(format!("Property with id \"{}\" already exists.",
                new_property.id))
        } else {
            Ok(())
        }
    }

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

    pub fn get_id(&self) -> &String {
        &self.id
    }
}
