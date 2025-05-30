use gpui::{
    div, linear_color_stop, linear_gradient, prelude::FluentBuilder, px, rgb, rgba, App,
    AppContext, Context, Entity, FocusHandle, Focusable, Hsla, IntoElement, ParentElement, Render,
    SharedString, Styled, Window,
};
use gpui_component::{
    chart::{AreaChart, BarChart, ChartDelegate, LineChart, PieChart, PieChartDelegate},
    divider::Divider,
    dock::PanelControl,
    h_flex, v_flex, ActiveTheme, StyledExt,
};

#[derive(Clone)]
struct DataItem {
    month: &'static str,
    desktop: f64,
    color: u32,
}

const CHART_DATA: &[DataItem] = &[
    DataItem {
        month: "January",
        desktop: 186.,
        color: 0x2a9d90,
    },
    DataItem {
        month: "February",
        desktop: 305.,
        color: 0xe76e50,
    },
    DataItem {
        month: "March",
        desktop: 237.,
        color: 0x274754,
    },
    DataItem {
        month: "April",
        desktop: 73.,
        color: 0xe8c468,
    },
    DataItem {
        month: "May",
        desktop: 209.,
        color: 0xf4a462,
    },
    DataItem {
        month: "June",
        desktop: 214.,
        color: 0x2563eb,
    },
];

impl ChartDelegate<&'static str, f64> for DataItem {
    fn x(&self) -> &'static str {
        self.month
    }

    fn y(&self) -> f64 {
        self.desktop
    }
}

impl PieChartDelegate for DataItem {
    fn value(&self) -> Option<f64> {
        Some(self.desktop)
    }

    fn color(&self) -> impl Into<Hsla> {
        rgb(self.color)
    }
}

#[derive(Clone)]
struct DataItem2 {
    date: &'static str,
    desktop: f64,
    #[allow(dead_code)]
    mobile: f64,
}

const CHART_DATA_2: &[DataItem2] = &[
    DataItem2 {
        date: "Apr 1",
        desktop: 222.,
        mobile: 150.,
    },
    DataItem2 {
        date: "Apr 2",
        desktop: 97.,
        mobile: 180.,
    },
    DataItem2 {
        date: "Apr 3",
        desktop: 167.,
        mobile: 120.,
    },
    DataItem2 {
        date: "Apr 4",
        desktop: 242.,
        mobile: 260.,
    },
    DataItem2 {
        date: "Apr 5",
        desktop: 373.,
        mobile: 290.,
    },
    DataItem2 {
        date: "Apr 6",
        desktop: 301.,
        mobile: 340.,
    },
    DataItem2 {
        date: "Apr 7",
        desktop: 245.,
        mobile: 180.,
    },
    DataItem2 {
        date: "Apr 8",
        desktop: 409.,
        mobile: 320.,
    },
    DataItem2 {
        date: "Apr 9",
        desktop: 59.,
        mobile: 110.,
    },
    DataItem2 {
        date: "Apr 10",
        desktop: 261.,
        mobile: 190.,
    },
    DataItem2 {
        date: "Apr 11",
        desktop: 327.,
        mobile: 350.,
    },
    DataItem2 {
        date: "Apr 12",
        desktop: 292.,
        mobile: 210.,
    },
    DataItem2 {
        date: "Apr 13",
        desktop: 342.,
        mobile: 380.,
    },
    DataItem2 {
        date: "Apr 14",
        desktop: 137.,
        mobile: 220.,
    },
    DataItem2 {
        date: "Apr 15",
        desktop: 120.,
        mobile: 170.,
    },
    DataItem2 {
        date: "Apr 16",
        desktop: 138.,
        mobile: 190.,
    },
    DataItem2 {
        date: "Apr 17",
        desktop: 446.,
        mobile: 360.,
    },
    DataItem2 {
        date: "Apr 18",
        desktop: 364.,
        mobile: 410.,
    },
    DataItem2 {
        date: "Apr 19",
        desktop: 243.,
        mobile: 180.,
    },
    DataItem2 {
        date: "Apr 20",
        desktop: 89.,
        mobile: 150.,
    },
    DataItem2 {
        date: "Apr 21",
        desktop: 137.,
        mobile: 200.,
    },
    DataItem2 {
        date: "Apr 22",
        desktop: 224.,
        mobile: 170.,
    },
    DataItem2 {
        date: "Apr 23",
        desktop: 138.,
        mobile: 230.,
    },
    DataItem2 {
        date: "Apr 24",
        desktop: 387.,
        mobile: 290.,
    },
    DataItem2 {
        date: "Apr 25",
        desktop: 215.,
        mobile: 250.,
    },
    DataItem2 {
        date: "Apr 26",
        desktop: 75.,
        mobile: 130.,
    },
    DataItem2 {
        date: "Apr 27",
        desktop: 383.,
        mobile: 420.,
    },
    DataItem2 {
        date: "Apr 28",
        desktop: 122.,
        mobile: 180.,
    },
    DataItem2 {
        date: "Apr 29",
        desktop: 315.,
        mobile: 240.,
    },
    DataItem2 {
        date: "Apr 30",
        desktop: 454.,
        mobile: 380.,
    },
    DataItem2 {
        date: "May 1",
        desktop: 165.,
        mobile: 220.,
    },
    DataItem2 {
        date: "May 2",
        desktop: 293.,
        mobile: 310.,
    },
    DataItem2 {
        date: "May 3",
        desktop: 247.,
        mobile: 190.,
    },
    DataItem2 {
        date: "May 4",
        desktop: 385.,
        mobile: 420.,
    },
    DataItem2 {
        date: "May 5",
        desktop: 481.,
        mobile: 390.,
    },
    DataItem2 {
        date: "May 6",
        desktop: 498.,
        mobile: 520.,
    },
    DataItem2 {
        date: "May 7",
        desktop: 388.,
        mobile: 300.,
    },
    DataItem2 {
        date: "May 8",
        desktop: 149.,
        mobile: 210.,
    },
    DataItem2 {
        date: "May 9",
        desktop: 227.,
        mobile: 180.,
    },
    DataItem2 {
        date: "May 10",
        desktop: 293.,
        mobile: 330.,
    },
    DataItem2 {
        date: "May 11",
        desktop: 335.,
        mobile: 270.,
    },
    DataItem2 {
        date: "May 12",
        desktop: 197.,
        mobile: 240.,
    },
    DataItem2 {
        date: "May 13",
        desktop: 197.,
        mobile: 160.,
    },
    DataItem2 {
        date: "May 14",
        desktop: 448.,
        mobile: 490.,
    },
    DataItem2 {
        date: "May 15",
        desktop: 473.,
        mobile: 380.,
    },
    DataItem2 {
        date: "May 16",
        desktop: 338.,
        mobile: 400.,
    },
    DataItem2 {
        date: "May 17",
        desktop: 499.,
        mobile: 420.,
    },
    DataItem2 {
        date: "May 18",
        desktop: 315.,
        mobile: 350.,
    },
    DataItem2 {
        date: "May 19",
        desktop: 235.,
        mobile: 180.,
    },
    DataItem2 {
        date: "May 20",
        desktop: 177.,
        mobile: 230.,
    },
    DataItem2 {
        date: "May 21",
        desktop: 82.,
        mobile: 140.,
    },
    DataItem2 {
        date: "May 22",
        desktop: 81.,
        mobile: 120.,
    },
    DataItem2 {
        date: "May 23",
        desktop: 252.,
        mobile: 290.,
    },
    DataItem2 {
        date: "May 24",
        desktop: 294.,
        mobile: 220.,
    },
    DataItem2 {
        date: "May 25",
        desktop: 201.,
        mobile: 250.,
    },
    DataItem2 {
        date: "May 26",
        desktop: 213.,
        mobile: 170.,
    },
    DataItem2 {
        date: "May 27",
        desktop: 420.,
        mobile: 460.,
    },
    DataItem2 {
        date: "May 28",
        desktop: 233.,
        mobile: 190.,
    },
    DataItem2 {
        date: "May 29",
        desktop: 78.,
        mobile: 130.,
    },
    DataItem2 {
        date: "May 30",
        desktop: 340.,
        mobile: 280.,
    },
    DataItem2 {
        date: "May 31",
        desktop: 178.,
        mobile: 230.,
    },
    DataItem2 {
        date: "Jun 1",
        desktop: 178.,
        mobile: 200.,
    },
    DataItem2 {
        date: "Jun 2",
        desktop: 470.,
        mobile: 410.,
    },
    DataItem2 {
        date: "Jun 3",
        desktop: 103.,
        mobile: 160.,
    },
    DataItem2 {
        date: "Jun 4",
        desktop: 439.,
        mobile: 380.,
    },
    DataItem2 {
        date: "Jun 5",
        desktop: 88.,
        mobile: 140.,
    },
    DataItem2 {
        date: "Jun 6",
        desktop: 294.,
        mobile: 250.,
    },
    DataItem2 {
        date: "Jun 7",
        desktop: 323.,
        mobile: 370.,
    },
    DataItem2 {
        date: "Jun 8",
        desktop: 385.,
        mobile: 320.,
    },
    DataItem2 {
        date: "Jun 9",
        desktop: 438.,
        mobile: 480.,
    },
    DataItem2 {
        date: "Jun 10",
        desktop: 155.,
        mobile: 200.,
    },
    DataItem2 {
        date: "Jun 11",
        desktop: 92.,
        mobile: 150.,
    },
    DataItem2 {
        date: "Jun 12",
        desktop: 492.,
        mobile: 420.,
    },
    DataItem2 {
        date: "Jun 13",
        desktop: 81.,
        mobile: 130.,
    },
    DataItem2 {
        date: "Jun 14",
        desktop: 426.,
        mobile: 380.,
    },
    DataItem2 {
        date: "Jun 15",
        desktop: 307.,
        mobile: 350.,
    },
    DataItem2 {
        date: "Jun 16",
        desktop: 371.,
        mobile: 310.,
    },
    DataItem2 {
        date: "Jun 17",
        desktop: 475.,
        mobile: 520.,
    },
    DataItem2 {
        date: "Jun 18",
        desktop: 107.,
        mobile: 170.,
    },
    DataItem2 {
        date: "Jun 19",
        desktop: 341.,
        mobile: 290.,
    },
    DataItem2 {
        date: "Jun 20",
        desktop: 408.,
        mobile: 450.,
    },
    DataItem2 {
        date: "Jun 21",
        desktop: 169.,
        mobile: 210.,
    },
    DataItem2 {
        date: "Jun 22",
        desktop: 317.,
        mobile: 270.,
    },
    DataItem2 {
        date: "Jun 23",
        desktop: 480.,
        mobile: 530.,
    },
    DataItem2 {
        date: "Jun 24",
        desktop: 132.,
        mobile: 180.,
    },
    DataItem2 {
        date: "Jun 25",
        desktop: 141.,
        mobile: 190.,
    },
    DataItem2 {
        date: "Jun 26",
        desktop: 434.,
        mobile: 380.,
    },
    DataItem2 {
        date: "Jun 27",
        desktop: 448.,
        mobile: 490.,
    },
    DataItem2 {
        date: "Jun 28",
        desktop: 149.,
        mobile: 200.,
    },
    DataItem2 {
        date: "Jun 29",
        desktop: 103.,
        mobile: 160.,
    },
    DataItem2 {
        date: "Jun 30",
        desktop: 446.,
        mobile: 400.,
    },
];

impl ChartDelegate<&'static str, f64> for DataItem2 {
    fn x(&self) -> &'static str {
        self.date
    }

    fn y(&self) -> f64 {
        self.desktop
    }

    fn label(&self) -> SharedString {
        self.date.into()
    }
}

pub struct PlotStory {
    focus_handle: FocusHandle,
}

impl PlotStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl super::Story for PlotStory {
    fn title() -> &'static str {
        "Chart"
    }

    fn description() -> &'static str {
        "A low-level approach to data analysis and visualization."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelControl> {
        None
    }
}

impl Focusable for PlotStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn chart_container(
    title: &str,
    chart: impl IntoElement,
    center: bool,
    cx: &mut Context<PlotStory>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_lg()
        .p_4()
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .child(title.to_string()),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("January-June 2025"),
        )
        .child(div().flex_1().py_4().child(chart))
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .text_sm()
                .child("Trending up by 5.2% this month"),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("Showing total visitors for the last 6 months"),
        )
}

impl Render for PlotStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_y_4()
            .bg(cx.theme().background)
            .child(
                div().h(px(400.)).child(chart_container(
                    "Area Chart - Stacked",
                    AreaChart::new(CHART_DATA_2.to_vec())
                        .fill(linear_gradient(
                            0.,
                            linear_color_stop(rgba(0x2563eb66), 1.),
                            linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                        ))
                        .tick_margin(8),
                    false,
                    cx,
                )),
            )
            .child(
                h_flex()
                    .gap_x_8()
                    .h(px(400.))
                    .child(chart_container(
                        "Area Chart",
                        AreaChart::new(CHART_DATA.to_vec()),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Area Chart - Linear",
                        AreaChart::new(CHART_DATA.to_vec()).linear(),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Area Chart - Linear Gradient",
                        AreaChart::new(CHART_DATA.to_vec()).fill(linear_gradient(
                            0.,
                            linear_color_stop(rgba(0x2563eb66), 1.),
                            linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                        )),
                        false,
                        cx,
                    )),
            )
            .child(Divider::horizontal().my_6())
            .child(
                h_flex()
                    .gap_x_8()
                    .h(px(400.))
                    .child(chart_container(
                        "Line Chart",
                        LineChart::new(CHART_DATA.to_vec()),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Line Chart - Linear",
                        LineChart::new(CHART_DATA.to_vec()).linear(),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Line Chart - Dots",
                        LineChart::new(CHART_DATA.to_vec()).point(),
                        false,
                        cx,
                    )),
            )
            .child(Divider::horizontal().my_6())
            .child(
                h_flex()
                    .gap_x_8()
                    .h(px(400.))
                    .child(chart_container(
                        "Bar Chart",
                        BarChart::new(CHART_DATA.to_vec()),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Bar Chart",
                        BarChart::new(CHART_DATA.to_vec()),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Bar Chart",
                        BarChart::new(CHART_DATA.to_vec()),
                        false,
                        cx,
                    )),
            )
            .child(Divider::horizontal().my_6())
            .child(
                h_flex()
                    .gap_x_8()
                    .h(px(450.))
                    .child(chart_container(
                        "Pie Chart",
                        PieChart::new(CHART_DATA.to_vec()).outer_radius(100.),
                        true,
                        cx,
                    ))
                    .child(chart_container(
                        "Pie Chart - Donut",
                        PieChart::new(CHART_DATA.to_vec())
                            .outer_radius(100.)
                            .inner_radius(60.),
                        true,
                        cx,
                    ))
                    .child(chart_container(
                        "Pie Chart - Pad Angle",
                        PieChart::new(CHART_DATA.to_vec())
                            .outer_radius(100.)
                            .inner_radius(60.)
                            .pad_angle(4. / 100.),
                        true,
                        cx,
                    )),
            )
    }
}
