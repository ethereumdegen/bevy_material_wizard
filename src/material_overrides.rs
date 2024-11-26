
 
//use crate::materials_config::MaterialShaderType;
/* use crate::{
	advanced_materials::foliage_material::FoliageMaterialExtension,
	 materials_config::MaterialTypesConfig}; */
use crate::MaterialImageHandlesCache;
use crate::{built_materials::BuiltMaterialsMap, material_definition::MaterialDefinitionsMap};
 
use bevy::prelude::*;
use bevy::utils::HashMap;

//use crate::loading::EditorLoadingState;  
use bevy::scene::SceneInstanceReady; 

use serde:: {Serialize,Deserialize};


/*

The materials MUST finish extraction before loading in the models 

*/
pub fn material_overrides_plugin(app: &mut App) {
    app 	

    	
      	.register_type::<MaterialOverrideComponent>()

      
      

       .add_systems(Update, (
       	handle_material_overrides_when_scene_ready,
       	handle_material_overrides
       	).chain() .in_set(MaterialOverridesSet) )

   

       ;
}


//attach this to signal that the material is supposed to be replaced 
#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialOverrideComponent {
 
	pub material_override: String
}

#[derive(Component,Debug)]
pub struct RefreshMaterialOverride ;


#[derive(Component,Debug)]
pub struct MaterialOverrideWhenSceneReadyComponent {
 
	pub material_override: String
}



#[derive(SystemSet,Hash,Clone,Debug,Eq,PartialEq)]
pub struct MaterialOverridesSet;

 


fn handle_material_overrides(
	mut commands:Commands, 
 
	material_override_query: Query<(Entity, &MaterialOverrideComponent), 
	Or<( Changed<MaterialOverrideComponent> , Added<RefreshMaterialOverride>) > >,

	 
	children_query: Query<&Children>,

	 mesh_query: Query<&Handle<Mesh>>,

 	
 	material_definitions_res: Res<MaterialDefinitionsMap>,
 	mut asset_server: ResMut<AssetServer>, 
 	mut material_images_cache: ResMut< MaterialImageHandlesCache>,
 	mut material_assets: ResMut<Assets<StandardMaterial>>,
	mut built_materials_resource: ResMut <BuiltMaterialsMap> ,
){


 
 
          for (mat_override_entity, mat_override_request) in  material_override_query.iter(){

                	 

            
             	let material_name = &mat_override_request.material_override ;

 				let material_definitions_map = &material_definitions_res.material_definitions;  

 

             	     let loaded_material = built_materials_resource.find_or_load_material  (

             	     	&material_name,
             	     	material_definitions_map,
             	     	&mut material_images_cache, 
             	     	&mut asset_server, 
             	     	&mut material_assets

             	     );

             		   

             		  if let Some(new_material_handle) = loaded_material {
 

             		  		if   mesh_query.get(mat_override_entity).ok().is_some() {
	             		 	 		  
					                  commands.entity(mat_override_entity).try_insert(new_material_handle.clone());
					                  	  info!("inserted new material as override");
                				  
	             		 	 	} 
 

	             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {

	             		 	  
	 								if   mesh_query.get(child).ok().is_some() {

	             		 	 		
	             		 	 		   commands.entity(child).try_insert(new_material_handle.clone());
					                  
 									  info!("inserted new material as override");

	             		 	 		} 
							     
								 } 

             		  }else {

             		  	  let warning_material = material_assets.add(Color::srgb(1.0, 0.0, 0.0)) ;
 
				             info!("inserted warning_material");
				          
				          

					        if   mesh_query.get(mat_override_entity).ok().is_some() {
	             		 	 		 commands
					                    .entity(mat_override_entity)
					                    .try_insert(warning_material.clone()); 

					                  //info!("inserted new material as override"); 
	             		 	 	}else {
	             		 	 		// warn!("no existing material to replace "); 
	             		 	 	}
 

             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {

             		 	 	if   mesh_query.get(child).ok().is_some() {
 

             		 	 		 commands
				                    .entity(child)
				                    .try_insert(warning_material.clone()); 

				                 // info!("inserted new material as override");


             		 	 	}else {
	             		 	 		// warn!("no existing material to replace "); 
	             		 	 	}
						     
						 }


             		  }

             	 


          }
           
 

}



fn handle_material_overrides_when_scene_ready(
	mut commands:Commands, 
	mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,  

	material_override_request_query: Query<&MaterialOverrideWhenSceneReadyComponent >,

	parent_query : Query<&Parent>, 
	// name_query: Query<&Name>,
	children_query: Query<&Children>,

	 

	 


	 
){




    for evt in scene_instance_evt_reader.read(){

          let parent = evt.parent; //the scene 

          let Some(parent_entity) = parent_query.get(parent).ok().map( |p| p.get() ) else {continue};

          if let Some(mat_override_request) = material_override_request_query.get(parent_entity).ok(){

         
             	let material_override = mat_override_request.material_override.clone() ;

 				if let Some(mut cmd) = commands.get_entity( parent_entity ) {

 					cmd.try_insert(  
 						MaterialOverrideComponent {
 							material_override 
 						}
 					);
 				}



          }
           

      }

}