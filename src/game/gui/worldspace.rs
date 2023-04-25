//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct WorldspaceGuiPlugin;
impl Plugin for WorldspaceGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sys_update_gui_follow);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct WorldspaceFollow(Entity, Vec2);
impl WorldspaceFollow {
    pub fn new(target: Entity, offset: Vec2) -> Self {
        Self { 0: target, 1: offset }
    }

    pub fn centered(target: Entity) -> Self {
        Self { 0: target, 1: Vec2::ZERO }
    }
}

#[derive(Component)]
pub struct WorldspaceScale(Val, Val);
impl WorldspaceScale {
    pub fn new(width: Val, height: Val) -> Self {
        Self { 0: width, 1: height }
    }

    pub fn all(val: Val) -> Self {
        Self { 0: val, 1: val }
    }
}

#[derive(Component)]
pub struct WorldspaceTextScale(f32);
impl WorldspaceTextScale {
    pub fn new(font_size: f32) -> Self {
        Self { 0: font_size }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_gui_follow(
    mut commands: Commands,
    mut follow_query: Query<(Entity, &WorldspaceFollow, &mut Style, &mut Visibility)>,
    // mut text_scale_query: Query<(&mut Text, &WorldspaceTextScale), With<WorldspaceFollow>>,
    gui_scale_query: Query<&WorldspaceScale, With<WorldspaceFollow>>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    transforms_query: Query<&GlobalTransform>,
    settings: Res<Settings>,
) {
    let (camera_transform, camera) = camera_query.single();
    let viewport_size = camera.logical_viewport_size().unwrap();

    for (follow_entity, follow, mut style, mut visibility) in follow_query.iter_mut() {
        if let Ok(target_transform) = transforms_query.get(follow.0) {
            if let Some(screen_pos) = camera.world_to_viewport(camera_transform, target_transform.translation()) {
                *visibility = Visibility::Visible;
                let distance = camera_transform.translation().distance(target_transform.translation());
                let scale_mult = ((settings.max_zoom + 5.0) - distance) / (settings.max_zoom + 5.0);
                // if distance >= 5.0 { scale *=  }

                if let Ok(scale) = gui_scale_query.get(follow_entity) {
                    let width = match scale.0 {
                            Val::Px(px) => { px * scale_mult }
                            Val::Percent(percent) => { ((percent * 0.01) * viewport_size.x) * scale_mult }
                            _ => { 0.0 }
                        };
                    let height = match scale.1 {
                            Val::Px(px) => { px * scale_mult }
                            Val::Percent(percent) => { ((percent * 0.01) * viewport_size.y) * scale_mult }
                            _ => { 0.0 }
                        };

                    style.size.width = Val::Px(width);
                    style.size.height = Val::Px(height);
                    style.position.left = Val::Px(screen_pos.x - (width * 0.5) + (follow.1.x * scale_mult));
                    style.position.bottom = Val::Px(screen_pos.y - (height * 0.5) + (follow.1.y * scale_mult));
                }
                
                // if let Ok((mut text, text_scale)) = text_scale_query.get_mut(follow_entity) {
                //     text.sections[0].style.font_size = text_scale.0 * scale_mult;
                // }
            } else {
                *visibility = Visibility::Hidden;
            }
        } else {
            commands.entity(follow_entity).despawn_recursive();
        }

    }
}