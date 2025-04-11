
 
use crate::material_overrides::MaterialOverridesSet;


use bevy::gltf::GltfMaterialName; 


use crate::material_replacements_map::MaterialReplacementsMap ; 
 
 
use crate::material_overrides::{MaterialOverrideComponent };
//use crate::materials_config::MaterialShaderType;
//use crate::{advanced_materials::foliage_material::FoliageMaterialExtension, materials_config::MaterialTypesConfig};
// use bevy::math::Affine2;
use bevy::prelude::*;
use bevy::utils::HashMap;

//use crate::loading::EditorLoadingState;  
use bevy::scene::SceneInstanceReady; 
 


/*

The materials MUST finish extraction before loading in the models 

*/
pub fn material_replacements_plugin(app: &mut App) {
    app 	

    		.register_type::<MaterialReplacementComponent>()
    			.register_type::<MaterialReplacementWhenSceneReadyComponent>()
    	.register_type::<MaterialReplacementApplySetWhenSceneReadyComponent>()
     
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


#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
pub struct MaterialReplacementWhenSceneReadyComponent {
 
	pub material_replacements: HashMap<String,String>
}

#[derive(Component,Debug,Reflect)]
#[reflect(Component)]
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
	material_override_request_query: Query< (Entity, &MaterialReplacementApplySetWhenSceneReadyComponent ), Added<MaterialReplacementApplySetWhenSceneReadyComponent>  >,

	material_replacements_config: Res<MaterialReplacementsMap> ,

) {


	for (entity, mat_replacement_request) in material_override_request_query.iter() {

 


		let material_replacement_sets = &material_replacements_config.material_replacement_sets ; 


        let material_replacement_set =  material_replacement_sets.get( &mat_replacement_request.0  ) ;

		/*if let Some(matching_set) = material_replacement_sets.get( &mat_replacement_request.0  ){

			material_replacements = Some( matching_set.clone() );

		}else {

			warn!("could not find material replacement set {}", &mat_replacement_request.0 );
		}*/

		// info!(" handle_material_replacement_sets {} " , &mat_replacement_request.0 );


		if let Some( material_replacements ) =  material_replacement_set {
	 	

	 	// info!(" handle_material_replacement_sets 2 " );

			commands.entity(entity).try_insert( (
				MaterialReplacementWhenSceneReadyComponent {
					 material_replacements: material_replacements.clone()
				},
				MaterialReplacementComponent {
						 material_replacements: material_replacements.clone()
				}
			 ));

		}else {
			panic!("could not find mat rep {}",  &mat_replacement_request.0);
		}

	}

}



fn handle_material_replacements_when_scene_ready(
    scene_instance_evt_trigger: Trigger<SceneInstanceReady>,

    material_override_request_query: Query<&MaterialReplacementWhenSceneReadyComponent>,

    mut commands: Commands,

    parent_query: Query<&Parent>,
) {

		let trig_entity = scene_instance_evt_trigger.entity();

	    let Some(parent_entity) = parent_query.get(trig_entity).ok().map(|p| p.get()) else {
	        return;
	    };

 	
	 	 let Some(mat_override_request) = material_override_request_query.get(parent_entity).ok() else {
	        return;
	    }; 


 	   let material_replacements = mat_override_request.material_replacements.clone() ;

		if let Some(mut cmd) = commands.get_entity( parent_entity ) {

			cmd.try_insert(  
				MaterialReplacementComponent {
					material_replacements 
				}
			);
		}



          
 

}