

use std::io::Read;
use std::fs::File;
use bevy::utils::HashMap;
use bevy::prelude::*;

use serde::Deserialize;

use serde::Serialize;

#[derive(  Resource,   Clone, Default )]
pub struct MaterialDefinitionsMap {
    
    pub material_definitions: HashMap<String, MaterialDefinition>,

    
}




#[derive(  Resource, Deserialize, Serialize, Clone, Default )]
pub struct MaterialDefinitionsLoadResource {
	pub material_defs_folder_path: String, 
}



#[derive(Deserialize, Serialize, Clone, Default )]

pub enum MaterialAlphaMode {
    #[default] 
    Opaque,
    AlphaBlend,
    AlphaMask(f32),

}

impl MaterialAlphaMode {
    pub fn to_alpha_mode(&self) -> AlphaMode {

        match self {

            Self::Opaque => AlphaMode::Opaque,
            Self::AlphaBlend => AlphaMode::Blend,
            Self::AlphaMask(x) => AlphaMode::Mask(*x)
        }

    }
}


#[derive(  Deserialize, Serialize, Clone)]
pub struct MaterialDefinition {
    
   
   pub material_name: String , 
   pub uv_scale_factor: f32, 
   pub diffuse_color_tint: Option<LinearRgba>, 

   #[serde(default)]
   pub alpha_mode: MaterialAlphaMode,



   //pub shader_type: Option<MaterialShaderType>
    
   

  pub  diffuse_texture: Option<String>,
  pub normal_texture: Option<String>,

  pub roughness: f32,

  pub  emissive_texture: Option<String> ,


}
 
impl MaterialDefinition {

      pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

}

pub fn load_material_definitions(
    mut material_definition_map: ResMut<MaterialDefinitionsMap>,
    material_load_res: Res<MaterialDefinitionsLoadResource>,
) {
    let folder_load_path = &material_load_res.material_defs_folder_path;

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
                            match MaterialDefinition::load_from_file(&file_path) {
                                Ok(mat_def) => {
                                    material_definition_map
                                        .material_definitions
                                        .insert(mat_def.material_name.clone(), mat_def);
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

/*

#[derive(  Default, Deserialize, Serialize, Clone)]
pub enum MaterialShaderType {
    #[default]
    StandardMaterial,

    FoliageMaterial 


}*/