use std::time::Duration;

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
    variance: f32,
}

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleColor {
    start: Color,
    mid: Option<Color>,
    end: Color,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_particles)
            .add_system(update_spawners)
            .add_startup_system(spawn_star_particle_system);
    }
}

fn spawn_star_particle_system(mut commands: Commands, assets: Res<AssetServer>) {
    spawn_particle_spawner(
        &mut commands,
        "config/star_particle_spawner.ron",
        Vec3::ZERO,
        &assets,
    );
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
            if let Some(mid) = color.mid {
                sprite.color = lerp_color(
                    lerp_color(color.start, mid, t),
                    lerp_color(mid, color.end, t),
                    t,
                );
            } else {
                sprite.color = lerp_color(color.start, color.end, t);
            }
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
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    for (ent, spawner, mut timer) in spawners.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            for _ in 0..spawner.amount_per_burst {
                //FIXME this appears to cause the size of children to grow without bounds
                //Honestly we should be using a particle pool for this
                let particle =
                    spawn_particle(&mut commands, Some(assets.load("star.png")), spawner);
                commands.entity(ent).add_child(particle);
            }
        }
    }
}

fn spawn_particle(
    commands: &mut Commands,
    image: Option<Handle<Image>>,
    spawner: &ParticleSpawner,
) -> Entity {
    let mut sprite = Sprite::default();
    let image = match image {
        Some(image) => image,
        None => Handle::<Image>::default(),
    };
    let particle = commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(
                spawner.particle_position_range * (rand::random::<f32>() - 0.5),
                spawner.particle_position_range * (rand::random::<f32>() - 0.5),
                000.0,
            ),
            texture: image,
            ..default()
        })
        .insert(Particle {
            lifetime: Timer::from_seconds(spawner.particle_lifetime, false),
        })
        .insert(Sprite::default())
        .id();
    if let Some(size) = spawner.particle_size {
        let start = size.start + (rand::random::<f32>() - 0.5) * size.variance;
        let end = size.end + (rand::random::<f32>() - 0.5) * size.variance;
        sprite.custom_size = Some(Vec2::splat(start));
        commands.entity(particle).insert(ParticleSize {
            start,
            end,
            variance: 0.0,
        });
    }
    if let Some(velocity) = spawner.particle_velocity {
        commands.entity(particle).insert(velocity);
    }
    if let Some(color) = spawner.particle_color {
        commands.entity(particle).insert(color);
        sprite.color = color.start;
    }
    commands.entity(particle).insert(sprite);
    particle
}

pub fn spawn_particle_spawner(
    commands: &mut Commands,
    config: &str,
    position: Vec3,
    assets: &AssetServer,
) {
    //let spawner = comp_from_config!(ParticleSpawner, "config/smoke_particle_spawner.ron");
    let spawner = comp_from_config!(ParticleSpawner, config);
    let spawner_ent = commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(position),
        ))
        .insert(ParticleSpawnerTimer {
            timer: Timer::from_seconds(spawner.rate, true),
        })
        .insert(spawner.clone())
        .insert(Name::new("ParticleSpawner"))
        .id();
    if spawner.precharge {
        for _x in
            0..((spawner.particle_lifetime / spawner.rate) as usize * spawner.amount_per_burst)
        {
            let ent = spawn_particle(commands, Some(assets.load(&spawner.image)), &spawner);
            let mut timer = Timer::from_seconds(spawner.particle_lifetime, false);
            timer.tick(Duration::from_secs_f32(
                spawner.particle_lifetime * rand::random::<f32>(),
            ));
            commands.entity(ent).insert(Particle { lifetime: timer });
            commands.entity(spawner_ent).add_child(ent);
        }
    }
}
