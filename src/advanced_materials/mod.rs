

use bevy::prelude::*;


pub mod foliage_material; 

pub fn advanced_materials_plugin(app: &mut App){

	app 


	.add_plugins( foliage_material::foliage_material_plugin )

	;

}