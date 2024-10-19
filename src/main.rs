use bevy::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
    app.add_systems(Update, (move_paddle, move_ball, ball_collide));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

#[derive(Component)]
struct Wall {
    top: f32,
    bottom: f32,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(700., 500.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(300., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyK,
            move_down: KeyCode::KeyJ,
        },
    ));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 100. * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(-250. + 75., 250. - 75.);
        }

        if input.pressed(settings.move_down) {
            pos.translation.y -= 100. * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(-250. + 75., 250. - 75.);
        }
    }
}

#[derive(Component)]
struct Ball {
    speed: f32,
    direction: Vec2,
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-100., 0., 1.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(25., 25.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball {
            speed: 250.,
            direction: Vec2::new(-1., 0.),
        }
    ));
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut balls {
        let velocity = Vec3::new(ball.direction.x * ball.speed, ball.direction.y * ball.speed, 1.);
        pos.translation += velocity * time.delta_seconds();
    }
}

const BWIDTH: f32 = 25.;
const PWIDTH: f32 = 10.;
const PHEIGHT: f32 = 150.;
// Make this not constant...
// Also add a bounce direction....

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        for paddle in &paddles {
            if ball.translation.x - BWIDTH / 2. < paddle.translation.x + PWIDTH / 2.
                && ball.translation.y - BWIDTH / 2. < paddle.translation.y + PHEIGHT / 2.
                && ball.translation.x + BWIDTH / 2. > paddle.translation.x - PWIDTH / 2.
                && ball.translation.y + BWIDTH / 2. > paddle.translation.y - PHEIGHT / 2.
            {
                let paddle_to_ball = ball.translation - paddle.translation;
                let magnitude = (paddle_to_ball.x.powf(2.) + paddle_to_ball.y.powf(2.)).sqrt();
                let new_direction = paddle_to_ball / magnitude;
                velocity.direction = Vec2::new(new_direction.x , new_direction.y / 2.);
            }
        }

        if ball.translation.y >= 250. || ball.translation.y <= -250. {
            velocity.direction.y *= -1.;
        }
    }
}
