use std::collections::HashMap;

// Structure of property of FGN
struct Property {
    id: String,
    value: f32,
}

// Structure of FGN
pub struct FGN {
    id: String,
    properties: HashMap<String, f32>,
}

impl FGN {
    // Add new property to FGN
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
}
