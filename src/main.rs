use attacks::{
    attack_decorators::{Execute, MultistrikeDecorator},
    base_attacks::PhysicalAttack,
};
use unit::{Attack, Targetable, Unit};

pub mod attacks;
pub mod unit;

fn main() {
    let player_base_attack: Box<dyn Attack> = Box::new(PhysicalAttack { damage: 15 });
    let player_execute_attack: Box<dyn Attack> = Box::new(Execute::new(30, player_base_attack));
    let mut player = Unit::new("Player".into(), 100, player_execute_attack);

    let enemy_base_attack: Box<dyn Attack> = Box::new(PhysicalAttack { damage: 10 });
    let enemy_multistrike_attack: Box<dyn Attack> =
        Box::new(MultistrikeDecorator::new(3, enemy_base_attack));
    let mut enemy = Unit::new("Enemy".into(), 70, enemy_multistrike_attack);

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
