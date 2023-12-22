// TODO

use bevy::{prelude::*, ecs::system::Command};




pub struct Node;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
impl Node {
    pub fn SpawnNode(cmd : &mut Commands){
        cmd.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(150.0),
                ..Default::default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        });


    }

}
