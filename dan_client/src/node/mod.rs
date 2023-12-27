use bevy::input::mouse::MouseButtonInput;
// TODO
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, ecs::system::Command};
use bevy::sprite::Anchor;
use std::time;
use bevy::sprite::collide_aabb::collide;


pub struct Node;

#[derive(Component)]
pub struct Meta {
    Name: String,
    Type: NodeType,
    Id: String
}
#[derive(Component)]
pub enum NodeType {
    Node
}
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);


// Private utility functions
fn InBoundingBox(cursor_pos : Vec2, bounding_box : Rect) -> bool {
    if(cursor_pos.x < bounding_box.max[0] &&
       cursor_pos.x > bounding_box.min[0] &&
       cursor_pos.y < bounding_box.max[1] &&
       cursor_pos.y > bounding_box.min[1])
    {
        return true;
    }
    return false;


}

fn getId() -> String {
    let start = time::SystemTime::now();

    let since_the_epoch = start
        .duration_since(time::UNIX_EPOCH)
        .expect("Shit happened...");
    return format!("{:?}", since_the_epoch.as_millis());
}

impl Node {
    pub fn SpawnNode(cmd : &mut Commands, asset_server : &Res<AssetServer>){
        let id = getId();
        let name = format!("node_gen_{}", id);
        cmd.spawn((SpriteBundle {
            texture: asset_server.load("icon.png"), 
            transform: Transform::from_xyz(100., 0., 0.),
            

            ..Default::default()
        },
        Meta {
            Name:  name.to_string(),
            Type: NodeType::Node, 
            Id: getId()
        },
        ));
        println!("Spawned!");

    }
    pub fn NodeSystem(
        
        interaction_query: Query<
        (
            &Interaction,
            &mut Transform,
            &mut Meta
            ),
            (Changed<Interaction>, With<Sprite>),
            >,
            mut cmd : Commands,
        ){
        for (interaction, mut trans, mut meta) in &interaction_query {
            match meta.Type {
                NodeType::Node => {
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
    pub fn ClickSprite(mut sprites: Query<(&Transform, &Handle<Image>, &Meta), With<Sprite>>,
    assets: Res<Assets<Image>>,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>)
    {
        let window = windows.single();
        
        let (camera, position) = cameras.single();
        for (transform, image_handle, meta) in &mut sprites {
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
                            if(InBoundingBox(world_position, bounding_box))
                            {
                                info!("Clicked the sprite with ID {} and name {}!!", meta.Id, meta.Name);
                            }
                        }
            }
        }

    }

    pub fn HandleCollisions(mut spriteschange: Query<(&mut Transform, &Meta), With<Sprite>>)
    {
        let mut combos = spriteschange.iter_combinations_mut();
        while let Some([(mut trans1, mut meta1), (mut trans2, mut meta2)]) = combos.fetch_next() {
            if(meta1.Id != meta2.Id){
                let collision = collide(
                        trans1.translation,
                        trans1.scale.truncate(),
                        trans2.translation,
                        trans2.scale.truncate()
                    );
                if let Some(collision) = collision {
                    trans1.translation.x += 256.;
                    trans1.translation.y += 256.;
                    println!("There was a collision!");
                }
            }

        }

    }
}


