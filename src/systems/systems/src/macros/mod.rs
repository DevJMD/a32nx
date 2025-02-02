/// Provide potential with the given normal range.
macro_rules! provide_frequency {
    ($t: ty, $normal_range: expr) => {
        impl ProvideFrequency for $t {
            fn frequency(&self) -> Frequency {
                self.output_frequency
            }

            fn frequency_normal(&self) -> bool {
                let hz = self.output_frequency.get::<hertz>();
                $normal_range.contains(&hz)
            }
        }
    };
}

/// Provide load with a normal range of 0% to 100%.
macro_rules! provide_load {
    ($t: ty) => {
        impl ProvideLoad for $t {
            fn load(&self) -> Ratio {
                self.load
            }

            fn load_normal(&self) -> bool {
                self.load <= Ratio::new::<percent>(100.)
            }
        }
    };
}

/// Provide potential with the given normal range.
macro_rules! provide_potential {
    ($t: ty, $normal_range: expr) => {
        impl ProvidePotential for $t {
            fn potential(&self) -> ElectricPotential {
                self.output_potential
            }

            fn potential_normal(&self) -> bool {
                let volts = self.output_potential.get::<volt>();
                $normal_range.contains(&volts)
            }
        }
    };
}

macro_rules! read_write_enum {
    ($t: ty) => {
        impl<T: Reader> Read<$t> for T {
            fn read(&mut self, name: &str) -> $t {
                self.read_f64(name).into()
            }
        }

        impl<T: Writer> Write<$t> for T {
            fn write(&mut self, name: &str, value: $t) {
                self.write_f64(name, value.into());
            }
        }

        impl From<$t> for f64 {
            fn from(value: $t) -> f64 {
                value as u8 as f64
            }
        }
    };
}
