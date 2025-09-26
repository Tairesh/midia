use roguemetry::Vec2;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameView {
    pub zoom: Zoom,
}

impl Default for GameView {
    fn default() -> Self {
        Self { zoom: Zoom(2) }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Zoom(pub u8);

impl Zoom {
    pub fn as_view(self) -> f32 {
        self.into()
    }

    pub fn as_scale(self) -> Vec2 {
        let f = self.as_view();
        Vec2::new(f, f)
    }

    pub fn inc(&mut self) -> bool {
        if self.0 < 4 {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    pub fn dec(&mut self) -> bool {
        if self.0 > 0 {
            self.0 -= 1;
            true
        } else {
            false
        }
    }
}

impl From<Zoom> for f32 {
    fn from(z: Zoom) -> Self {
        match z.0 {
            0 => 1.0,
            1 => 2.0,
            2 => 3.0,
            3 => 4.0,
            4.. => 5.0,
        }
    }
}
