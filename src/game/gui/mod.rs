//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::{
    app::AppExit,
    ui::RelativeCursorPosition
};

mod binding;
pub use binding::*;
mod button;
pub use button::*;
mod context;
pub use context::*;
mod quickslots;
pub use quickslots::*;
mod worldspace;
pub use worldspace::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BindingGuiPlugin)
            .add_plugin(ButtonGuiPlugin)
            .add_plugin(ContextGuiPlugin)
            .add_plugin(QuickslotsGuiPlugin)
            .add_plugin(WorldspaceGuiPlugin);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
#[derive(Bundle)]
pub struct GuiTextBundle {
    text_bundle: TextBundle,
}

impl GuiTextBundle {
    pub fn new(
        text: &str,
        font: &str,
        font_size: f32,
        font_color: &Color,
        asset_loader: &Res<AssetLoader>,
    ) -> Self {
        Self {
            text_bundle: TextBundle::from_section(text, TextStyle {
                    font: asset_loader.fonts.get_handle(font),
                    font_size: font_size,
                    color: *font_color,
                }),
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
pub fn spawn_button(
    child_builder: &mut ChildBuilder,
    asset_loader: &Res<AssetLoader>,
    button_text: &str,
    font: &str,
) -> Entity {
    child_builder.spawn(ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    bottom: Val::Percent(5.0),
                    ..default()
                },
                size: Size::new(Val::Percent(30.0), Val::Percent(10.0)),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(RelativeCursorPosition::default())
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle {
                text: Text::from_section(button_text.to_string(), TextStyle {
                    font: asset_loader.fonts.get_handle(font),
                    font_size: 32.0,
                    color: Color::WHITE,
                }),
                ..default()
            });
        })
        .id()
}
// GUI Test
// commands.spawn(NodeBundle {
//         style: Style {
//             size: Size::all(Val::Percent(100.0)),
//             ..default()
//         },
//         ..default()
//     })
//     .with_children(|child_builder| {
//         spawn_button(child_builder, &asset_loader, ButtonFn::NextState(AppState::Gameplay), "Gameplay");
//         spawn_button(child_builder, &asset_loader, ButtonFn::NextState(AppState::MainMenu), "MainMenu");

//         child_builder.spawn(TextBundle::from_section("TEST TEXT", TextStyle {
//             font: asset_loader.fonts.get_handle("rock_salt/regular"),
//             font_size: 32.0,
//             color: Color::BEIGE,
//         }));

//         child_builder.spawn(ImageBundle {
//             style: Style {
//                 size: Size::height(Val::Px(256.0)),
//                 ..default()
//             },
//             image: asset_loader.images.get_handle("dough").into(),
//             ..default()
//         });
//     })
//     .insert(Name::new("GUI Test"));