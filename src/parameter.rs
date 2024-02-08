#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Parameter {
    #[default]
    Frequency,
    Amplitude,
    Waveform,
    PhaseOffset,
    DcOffset,
    Pan,
}

impl TryFrom<usize> for Parameter {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Parameter::Frequency),
            1 => Ok(Parameter::Amplitude),
            2 => Ok(Parameter::Waveform),
            3 => Ok(Parameter::PhaseOffset),
            4 => Ok(Parameter::DcOffset),
            5 => Ok(Parameter::Pan),
            _ => Err(()),
        }
    }
}

impl Parameter {
    pub fn count() -> usize {
        6
    }

    pub fn next(&self) -> Self {
        let value = (*self as usize + 1) % Self::count();
        value.try_into().unwrap_or_default()
    }

    pub fn previous(&self) -> Self {
        let mut value = *self as usize;
        if value > 0 {
            value = value - 1;
        } else {
            value = Self::count() - 1;
        }
        value.try_into().unwrap_or_default()
    }
}
