use crate::unit::Attack;

pub struct PhysicalAttack {
    pub damage: i32,
    pub num_strikes: u8,
    pub execute_health_threshold: i32,
}

impl Attack for PhysicalAttack {
    fn attack(&self, target: &mut dyn crate::unit::Targetable) {
        for _ in 0..self.num_strikes {
            if target.health() < self.execute_health_threshold {
                println!("Executing {} ...", target.name());
                target.take_damage(target.health());
            } else {
                target.take_damage(self.damage);
            }
        }
    }
}
