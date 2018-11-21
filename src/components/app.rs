use conrod::{widget::Polygon, widget, Widget, Labelable, Positionable, Sizeable, color, Colorable, Borderable};
use std::iter::once;

widget_ids!(struct Ids {
    canvas,
    button,
    polygon,
});

pub struct State {
    ids: Ids,
    clicks: usize,
}

#[derive(WidgetCommon)]
pub struct App {
    #[conrod(common_builder)] common: widget::CommonBuilder,
}

impl App {
    pub fn new() -> Self {
        App {
            common: widget::CommonBuilder::default(),
        }
    }
}

impl Widget for App {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            clicks: 0,
        }
    }

    fn style(&self) -> Self::Style {}

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            state,
            ui,
            id,
            ..
        } = args;
        
        // The apps background
        widget::Canvas::new()
            .parent(id)
            .color(color::DARK_CHARCOAL)
            .border(0.0)
            .wh_of(id)
            .set(state.ids.canvas, ui);


        // For performance reasons this string should be cached in the state
        // (it can be created 60 times per second) but I want to keep this simple
        let label = format!("Clicked {} times", state.clicks);

        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .top_left_of(state.ids.canvas)
            .w_h(200.0, 100.0)
            .label(&label)
            .set(state.ids.button, ui)
        {
            state.update(|state| state.clicks += 1);
        }

        let bl = [0.0, -40.0];
        let tl = [-20.0, 40.0];
        let tr = [20.0, 40.0];
        let br = [40.0, -40.0];
        let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
        Polygon::abs_fill_with(points, color::Color::Rgba(1.0, 0.0, 1.0, 1.0))
            .middle_of(state.ids.canvas)
            .set(state.ids.polygon, ui);
    }
}