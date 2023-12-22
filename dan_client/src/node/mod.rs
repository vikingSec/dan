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
    pub fn NodeSystem(
        
        mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            ),
            (Changed<Interaction>, With<Button>),
            >,
            mut text_query: Query<&mut Text>,
            mut cmd : Commands
        ){

        // TODO
        for (interaction, mut color) in &mut interaction_query {
            println!("Interaction loop");
            match *interaction {
                Interaction::Pressed => {
                    println!("Pressed the node!");
                    *color = BackgroundColor(Color::rgb(0.9, 0.9, 0.9));
                },
                Interaction::Hovered => {
                    println!("Hovered!");
                },
                Interaction::None => {
                    println!("None!");
                }
            }
        }
    }
}
