use std::{ops::Deref, sync::Arc};

use crate::{
    soundfont::SoundfontBase,
    voice::{Voice, VoiceControlData},
};

use super::voice_spawner::VoiceSpawnerMatrix;

pub struct ChannelSoundfont {
    soundfonts: Vec<Arc<dyn SoundfontBase>>,
    matrix: VoiceSpawnerMatrix,
}

impl Deref for ChannelSoundfont {
    type Target = VoiceSpawnerMatrix;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl ChannelSoundfont {
    pub fn new() -> Self {
        ChannelSoundfont {
            soundfonts: Vec::new(),
            matrix: VoiceSpawnerMatrix::new(),
        }
    }

    pub fn set_soundfonts(&mut self, soundfonts: Vec<Arc<dyn SoundfontBase>>) {
        self.soundfonts = soundfonts;
        self.rebuild_matrix();
    }

    fn rebuild_matrix(&mut self) {
        for k in 0..128u8 {
            for v in 0..128u8 {
                let vec = self
                    .soundfonts
                    .iter()
                    .map(|sf| sf.get_attack_voice_spawners_at(k, v))
                    .find(|vec| !vec.is_empty())
                    .unwrap_or_default();
                self.matrix.set_spawners_attack(k, v, vec);

                let vec = self
                    .soundfonts
                    .iter()
                    .map(|sf| sf.get_release_voice_spawners_at(k, v))
                    .find(|vec| !vec.is_empty())
                    .unwrap_or_default();
                self.matrix.set_spawners_release(k, v, vec);
            }
        }
    }

    pub fn spawn_voices_attack<'a>(
        &'a self,
        control: &'a VoiceControlData,
        key: u8,
        vel: u8,
    ) -> impl Iterator<Item = Box<dyn Voice>> + 'a {
        self.matrix.spawn_voices_attack(control, key, vel)
    }

    pub fn spawn_voices_release<'a>(
        &'a self,
        control: &'a VoiceControlData,
        key: u8,
        vel: u8,
    ) -> impl Iterator<Item = Box<dyn Voice>> + 'a {
        self.matrix.spawn_voices_release(control, key, vel)
    }
}
