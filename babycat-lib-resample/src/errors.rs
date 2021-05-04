#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    WrongInputFrameRate(u32),
    WrongOutputFrameRate(u32),
    WrongNumChannels(u32),
    WrongFrameRate(u32, u32),
    WrongFrameRateRatio(u32, u32),
    UnknownError(&'static str),
    FeatureNotCompiled,
}
