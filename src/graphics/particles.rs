use crate::prelude::*;

use super::ParticlePlugin;

#[derive(Component)]
struct Particle {
    lifetime: Timer,
}

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleVelocity {
    start: Vec2,
    end: Vec2,
}

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleSize {
    start: f32,
    end: f32,
}

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleColor {
    start: Color,
    end: Color,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_particles).add_system(update_spawners);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::rgba(
        lerp(a.r(), b.r(), t),
        lerp(a.g(), b.g(), t),
        lerp(a.b(), b.b(), t),
        lerp(a.a(), b.a(), t),
    )
}

//TODO break into multiple systems?
fn update_particles(
    mut commands: Commands,
    mut particles: Query<(
        Entity,
        &mut Particle,
        //Using sprite not texture atlas
        &mut Sprite,
        &mut Transform,
        Option<&ParticleSize>,
        Option<&ParticleColor>,
        Option<&ParticleVelocity>,
    )>,
    time: Res<Time>,
) {
    for (ent, mut particle, mut sprite, mut transform, size, color, velocity) in
        particles.iter_mut()
    {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.just_finished() {
            commands.entity(ent).despawn();
        }
        let t = particle.lifetime.percent();
        if let Some(size) = size {
            sprite.custom_size = Some(Vec2::splat(lerp(size.start, size.end, t)));
        }
        if let Some(color) = color {
            sprite.color = lerp_color(color.start, color.end, t);
        }
        if let Some(velocity) = velocity {
            let velocity = lerp_vec2(velocity.start, velocity.end, t);
            transform.translation += velocity.extend(0.0) * time.delta_seconds();
        }
    }
}

fn update_spawners(
    mut commands: Commands,
    mut spawners: Query<(Entity, &ParticleSpawner, &mut ParticleSpawnerTimer)>,
    time: Res<Time>,
) {
    for (ent, spawner, mut timer) in spawners.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            for _ in 0..spawner.amount_per_burst {
                let mut sprite = Sprite::default();
                let particle = commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform::from_xyz(
                            spawner.particle_position_range * rand::random::<f32>(),
                            spawner.particle_position_range * rand::random::<f32>(),
                            900.0,
                        ),
                        ..default()
                    })
                    .insert(Particle {
                        lifetime: Timer::from_seconds(spawner.particle_lifetime, false),
                    })
                    .insert(Sprite::default())
                    .id();
                if let Some(size) = spawner.particle_size {
                    sprite.custom_size = Some(Vec2::splat(size.start));
                    commands.entity(particle).insert(size);
                }
                if let Some(velocity) = spawner.particle_velocity {
                    commands.entity(particle).insert(velocity);
                }
                if let Some(color) = spawner.particle_color {
                    commands.entity(particle).insert(color);
                    sprite.color = color.start;
                }
                commands.entity(particle).insert(sprite);
                //FIXME this appears to cause the size of children to grow without bounds
                //Honestly we should be using a particle pool for this
                commands.entity(ent).add_child(particle);
            }
        }
    }
}

pub fn spawn_particle_spawner(commands: &mut Commands, config: &str, position: Vec2) {
    //let spawner = comp_from_config!(ParticleSpawner, "config/smoke_particle_spawner.ron");
    let spawner = comp_from_config!(ParticleSpawner, config);
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(position.extend(0.0)),
        ))
        .insert(ParticleSpawnerTimer {
            timer: Timer::from_seconds(spawner.rate, true),
        })
        .insert(spawner)
        .insert(Name::new("ParticleSpawner"));
}
