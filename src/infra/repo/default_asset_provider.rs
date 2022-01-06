use std::ops::Not;
use std::{collections::HashMap, rc::Rc};

use ggez::{graphics::Image, Context, GameError, GameResult};

use crate::domain::repo::asset_provider::AssetProvider;

enum Asset<T> {
    Unloaded,
    Loaded { value: Rc<T> },
    Missing { error: GameError },
}

pub struct DefaultAssetProvider {
    image_map: HashMap<String, Asset<Image>>,
}

impl AssetProvider for DefaultAssetProvider {
    fn image(&mut self, ctx: &mut Context, path: &String) -> GameResult<&Image> {
        if !self.image_map.contains_key(path) {
            let asset = match Image::new(ctx, path) {
                Ok(image) => Asset::Loaded {
                    value: Rc::new(image),
                },
                Err(error) => Asset::Missing { error },
            };

            self.image_map.insert(path.clone(), asset);
        }

        match self.image_map.get(path) {
            Some(Asset::Loaded { value }) => Ok(value),
            Some(Asset::Missing { error }) => Err(error.clone()),
            _ => unreachable!(),
        }
    }
}
