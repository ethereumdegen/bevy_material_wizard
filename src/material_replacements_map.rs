

use std::io::Read;
use std::fs::File;
use bevy::utils::HashMap;
use bevy::prelude::*;

use serde::Deserialize;

use serde::Serialize;


/*

Quite similar to material_definition.rs 

should be loaded from individual asset files ! 


*/



#[derive(  Resource, Deserialize, Serialize, Clone  )]
pub struct MaterialReplacementsLoadResource {
	pub material_replacements_folder_path: String, 
}



#[derive(  Resource,   Clone, Default )]
pub struct MaterialReplacementsMap {
    
    pub material_replacement_sets:  HashMap < ReplacementSetName,  HashMap<  String, String   > >   
    
}


pub type ReplacementSetName = String ;


/*
impl MaterialReplacementsMap {

      pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

}

*/


#[derive(  Deserialize, Serialize, Clone)]
pub struct MaterialReplacementManifest {
    pub replacement_set_name: ReplacementSetName, // the key for the outer hashmap 
    pub material_replacements:  HashMap<  String, String   >   
    

}
 
impl MaterialReplacementManifest {

      pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

}

pub fn load_replacement_definitions(
     material_load_res: Res<MaterialReplacementsLoadResource>,
    mut material_definition_map: ResMut<MaterialReplacementsMap>,
   
) {
    let folder_load_path = &material_load_res.material_replacements_folder_path;

    // Iterate through all the files in the folder
    if let Ok(entries) = std::fs::read_dir(folder_load_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Check if the entry is a file
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        // Check for appropriate file extension (e.g., `.ron`)
                        if extension == "ron" {
                            let file_path = path.to_string_lossy().to_string();

                            // Attempt to load the material definition
                            match MaterialReplacementManifest::load_from_file(&file_path) {
                                Ok(repl_def) => {

                                    material_definition_map
                                        .material_replacement_sets
                                        .insert(repl_def.replacement_set_name.clone(), repl_def.material_replacements.clone());
                                }
                                Err(err) => {
                                    eprintln!("Failed to load material definition from {}: {}", file_path, err);
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory: {}", folder_load_path);
    }
}