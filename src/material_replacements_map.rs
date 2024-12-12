

use std::io::Read;
use std::fs::File;
use bevy::utils::HashMap;
use bevy::prelude::*;

use serde::Deserialize;

use serde::Serialize;


/*

Quite similar to material_definition.rs 


*/



#[derive(  Resource, Deserialize, Serialize, Clone  )]
pub struct MaterialReplacementsLoadResource {
	pub material_replacements_folder_path: String, 
}



#[derive(  Resource,   Clone, Default )]
pub struct MaterialReplacementsMap {
    
    pub material_replacement_sets:  HashMap < String,  HashMap<  String, String   > >   
    
}

impl MaterialReplacementsMap {

      pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

}

