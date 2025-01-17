#[derive(PartialEq, Clone, Copy)]
pub enum XSynthRenderAudioFormat {
    Wav,
}

pub struct XSynthRenderConfig {
    pub channel_count: u32,
    pub use_threadpool: bool,
    pub use_limiter: bool,
    pub sample_rate: u32,
    pub audio_channels: u16,
    pub audio_format: XSynthRenderAudioFormat,
}

impl Default for XSynthRenderConfig {
    fn default() -> Self {
        Self {
            channel_count: 16,
            use_threadpool: true,
            use_limiter: true,
            sample_rate: 48000,
            audio_channels: 2,
            audio_format: XSynthRenderAudioFormat::Wav,
        }
    }
}

impl Clone for XSynthRenderConfig {
    fn clone(&self) -> Self {
        XSynthRenderConfig {
            channel_count: self.channel_count,
            use_threadpool: self.use_threadpool,
            use_limiter: self.use_limiter,
            sample_rate: self.sample_rate,
            audio_channels: self.audio_channels,
            audio_format: self.audio_format,
        }
    }
}
