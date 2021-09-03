use bevy::{
    prelude::*,
    diagnostic::{
        Diagnostics,
        FrameTimeDiagnosticsPlugin,
    },
    math::DVec3,
};
use crate::{
    debug::components::{
        FpsDebugText,
        PlayerDebugText,
    },
    physics::components::{
        Drag,
        Impulse,
        ImpulsiveTorque,
        PhysTransform,
        Thrust,
    },
    user_interaction::components::{
        Player,
    },
};

/// A SystemSet that overlays debugging information onto the screen.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(update_player_debug_text.system()
        )
        .with_system(update_fps_text.system()
        )
}

/// A system to be run at startup that performs necessary setup. Overlays text onto the screen that
/// will be later updated/populated with data.
pub fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Bold.ttf");
    commands.spawn_bundle(UiCameraBundle::default());

    // -- TOP LEFT: PLAYER INFORMATION.

    let mut player_sections = vec![];

    for _ in 0..10 {
        player_sections.push(TextSection {
            value: "".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::WHITE,
            },
        })
    }

    // odd-numbered sections will be updated with the corresponding data.
    player_sections[0].value = "PLAYER DATA\n\nposition = ".to_string();
    player_sections[2].value = "\nthrust = ".to_string();
    player_sections[4].value = "\ndrag = ".to_string();
    player_sections[6].value = "\nimpulse = ".to_string();
    player_sections[8].value = "\nimpulsive_torque = ".to_string();

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: player_sections,
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(PlayerDebugText);

    // -- BOTTOM RIGHT: FPS.

    let mut fps_sections = vec![];

    for _ in 0..4 {
        fps_sections.push(TextSection {
            value: "".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::GREEN,
            },
        })
    }

    // even-numbered sections will be updated with the corresponding data.
    fps_sections[1].value = " fps, ".to_string();
    fps_sections[3].value = " ms/frame".to_string();

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: fps_sections,
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(FpsDebugText);
}

/// Populates the player section of text with relevant up-to-date values.
fn update_player_debug_text(
    mut query: Query<&mut Text, With<PlayerDebugText>>,
    player: Query<(&Drag, &PhysTransform, &Thrust), With<Player>>,
    player_optional: Query<(&Impulse, &ImpulsiveTorque), With<Player>>,
) {
    for mut text in query.iter_mut() {
        let mut player_drag = DVec3::ZERO;
        let mut player_impulse = DVec3::ZERO;
        let mut player_impulsive_torque = DVec3::ZERO;
        let mut player_position = DVec3::ZERO;
        let mut player_thrust = DVec3::ZERO;

        if let Ok((drag, transform, thrust)) = player.single() {
            player_drag = drag.vector();
            player_position = transform.translation();
            player_thrust = thrust.vector();
        }
        if let Ok((impulse, impulsive_torque)) = player_optional.single() {
            player_impulse = impulse.0;
            player_impulsive_torque = impulsive_torque.0;
        };

        text.sections[1].value = format!("[{:.2}, {:.2}, {:.2}]",
                                         player_position.x, player_position.y, player_position.z);

        text.sections[3].value = format!("[{:.2}, {:.2}, {:.2}]",
                                         player_thrust.x, player_thrust.y, player_thrust.z);

        text.sections[5].value = format!("[{:.2}, {:.2}, {:.2}]",
                                         player_drag.x, player_drag.y, player_drag.z);

        text.sections[7].value = format!("[{:.2}, {:.2}, {:.2}]",
                                         player_impulse.x, player_impulse.y, player_impulse.z);

        text.sections[9].value = format!("[{:.2}, {:.2}, {:.2}]",
                                         player_impulsive_torque.x, player_impulsive_torque.y, player_impulsive_torque.z);
    }
}

/// Populates the fps section of text with relevant up-to-date values.
fn update_fps_text(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsDebugText>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.sections[0].value = format!("{:5.1}", fps);

        text.sections[2].value = format!("{:7.3}", frame_time * 1000.0);
    }
}
