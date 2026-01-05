
 
use crate::material_overrides::MaterialOverridesSet;


use bevy::gltf::GltfMaterialName; 


use crate::material_replacements_map::MaterialReplacementsMap ; 
 
 
use crate::material_overrides::{MaterialOverrideComponent };
 
use bevy::prelude::*;
use bevy::platform::collections::hash_map::HashMap;
 
use bevy::scene::SceneInstanceReady; 
 
use bevy::ecs::relationship::DescendantIter;

/*

The materials MUST finish extraction before loading in the models 

*/
pub fn material_replacements_plugin(app: &mut App) {
    app 	

    		  .register_type::<MaterialReplacementSet>()   
    		.register_type::<MaterialReplacementComponent>()


    		// .register_type::<MaterialReplacementWhenSceneReadyComponent>()
    	  //  .register_type::<MaterialReplacementApplySetWhenSceneReadyComponent>()

    	  

     
    	.add_observer( handle_material_replacements_when_scene_ready )

    	.init_resource::< MaterialReplacementsMap >()


       .add_systems(Update, (
       		handle_material_replacement_sets ,
       
       		handle_material_replacements
       	).chain().before( MaterialOverridesSet ) )

   

       ;
}






//attach this to signal that the materials are supposed to be replaced 
#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialReplacementComponent {

 		//old gltf material name,   new registered material name 
	pub material_replacements: HashMap<String,String>
}

#[derive(Component,Debug)]
pub struct RefreshMaterialReplacement ;

/*
#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialReplacementWhenSceneReadyComponent {
 
	pub material_replacements: HashMap<String,String>
}*/

#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialReplacementSet(pub String);

/*
#[derive(Component,Debug)]
pub struct ReadyForMaterialOverride ;
*/



//this should just be inserting material overrides to the children ... 

fn handle_material_replacements(
	mut commands:Commands,  

	material_override_query: Query<(Entity, &MaterialReplacementComponent), 
	  Changed<MaterialReplacementComponent> /*, Added<RefreshMaterialOverride>*/  >,
 
	children_query: Query<&Children>,

	  


	 material_name_query: Query<&GltfMaterialName>,

 
){


 

          for (mat_override_entity, mat_replacement_request) in  material_override_query.iter(){

                	  

	             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {



	             		 	 	let Some(material_metadata_comp) = material_name_query.get(child).ok() else {continue};


  								for (original_mat_name, new_mat_name) in &mat_replacement_request.material_replacements {


  									if &material_metadata_comp.0 ==  original_mat_name {

  										commands.entity(child).try_insert( MaterialOverrideComponent  {
  											material_override: new_mat_name.clone() ,
  											cascade : false  

  										}   );

  									}


  								}
								  


             		  }  


          }
           

     // }

}



fn handle_material_replacement_sets(
 
	mut commands:Commands, 
	material_override_request_query: Query< (Entity, &MaterialReplacementSet ), Or<( Added<MaterialReplacementSet>, Changed<MaterialReplacementSet> )>  >,

	material_replacements_config: Res<MaterialReplacementsMap> ,

) {


	for (entity, mat_replacement_request) in material_override_request_query.iter() {

 


		let material_replacement_sets = &material_replacements_config.material_replacement_sets ; 


        let material_replacement_set =  material_replacement_sets.get( &mat_replacement_request.0  ) ;

	


		if let Some( material_replacements ) =  material_replacement_set {
	 	

	 	// info!(" handle_material_replacement_sets 2 " );

			commands.entity(entity).try_insert( 
				//MaterialReplacementComponent {
				//	 material_replacements: material_replacements.clone()
				//},
				MaterialReplacementComponent {
						 material_replacements: material_replacements.clone()
				}
			 ) ;

		}else {
			panic!("could not find mat rep {}",  &mat_replacement_request.0);
		}

	}

}


 //this will force handle_material_replacements  to occur ! 
fn handle_material_replacements_when_scene_ready(  
    scene_instance_evt_trigger: Trigger<SceneInstanceReady>,

    mut material_replacement_comp_query: Query<&mut MaterialReplacementComponent>,
 
    parent_query: Query<&ChildOf>,
) {

		let trig_entity = scene_instance_evt_trigger.target();

	    let Some(parent_entity) = parent_query.get(trig_entity).ok().map(|p| p.parent()) else {
	        return;
	    };

 	
	 	 let Some(mut mat_replacement_comp) = material_replacement_comp_query.get_mut(parent_entity).ok() else {
	        return;
	    }; 


	    mat_replacement_comp.set_changed(); 

 


          
 

}