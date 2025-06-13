use std::time::Instant;

use shared::effect::area;
use shared::projectile::Projectile;

use crate::boss::Boss;
use crate::hero::Hero;

pub struct Scene {
    pub last_updated: Instant,
    pub hero: Hero,
    pub boss: Boss,
    pub effects: Vec<area::Effect>,
    pub projectiles: Vec<Projectile>,
}

impl Scene {
    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.saturating_duration_since(self.last_updated).as_millis();
        self.last_updated = now;
        self.hero.update(&mut self.boss); // use dt
        self.boss.update(&mut self.hero); // use dt
        for effect in self.effects.iter_mut() {
            effect.update(dt);
        }
        for projectile in self.projectiles.iter_mut() {
            projectile.update(dt);
        }
    }
}
