use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use serde::Deserialize;

use crate::{
    assets::{spawn_sprite, Graphic, Graphics},
    comp_from_config,
    gas::Gas,
    world_object::WorldObject,
};

pub struct CanisterPlugin;

impl Plugin for CanisterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_canister)
            .add_startup_system(spawn_canister_machine)
            .add_system(update_canister_graphics)
            .register_inspectable::<Canister>()
            .register_inspectable::<CanisterMachine>();
    }
}

#[derive(Component, Default, Inspectable, Deserialize)]
//TODO mols, temp, pressure
pub struct Canister {
    percent_full: f32,
    gas: Gas,
}

#[derive(Component, Deserialize, Default, Inspectable)]
pub struct CanisterMachine {
    canisters: [Canister; 4],
}

#[derive(Component, Default)]
struct Label {
    id: usize,
    states: usize,
}

fn update_canister_graphics(
    canister_query: Query<(&Children, &Canister), Changed<Canister>>,
    machine_query: Query<(&Children, &CanisterMachine), Changed<CanisterMachine>>,
    mut label_query: Query<(&mut Graphic, &Label)>,
) {
    for (children, canister) in canister_query.iter() {
        for child in children.iter() {
            if let Ok((mut sprite, label)) = label_query.get_mut(*child) {
                update_small_label(canister.percent_full, label.states, &mut sprite)
            }
        }
    }
    for (children, machine) in machine_query.iter() {
        for child in children.iter() {
            if let Ok((mut sprite, label)) = label_query.get_mut(*child) {
                update_small_label(
                    machine.canisters[label.id].percent_full,
                    label.states,
                    &mut sprite,
                );
            }
        }
    }
}

fn spawn_canister_machine(mut commands: Commands, graphics: Res<Graphics>) {
    let ent = spawn_sprite(
        &mut commands,
        &graphics,
        Graphic::WorldObject(WorldObject::CanisterMachine),
    );
    let x_values = [-10.5, -3.5, 3.5, 10.5];
    for (i, x) in x_values.iter().enumerate() {
        let label = spawn_sprite(
            &mut commands,
            &graphics,
            Graphic::WorldObject(WorldObject::SmallLabel(0)),
        );
        commands
            .entity(label)
            .insert(Label { id: i, states: 8 })
            .insert(Transform::from_xyz(*x, 4.5, 0.01));
        commands.entity(ent).add_child(label);
    }
    commands
        .entity(ent)
        .insert(comp_from_config!(
            CanisterMachine,
            "config/canister_machine.ron"
        ))
        .insert(Name::new("Machine"))
        .insert(Transform::from_xyz(40.0, 10.0, 300.0));
}

fn spawn_canister(mut commands: Commands, graphics: Res<Graphics>) {
    let ent = spawn_sprite(
        &mut commands,
        &graphics,
        Graphic::WorldObject(WorldObject::Canister),
    );
    let label = spawn_sprite(
        &mut commands,
        &graphics,
        Graphic::WorldObject(WorldObject::SmallLabel(0)),
    );
    commands
        .entity(label)
        .insert(Label { id: 0, states: 8 })
        .insert(Transform::from_xyz(0.01, -1.01, 0.01));
    commands
        .entity(ent)
        .insert(Canister {
            percent_full: 0.50,
            gas: Gas::Hydrogen,
        })
        .insert(Transform::from_xyz(10.0, 10.0, 300.0))
        .insert(Name::new("Canister"))
        .add_child(label);
}

fn update_small_label(percent_full: f32, states: usize, sprite: &mut Graphic) {
    for i in (0..states).rev() {
        if percent_full > (i as f32 / states as f32) {
            *sprite = Graphic::WorldObject(WorldObject::SmallLabel(i));
            break;
        }
    }
}
