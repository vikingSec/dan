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
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: NORMAL_BUTTON.into(),
            transform: Transform::from_xyz(15.0,15.0,15.0),
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
            mut node_query: Query<&mut Style, With<Button>>,
            mut cmd : Commands
        ){

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
    
        //TODO: Here we need to create some sort of collision system. That shit seems hard lol
    }
}
