use gpui::{App, KeyBinding, Menu, MenuItem, actions};

actions!(nb, [Quit]);

pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    cx.on_action(|_: &Quit, cx| cx.quit());

    cx.set_menus(vec![Menu {
        name: "nb".into(),
        items: vec![MenuItem::action("Quit nb", Quit)],
    }]);
}
