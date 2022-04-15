use bevy::prelude::*;

use crate::global_types::{AppState, ScoreStatus};
use crate::loading::FontAssets;

pub struct ScoreDisplayPlugin;

impl Plugin for ScoreDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreStatus::default());
        app.add_startup_system(setup_score_display);
        app.add_system(update_score_display);
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_time));
    }
}

#[derive(Component)]
struct ScoreDisplayText;

fn setup_score_display(mut commands: Commands, font_assets: Res<FontAssets>) {
    let mut cmd = commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(0.0), Val::Px(90.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            position: Rect {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::YELLOW.into(),
        ..Default::default()
    });
    let text_style = TextStyle {
        font: font_assets.fira_sans.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };
    cmd.with_children(|commands| {
        let mut cmd = commands.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Time: ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\n".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "Logs Chipped: ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\n".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "Wood Chips Cleared: ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Left,
                },
            },
            ..Default::default()
        });
        cmd.insert(ScoreDisplayText);
    });
}

fn update_time(time: Res<Time>, mut score_status: ResMut<ScoreStatus>) {
    score_status.time += time.delta();
}

fn update_score_display(
    mut query: Query<&mut Text, With<ScoreDisplayText>>,
    score_status: Res<ScoreStatus>,
) {
    for mut score_text in query.iter_mut() {
        score_text.sections[1].value = score_status.format_time();
        score_text.sections[4].value = score_status.logs_chipped.to_string();
        score_text.sections[7].value = score_status.woodchips_cleared.to_string();
    }
}
