
 
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
pub fn material_overrides_plugin(app: &mut App) {
    app 	

    	
    	.init_state::<MaterialOverridesLoadingState>()
    	 
    	.add_systems(
    		OnEnter(MaterialOverridesLoadingState::Extracting),
    	      load_material_overrides
    	      )

     	
     	 .add_systems(OnEnter(MaterialOverridesLoadingState::Building), 
       			build_material_overrides

       	 )


       .add_systems(Update, 
       	extract_material_overrides 
       	 )

      

       .add_systems(Update, (
       	handle_material_overrides_when_scene_ready,
       	handle_material_overrides
       	).chain() .in_set(MaterialOverridesSet) )

   

       ;
}

#[derive(SystemSet,Hash,Clone,Debug,Eq,PartialEq)]
pub struct MaterialOverridesSet;


#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum MaterialOverridesLoadingState{
	#[default]
   Init,
   Extracting,
   Building,
   Complete
}


 


#[derive(Resource)]
pub struct MaterialOverridesResource {

	pub doodad_materials_gltf_path: String,
	pub material_types_config_path: String, 

	pub doodad_materials_gltf: Option<Handle<Gltf>>,

	pub extracted_materials_map : HashMap< String, Handle<StandardMaterial> >,

	//pub built_materials_map: HashMap< String, Handle<StandardMaterial> >, //uses materials config 

}
 



#[derive(Resource,Default)]
pub struct BuiltMaterialsResource {

	 
	pub built_materials_map: HashMap< String, OverrideMaterialHandle >, //uses materials config 

}

impl BuiltMaterialsResource{

	pub fn find_material_by_name(&self, mat_name: &String ) -> Option<& OverrideMaterialHandle > {


		self.built_materials_map.get( mat_name )
	}

}



pub enum OverrideMaterialHandle {

	Standard(Handle<StandardMaterial>),
	Foliage(Handle<FoliageMaterialExtension>)

}



//attach this to signal that the material is supposed to be replaced 
#[derive(Component,Debug)]
pub struct MaterialOverrideComponent {
 
	pub material_override: String
}

#[derive(Component,Debug)]
pub struct RefreshMaterialOverride ;


#[derive(Component,Debug)]
pub struct MaterialOverrideWhenSceneReadyComponent {
 
	pub material_override: String
}



/*
#[derive(Component,Debug)]
pub struct ReadyForMaterialOverride ;
*/


pub fn begin_loading_materials(
	mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,
	){


		next_state.set(MaterialOverridesLoadingState::Extracting);


}


fn load_material_overrides(

	asset_server: ResMut<AssetServer> ,

	mut material_overrides_resource: ResMut<MaterialOverridesResource>,

	


){	

	let material_overrides_path = &material_overrides_resource.doodad_materials_gltf_path;
	//let material_overrides_path = "material_overrides/doodad_material_overrides.glb";

	let doodad_materials_gltf = asset_server.load::<Gltf>( material_overrides_path  );

	material_overrides_resource.doodad_materials_gltf = Some(doodad_materials_gltf);

//	next_state.set(MaterialOverridesLoadingState::Extracting);


}


fn extract_material_overrides(
	  mut asset_ready_event: EventReader<AssetEvent<Gltf>>,
     mut material_overrides_resource: ResMut<MaterialOverridesResource>,
     mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,

     gltf_assets: Res<Assets<Gltf>>,

  //   mut material_assets: ResMut<Assets<StandardMaterial>>

	 
){

	for evt in asset_ready_event.read(){
		match evt {
   
			    AssetEvent::LoadedWithDependencies { id } => {
			    	if material_overrides_resource.doodad_materials_gltf.as_ref().is_some_and(|h| h.id() == *id ){

			    		let Some(doodad_materials_gltf) = gltf_assets.get( *id ) else {continue};

			    		for (material_name, material_handle) in &doodad_materials_gltf.named_materials {
			    			
			    			

			    			info!("extracted override material: {}", material_name.to_string());
			    			material_overrides_resource.extracted_materials_map.insert(material_name.to_string(), material_handle.clone());


			    		}


			    		next_state.set(MaterialOverridesLoadingState::Building);

			    	}
			    }

			    _ => {}
			}



	} 

}


//read the config, loop through it, and populate 

fn build_material_overrides(



	    material_overrides_resource: Res<MaterialOverridesResource>,

	    mut built_materials_resource: ResMut<BuiltMaterialsResource>,

	    mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,


	    material_types_config: Res<MaterialTypesConfig>,


	       material_assets: Res<Assets<StandardMaterial>>, //for reading the materials from the glb 
	   // mut material_assets: ResMut<Assets<StandardMaterial>>,

	    asset_server: ResMut<AssetServer>, 


	 ){


	let extracted_materials = &material_overrides_resource.extracted_materials_map;
	//let material_types_config_path = &material_overrides_resource.material_types_config_path;
	 
 



	for (built_material_name, material_config) in material_types_config.material_types.iter(){


		if let Some( extracted_material_handle ) = extracted_materials.get( & material_config.material_name ) .clone() {

			let Some(extracted_material) = material_assets.get(  extracted_material_handle  ) else {continue};
 


			let mut cloned_base_material = extracted_material.clone() ;

					let uv_scale = material_config.uv_scale_factor;
					cloned_base_material.uv_transform = Affine2::from_scale(Vec2::splat(uv_scale));

					if let Some(new_color) = material_config.diffuse_color_tint {
				 		cloned_base_material.base_color = new_color.clone().into(); 
				 	}

			let   built_material =  match material_config.shader_type.clone()
			.unwrap_or(MaterialShaderType::StandardMaterial) {
			    MaterialShaderType::StandardMaterial =>  {


			    	//let mut new_standard_material = cloned_base_material;

			    	 

				 	//new_standard_material.base_color_texture = extracted_material_diffuse_texture;
				 	//new_standard_material.normal_map_texture = extracted_material_normal_texture;
				 		//waht else to apply ?? 

			    	OverrideMaterialHandle::Standard( 
			    		asset_server.add( cloned_base_material )
			    	 )
			    },
			    MaterialShaderType::FoliageMaterial =>  {


			    	cloned_base_material.alpha_mode = AlphaMode::Blend;

			    	let new_foliage_material = FoliageMaterialExtension {
			    		base: cloned_base_material,

			    		..default()
			    	};



			    

				 	//new_foliage_material.base.base_color_texture = extracted_material_diffuse_texture;
				 	//new_foliage_material.base.normal_map_texture = extracted_material_normal_texture;


			    	OverrideMaterialHandle::Foliage( 
			    		asset_server.add( new_foliage_material )
			    	 )

			    },
			};

			 

			
		 	
		 	//apply diffuse color ? 


		 	

			//let built_material_handle = material_assets.add( built_material );


				built_materials_resource.built_materials_map.insert(
					built_material_name.clone(),

					built_material 

				);



		}

	


	}


	next_state.set(MaterialOverridesLoadingState::Complete);



}


fn handle_material_overrides(
	mut commands:Commands, 
//	mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,  

	material_override_query: Query<(Entity, &MaterialOverrideComponent), 
	Or<( Changed<MaterialOverrideComponent> , Added<RefreshMaterialOverride>) > >,

	//parent_query : Query<&Parent>, 
	// name_query: Query<&Name>,
	children_query: Query<&Children>,

	//material_handle_query: Query<&Handle<StandardMaterial>>,

	 mut materials: ResMut<Assets<StandardMaterial>>,

	 mesh_query: Query<&Handle<Mesh>>,


	//material_overrides_resource: Res<MaterialOverridesResource>,
	built_materials_resource: Res<BuiltMaterialsResource> ,
){




   // for evt in scene_instance_evt_reader.read(){

       //   let parent = evt.parent; //the scene 

//          let Some(parent_entity) = parent_query.get(parent).ok().map( |p| p.get() ) else {continue};

          for (mat_override_entity, mat_override_request) in  material_override_query.iter(){

                	 

             	//info!("about to handle material override {:?}", mat_override_request);

          
             	let material_name = &mat_override_request.material_override ;




             //	for (mat_base,mat_type) in mat_override_request.material_overrides.iter() {


             	     let extracted_material = built_materials_resource
             		   .find_material_by_name(&material_name);

             		   

             		  if let Some(new_material_handle) =extracted_material {
 

             		  		if   mesh_query.get(mat_override_entity).ok().is_some() {
	             		 	 		 


	             		 	 		 match new_material_handle {
					                    OverrideMaterialHandle::Standard(mat_handle) => {
					                        commands.entity(mat_override_entity).try_insert(mat_handle.clone());
					                    }
					                    OverrideMaterialHandle::Foliage(mat_handle) => {
					                        commands.entity(mat_override_entity).try_insert(mat_handle.clone())
					                        .remove::<Handle<StandardMaterial>>( );

					                    }
					                }


                				 

					             //     info!("inserted new material as override"); 
	             		 	 	}else {
	             		 	 		// warn!("no existing material to replace "); 
	             		 	 	}
 

	             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {

	             		 	 	//if let Some( _mat_handle) = material_handle_query.get(child).ok(){
	 								if   mesh_query.get(child).ok().is_some() {

	             		 	 		
	             		 	 		 match new_material_handle {
					                    OverrideMaterialHandle::Standard(mat_handle) => {
					                        commands.entity(child).try_insert(mat_handle.clone());
					                    }
					                    OverrideMaterialHandle::Foliage(mat_handle) => {
					                        commands.entity(child).try_insert(mat_handle.clone())
					                        .remove::<Handle<StandardMaterial>>( );
					                    }
					                } 

					                //  info!("inserted new material as override");


	             		 	 		}else {
		             		 	 		// warn!("no existing material to replace "); 
		             		 	 	}
							     
								 }







             		  }else {

             		  	  let warning_material = materials.add(Color::srgb(1.0, 0.0, 0.0)) ;
 
				             info!("inserted warning_material");
				          
				          

					        if   mesh_query.get(mat_override_entity).ok().is_some() {
	             		 	 		 commands
					                    .entity(mat_override_entity)
					                    .try_insert(warning_material.clone()); 

					                  info!("inserted new material as override"); 
	             		 	 	}else {
	             		 	 		// warn!("no existing material to replace "); 
	             		 	 	}
 

             		 	 for child in DescendantIter::new(&children_query, mat_override_entity) {

             		 	 	if   mesh_query.get(child).ok().is_some() {
 

             		 	 		 commands
				                    .entity(child)
				                    .try_insert(warning_material.clone()); 

				                  info!("inserted new material as override");


             		 	 	}else {
	             		 	 		// warn!("no existing material to replace "); 
	             		 	 	}
						     
						 }


             		  }

             		//let mat_base_name = mat_base.get_material_layer_name();
             		/*let Some(new_material_handle) = material_overrides_resource
             		   .find_material_by_name(&material_name) else {
             		   	warn!("could not get override material");
             		   	continue
             		     }; */



             		     


				             



             	//}



          }
           

     // }

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

                	/*commands
	                    .entity(doodad_entity)
	                    .remove::<MaterialOverrideRequestComponent>( ); */



             //	info!("about to handle material override {:?}", mat_override_request);

           //  	let Some(children) = children_query.get(doodad_entity).ok() else {continue};

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