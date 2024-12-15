use crate::unit::Attack;

pub struct PhysicalAttack {
    pub damage: i32,
}

impl Attack for PhysicalAttack {
    fn attack(&self, target: &mut dyn crate::unit::Targetable) {
        target.take_damage(self.damage);
    }
}
