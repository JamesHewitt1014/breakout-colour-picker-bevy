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

// TODO: Move paddle and ball into their own files? 
#[derive(Component)]
struct Paddle {
    move_right: KeyCode,
    move_left: KeyCode,
    height: f32,
    width: f32,
}

#[derive(Component)]
struct Wall {
    top: f32,
    bottom: f32,
}

fn spawn_players(mut commands: Commands) {
    //PLAY AREA
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
            transform: Transform::from_translation(Vec3::new(0., -225., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(150., 10.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_right: KeyCode::KeyD,
            move_left:  KeyCode::KeyA,
            height:     10.,
            width:      150.
        },
    ));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_right) {
            pos.translation.x += 100. * time.delta_seconds();
            pos.translation.x = pos.translation.x.clamp(-350. + 75., 350. - 75.);
        }

        if input.pressed(settings.move_left) {
            pos.translation.x -= 100. * time.delta_seconds();
            pos.translation.x = pos.translation.x.clamp(-350. + 75., 350. - 75.);
        }
    }
}

#[derive(Component)]
struct Ball {
    speed: f32,
    direction: Vec2,
}

#[derive(Component)]
struct Collider {
    height: f32,
    width: f32,
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(25., 25.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball {
            speed: 250.,
            direction: Vec2::new(0., -1.),
        },
        Collider {
            width: 25.,
            height: 25.
        },
    ));
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut balls {
        let velocity = Vec3::new(
            ball.direction.x * ball.speed,
            ball.direction.y * ball.speed,
            1.,
        );
        pos.translation += velocity * time.delta_seconds();
    }
}

fn ball_collide(
    mut balls: Query<(&Transform, &Collider, &mut Ball)>,
    paddles: Query<(&Transform, &Paddle)>,
) {
    for (ball, ball_collider, mut velocity) in &mut balls {
        for (paddle, paddle_collider) in &paddles {
            if     ball.translation.x - ball_collider.width  / 2. < paddle.translation.x + paddle_collider.width / 2.
                && ball.translation.y - ball_collider.height / 2. < paddle.translation.y + paddle_collider.height / 2.
                && ball.translation.x + ball_collider.width  / 2. > paddle.translation.x - paddle_collider.width / 2.
                && ball.translation.y + ball_collider.height / 2. > paddle.translation.y - paddle_collider.height / 2.
            {
                let direction_paddle_to_ball = ball.translation - paddle.translation;
                let magnitude = (direction_paddle_to_ball.x.powf(2.) + direction_paddle_to_ball.y.powf(2.)).sqrt();
                let new_direction = direction_paddle_to_ball / magnitude;
                velocity.direction = Vec2::new(new_direction.x, new_direction.y / 2.);
            }
        }

        //Collide with walls
        if ball.translation.x >= 350. - ball_collider.width / 2. || ball.translation.x <= -350. + ball_collider.width / 2. {
            velocity.direction.x *= -1.;
        }
        if ball.translation.y >= 250. - ball_collider.height / 2. {
            velocity.direction.y *= -1.;
        }
    }
}
