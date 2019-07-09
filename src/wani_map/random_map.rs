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
use crate::wani_core::color::Color;
use crate::wani_core::rect::Rect;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::map_component::MapComponent;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::game_object::GameObject;
use crate::wani_trait::updater::Updater;

type Map = Vec<Vec<MapComponent>>;

/// Entity of map
pub struct RandomMap {
    pub map: Map,
}

impl RandomMap {
    /// Generate random map.
    pub fn new(width: usize, height: usize) -> Self {
        let (w, h) = RandomMap::correct_size(width, height);

        let mut map = vec![vec![MapComponent::NONE; w]; h];

        log("map_init");
        map = RandomMap::build_maze(map);
        log("build_maze");
        map = RandomMap::build_room(map);
        log("build_rooom");
        map = RandomMap::remove_deadend(map);
        log("remove_deadend");

        RandomMap { map }
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
    fn build_maze(mut map: Map) -> Map {
        map = RandomMap::build_outerwall(map);

        let mut posts = RandomMap::make_post(&map);

        'post: while posts.len() > 0 {
            // Create base point for wall stretching
            let post_index_start = random(0..posts.len() as isize) as usize;
            let post_start = &posts[post_index_start];

            match RandomMap::_get_component(&map, post_start) {
                MapComponent::WALL => {
                    posts.remove(post_index_start);
                    continue;
                }
                MapComponent::NONE => {
                    let mut cursor = *post_start;
                    let mut wall_candidacy = Vec::new();
                    wall_candidacy.push(cursor);

                    'grow: loop {
                        match RandomMap::_get_component(&map, &cursor) {
                            MapComponent::NONE => {
                                let mut direction = vec![
                                    Vec2::new(0, -1), // UP
                                    Vec2::new(0, 1),  // DOWN
                                    Vec2::new(-1, 0), // LEFT
                                    Vec2::new(1, 0),  // RIGHT
                                ];

                                'dir: while direction.len() > 0 {
                                    // Decide direction to stretch the wall
                                    let rand = random(0..direction.len() as isize) as usize;

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
                                    if (wall_candidacy[i] - wall_prev).len() <= 1.0 {
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
                                    if (*v - wall_prev).len() <= 1.0 {
                                        map[v.y as usize][v.x as usize] = MapComponent::WALL;
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

        map
    }

    fn build_outerwall(mut map: Map) -> Map {
        let w = map[0].len();
        let h = map.len();

        for i in 0..w {
            for j in 0..h {
                if i == 0 || i == w - 1 || j == 0 || j == h - 1 {
                    map[j][i] = MapComponent::WALL;
                }
            }
        }

        map
    }

    fn make_post(map: &Map) -> Vec<Vec2<isize>> {
        let w = map[0].len();
        let h = map.len();

        let mut posts = Vec::new();
        for i in 0..((w - 3) / 2) {
            for j in 0..((h - 3) / 2) {
                posts.push(Vec2::new((i as isize + 1) * 2, (j as isize + 1) * 2));
            }
        }

        posts
    }

    /// Initialize step 2: Build room.
    fn build_room(mut map: Map) -> Map {
        let count_room = random(5..10);

        for _ in 0..count_room {
            let retry = random(5..7);

            'retry: for _ in 0..retry {
                let w = random(8..16) as usize / 2 * 2 + 1;
                let h = random(8..16) as usize / 2 * 2 + 1;
                let x = random(1..(map[0].len() - w - 1) as isize) as usize / 2 * 2 + 1;
                let y = random(1..(map.len() - h - 1) as isize) as usize / 2 * 2 + 1;

                // Check if the room is not already
                for i in y..y + h {
                    for j in x..x + w {
                        match map[i][j] {
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
                        map[i][j] = MapComponent::ROOM;
                    }
                }
                break;
            }
        }

        map
    }

    /// Initialize step 3: Remove dead end.
    pub fn remove_deadend(mut map: Map) -> Map {
        let w = map[0].len();
        let h = map.len();
        let mut count_road = vec![vec![-1; w]; h];

        // Count the number of branches
        for i in 1..h - 1 {
            for j in 1..w - 1 {
                match map[i][j] {
                    MapComponent::NONE => {
                        let mut count = 0;
                        match map[i - 1][j] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        match map[i + 1][j] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        match map[i][j - 1] {
                            MapComponent::NONE => count += 1,
                            MapComponent::ROOM => count += 1,
                            _ => {}
                        }
                        match map[i][j + 1] {
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
                        map[k][l] = MapComponent::WALL;
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

        map
    }

    pub fn get_component(&self, coord: &Vec2<isize>) -> Option<MapComponent> {
        let mut comp = None;
        if Rect::new(0, 0, self.map[0].len() - 1, self.map.len() - 1).contains(coord) {
            comp = Some(self.map[coord.y as usize][coord.x as usize])
        }
        comp
    }
    fn _get_component(map: &Map, coord: &Vec2<isize>) -> MapComponent {
        map[coord.y as usize][coord.x as usize]
    }

    pub fn respawnable_coord(&self) -> Vec2<isize> {
        let w = self.map[0].len();
        let h = self.map.len();
        loop {
            let rand_pos = Vec2::new(random(0..w as isize), random(0..h as isize));
            match RandomMap::_get_component(&self.map, &rand_pos) {
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
        let os = *DRAW_OFFSET.lock().unwrap();

        let mut rect = Rect::new(os.x, os.y, 32, 32);
        let mut color;
        let x_slide = Vec2::new(32, 0);
        let y_slide = Vec2::new(0, 32);

        for i in &self.map {
            for j in i {
                match j {
                    MapComponent::WALL => color = Color::new(0x5f, 0x5f, 0x5f, 0xff),
                    MapComponent::NONE => color = Color::new(0xff, 0xff, 0xff, 0xff),
                    MapComponent::ROOM => color = Color::new(0xff, 0xff, 0xff, 0xff),
                }
                draw_rect(rect, color);
                rect.slide(&x_slide);
            }
            rect.x = os.x;
            rect.slide(&y_slide);
        }
    }
}

impl GameObject for RandomMap {
    fn get_position(&self) -> Vec2<isize> {
        Vec2::new(0, 0)
    }

    fn as_any(&self) -> &Any {
        self
    }
}
