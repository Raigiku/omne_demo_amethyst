use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::{math::base::Vector3, transform::Transform},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use derive_new::new;
use std::collections::HashMap;
use std::path::Path;
use tiled::parse_file;

use crate::prefab;

#[derive(new)]
pub struct Gameplay {
    #[new(value = "Some(Default::default())")]
    pub progress_counter: Option<ProgressCounter>,
    #[new(default)]
    pub prefab_handle: Option<Handle<Prefab<prefab::Scene1>>>,
}

impl SimpleState for Gameplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let scene1_prefab = world.exec(|loader: PrefabLoader<'_, prefab::Scene1>| {
            loader.load(
                "prefab/scene1.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap(),
            )
        });

        world.delete_all();
        world.create_entity().with(scene1_prefab.clone()).build();

        self.init_tiles(world);
        self.init_camera(world);
        self.prefab_handle = Some(scene1_prefab);
    }

    // fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    // if let Some(ref progress_counter) = self.progress_counter {
    //     if progress_counter.is_complete() {
    //         data.world.exec(
    //             |(mut transforms, players): (WriteStorage<Transform>, ReadStorage<PlayerComponent>)| {
    //                 for (transform, _) in (&mut transforms, &players).join() {
    //                     transform.prepend_translation_x(40.0);
    //                 }
    //             }
    //         );
    //         self.progress_counter = None;
    //     }
    // }
    // Trans::None
    // }
}

impl Gameplay {
    fn init_camera(&self, world: &mut World) {
        let screen_dimensions = world.read_resource::<ScreenDimensions>().clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            screen_dimensions.width() / 2.,
            screen_dimensions.height() / 2.,
            10.,
        );

        world
            .create_entity()
            .with(Camera::standard_2d(
                screen_dimensions.width(),
                screen_dimensions.height(),
            ))
            .with(transform)
            .build();
    }

    fn load_sprite_sheet(&self, world: &mut World, path: &mut String) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            path.push_str(".png");
            loader.load(
                path.to_string(),
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        for _ in 0..3 {
            path.pop();
        }
        path.push_str("ron");
        loader.load(
            path.to_string(),
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    }

    fn init_tiles(&self, world: &mut World) {
        let map = parse_file(Path::new("assets/map/scene1.tmx")).unwrap();

        // Loading sprite sheet handlers into hashmap
        let mut sprite_sheet_handles = HashMap::new();
        for tileset in map.tilesets.iter() {
            let mut tiles_dir = String::from("image/tile_sheet/");
            tiles_dir.push_str(tileset.name.as_str());
            sprite_sheet_handles.insert(
                tileset.name.as_str(),
                self.load_sprite_sheet(world, &mut tiles_dir),
            );
        }

        // Iterating through layers so that Z translation can be set latter for overlapping tiles
        for layer in map.layers.iter() {
            // Iterating through rows of the matrix of the map
            for (y, gid_row) in layer.tiles.iter().enumerate() {
                // Iterating through columns of the rows of the matrix of the map
                for (x, gid) in gid_row.iter().enumerate() {
                    // Getting the tileset of the selected tile so that the transforms can be set according to the dimensions of the tile sprite
                    // and also to get the according sprite sheet handle
                    if let Some(tileset) = map.get_tileset_by_gid(*gid) {
                        let mut tile_transform = Transform::default();
                        // The Tiled crate puts the origin of the map at the top right, meanwhile Amethyst has it in the bottom right. Therefore,
                        // an adjustment to the Y coordinate has to be made
                        let screen_dimensions = world.read_resource::<ScreenDimensions>().clone();
                        let map_y_offset = screen_dimensions.height();

                        // Constant that allows for tiles to scale
                        const SCALE_MULTIPLIER: f32 = 5.;
                        tile_transform.set_scale(Vector3::new(SCALE_MULTIPLIER, SCALE_MULTIPLIER, 1.));

                        // Setting the translation of the 4x8 tiles
                        if tileset.name == "tiles_4x8" {
                            tile_transform.set_translation_xyz(
                                x as f32 * map.tile_width as f32 * SCALE_MULTIPLIER - (map.tile_width as f32 / 2. * SCALE_MULTIPLIER),
                                map_y_offset - (y as f32 * map.tile_width as f32 * 2. * SCALE_MULTIPLIER),
                                0.,
                            );
                        }
                        // Setting the translation of the 8x8 tiles
                        else if tileset.name == "tiles_8x8" {
                            tile_transform.set_translation_xyz(
                                x as f32 * map.tile_width as f32 * SCALE_MULTIPLIER,
                                map_y_offset - (y as f32 * map.tile_width as f32 * 2. * SCALE_MULTIPLIER),
                                0.,
                            );
                        }

                        // Changing the Z translation of the Wall since they are overlap the floor tiles
                        if layer.name == "Wall" {
                            tile_transform.set_translation_z(0.1);
                        }

                        // Calculating the sprite number since Tiled doesn't index them depending on their tileset
                        let sprite_number = gid - tileset.first_gid;
                        // Getting the sprite sheet handle to create the sprite render and start creating an entity
                        if let Some(sprite_sheet_handle) =
                            sprite_sheet_handles.get(tileset.name.as_str())
                        {
                            let sprite_sheet_handle = sprite_sheet_handle.clone();
                            let sprite_render = SpriteRender {
                                sprite_sheet: sprite_sheet_handle,
                                sprite_number: sprite_number as usize,
                            };

                            world
                                .create_entity()
                                .with(sprite_render.clone())
                                .with(tile_transform)
                                .build();
                        }
                    }
                }
            }
        }
    }
}
