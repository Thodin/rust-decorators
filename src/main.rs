use attacks::PhysicalAttack;
use unit::{Attack, Unit};

pub mod attacks;
pub mod unit;

fn main() {
    let player_attack: Box<dyn Attack> = Box::new(PhysicalAttack { damage: 15 });
    let mut player = Unit::new(100, player_attack);

    let enemy_attack: Box<dyn Attack> = Box::new(PhysicalAttack { damage: 10 });
    let mut enemy = Unit::new(70, enemy_attack);

    while player.health() > 0 && enemy.health() > 0 {
        player.attack_target(&mut enemy);
        if enemy.health() > 0 {
            enemy.attack_target(&mut player);
        }
        println!(
            "Player health: {}, enemy health: {}",
            player.health(),
            enemy.health()
        );
    }
}
