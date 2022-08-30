mod common;
use common::*;
use drawer::gadgets::gauge;
use tui::layout::Rect;

struct ExampleGaugeAppdata;

impl gauge::GaugeAppData for ExampleGaugeAppdata {
    fn is_gauge_active(&self) -> bool {
        false
    }
    fn music_total_duration(&self) -> String {
        "05:35".to_string()
    }
    fn played_music_duration(&self) -> String {
        "03:54".to_string()
    }
    fn music_title(&self) -> String {
        "Bimbakash (Maya Jastai) - From album Bimbakash - Bartika Eam Rai".to_string()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    entrypoint(draw_gauge)
}

fn draw_gauge<B: Backend>(f: &mut Frame<B>) {
    let theme = get_default_theme();
    let gauge = gauge::get_gauge(&ExampleGaugeAppdata, &theme);
    let place = Rect {
        x: 0,
        y: f.size().height - 3,
        height: 3,
        width: f.size().width,
    };

    f.render_widget(gauge, place);
}
