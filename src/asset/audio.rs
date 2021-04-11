use std::collections::HashMap;
use std::time::Duration;

use ggez::audio::SoundSource;
use ggez::{audio, Context, GameResult};

pub struct Audio {
    bgm_data_map: HashMap<Bgm, audio::SoundData>,
    se_data_map: HashMap<Se, audio::SoundData>,

    #[allow(dead_code)]
    playing_src: Option<audio::Source>,
}

impl Audio {
    pub(super) fn new(ctx: &mut Context) -> GameResult<Audio> {
        // for exhaustive checking on compile
        fn bgm_path(bgm: Bgm) -> &'static str {
            match bgm {
                Bgm::Title => "/sound/bgm/bgm_classic_etc_scarboroughfair.wav",
                Bgm::InGame => "/sound/bgm/bgm_classic_etc_korobushka.wav",
            }
        }
        let bgm_data_map = maplit::hashmap! {
            Bgm::Title => audio::SoundData::new(ctx, bgm_path(Bgm::Title))?,
            Bgm::InGame => audio::SoundData::new(ctx, bgm_path(Bgm::InGame))?,
        };

        // for exhaustive checking on compile
        fn se_path(se: Se) -> &'static str {
            match se {
                Se::MenuClick => "/sound/se/se_maoudamashii_system26.mp3",
                Se::CountdownTick => "/sound/se/se_maoudamashii_instruments_drum1_hat.mp3",
                Se::GameStart => "/sound/se/se_maoudamashii_instruments_drum1_tom3.mp3",
                Se::MinoMove => "/sound/se/thinkpad_shift_click.mp3",
                Se::MinoSpin => "/sound/se/se_maoudamashii_se_pc03.mp3",
                Se::MinoSoftDrop => "/sound/se/hhkb_fn_click.mp3",
                Se::MinoHardDrop => "/sound/se/hhkb_fn_click.mp3",
                Se::RemoveLine => "/sound/se/se_maoudamashii_onepoint09.mp3",
            }
        }
        let se_data_map = maplit::hashmap! {
            Se::MenuClick => audio::SoundData::new(ctx, se_path(Se::MenuClick))?,
            Se::CountdownTick => audio::SoundData::new(ctx, se_path(Se::CountdownTick))?,
            Se::GameStart => audio::SoundData::new(ctx, se_path(Se::GameStart))?,
            Se::MinoMove => audio::SoundData::new(ctx, se_path(Se::MinoMove))?,
            Se::MinoSpin => audio::SoundData::new(ctx, se_path(Se::MinoSpin))?,
            Se::MinoSoftDrop => audio::SoundData::new(ctx, se_path(Se::MinoSoftDrop))?,
            Se::MinoHardDrop => audio::SoundData::new(ctx, se_path(Se::MinoHardDrop))?,
            Se::RemoveLine => audio::SoundData::new(ctx, se_path(Se::RemoveLine))?,
        };

        Ok(Audio {
            bgm_data_map,
            se_data_map,
            playing_src: None,
        })
    }

    pub fn play_bgm(&mut self, ctx: &mut Context, bgm: Bgm) -> GameResult {
        self.stop_bgm();

        let src = self
            .bgm_data_map
            .get(&bgm)
            .and_then(|data| audio::Source::from_data(ctx, data.to_owned()).ok())
            .map(|mut src| {
                src.set_repeat(true);
                src.set_query_interval(Duration::ZERO);
                src
            })
            .map(|mut src| match bgm {
                Bgm::Title => {
                    src.set_volume(0.2);
                    src.set_fade_in(Duration::from_secs(2));
                    src
                }
                Bgm::InGame => {
                    src.set_volume(0.25);
                    src
                }
            });

        if let Some(src) = src {
            src.play_later()?;
            self.playing_src = Some(src);
        }

        Ok(())
    }

    pub fn stop_bgm(&mut self) {
        self.playing_src = None
    }

    pub fn play_se(&self, ctx: &mut Context, se: Se) -> GameResult {
        let src = self
            .se_data_map
            .get(&se)
            .and_then(|data| audio::Source::from_data(ctx, data.to_owned()).ok())
            .map(|mut src| {
                match se {
                    Se::MenuClick => {
                        src.set_volume(0.15);
                    }
                    Se::GameStart => {
                        src.set_volume(0.4);
                    }
                    Se::CountdownTick => {
                        src.set_volume(0.45);
                    }
                    Se::MinoMove => {
                        src.set_volume(0.8);
                        src.set_pitch(0.6);
                    }
                    Se::MinoSpin => {
                        src.set_volume(0.75);
                    }
                    Se::MinoSoftDrop => {
                        src.set_volume(0.15);
                    }
                    Se::MinoHardDrop => {
                        src.set_volume(0.6);
                        src.set_pitch(0.65);
                    }
                    Se::RemoveLine => {
                        src.set_pitch(1.3);
                    }
                }
                src
            });

        if let Some(mut src) = src {
            src.play_detached(ctx)?;
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum Bgm {
    Title,
    InGame,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Se {
    MenuClick,
    CountdownTick,
    GameStart,
    MinoMove,
    MinoSpin,
    MinoSoftDrop,
    MinoHardDrop,
    RemoveLine,
}
