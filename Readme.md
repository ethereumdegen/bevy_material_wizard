 

# Bevy Material Wizard Plugin

A plugin for [Bevy](https://bevyengine.org/) that enables lazy-loading of material definitions from `.ron` files. This plugin allows for easy deserialization of material configurations and provides functionality to override materials in GLTF files using a helper component.

## Features

- **Lazy Loading**: Materials are loaded on-demand only when needed in a scene.
- **RON-based Configuration**: Define your materials in `.ron` files for easy customization.
- **Material Overrides for GLTF**: Replace default materials in GLTF models with custom materials using a helper component.
- **UV Transformations**: Supports scaling of UV coordinates via a `uv_scale_factor` in material definitions.

## Installation

cargo add bevy_material_wizard 



## Usage

### Defining Materials in RON

Create a `.ron` file defining your materials. For example:

```ron
MaterialDefinition(
    uv_scale_factor: 1.0,
    diffuse_color_tint: Some((1.0, 1.0, 1.0, 1.0)), // RGBA
    diffuse_texture: Some("textures/diffuse.png"),
    normal_texture: Some("textures/normal.png"),
    roughness: 0.8,
    metallic: 0.5,
    alpha_mode: "Opaque", // or "Blend"
)
```

### Loading Materials

The `BuiltMaterialsMap` resource provides a method to load or retrieve materials dynamically:

```rust
fn setup(
    mut commands: Commands,
    mut built_materials: ResMut<BuiltMaterialsMap>,
    material_definitions: Res<HashMap<String, MaterialDefinition>>,
    asset_server: Res<AssetServer>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let material_name = "example_material".to_string();

    if let Some(material_handle) = built_materials.find_or_load_material(
        &material_name,
        &material_definitions,
        &mut asset_server,
        &mut material_assets,
    ) {
        println!("Material loaded: {:?}", material_handle);
    }
}
```

### Overriding GLTF Materials

Attach a helper component to entities to override their materials:

```rust
commands.entity(entity).insert(MaterialOverride {
    material_name: "custom_material".to_string(),
});
```

## API Reference

### `BuiltMaterialsMap`

A resource that manages material handles. 

#### Methods

- `find_or_load_material`: Finds an existing material or loads a new one from the provided material definitions.

### Material Definition RON Schema

| Field                  | Type               | Description                                   |
|------------------------|--------------------|-----------------------------------------------|
| `uv_scale_factor`      | `f32`             | Scale factor for UV coordinates.             |
| `diffuse_color_tint`   | `Option<(f32, f32, f32, f32)>` | RGBA color tint for the material. |
| `diffuse_texture`      | `Option<String>`  | Path to the diffuse texture.                 |
| `normal_texture`       | `Option<String>`  | Path to the normal map texture.              |
| `roughness`            | `f32`             | Roughness value of the material.             |
| `metallic`             | `Option<f32>`     | Metallic value of the material.              |
| `alpha_mode`           | `String`          | Alpha mode, e.g., `"Opaque"` or `"Blend"`.   |

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you'd like to improve this plugin.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
