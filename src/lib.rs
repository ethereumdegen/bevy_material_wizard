
use crate::material_replacements_map::MaterialReplacementsLoadResource;
use crate::material_replacements_map::load_replacement_definitions;
use crate::built_materials::MaterialImageHandlesCache;
use crate::built_materials::update_image_sampler_settings;
use crate::material_definition::load_material_definitions;
use crate::material_definition::MaterialDefinitionsLoadResource;
use crate::material_definition::MaterialDefinitionsMap;
use bevy::prelude::*;
use bevy::utils::HashMap;
use built_materials::BuiltMaterialsMap;
//use material_overrides::BuiltMaterialsResource;
 
pub mod built_materials;
pub mod registered_materials;

pub mod material_definition;
pub mod material_overrides; 

pub mod material_replacements_map; 
pub mod material_replacements;

pub mod gltf_models;
mod util;



pub struct BevyMaterialWizardPlugin {     
    pub material_defs_folder_path: String,
    pub material_replacements_folder_path : String ,
}
 
impl Plugin for BevyMaterialWizardPlugin {
    fn build(&self, app: &mut App) {

        let material_defs_folder_path = &self.material_defs_folder_path;
        let material_replacements_folder_path = &self.material_replacements_folder_path;

         app

         


        // .init_state::<MaterialOverridesLoadingState>()
         
         .insert_resource( 
            MaterialImageHandlesCache::default()
            )
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
         .insert_resource(
            MaterialReplacementsLoadResource {
                material_replacements_folder_path: material_replacements_folder_path.clone(),

            }  )

         .add_systems(Startup, load_material_definitions)
         .add_systems(Startup, load_replacement_definitions)
         .add_systems(Update, update_image_sampler_settings)

         .add_plugins(material_overrides::material_overrides_plugin)
         .add_plugins(material_replacements::material_replacements_plugin)
          .add_plugins(gltf_models::gltf_models_plugin) // make this optionally separate ? 
 
         ;

    }
} 


/*
#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum MaterialOverridesLoadingState{
    #[default]
   Init,
   Extracting,
   Building,
   Complete
}
*/

 

