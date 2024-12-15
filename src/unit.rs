pub trait Attack {
    fn attack(&self, target: &mut dyn Targetable);
}

pub trait Targetable {
    fn take_damage(&mut self, damage: i32);
}

pub struct Unit {
    health: i32,
    attack_ability: Box<dyn Attack>,
}

impl Unit {
    pub fn new(health: i32, attack: Box<dyn Attack>) -> Unit {
        Unit {
            health,
            attack_ability: attack,
        }
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn attack_target(&self, target: &mut dyn Targetable) {
        self.attack_ability.attack(target);
    }
}

impl Targetable for Unit {
    fn take_damage(&mut self, damage: i32) {
        if damage > 0 {
            self.health -= damage;
        }
    }
}
