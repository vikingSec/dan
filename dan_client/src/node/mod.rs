use bevy::input::mouse::MouseButtonInput;
// TODO
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, ecs::system::Command};
use bevy::sprite::Anchor;



pub struct Node;
#[derive(Component)]
pub struct SizeXY {
    x: f32,
    y: f32
}


#[derive(Component)]
pub enum Type {
    Node
}
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
impl Node {
    pub fn SpawnNode(cmd : &mut Commands, asset_server : &Res<AssetServer>){
        cmd.spawn((SpriteBundle {
            texture: asset_server.load("icon.png"), 
            transform: Transform::from_xyz(100., 0., 0.),
            

            ..Default::default()
        },
        Type::Node,
        ));
        println!("Spawned!");

    }
    pub fn NodeSystem(
        
        interaction_query: Query<
        (
            &Interaction,
            &mut Transform,
            &mut Type
            ),
            (Changed<Interaction>, With<Sprite>),
            >,
            mut cmd : Commands,
        ){
        for (interaction, mut trans, mut ty) in &interaction_query {
            match *ty {
                Type::Node => {
                    println!("Interaction loop");
                    match *interaction {
                        Interaction::Pressed => {
                            println!("Pressed the node!");
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
        //TODO: Here we need to create some sort of collision system. That shit seems hard lo

    }
    pub fn MoveSystem(time: Res<Time>, mut node_query: Query<(&mut Transform, &mut Type)>){
        for (mut transform, mut ty) in &mut node_query {
            match *ty {
                Type::Node => transform.translation.x += 15. * time.delta_seconds(),
            }
        }
    }
    pub fn ClickSprite(mut sprites: Query<(&Transform, &Handle<Image>), With<Sprite>>,
    assets: Res<Assets<Image>>,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>)
    {
        let window = windows.single();
        let (camera, position) = cameras.single();
        for (transform, image_handle) in &mut sprites {
            let image_size = assets
                .get(image_handle)
                .unwrap_or(&Image {..Default::default()})
                .size_f32();

            let scaled = image_size * transform.scale.truncate();
            let bounding_box = Rect::from_center_size(
                transform.translation.truncate(),
                scaled
                );
            if buttons.just_pressed(MouseButton::Left){ 
                if let Some(world_position) = window.cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(position, cursor))
                        .map(|ray| ray.origin.truncate())
                        {
                            if(world_position.x < bounding_box.max[0] &&
                               world_position.x > bounding_box.min[0] &&
                               world_position.y < bounding_box.max[1] &&
                               world_position.y > bounding_box.min[1])
                            {
                                info!("Clicked the sprite!!!");
                            }
                        }
            }
        }

    }
}


