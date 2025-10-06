use bevy::prelude::*;
use std::collections::HashMap;

/// Metadata about trees in the world
#[derive(Resource, Default)]
pub struct TreeRegistry {
    pub trees: HashMap<u32, TreeData>,
    next_id: u32,
}

impl TreeRegistry {
    pub fn register_tree(&mut self, x: i32, y: i32, width: i32, height: i32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.trees.insert(id, TreeData {
            id,
            root_x: x,
            root_y: y,
            width,
            height,
            log_split_rules: LogSplitRules {
                min_log_length: 20,
                max_log_length: 60,
                min_log_width: 10,
                max_log_width: 20,
                split_randomness: 0.3,
            },
        });

        id
    }

    pub fn remove_tree(&mut self, id: u32) {
        self.trees.remove(&id);
    }

    pub fn get_tree_at(&self, x: i32, y: i32) -> Option<&TreeData> {
        self.trees.values().find(|tree| {
            x >= tree.root_x
                && x < tree.root_x + tree.width
                && y >= tree.root_y
                && y < tree.root_y + tree.height
        })
    }
}

#[derive(Debug, Clone)]
pub struct TreeData {
    pub id: u32,
    pub root_x: i32,
    pub root_y: i32,
    pub width: i32,
    pub height: i32,
    pub log_split_rules: LogSplitRules,
}

#[derive(Debug, Clone)]
pub struct LogSplitRules {
    pub min_log_length: i32,
    pub max_log_length: i32,
    pub min_log_width: i32,
    pub max_log_width: i32,
    pub split_randomness: f32,
}

pub struct TreeMetadataPlugin;

impl Plugin for TreeMetadataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeRegistry>();
    }
}
