
use bevy::prelude::*;

use bevy::utils::HashMap;

use crate::material_definition::MaterialDefinition;




#[derive(  Resource,   Clone, Default )]
pub struct BuiltMaterialsMap {
 
    pub built_materials: HashMap<String,Handle<StandardMaterial>>,

}



impl BuiltMaterialsMap {

	pub fn find_or_load_material( 
		&mut self , 
		material_name: &String,
		material_definitions_map: &HashMap<String,MaterialDefinition>, 
		asset_server: &mut AssetServer,
		material_assets: &mut Assets<StandardMaterial> ,

		) -> Option< Handle<StandardMaterial> > {

		if let Some(existing_loaded_mat_handle) = self.built_materials.get( material_name  ) {

			return Some( existing_loaded_mat_handle.clone() );
		}else {


			let material_definition = material_definitions_map.get( material_name )?;



			let base_color_texture_handle: Option<Handle<Image>> = material_definition.diffuse_texture.as_ref().map(|tex| asset_server.load(tex.to_string())) ;
 

			let loaded_material = StandardMaterial{

				base_color_texture:  base_color_texture_handle ,


				..default() 
			};


			let loaded_material_handle = material_assets.add( loaded_material );

			self.built_materials.insert( material_name .to_string(), loaded_material_handle.clone() );

			return Some(loaded_material_handle.clone());

		}




	}


}