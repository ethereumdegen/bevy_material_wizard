 

use crate::BevyMaterialWizardConfigResource;
use bevy::prelude::*;

use bevy::utils::HashMap;
use bevy_materialize::GenericMaterial;

//use crate::material_definition::MaterialDefinition;

 
 
  

#[derive(  Resource, Clone )]
pub struct RegisteredMaterialsMap  (  pub HashMap<String, Handle<GenericMaterial>>  );  



impl RegisteredMaterialsMap {

	pub fn find_material(&self, mat_name: &String) -> Option<&Handle<GenericMaterial>> {


		return self.0.get(  mat_name )

	}

	pub fn from_manifest_path(  manifest_path: &str, asset_server:  &mut AssetServer  ) -> Option<Self> {

		let mut registered_materials = HashMap::new(); 

		 // Attempt to read the manifest file
        let file_content = match std::fs::read_to_string(manifest_path) {
            Ok(content) => content,
            Err(_) => return None,
        };
        
        // Parse the RON file
        let manifest: HashMap<String, String> = match ron::from_str(&file_content) {
            Ok(parsed) => parsed,
            Err(_) => return None,
        };
        
        // Process each material in the manifest
        for (material_name, material_path) in manifest {
            // Create a new handle for each material
            let material_handle = asset_server.load( material_path ); // Or use your asset loading system
            
            // Store the handle in our map
            registered_materials.insert(material_name, material_handle);
            
            // Here you would typically load the actual material resource
            // This is just a placeholder - in a real implementation you'd use 
            // your game engine's asset loading system
            // Example: asset_server.load(&material_path);
        }


		Some(Self( registered_materials ))


	}

}
   

   impl FromWorld for RegisteredMaterialsMap {

	fn from_world(world: &mut World) -> Self {


		let manifest_path = world.get_resource::< BevyMaterialWizardConfigResource >().unwrap().material_defs_manifest_path.clone() ;
		let mut asset_server = world.get_resource_mut::<AssetServer>().unwrap() ;

		RegisteredMaterialsMap::from_manifest_path( &manifest_path, &mut asset_server  )  .unwrap() 

	}
}