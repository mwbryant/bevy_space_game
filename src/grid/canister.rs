//TODO create a gas module and move canister there
use bevy_inspector_egui::RegisterInspectable;

use crate::{grid::CanisterPlugin, prelude::*};

impl Plugin for CanisterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_canister)
            .add_startup_system(spawn_canister_machine)
            .add_system(update_canister_graphics)
            .register_inspectable::<Canister>()
            .register_inspectable::<CanisterMachine>();
    }
}

//TODO move to graphics module
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
                update_small_label(
                    canister.gases.get_total_pressure() / canister.max_pressure,
                    label.states,
                    &mut sprite,
                )
            }
        }
    }
    for (children, machine) in machine_query.iter() {
        for child in children.iter() {
            if let Ok((mut sprite, label)) = label_query.get_mut(*child) {
                update_small_label(
                    machine.canisters[label.id].gases.get_total_pressure()
                        / machine.canisters[label.id].max_pressure,
                    label.states,
                    &mut sprite,
                );
            }
        }
    }
}

fn spawn_canister_machine(mut commands: Commands) {
    let ent = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::CanisterMachine))
        .id();
    let x_values = [-10.5, -3.5, 3.5, 10.5];
    for (i, x) in x_values.iter().enumerate() {
        let label = commands
            .spawn()
            .insert(Graphic::WorldObject(WorldObject::SmallLabel(0)))
            .insert(Label { id: i, states: 8 })
            .insert(Transform::from_xyz(*x, 4.5, 0.01))
            .id();
        commands.entity(ent).add_child(label);
    }
    commands
        .entity(ent)
        .insert(CanisterMachine::default())
        .insert(Name::new("Machine"))
        .insert(Clickable::default())
        .insert(Transform::from_xyz(32.0, 10.0, 300.0));
}

fn spawn_canister(mut commands: Commands) {
    let ent = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::Canister))
        .id();
    let label = commands
        .spawn()
        .insert(Graphic::WorldObject(WorldObject::SmallLabel(0)))
        .id();
    commands
        .entity(label)
        .insert(Label { id: 0, states: 8 })
        .insert(Transform::from_xyz(0.01, -1.01, 0.01));
    commands
        .entity(ent)
        .insert(Canister {
            gases: GasMixture::single_gas(Gas::Oxygen, 500.0, 293.0),
            volume: 0.5,
            max_pressure: 10.0,
        })
        .insert(Clickable::default())
        .insert(Transform::from_xyz(06.0, 10.0, 300.0))
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
