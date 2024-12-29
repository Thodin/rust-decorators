use crate::unit::{Attack, Targetable};

pub struct MultistrikeDecorator<T: Attack> {
    num_strikes: u8,
    wrapped_attack: T,
}

impl<T: Attack> MultistrikeDecorator<T> {
    pub fn new(num_strikes: u8, attack_to_wrap: T) -> Self {
        MultistrikeDecorator {
            num_strikes,
            wrapped_attack: attack_to_wrap,
        }
    }
}

impl<T: Attack> Attack for MultistrikeDecorator<T> {
    fn attack<Target: Targetable>(&self, target: &mut Target) {
        for _ in 0..self.num_strikes {
            self.wrapped_attack.attack(target);
        }
    }
}

pub struct Execute<T: Attack> {
    health_threshold: i32,
    wrapped_attack: T,
}

impl<T: Attack> Execute<T> {
    pub fn new(health_threshold: i32, attack_to_wrap: T) -> Self {
        Execute {
            health_threshold,
            wrapped_attack: attack_to_wrap,
        }
    }
}

impl<T: Attack> Attack for Execute<T> {
    fn attack<Target: Targetable>(&self, target: &mut Target) {
        if target.health() < self.health_threshold {
            println!("Executing {} ...", target.name());
            target.take_damage(target.health());
        } else {
            self.wrapped_attack.attack(target);
        }
    }
}
