
use bevy::{math::Affine2, };

use bevy::utils::HashSet;

use bevy::prelude::*;

use bevy::utils::HashMap;

use crate::material_definition::MaterialDefinition;

 


use bevy::image::{ImageSamplerDescriptor, ImageSampler  };
use bevy::render::render_resource::{
    AddressMode, FilterMode,
};
 


#[derive(  Resource,   Clone, Default )]
pub struct BuiltMaterialsMap {
 
    pub built_materials: HashMap<String,Handle<StandardMaterial>>,

}



#[derive(  Resource,   Clone, Default )]
pub struct MaterialImageHandlesCache(pub HashSet<AssetId<Image>>);


impl BuiltMaterialsMap {

	pub fn find_or_load_material( 
		&mut self , 
		material_name: &String,
		material_definitions_map: &HashMap<String,MaterialDefinition>, 
		material_images_cache: &mut MaterialImageHandlesCache,
		asset_server: &mut AssetServer,
		material_assets: &mut Assets<StandardMaterial> ,

		) -> Option< Handle<StandardMaterial> > {

		if let Some(existing_loaded_mat_handle) = self.built_materials.get( material_name  ) {

				// info!("found built material ");

			return Some( existing_loaded_mat_handle.clone() );
		}else {


			  let material_definition = material_definitions_map.get( material_name )?;

			  let uv_scale = material_definition.uv_scale_factor; 

			  let alpha_mode = material_definition.alpha_mode.to_alpha_mode();

			  let base_color = material_definition.diffuse_color_tint.unwrap_or(LinearRgba::WHITE.into() );

			 
			//bevy is bugged so this doesnt work ! 

			let base_color_texture_handle: Option<Handle<Image>> = material_definition.diffuse_texture.as_ref().map(
				|tex| asset_server.load(
					tex.to_string() )) ;

			if let Some(ref base_color_texture_handle) = base_color_texture_handle {
				material_images_cache.0.insert( base_color_texture_handle.id() );
			}

 			let normal_texture_handle: Option<Handle<Image>> = material_definition.normal_texture.as_ref().map(
 				|tex| asset_server.load(
 					tex.to_string() )) ;

 			if let Some(ref normal_texture_handle) = normal_texture_handle {
				material_images_cache.0.insert( normal_texture_handle.id() );
			}



			let emissive_texture_handle: Option<Handle<Image>> = material_definition.emissive_texture.as_ref().map(
				|tex| asset_server.load(
					tex.to_string() )) ;

			if let Some(ref emissive_texture_handle) = emissive_texture_handle {
				material_images_cache.0.insert( emissive_texture_handle.id() );
			}


			let emissive_color_tint = material_definition
				.emissive_color_tint.unwrap_or(LinearRgba::BLACK.into());


 
		  //	info!("create new built material ");
			let loaded_material = StandardMaterial{
				base_color: base_color.into(), 
				base_color_texture:  base_color_texture_handle ,
				normal_map_texture: normal_texture_handle,
				perceptual_roughness: material_definition.roughness,

				
				metallic: material_definition.metallic.unwrap_or(0.0),

				cull_mode: None , //two sided 

				alpha_mode, 

				uv_transform: Affine2::from_scale(Vec2::splat(uv_scale)) ,


				emissive_texture: emissive_texture_handle,
				emissive: emissive_color_tint.into(),

				//fix uv stretch ?

				..default() 
			};


			let loaded_material_handle = material_assets.add( loaded_material );

			self.built_materials.insert( material_name .to_string(), loaded_material_handle.clone() );

			return Some(loaded_material_handle.clone());

		}




	}


}


//when the image asset is loaded...
pub fn update_image_sampler_settings(
    mut image_events: EventReader<AssetEvent<Image>>,
    mut image_assets: ResMut<Assets<Image>>,
    material_image_handles_cache: Res<MaterialImageHandlesCache>,
) {
    // Iterate over all asset events for images
    for event in image_events.read() {
        if let AssetEvent::LoadedWithDependencies { id }  = event {


            // Check if the handle is in the MaterialImageHandlesCache
            if material_image_handles_cache.0.contains(id) {
                if let Some(texture_image) = image_assets.get_mut(*id) {
                   	info!("bevy material wizard: update image sampler !");


					 // Update the sampler settings for the loaded image
                    texture_image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        label: None,
                        address_mode_u:  AddressMode::Repeat.into(),
                        address_mode_v:  AddressMode::Repeat.into(),
                        address_mode_w:  AddressMode::Repeat.into(),
                        mag_filter: FilterMode::Linear.into(),
                        min_filter: FilterMode::Linear.into(),
                        mipmap_filter: FilterMode::Linear.into(),
                        ..default()
                    });
                }
            }
        }
    }
}