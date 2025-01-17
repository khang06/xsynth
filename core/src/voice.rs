mod envelopes;
pub use envelopes::*;

mod simd;
pub use simd::*;

mod simdvoice;
pub use simdvoice::*;

mod base;
pub use base::*;

mod squarewave;
pub use squarewave::*;

mod channels;
pub use channels::*;

mod constant;
pub use constant::*;

mod sampler;
pub use sampler::*;

mod control;
pub use control::*;

mod cutoff;
pub use cutoff::*;

#[derive(Copy, Clone)]
pub struct EnvelopeControlData {
    pub attack: Option<u8>,
    pub release: Option<u8>,
}

#[derive(Copy, Clone)]
pub struct VoiceControlData {
    pub voice_pitch_multiplier: f32,
    pub envelope: EnvelopeControlData,
}

impl VoiceControlData {
    pub fn new_defaults() -> Self {
        VoiceControlData {
            voice_pitch_multiplier: 1.0,
            envelope: EnvelopeControlData {
                attack: None,
                release: None,
            },
        }
    }
}

pub trait VoiceGeneratorBase: Sync + Send {
    fn ended(&self) -> bool;
    fn signal_release(&mut self);
    fn process_controls(&mut self, control: &VoiceControlData);
}

pub trait VoiceSampleGenerator: VoiceGeneratorBase {
    fn render_to(&mut self, buffer: &mut [f32]);
}

pub trait Voice: VoiceSampleGenerator + Send + Sync {
    fn is_releasing(&self) -> bool;

    fn velocity(&self) -> u8;
}
