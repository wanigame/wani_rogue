//! Struct of random map generator.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use super::super::entry::*;
use super::super::wani_core::vector2::Vec2;

use super::map_component::MapComponent;

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

        map = RandomMap::build_maze(map);

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
        let w = map[0].len();
        let h = map.len();

        // Build Wall
        for i in 0..w {
            for j in 0..h {
                if i == 0 || i == w - 1 || j == 0 || j == h - 1 {
                    map[j][i] = MapComponent::WALL;
                }
            }
        }

        // Create Post
        let mut posts = Vec::new();
        for i in 0..((w - 3) / 2) {
            for j in 0..((h - 3) / 2) {
                posts.push(Vec2::new((i as isize + 1) * 2, (j as isize + 1) * 2));
            }
        }

        'post: while posts.len() > 0 {
            let post_index_start = random(0..posts.len() as isize) as usize;
            let post_start = &posts[post_index_start];

            match RandomMap::_get_component(&map, post_start) {
                MapComponent::WALL => {
                    posts.remove(post_index_start);
                    continue;
                }
                MapComponent::NONE => {
                    use std::collections::VecDeque;

                    let mut cursor = *post_start;
                    let mut wall_candidacy = VecDeque::new();
                    wall_candidacy.push_back(cursor);
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
                                    let rand = random(0..direction.len() as isize) as usize;

                                    let dir = direction[rand];
                                    direction.remove(rand);

                                    let cursor_next = cursor + dir * 2;
                                    for v in &wall_candidacy {
                                        if *v == cursor_next {
                                            continue 'dir;
                                        }
                                    }

                                    wall_candidacy.push_back(cursor + dir);
                                    wall_candidacy.push_back(cursor_next);
                                    cursor = cursor_next;

                                    continue 'grow;
                                }

                                let cursor_prev = wall_candidacy.pop_back();
                                wall_candidacy.pop_back();
                                wall_candidacy.push_back(cursor_prev.unwrap());
                            }
                            MapComponent::WALL => {
                                let mut wall_prev = *post_start;
                                for v in &wall_candidacy {
                                    if (*v - wall_prev).len() <= 1.0 {
                                        map[v.y as usize][v.x as usize] = MapComponent::WALL;
                                        wall_prev = *v;
                                    }
                                }
                                wall_candidacy.clear();

                                continue 'post;
                            }
                        }
                    }
                }
            }
        }

        map
    }

    pub fn get_component(self, coord: &Vec2<isize>) -> MapComponent {
        self.map[coord.y as usize][coord.x as usize]
    }
    fn _get_component(map: &Map, coord: &Vec2<isize>) -> MapComponent {
        map[coord.y as usize][coord.x as usize]
    }
}
