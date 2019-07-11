//! Struct of playable character.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::any::Any;

use crate::entry::*;

use crate::game_manager::GameManager;
use crate::wani_core::camera::DRAW_OFFSET;
use crate::wani_core::color::Color;
use crate::wani_core::input_manager::InputKey;
use crate::wani_core::input_manager::InputManager;
use crate::wani_core::input_manager::INPUT_MANAGER;
use crate::wani_core::rect::Rect;
use crate::wani_core::vector2;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::map_component::MapComponent;
use crate::wani_map::random_map::RandomMap;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::game_object::GameObject;
use crate::wani_trait::updater::Updater;

enum Animation {
    None,
    MoveUp { frame: isize },
    MoveDown { frame: isize },
    MoveLeft { frame: isize },
    MoveRight { frame: isize },
    MoveUpLeft { frame: isize },
    MoveUpRight { frame: isize },
    MoveDownLeft { frame: isize },
    MoveDownRight { frame: isize },
}

pub struct Hero {
    position: Vec2<isize>,

    anime: Animation,
    animating: bool,
    anime_frame: isize,
}

impl Hero {
    pub fn new() -> Self {
        Hero {
            position: Vec2::new(0, 0),

            anime: Animation::None,
            animating: false,
            anime_frame: 0,
        }
    }

    fn r#move(&mut self, direction: Vec2<isize>) {
        self.position += direction;
    }

    pub fn teleport(&mut self, coord: &Vec2<isize>) {
        self.position = *coord;
    }

    fn input_proc(&mut self, gm: &GameManager, im: &InputManager) {
        let mut in_dir = Vec2::new(0, 0);
        if im.get_key(InputKey::UP) {
            in_dir += Vec2::new(0, -1)
        }
        if im.get_key(InputKey::DOWN) {
            in_dir += Vec2::new(0, 1)
        }
        if im.get_key(InputKey::LEFT) {
            in_dir += Vec2::new(-1, 0)
        }
        if im.get_key(InputKey::RIGHT) {
            in_dir += Vec2::new(1, 0)
        }

        if in_dir != vector2::ZERO {
            let rmap = gm.get_map().lock().unwrap();
            let map = rmap.as_any().downcast_ref::<RandomMap>().unwrap();

            let mut move_dir = Vec2::new(0, 0);
            let mut dir;
            dir = Vec2::new(in_dir.x, 0);
            match map.get_component(&(self.position / 32 + dir)) {
                Some(comp) => match comp {
                    MapComponent::WALL => {}
                    _ => move_dir += dir,
                },
                None => {}
            }
            dir = Vec2::new(0, in_dir.y);
            match map.get_component(&(self.position / 32 + dir)) {
                Some(comp) => match comp {
                    MapComponent::WALL => {}
                    _ => move_dir += dir,
                },
                None => {}
            }
            if move_dir != vector2::ZERO {
                match map.get_component(&(self.position / 32 + move_dir)) {
                    Some(comp) => match comp {
                        MapComponent::WALL => {}
                        _ => {
                            if 0 < move_dir.x {
                                if 0 < move_dir.y {
                                    self.anime_start(Animation::MoveDownRight { frame: 8 })
                                } else if move_dir.y < 0 {
                                    self.anime_start(Animation::MoveUpRight { frame: 8 })
                                } else {
                                    self.anime_start(Animation::MoveRight { frame: 8 })
                                }
                            } else if move_dir.x < 0 {
                                if 0 < move_dir.y {
                                    self.anime_start(Animation::MoveDownLeft { frame: 8 })
                                } else if move_dir.y < 0 {
                                    self.anime_start(Animation::MoveUpLeft { frame: 8 })
                                } else {
                                    self.anime_start(Animation::MoveLeft { frame: 8 })
                                }
                            } else {
                                if 0 < move_dir.y {
                                    self.anime_start(Animation::MoveDown { frame: 8 })
                                } else if move_dir.y < 0 {
                                    self.anime_start(Animation::MoveUp { frame: 8 })
                                }
                            }
                        }
                    },
                    None => {}
                }
            }
        }
    }

    fn anime_start(&mut self, anime: Animation) {
        self.anime = anime;
        self.animating = true;
        self.anime_frame = 0;
    }

    fn animate(&mut self) {
        self.anime_frame += 1;
        match self.anime {
            Animation::MoveUp { frame } => {
                self.r#move(Vec2::new(0, -4));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveDown { frame } => {
                self.r#move(Vec2::new(0, 4));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveLeft { frame } => {
                self.r#move(Vec2::new(-4, 0));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveRight { frame } => {
                self.r#move(Vec2::new(4, 0));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveUpLeft { frame } => {
                self.r#move(Vec2::new(-4, -4));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveUpRight { frame } => {
                self.r#move(Vec2::new(4, -4));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveDownLeft { frame } => {
                self.r#move(Vec2::new(-4, 4));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            Animation::MoveDownRight { frame } => {
                self.r#move(Vec2::new(4, 4));
                if frame <= self.anime_frame {
                    self.anime = Animation::None
                }
            }
            _ => self.animating = false,
        }
    }
}

impl Updater for Hero {
    fn update(&mut self, gm: &GameManager) {
        let im = INPUT_MANAGER.lock().unwrap();

        if !self.animating {
            self.input_proc(gm, &*im);
        }
        self.animate();
    }
}

impl Drawer for Hero {
    fn draw(&self) {
        let pos = self.position + *DRAW_OFFSET.lock().unwrap();
        draw_rect(
            Rect::new(pos.x, pos.y, 32, 32),
            Color::new(0x00, 0x00, 0xff, 0xff),
        );
    }
}

impl GameObject for Hero {
    fn get_position(&self) -> Vec2<isize> {
        self.position
    }

    fn as_any(&self) -> &Any {
        self
    }
}
