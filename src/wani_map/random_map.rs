//! Struct of random map generator.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::any::Any;

use crate::entry::*;
use crate::game_manager::GameManager;
use crate::wani_core::camera::DRAW_OFFSET;
// use crate::wani_core::color::Color;
use crate::wani_core::rect::Rect;
use crate::wani_core::vector2;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::map_component::MapComponent;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::game_object::GameObject;
use crate::wani_trait::updater::Updater;

type Map = Vec<Vec<MapComponent>>;

/// Size of Map
pub struct MapSize {
    width: usize,
    height: usize,
}

/// Entity of map
pub struct RandomMap {
    pub map: Map,
    draw_map: Vec<Vec<usize>>,
    pub size: MapSize,
}

impl RandomMap {
    /// Generate random map.
    pub fn new(width: usize, height: usize) -> Self {
        let (w, h) = RandomMap::correct_size(width, height);

        let mut rm = RandomMap {
            map: vec![vec![MapComponent::NONE; w]; h],
            draw_map: vec![vec![23; w]; h],
            size: MapSize {
                width: w,
                height: h,
            },
        };

        rm.build_maze();
        rm.build_room();
        rm.remove_deadend();

        rm.build_draw_map();

        rm
    }

    /// Correct size to the closest 2n + 3 (0 < n) size.
    fn correct_size(width: usize, height: usize) -> (usize, usize) {
        let mut w = width / 2;
        w = if w > 0 { w } else { 1 };
        w = w * 2 + 3;

        let mut h = height / 2;
        h = if h > 0 { h } else { 1 };
        h = h * 2 + 3;

        (w, h)
    }

    /// Initialize step 1: Build a maze by stretching the wall.
    fn build_maze(&mut self) {
        self.build_outerwall();

        let mut posts = self.make_post();

        'post: while posts.len() > 0 {
            // Create base point for wall stretching
            let post_index_start = urandom(0..posts.len());
            let post_start = &posts[post_index_start];

            match self.get_component(*post_start).unwrap() {
                MapComponent::WALL => {
                    posts.remove(post_index_start);
                    continue;
                }
                MapComponent::NONE => {
                    let mut cursor = *post_start;
                    let mut wall_candidacy = Vec::new();
                    wall_candidacy.push(cursor);

                    'grow: loop {
                        match self.get_component(cursor).unwrap() {
                            MapComponent::NONE => {
                                let mut direction =
                                    vec![vector2::UP, vector2::DOWN, vector2::LEFT, vector2::RIGHT];

                                'dir: while direction.len() > 0 {
                                    // Decide direction to stretch the wall
                                    let rand = urandom(0..direction.len());

                                    let dir = direction[rand];
                                    direction.remove(rand);

                                    let cursor_next = cursor + dir * 2;

                                    // Check if next cursor is already candidates for the wall
                                    for v in &wall_candidacy {
                                        if *v == cursor_next {
                                            // Redetermine the direction
                                            continue 'dir;
                                        }
                                    }

                                    // Stretch the wall
                                    wall_candidacy.push(cursor + dir);
                                    wall_candidacy.push(cursor_next);
                                    cursor = cursor_next;

                                    continue 'grow;
                                }

                                // If the next cursor is only wall candidate,
                                // rewind the cursor to the end on consecutive wall candidates
                                let mut wall_prev = wall_candidacy[0];
                                let mut index_end = 0;
                                for i in 0..wall_candidacy.len() {
                                    if wall_candidacy[i].dist(wall_prev) <= 1.0 {
                                        wall_prev = wall_candidacy[i];
                                        index_end = i;
                                    }
                                }
                                cursor = wall_candidacy[index_end - 2];
                                // Leave the end of wall candidate as a dummy candidate
                                wall_candidacy.remove(index_end - 1);
                            }
                            MapComponent::WALL => {
                                // Build walls on consecutive wall candidates
                                let mut wall_prev = wall_candidacy[0];
                                for v in &wall_candidacy {
                                    if v.dist(wall_prev) <= 1.0 {
                                        self.map[v.y as usize][v.x as usize] = MapComponent::WALL;
                                        wall_prev = *v;
                                    }
                                }
                                wall_candidacy.clear();

                                continue 'post;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Build the outer wall.
    fn build_outerwall(&mut self) {
        let w = self.size.width;
        let h = self.size.height;

        for i in 0..w {
            for j in 0..h {
                if i == 0 || i == w - 1 || j == 0 || j == h - 1 {
                    self.map[j][i] = MapComponent::WALL;
                }
            }
        }
    }

    /// Make posts and return coordinate of posts.
    fn make_post(&self) -> Vec<Vec2> {
        let w = self.size.width;
        let h = self.size.height;

        let mut posts = Vec::new();
        for i in 0..(w - 3) / 2 {
            for j in 0..(h - 3) / 2 {
                posts.push(Vec2::new((i as isize + 1) * 2, (j as isize + 1) * 2));
            }
        }

        posts
    }

    /// Initialize step 2: Build room.
    fn build_room(&mut self) {
        let count_room = random(5..10);

        for _ in 0..count_room {
            let retry = random(5..7);

            'retry: for _ in 0..retry {
                let w = random(8..16) as usize / 2 * 2 + 1;
                let h = random(8..16) as usize / 2 * 2 + 1;
                let x = random(1..(self.size.width - w - 1) as isize) as usize / 2 * 2 + 1;
                let y = random(1..(self.size.height - h - 1) as isize) as usize / 2 * 2 + 1;

                // Check if the room is not already
                for i in y..y + h {
                    for j in x..x + w {
                        match self.map[i][j] {
                            MapComponent::ROOM => {
                                continue 'retry;
                            }
                            _ => {}
                        }
                    }
                }

                // Build the room
                for i in y..y + h {
                    for j in x..x + w {
                        self.map[i][j] = MapComponent::ROOM;
                    }
                }
                break;
            }
        }
    }

    /// Initialize step 3: Remove dead end.
    fn remove_deadend(&mut self) {
        let w = self.size.width;
        let h = self.size.height;
        let mut count_road = vec![vec![-1; w]; h];

        // Count the number of branches
        for i in 1..h - 1 {
            for j in 1..w - 1 {
                match self.map[i][j] {
                    MapComponent::NONE => {
                        let mut count = 0;
                        match self.map[i - 1][j] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        match self.map[i + 1][j] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        match self.map[i][j - 1] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        match self.map[i][j + 1] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        count_road[i][j] = count;
                    }
                    _ => {}
                }
            }
        }

        // Remove dead end
        for i in 1..h - 1 {
            for j in 1..w - 1 {
                if count_road[i][j] == 1 {
                    let mut k = i;
                    let mut l = j;

                    loop {
                        self.map[k][l] = MapComponent::WALL;
                        count_road[k][l] -= 1;
                        count_road[k - 1][l] -= 1; // up
                        count_road[k + 1][l] -= 1; // down
                        count_road[k][l - 1] -= 1; // left
                        count_road[k][l + 1] -= 1; // right

                        // Track dead end
                        if count_road[k - 1][l] == 1 {
                            k -= 1;
                        } else if count_road[k + 1][l] == 1 {
                            k += 1;
                        } else if count_road[k][l - 1] == 1 {
                            l -= 1;
                        } else if count_road[k][l + 1] == 1 {
                            l += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Build draw map.
    fn build_draw_map(&mut self) {
        let w = self.size.width;
        let h = self.size.height;

        let mut wall_flag = vec![vec![0u8; w]; h];

        // 8 direction
        let u = vector2::UP;
        let d = vector2::DOWN;
        let l = vector2::LEFT;
        let r = vector2::RIGHT;

        fn is_wall(rm: &RandomMap, coord: Vec2) -> bool {
            match rm.get_component(coord) {
                Some(comp) => match comp {
                    MapComponent::WALL => true,
                    _ => false,
                },
                None => true,
            }
        }

        // Flag if there is a wall
        for i in 0..w {
            for j in 0..h {
                let mut flag = 0u8;
                let target = Vec2::new(i as isize, j as isize);

                flag |= if is_wall(self, target + u + l) {
                    0b10000000
                } else {
                    0
                };
                flag |= if is_wall(self, target + u) {
                    0b01000000
                } else {
                    0
                };
                flag |= if is_wall(self, target + u + r) {
                    0b00100000
                } else {
                    0
                };
                flag |= if is_wall(self, target + l) {
                    0b00010000
                } else {
                    0
                };
                flag |= if is_wall(self, target + r) {
                    0b00001000
                } else {
                    0
                };
                flag |= if is_wall(self, target + d + l) {
                    0b00000100
                } else {
                    0
                };
                flag |= if is_wall(self, target + d) {
                    0b00000010
                } else {
                    0
                };
                flag |= if is_wall(self, target + d + r) {
                    0b00000001
                } else {
                    0
                };

                wall_flag[j][i] = flag;
            }
        }

        fn check_flag(chk: u8, flag: u8) -> bool {
            chk & flag == flag
        }

        // Pattern match with flag and image
        for i in 0..w {
            for j in 0..h {
                match self.map[j][i] {
                    MapComponent::WALL => {
                        let flag = wall_flag[j][i];

                        let mut cross_wall_count = 0;
                        cross_wall_count += (flag & 0b01000000) >> 6;
                        cross_wall_count += (flag & 0b00010000) >> 4;
                        cross_wall_count += (flag & 0b00001000) >> 3;
                        cross_wall_count += (flag & 0b00000010) >> 1;
                        let mut slant_wall_count = 0;
                        slant_wall_count += (flag & 0b10000000) >> 7;
                        slant_wall_count += (flag & 0b00100000) >> 5;
                        slant_wall_count += (flag & 0b00000100) >> 2;
                        slant_wall_count += (flag & 0b00000001) >> 0;

                        match cross_wall_count {
                            0 => self.draw_map[j][i] = 22,
                            1 => {
                                if check_flag(flag, 0b01000000) {
                                    self.draw_map[j][i] = 20
                                } else if check_flag(flag, 0b00010000) {
                                    self.draw_map[j][i] = 13
                                } else if check_flag(flag, 0b00001000) {
                                    self.draw_map[j][i] = 11
                                } else if check_flag(flag, 0b00000010) {
                                    self.draw_map[j][i] = 4
                                }
                            }
                            2 => {
                                if check_flag(flag, 0b01000010) {
                                    self.draw_map[j][i] = 6
                                } else if check_flag(flag, 0b00011000) {
                                    self.draw_map[j][i] = 7
                                } else if check_flag(flag, 0b01010000) {
                                    if check_flag(flag, 0b10000000) {
                                        self.draw_map[j][i] = 18
                                    } else {
                                        self.draw_map[j][i] = 21
                                    }
                                } else if check_flag(flag, 0b01001000) {
                                    if check_flag(flag, 0b00100000) {
                                        self.draw_map[j][i] = 16
                                    } else {
                                        self.draw_map[j][i] = 19
                                    }
                                } else if check_flag(flag, 0b00010010) {
                                    if check_flag(flag, 0b00000100) {
                                        self.draw_map[j][i] = 2
                                    } else {
                                        self.draw_map[j][i] = 5
                                    }
                                } else if check_flag(flag, 0b00001010) {
                                    if check_flag(flag, 0b00000001) {
                                        self.draw_map[j][i] = 0
                                    } else {
                                        self.draw_map[j][i] = 3
                                    }
                                }
                            }
                            3 => {
                                if check_flag(flag, 0b01011000) {
                                    if check_flag(flag, 0b10100000) {
                                        self.draw_map[j][i] = 17
                                    } else if check_flag(flag, 0b10000000) {
                                        self.draw_map[j][i] = 36
                                    } else if check_flag(flag, 0b00100000) {
                                        self.draw_map[j][i] = 34
                                    } else {
                                        self.draw_map[j][i] = 32
                                    }
                                }
                                if check_flag(flag, 0b01001010) {
                                    if check_flag(flag, 0b00100001) {
                                        self.draw_map[j][i] = 8
                                    } else if check_flag(flag, 0b00100000) {
                                        self.draw_map[j][i] = 28
                                    } else if check_flag(flag, 0b00000001) {
                                        self.draw_map[j][i] = 26
                                    } else {
                                        self.draw_map[j][i] = 24
                                    }
                                }
                                if check_flag(flag, 0b00011010) {
                                    if check_flag(flag, 0b00000101) {
                                        self.draw_map[j][i] = 1
                                    } else if check_flag(flag, 0b00000001) {
                                        self.draw_map[j][i] = 29
                                    } else if check_flag(flag, 0b00000100) {
                                        self.draw_map[j][i] = 27
                                    } else {
                                        self.draw_map[j][i] = 25
                                    }
                                }
                                if check_flag(flag, 0b01010010) {
                                    if check_flag(flag, 0b10000100) {
                                        self.draw_map[j][i] = 10
                                    } else if check_flag(flag, 0b00000100) {
                                        self.draw_map[j][i] = 37
                                    } else if check_flag(flag, 0b10000000) {
                                        self.draw_map[j][i] = 35
                                    } else {
                                        self.draw_map[j][i] = 33
                                    }
                                }
                            }
                            4 => match slant_wall_count {
                                0 => self.draw_map[j][i] = 12,
                                1 => {
                                    if check_flag(flag, 0b10000000) {
                                        self.draw_map[j][i] = 39
                                    }
                                    if check_flag(flag, 0b00100000) {
                                        self.draw_map[j][i] = 38
                                    }
                                    if check_flag(flag, 0b00000001) {
                                        self.draw_map[j][i] = 30
                                    }
                                    if check_flag(flag, 0b00100100) {
                                        self.draw_map[j][i] = 31
                                    }
                                }
                                2 => {
                                    if check_flag(flag, 0b10000001) {
                                        self.draw_map[j][i] = 15
                                    }
                                    if check_flag(flag, 0b00100100) {
                                        self.draw_map[j][i] = 14
                                    }

                                    if check_flag(flag, 0b10100000) {
                                        self.draw_map[j][i] = 49
                                    }
                                    if check_flag(flag, 0b00100001) {
                                        self.draw_map[j][i] = 48
                                    }
                                    if check_flag(flag, 0b00000101) {
                                        self.draw_map[j][i] = 40
                                    }
                                    if check_flag(flag, 0b10000100) {
                                        self.draw_map[j][i] = 41
                                    }
                                }
                                3 => {
                                    if !check_flag(flag, 0b10000000) {
                                        self.draw_map[j][i] = 42
                                    }
                                    if !check_flag(flag, 0b00100000) {
                                        self.draw_map[j][i] = 43
                                    }
                                    if !check_flag(flag, 0b00000001) {
                                        self.draw_map[j][i] = 51
                                    }
                                    if !check_flag(flag, 0b00000100) {
                                        self.draw_map[j][i] = 50
                                    }
                                }
                                4 => self.draw_map[j][i] = 9,
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Return map component of given coordinates.
    pub fn get_component(&self, coord: Vec2) -> Option<MapComponent> {
        let mut comp = None;
        if Rect::new(0, 0, self.map[0].len() - 1, self.map.len() - 1).contains(coord) {
            comp = Some(self.map[coord.y as usize][coord.x as usize])
        }
        comp
    }

    /// Return random coordinate of room.
    pub fn respawnable_coord(&self) -> Vec2 {
        let w = self.size.width;
        let h = self.size.height;
        loop {
            let rand_pos = Vec2::new(random(0..w as isize), random(0..h as isize));
            match self.get_component(rand_pos).unwrap() {
                MapComponent::ROOM => return rand_pos * 32,
                _ => {}
            }
        }
    }
}

impl Updater for RandomMap {
    fn update(&mut self, _gm: &GameManager) {}
}

impl Drawer for RandomMap {
    fn draw(&self) {
        let screen = SCREEN_SIZE.lock().unwrap();

        let os = *DRAW_OFFSET.lock().unwrap();

        let mut rect = Rect::new(os.x, os.y, 32, 32);
        let x_slide = Vec2::new(32, 0);
        let y_slide = Vec2::new(0, 32);

        for i in &self.draw_map {
            for j in i {
                if screen.contains_rect(rect) {
                    draw_image(
                        0,
                        Rect::new((j % 8 * 32) as isize, (j / 8 * 32) as isize, 32, 32),
                        rect,
                    );
                }
                rect.slide(&x_slide);
            }
            rect.x = os.x;
            rect.slide(&y_slide);
        }
    }
}

impl GameObject for RandomMap {
    fn as_any(&self) -> &Any {
        self
    }
}
