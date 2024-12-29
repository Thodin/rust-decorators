use crate::unit::Attack;

pub struct MultistrikeDecorator {
    num_strikes: u8,
    wrapped_attack: Box<dyn Attack>,
}

impl MultistrikeDecorator {
    pub fn new(num_strikes: u8, attack_to_wrap: Box<dyn Attack>) -> Self {
        MultistrikeDecorator {
            num_strikes,
            wrapped_attack: attack_to_wrap,
        }
    }
}

impl Attack for MultistrikeDecorator {
    fn attack(&self, target: &mut dyn crate::unit::Targetable) {
        for _ in 0..self.num_strikes {
            self.wrapped_attack.attack(target);
        }
    }
}

pub struct ExecuteDecorator {
    health_threshold: i32,
    wrapped_attack: Box<dyn Attack>,
}

impl ExecuteDecorator {
    pub fn new(health_threshold: i32, attack_to_wrap: Box<dyn Attack>) -> Self {
        ExecuteDecorator {
            health_threshold,
            wrapped_attack: attack_to_wrap,
        }
    }
}

impl Attack for ExecuteDecorator {
    fn attack(&self, target: &mut dyn crate::unit::Targetable) {
        if target.health() < self.health_threshold {
            println!("Executing {} ...", target.name());
            target.take_damage(target.health());
        } else {
            self.wrapped_attack.attack(target);
        }
    }
}
