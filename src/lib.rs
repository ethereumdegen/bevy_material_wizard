
use crate::material_replacements_map::MaterialReplacementsMap;
use crate::registered_materials::RegisteredMaterialsMap;
  
 
use bevy::prelude::*;
use bevy::platform::collections::hash_map::HashMap;
 
pub mod registered_materials;
 
pub mod material_overrides; 

pub mod material_replacements_map; 
pub mod material_replacements;

pub mod gltf_models;
 



pub struct BevyMaterialWizardPlugin {     
    pub material_defs_folder_prefix: Option<String>, 
    pub material_defs_manifest_path: String,
    pub material_replacements_folder_path : String ,
}
 
impl Plugin for BevyMaterialWizardPlugin {
    fn build(&self, app: &mut App) {

         



         app

         .insert_resource( BevyMaterialWizardConfigResource {

            material_defs_manifest_path: self.material_defs_manifest_path.clone(),
            material_replacements_folder_path: self.material_replacements_folder_path.clone(), 

            material_defs_folder_prefix: self.material_defs_folder_prefix.clone().unwrap_or("".into())

         })

        .add_systems(Startup ,  init_resources ) 
       //  .init_resource::<  RegisteredMaterialsMap  >()
       //   .init_resource::<  MaterialReplacementsMap  >()
 
        // .add_systems(Startup, (  load_replacement_definitions).chain()) 

         .add_plugins(material_overrides::material_overrides_plugin)
         .add_plugins(material_replacements::material_replacements_plugin)
          .add_plugins(gltf_models::gltf_models_plugin) // make this optionally separate ? 
 
         ;

    }
} 

#[derive(Resource,Clone,Debug) ]
pub struct BevyMaterialWizardConfigResource {

    pub material_defs_manifest_path: String,
    pub material_replacements_folder_path : String ,

    pub material_defs_folder_prefix: String , 

}

fn init_resources(world: &mut World) {
    let registered_materials = RegisteredMaterialsMap::from_world(world);
    world.insert_resource(registered_materials);
    
    let material_replacements = MaterialReplacementsMap::from_world(world);
    world.insert_resource(material_replacements);
}