use crate::materials_config::MaterialTypesConfig;
use bevy::prelude::*;
use bevy::utils::HashMap;
use material_overrides::BuiltMaterialsResource;

use crate::material_overrides::MaterialOverridesResource;

pub mod material_overrides;
pub mod material_replacements; 

pub mod materials_config;
pub mod material_name_map;

pub mod advanced_materials; 

pub mod gltf_models; 

pub struct BevyMaterialToolPlugin {
    pub material_overrides_gltf_path: String ,
    pub material_types_config_path: String,
}
 
impl Plugin for BevyMaterialToolPlugin {
    fn build(&self, app: &mut App) {


    	let gltf_path = &self.material_overrides_gltf_path;
        let types_config_path = &self.material_types_config_path;

    	app 
        .init_resource::<BuiltMaterialsResource>()
    	 .insert_resource(
    	 	MaterialOverridesResource{

    	 		doodad_materials_gltf_path: gltf_path.to_string(),
                material_types_config_path: types_config_path.to_string(),
    	 		doodad_materials_gltf: None,
    	 		extracted_materials_map: HashMap::new(),
                 

    	 	} )

          .insert_resource(
            MaterialTypesConfig::load_from_file(
                types_config_path
                ).expect("unable to load material types config")
         )


    	 .add_plugins(material_overrides::material_overrides_plugin)
         .add_plugins(material_replacements::material_replacements_plugin)
         .add_plugins(advanced_materials::advanced_materials_plugin)
         .add_plugins(material_name_map::material_name_map_plugin)
         .add_plugins(gltf_models::gltf_models_plugin)
 
    	;  


	}
} 