use crate::unit::{Attack, Targetable};

pub struct PhysicalAttack {
    pub damage: i32,
}

impl Attack for PhysicalAttack {
    fn attack<Target: Targetable>(&self, target: &mut Target) {
        target.take_damage(self.damage);
    }
}
