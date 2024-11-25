 

# Material Overrides Tool for Bevy

This Rust crate provides functionality for loading materials from a `.glb` file and additional material configuration data from a separate file. It supports hot-swapping materials in a Bevy scene using two custom components: `MaterialOverrideComponent` and `MaterialOverrideWhenSceneReadyComponent`.

## Features

- **Material Loading**: Automatically load materials from a specified `.glb` file and apply additional configurations, such as UV scaling and color tinting, from a separate configuration file.
- **Hot-Swapping**: Dynamically swap materials in the scene using the `MaterialOverrideComponent` and `MaterialOverrideWhenSceneReadyComponent`.
- **Flexible Configuration**: Customize materials using a configuration file that allows for UV scale adjustments and diffuse color tinting.



### Define your materials like this: 

![image](https://github.com/user-attachments/assets/1d5fda33-8916-4dbd-95aa-c10f22041939)




## Getting Started

### 1. Installation

To use this crate, add it to your `Cargo.toml`:

```
cargo add bevy_material_tool
```

### 2. Usage

#### Setting Up the Plugin

First, set up the `plugin` in your Bevy application:

```rust

    app.add_plugins(BevyMaterialToolPlugin{
                    material_types_config_path: "assets/material_overrides/material_types.ron".to_string(),
                    material_overrides_gltf_path : "material_overrides/doodad_material_overrides.glb".to_string()
            }  );

 
```


#### Configuring Material Overrides

Material override means that every child of the entity that has a handle<mesh> will have its handle<material> overridden by the specified material, indexed by string.  


The plugin uses either of two components to manage material overrides:

1. **`MaterialOverrideComponent`**: Attach this component to entities that require material replacement immediately. 
   
   ```rust
   commands.entity(entity)
       .insert(MaterialOverrideComponent {
           material_override: "Wall1".to_string(),
       });
   ```

2. **`MaterialOverrideWhenSceneReadyComponent`**: Use this component to defer material overrides until the scene is ready.

   ```rust
   commands.entity(entity)
       .insert(MaterialOverrideWhenSceneReadyComponent {
           material_override: "StoneToon1".to_string(),
       });
   ```


Take care not to add these components until after 'MaterialOverridesLoadingState::Complete' state has been entered.  !


#### Configuring Material Replacements 

To use material replacement, you need to load your GLTFs in a special way:  First you must load the GLTF into the asset server (not the scene!) and then to attach it to your entity to see it in engine , you would use something like 'AddGltfModelComponent' in gltf_models.rs.  

This way, the material_name_map plugin will be able to index every single named material in every GLTF you load into its resource and add a 'MaterialMetadataName' component to each entity with handle<StandardMaterial> loaded this way. 


Material replacement means that you specify a HashMap<String,String> (old material name -> new material name)  and then every child of the entity will be inspected for a 'MaterialMetadataName' component .  If that exists, and if its material name (from blender) matches any of thte 'keys' of the replacement hashmap, then that material will be swapped out for the new material using MaterialOverride.  

While MaterialOverride component only lets you make a GLTFs meshes all use a single material, MaterialReplacements component allows you to swap multiple materials for multiple other materials, surgically.  


#### Loading Materials

The materials are loaded from a `.glb` file specified in the `MaterialOverridesResource`:


### 3. Configuration File

The material configuration file is a RON (Rusty Object Notation) file that defines the material types and their properties, such as UV scale and color tint.

Example `material_types_config.ron`:

```ron
(
  material_types: {
    //these keys are material names used for MaterialOverride.0 or for the values of MaterialReplacement. The values should match your materials in the blender file of the .glb material swatch template that gets extracted by material_overrides.rs.

    "Wall1": (
      material_name: "Wall1",
      uv_scale_factor: 16.0,
      diffuse_color_tint: Some(LinearRgba(
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
      )),
    ),
    "StoneToon1": (
      material_name: "StoneToon1",
      uv_scale_factor: 2.0,
    ),
    "StoneToon2": (
      material_name: "StoneToon2",
      uv_scale_factor: 2.0,
    ),
    "DirtToon1": (
      material_name: "DirtToon1",
      uv_scale_factor: 2.0,
    ),
    "TiledWall1": (
      material_name: "Wall1",
      uv_scale_factor: 1.0,
    ),
  }, 


  //this is optional and it makes material replacement more succinct.  You can use the component MaterialReplacementApplySetWhenSceneReadyComponent("tree_fir") for example to specify that an entities children should experience material replacement according to the replacement map where the keys of the map are the material names in blender for the asset and the values are your custom material names defined as keys in material_types above.  

    material_replacement_sets: Some( {


    "tree_fir" : {

        "bark" : "TreeBark1",
        "leaves" : "TreeLeavesFir",
        "branches": "Branches1"

    },

    "tree_suntail" : {

        "bark" : "Wood1",
        "leaves" : "Leaves3",
        "branches": "Branches1"


    }

  } )
)
```

### 4. Advanced Usage

#### MaterialOverridesLoadingState

The crate defines a state machine to manage material loading and processing:

- **`Init`**: Initial state.
- **`Extracting`**: Extracts materials from the `.glb` file.
- **`Building`**: Builds materials based on the configuration file.
- **`Complete`**: All materials are loaded and ready.

You can customize or extend these states for more complex material management workflows.

### 5. Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

### 6. License

The code in this project is licensed under the MIT License.  Texture/material assets are not included in this license.  See the [LICENSE](LICENSE) file for details.

 
