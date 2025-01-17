use std::marker::PhantomData;

use simdeez::Simd;

use crate::voice::VoiceControlData;

use super::{SIMDSampleMono, SIMDVoiceGenerator, VoiceGeneratorBase};

pub struct SIMDSquareWaveGenerator<S: Simd, Pitch: SIMDVoiceGenerator<S, SIMDSampleMono<S>>> {
    phase: f32,

    pitch_gen: Pitch,

    _s: PhantomData<S>,
}

impl<S, Pitch> SIMDSquareWaveGenerator<S, Pitch>
where
    S: Simd,
    Pitch: SIMDVoiceGenerator<S, SIMDSampleMono<S>>,
{
    pub fn new(pitch_gen: Pitch) -> Self {
        Self {
            phase: 0.0,
            pitch_gen,
            _s: PhantomData,
        }
    }

    fn next_phase(&mut self, step: f32) -> f32 {
        self.phase += step;
        self.phase %= 1.0;
        self.phase
    }
}

impl<S, Pitch> VoiceGeneratorBase for SIMDSquareWaveGenerator<S, Pitch>
where
    S: Simd,
    Pitch: SIMDVoiceGenerator<S, SIMDSampleMono<S>>,
{
    #[inline(always)]
    fn ended(&self) -> bool {
        false
    }

    #[inline(always)]
    fn signal_release(&mut self) {
        self.pitch_gen.signal_release();
    }

    #[inline(always)]
    fn process_controls(&mut self, control: &VoiceControlData) {
        self.pitch_gen.process_controls(control);
    }
}

impl<S, Pitch> SIMDVoiceGenerator<S, SIMDSampleMono<S>> for SIMDSquareWaveGenerator<S, Pitch>
where
    S: Simd,
    Pitch: SIMDVoiceGenerator<S, SIMDSampleMono<S>>,
{
    #[inline(always)]
    fn next_sample(&mut self) -> SIMDSampleMono<S> {
        let mut values = unsafe { S::set1_ps(0.0) };
        let pitch_step = self.pitch_gen.next_sample().0;
        for i in 0..S::VF32_WIDTH {
            let phase = self.next_phase(pitch_step[i]);
            let val = if phase > 0.5 { 1.0 } else { -1.0 };
            values[i] = val;
        }

        SIMDSampleMono(values)
    }
}
