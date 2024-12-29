use attacks::{
    attack_decorators::{ExecuteDecorator, MultistrikeDecorator},
    base_attacks::PhysicalAttack,
};
use unit::{Attack, Targetable, Unit};

pub mod attacks;
pub mod unit;

// Alternative to the decorator pattern: Explicit implementations in structs
// - either one struct that does it all -> spaghetti code
// - or a struct for each combination -> combinatoric explosion of number of structs

fn main() {
    let player_base_attack: Box<dyn Attack> = Box::new(PhysicalAttack { damage: 15 });
    let player_execute_attack: Box<dyn Attack> =
        Box::new(ExecuteDecorator::new(30, player_base_attack));
    let mut player = Unit::new("Player".into(), 100, player_execute_attack);

    let enemy_base_attack: Box<dyn Attack> = Box::new(PhysicalAttack { damage: 10 });
    let enemy_multistrike_attack: Box<dyn Attack> =
        Box::new(MultistrikeDecorator::new(3_u8, enemy_base_attack));
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
