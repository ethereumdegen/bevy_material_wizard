
use crate::material_replacements_map::MaterialReplacementsMap;
use crate::registered_materials::RegisteredMaterialsMap;
  
 
use bevy::prelude::*;
use bevy::utils::HashMap;
// use built_materials::BuiltMaterialsMap;
//use material_overrides::BuiltMaterialsResource;
 
//pub mod built_materials;
pub mod registered_materials;

//pub mod material_definition;
pub mod material_overrides; 

pub mod material_replacements_map; 
pub mod material_replacements;

pub mod gltf_models;
mod util;



pub struct BevyMaterialWizardPlugin {     
    pub material_defs_manifest_path: String,
    pub material_replacements_folder_path : String ,
}
 
impl Plugin for BevyMaterialWizardPlugin {
    fn build(&self, app: &mut App) {

         //let material_defs_manifest_path = &self.material_defs_manifest_path;
     //   let material_replacements_folder_path = &self.material_replacements_folder_path;





         app

         .insert_resource( BevyMaterialWizardConfigResource {

            material_defs_manifest_path: self.material_defs_manifest_path.clone(),
            material_replacements_folder_path: self.material_replacements_folder_path.clone() 


         })


        //.insert_resource(  RegisteredMaterialsMap ::from_manifest_path( material_defs_manifest_path, asset_server ).unwrap() )
        /* .insert_resource(
            MaterialReplacementsLoadResource {
                material_replacements_folder_path: material_replacements_folder_path.clone(),

            }  )*/

         .init_resource::<  RegisteredMaterialsMap  >()
          .init_resource::<  MaterialReplacementsMap  >()
 
        // .add_systems(Startup, (  load_replacement_definitions).chain()) 

         .add_plugins(material_overrides::material_overrides_plugin)
         .add_plugins(material_replacements::material_replacements_plugin)
          .add_plugins(gltf_models::gltf_models_plugin) // make this optionally separate ? 
 
         ;

    }
} 

#[derive(Resource) ]
pub struct BevyMaterialWizardConfigResource {

    pub material_defs_manifest_path: String,
    pub material_replacements_folder_path : String ,

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

 

