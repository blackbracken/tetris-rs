use std::collections::HashMap;

use ggez::{
    Context,
    GameError,
    GameResult,
    graphics::{Font, Image},
};

use crate::kernel::repo::asset_provider::{AssetProvider, FontPath, ImagePath};

enum Asset<T> {
    Unloaded,
    Loaded { value: Box<T> },
    Missing { error: GameError },
}

pub struct DefaultAssetProvider {
    image_map: HashMap<String, Asset<Image>>,
    font_map: HashMap<String, Asset<Font>>,
}

impl AssetProvider for DefaultAssetProvider {
    fn image(&mut self, ctx: &mut Context, path: ImagePath) -> GameResult<&Image> {
        let path = path.0;

        if !self.image_map.contains_key(path) {
            let asset = match Image::new(ctx, path) {
                Ok(image) => Asset::Loaded {
                    value: Box::new(image),
                },
                Err(error) => Asset::Missing { error },
            };

            self.image_map.insert(path.to_owned(), asset);
        }

        match self.image_map.get(path) {
            Some(Asset::Loaded { value }) => Ok(value),
            Some(Asset::Missing { error }) => Err(error.clone()),
            _ => unreachable!(),
        }
    }

    fn font(&mut self, ctx: &mut Context, path: FontPath) -> GameResult<&Font> {
        let path = path.0;

        if !self.font_map.contains_key(path) {
            let asset = match Font::new(ctx, path) {
                Ok(font) => Asset::Loaded {
                    value: Box::new(font),
                },
                Err(error) => Asset::Missing { error },
            };

            self.font_map.insert(path.to_owned(), asset);
        }

        match self.font_map.get(path) {
            Some(Asset::Loaded { value }) => Ok(value),
            Some(Asset::Missing { error }) => Err(error.clone()),
            _ => unreachable!(),
        }
    }
}

impl DefaultAssetProvider {
    pub fn new() -> DefaultAssetProvider {
        DefaultAssetProvider {
            image_map: Default::default(),
            font_map: Default::default(),
        }
    }
}
