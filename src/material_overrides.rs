 
 
use bevy_materialize::GenericMaterial3d;
use crate::RegisteredMaterialsMap;
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
      	 	.register_type::<MaterialOverrideWhenSceneReadyComponent>()
      	
      	.add_event::<MaterialOverrideCompleted>()
      
      

       .add_systems(Update, (
      
       	handle_material_overrides
       	).chain() .in_set(MaterialOverridesSet) )

   
        .add_observer(handle_material_overrides_when_scene_ready)
       ;
}


//attach this to signal that the material is supposed to be replaced 
#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialOverrideComponent {
 
	pub material_override: String,


	pub cascade: bool // to all children 
}

/*
#[derive(Component,Debug)]
pub struct RefreshMaterialOverride ;  //change me into a command !? 
*/


// not impl yet 
//#[derive(Event)]
//pub struct PerformMaterialOverride(String); // used as a trigger 


#[derive(Event)]
pub struct MaterialOverrideCompleted(pub String); // used as a trigger 


#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialOverrideWhenSceneReadyComponent {
 
	pub material_override: String,
	pub cascade: bool 
}



#[derive(SystemSet,Hash,Clone,Debug,Eq,PartialEq)]
pub struct MaterialOverridesSet;

 


// turn this into a command !? 

fn handle_material_overrides(
	mut commands:Commands, 
 
	material_override_query: Query<(Entity, &MaterialOverrideComponent), 
	  Changed<MaterialOverrideComponent>    >,

	 
	children_query: Query<&Children>,

	 mesh_query: Query< &Mesh3d >,

 	
 	//material_definitions_res: Res<MaterialDefinitionsMap>,
 	mut asset_server: ResMut<AssetServer>, 
 	//mut material_images_cache: ResMut< MaterialImageHandlesCache>,
 	mut material_assets: ResMut<Assets<StandardMaterial>>,


	  built_materials_resource: Res  <RegisteredMaterialsMap> ,
 
    //image_assets: Res<Assets<Image>>,
){


 
 
          for (mat_override_entity, mat_override_request) in  material_override_query.iter(){

                	 

            
             	let material_name = &mat_override_request.material_override ;

 		 

             	     let loaded_material = built_materials_resource.find_material  (

             	     	&material_name,
             	     	 

             	     );

             		   

             		  if let Some(new_material_handle) = loaded_material {
 

             		  		if   mesh_query.get(mat_override_entity).ok().is_some() {
	             		 	 		  
	             		 	 		  commands.entity(mat_override_entity).remove::<MeshMaterial3d<StandardMaterial>>() ;
					                  commands.entity(mat_override_entity).try_insert( GenericMaterial3d( new_material_handle.clone() )) ;
					                  	//  info!("inserted new material as override");

					                   commands.trigger_targets(MaterialOverrideCompleted(material_name.clone()), mat_override_entity.clone());
                				  
	             		 	 	} 
 	
	             		 	 if mat_override_request.cascade {
	             		 	    for child in DescendantIter::new(&children_query, mat_override_entity) {

	             		 	  
	 								if   mesh_query.get(child).ok().is_some() {

	             		 	 			  commands.entity(mat_override_entity).remove::<MeshMaterial3d<StandardMaterial>>() ;
	             		 	 		      commands.entity(child).try_insert( GenericMaterial3d( new_material_handle.clone() ) );
					                  	
					                  	 commands.trigger_targets(MaterialOverrideCompleted(material_name.clone()), child.clone());
 									//  info!("inserted new material as override");

	             		 	 		} 
							     
								 }
								 }

             		  }else {

             		  	  let warning_material = material_assets.add(Color::srgb(1.0, 0.0, 0.0)) ;
 
				             warn!("inserted warning_material");
				          
				          

					          if   mesh_query.get(mat_override_entity).ok().is_some() {
	             		 	 		 commands
					                    .entity(mat_override_entity)
					                    .try_insert( MeshMaterial3d ( warning_material.clone() )) ; 

					                 
	             		 	 	} 
 

             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {

             		 	 	if   mesh_query.get(child).ok().is_some() {
 

             		 	 		 commands
				                    .entity(child)
				                    .try_insert(MeshMaterial3d ( warning_material.clone() )) ; 

				                 

             		 	 	} 						     
						 }


             		  }

             	 


          }
           
 

}





fn handle_material_overrides_when_scene_ready(
    scene_instance_evt_trigger: Trigger<SceneInstanceReady>,

    material_override_request_query: Query<&MaterialOverrideWhenSceneReadyComponent>,

    mut commands: Commands,

    parent_query: Query<&Parent>,
) {
    let trig_entity = scene_instance_evt_trigger.entity();

    let Some(parent_entity) = parent_query.get(trig_entity).ok().map(|p| p.get()) else {
        return;
    };

    // need to check parent !?
    let Some(mat_override_request) = material_override_request_query.get(parent_entity).ok() else {
        return;
    };

    let material_override = mat_override_request.material_override.clone();
     let cascade = mat_override_request.cascade.clone();

    if let Some(mut cmd) = commands.get_entity(trig_entity) {
        cmd.try_insert(MaterialOverrideComponent { material_override , cascade });
    }
}
 