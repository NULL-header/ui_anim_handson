use bevy::prelude::*;

const MAX: f32 = 50.;

#[derive(Debug)]
pub struct PercentRound {
    value: f32,
    should_finish: bool,
}

impl PercentRound {
    pub fn new(value: f32) -> Self {
        let (value, should_finish) = if value >= MAX {
            (MAX, true)
        } else {
            (value, false)
        };
        Self {
            value,
            should_finish,
        }
    }
    pub fn get(&self) -> Val {
        Val::Percent(self.value)
    }

    pub fn should_finish(&self) -> bool {
        self.should_finish
    }
}

#[derive(Debug)]
pub struct Percent {
    value: f32,
}

impl Percent {
    pub fn new(val: Val) -> Self {
        let Val::Percent(value) = val else {
            panic!("MUST set the value as Val::Percent.");
        };
        Self { value }
    }
    pub fn add(&mut self, percent: f32) -> &Self {
        self.value += percent;
        self
    }
    pub fn round(&self) -> PercentRound {
        PercentRound::new(self.value)
    }
}

#[cfg(test)]
mod unittest {
    use super::*;
    use rstest::*;

    #[fixture]
    fn percent() -> Percent {
        let val = Val::Percent(25.);
        Percent::new(val)
    }

    #[rstest]
    fn normal(mut percent: Percent) {
        let round = percent.add(10.).round();
        assert_eq!(round.get(), Val::Percent(35.));
        assert!(!round.should_finish())
    }

    #[rstest]
    fn round(mut percent: Percent, #[values(25., 50.)] added: f32) {
        let round = percent.add(added).round();
        assert_eq!(round.get(), Val::Percent(50.));
        assert!(round.should_finish())
    }

    #[rstest]
    #[should_panic]
    fn not_percent() {
        let val = Val::Px(25.);
        Percent::new(val);
    }
}
