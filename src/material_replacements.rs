
 
use crate::material_overrides::MaterialOverridesSet;
use crate::material_name_map::MaterialMetadataName;
use crate::BuiltMaterialsResource;
use crate::material_overrides::{MaterialOverrideComponent, RefreshMaterialOverride};
use crate::materials_config::MaterialShaderType;
use crate::{advanced_materials::foliage_material::FoliageMaterialExtension, materials_config::MaterialTypesConfig};
use bevy::math::Affine2;
use bevy::prelude::*;
use bevy::utils::HashMap;

//use crate::loading::EditorLoadingState;  
use bevy::scene::SceneInstanceReady; 

use serde:: {Serialize,Deserialize};


/*

The materials MUST finish extraction before loading in the models 

*/
pub fn material_replacements_plugin(app: &mut App) {
    app 	

    	
    	.init_state::<MaterialOverridesLoadingState>()
    	 
     


       .add_systems(Update, (
       	handle_material_replacement_sets ,
       	handle_material_replacements_when_scene_ready,
       	handle_material_replacements
       	).chain().before( MaterialOverridesSet ) )

   

       ;
}



#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum MaterialOverridesLoadingState{
	#[default]
   Init,
   Extracting,
   Building,
   Complete
}


 





//attach this to signal that the materials are supposed to be replaced 
#[derive(Component,Debug)]
pub struct MaterialReplacementComponent {

 		//old gltf material name,   new registered material name 
	pub material_replacements: HashMap<String,String>
}

#[derive(Component,Debug)]
pub struct RefreshMaterialReplacement ;


#[derive(Component,Debug)]
pub struct MaterialReplacementWhenSceneReadyComponent {
 
	pub material_replacements: HashMap<String,String>
}

#[derive(Component,Debug)]
pub struct MaterialReplacementApplySetWhenSceneReadyComponent(pub String);

/*
#[derive(Component,Debug)]
pub struct ReadyForMaterialOverride ;
*/



//this should just be inserting material overrides to the children ... 

fn handle_material_replacements(
	mut commands:Commands, 
//	mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,  

	material_override_query: Query<(Entity, &MaterialReplacementComponent), 
	  Changed<MaterialReplacementComponent> /*, Added<RefreshMaterialOverride>*/  >,

	//parent_query : Query<&Parent>, 
	// name_query: Query<&Name>,
	children_query: Query<&Children>,

	  


	 material_metadata_query: Query<&MaterialMetadataName>,

 
){


 

          for (mat_override_entity, mat_replacement_request) in  material_override_query.iter(){

                	  

	             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {



	             		 	 	let Some(material_metadata_comp) = material_metadata_query.get(child).ok() else {continue};


  								for (original_mat_name, new_mat_name) in &mat_replacement_request.material_replacements {


  									if &material_metadata_comp.0 ==  original_mat_name {

  										commands.entity(child).try_insert( MaterialOverrideComponent  {material_override: new_mat_name.clone() }   );

  									}


  								}
								  


             		  }  


          }
           

     // }

}



fn handle_material_replacement_sets(


	mut commands:Commands, 
	material_override_request_query: Query< (Entity, &MaterialReplacementApplySetWhenSceneReadyComponent ), Added<MaterialReplacementApplySetWhenSceneReadyComponent>  >,

	material_types_config: Res<MaterialTypesConfig> ,

) {


	for (entity, mat_replacement_request) in material_override_request_query.iter() {




		let mut material_replacements = None ;
		if let Some(  material_replacement_sets  ) = &material_types_config.material_replacement_sets {

			if let Some(matching_set) = material_replacement_sets.get( &mat_replacement_request.0  ){

				material_replacements = Some( matching_set.clone() );

			}

		} 


		if let Some( material_replacements )=  material_replacements {
	 
			commands.entity(entity).try_insert( 
				MaterialReplacementWhenSceneReadyComponent {
					material_replacements
				}
			 );

		}

	}

}

fn handle_material_replacements_when_scene_ready(
	mut commands:Commands, 
	mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,  

	material_override_request_query: Query<&MaterialReplacementWhenSceneReadyComponent >,

	parent_query : Query<&Parent>, 
	// name_query: Query<&Name>,
	//children_query: Query<&Children>,

	 
 
	 
){ 

    for evt in scene_instance_evt_reader.read(){

          let parent = evt.parent; //the scene 

          let Some(parent_entity) = parent_query.get(parent).ok().map( |p| p.get() ) else {continue};

          if let Some(mat_override_request) = material_override_request_query.get(parent_entity).ok(){
    
             	let material_replacements = mat_override_request.material_replacements.clone() ;

 				if let Some(mut cmd) = commands.get_entity( parent_entity ) {

 					cmd.try_insert(  
 						MaterialReplacementComponent {
 							material_replacements 
 						}
 					);
 				}



          }
           

      }

}