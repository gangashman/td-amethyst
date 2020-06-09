use amethyst::{
    ui::{UiText, Anchor, UiTransform, TtfFormat},
    prelude::{World, WorldExt, Builder},
    assets::Loader,
};

pub fn create_menu(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf", TtfFormat, (), &world.read_resource(),
    );

    let ui_text = UiText::new(font, "123".to_string(), [1.0, 1.0, 1.0, 1.0], 25.0);
    let ui_transform = UiTransform::new(
        "activate".to_string(),
        Anchor::TopRight,
        Anchor::Middle,
        -200.0, -100.0, 200.0, 50.0, 50.0,
    );

    world
        .create_entity()
        .with(ui_text)
        .with(ui_transform)
        .build();
}
