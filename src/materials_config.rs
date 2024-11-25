

use std::io::Read;
use std::fs::File;
use bevy::utils::HashMap;
use bevy::prelude::*;

use serde::Deserialize;

use serde::Serialize;

#[derive(  Resource, Deserialize, Serialize, Clone)]
pub struct MaterialTypesConfig {
    
    pub material_types: HashMap<String, MaterialTypeConfig>,

    pub material_replacement_sets: Option< HashMap < String,  HashMap<  String, String   > >  >
    
   
}



#[derive(  Deserialize, Serialize, Clone)]
pub struct MaterialTypeConfig {
    
   //pub name: String,
 
   pub material_name: String , 
   pub uv_scale_factor: f32, 
   pub diffuse_color_tint: Option<LinearRgba>, 


   pub shader_type: Option<MaterialShaderType>
    
   
}

/*
impl Default for TileTypeConfig {


    fn default() -> Self { 

        Self {
            name: "UnknownTileType".to_string(),
            diffuse_texture_index: 0,
            diffuse_uv_expansion_factor: 1.0,
            diffuse_color_tint: None,
        }

     }
}*/

impl MaterialTypesConfig {

      pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

}



#[derive(  Default, Deserialize, Serialize, Clone)]
pub enum MaterialShaderType {
    #[default]
    StandardMaterial,

    FoliageMaterial 


}