use std::collections::HashMap;

use ggez::{graphics::Image, Context, GameError, GameResult};

use crate::kernel::repo::asset_provider::{AssetProvider, ImagePath};

enum Asset<T> {
    Unloaded,
    Loaded { value: Box<T> },
    Missing { error: GameError },
}

pub struct DefaultAssetProvider {
    image_map: HashMap<String, Asset<Image>>,
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
}

impl DefaultAssetProvider {
    pub fn new() -> DefaultAssetProvider {
        DefaultAssetProvider {
            image_map: Default::default(),
        }
    }
}
