extern crate specs;
use specs::prelude::*;
use super::{CombatStats, SufferDamage, Player, gamelog::GameLog, Name};
use rltk::console;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = ( WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount;
        }
        damage.clear();
    }
}

pub fn delete_the_dead(ecs : &mut World) {
    let mut dead : Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();
        let names = ecs.read_storage::<Name>();
        let players = ecs.read_storage::<Player>();
        let mut log = ecs.write_resource::<GameLog>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 { 
                    let player = players.get(entity);
                    match player {
                        None => {
                            let victim = names.get(entity); 
                            if let Some(victim) = victim {
                                log.entries.insert(0, format!("{}  is dead", &victim.name));
                            }
                            dead.push(entity)
                        }
                        Some(_) => console::log("YOU DIED.")
                    }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete entity");
    }
}