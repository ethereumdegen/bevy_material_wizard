
use crate::material_definition::load_material_definitions;
use crate::material_definition::MaterialDefinitionsLoadResource;
use crate::material_definition::MaterialDefinitionsMap;
use bevy::prelude::*;
use bevy::utils::HashMap;
use built_materials::BuiltMaterialsMap;
//use material_overrides::BuiltMaterialsResource;
 

pub mod material_definition;
pub mod built_materials;
pub mod material_overrides;
// pub mod material_replacements;
pub mod gltf_models;



pub struct BevyMaterialWizardPlugin {     
    pub material_defs_folder_path: String,
}
 
impl Plugin for BevyMaterialWizardPlugin {
    fn build(&self, app: &mut App) {

        let material_defs_folder_path = &self.material_defs_folder_path;

         app
         .insert_resource(
            MaterialDefinitionsMap::default()
            )
         .insert_resource(
            BuiltMaterialsMap::default()
            )

         .insert_resource(
            MaterialDefinitionsLoadResource {
                material_defs_folder_path: material_defs_folder_path.clone(),

            }  )

         .add_systems(Startup, load_material_definitions)

         .add_plugins(material_overrides::material_overrides_plugin)
          .add_plugins(gltf_models::gltf_models_plugin)
 
         ;

    }
} 