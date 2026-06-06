use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

#[component]
pub fn Slider(
    #[prop(into)] value: MaybeSignal<f64>,
    #[prop(into)] on_change: Callback<f64>,
    #[prop(into, default = 0.0.into())] min: MaybeSignal<f64>,
    #[prop(into, default = 100.0.into())] max: MaybeSignal<f64>,
    #[prop(into, default = 1.0.into())] step: MaybeSignal<f64>,
    #[prop(into)] orientation: MaybeSignal<SliderOrientation>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] show_value: MaybeSignal<bool>,
    #[prop(into, default = false.into())] reverse: MaybeSignal<bool>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] on_start: Option<Callback<(), ()>>,
    #[prop(into, default = None.into())] on_end: Option<Callback<f64, ()>>,
) -> impl IntoView {
    let percentage = create_memo(move |_| {
        let min_val = min.get();
        let max_val = max.get();
        let val = value.get();
        if max_val <= min_val {
            return 0.0;
        }
        ((val - min_val) / (max_val - min_val) * 100.0).clamp(0.0, 100.0)
    });
    let orientation_class = move || match orientation.get() {
        SliderOrientation::Horizontal => "",
        SliderOrientation::Vertical => "weui-slider--vertical",
    };
    let handle_input = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(val) = target.value().parse::<f64>() {
            leptos::Callable::call(&on_change, val);
        }
    };
    let handle_mousedown = move |_: ev::MouseEvent| {
        if let Some(cb) = &on_start {
            leptos::Callable::call(cb, ());
        }
    };
    let handle_mouseup = move |_: ev::MouseEvent| {
        if let Some(cb) = &on_end {
            leptos::Callable::call(cb, value.get());
        }
    };
    view! {
        <div
            class=move || format!("weui-slider {} {}", orientation_class(), class.get())
            class=("weui-slider--disabled", move || disabled.get())
            class=("weui-slider--reverse", move || reverse.get())
            role="slider"
            aria-valuemin=move || min.get()
            aria-valuemax=move || max.get()
            aria-valuenow=move || value.get()
            aria-disabled=move || disabled.get().to_string()
        >
            <div class="weui-slider__track">
                <div
                    class="weui-slider__track-fill"
                    style=move || {
                        if orientation.get() == SliderOrientation::Vertical {
                            format!("height: {}%", percentage.get())
                        } else {
                            format!("width: {}%", percentage.get())
                        }
                    }
                />
            </div>
            <div
                class="weui-slider__thumb"
                style=move || {
                    if orientation.get() == SliderOrientation::Vertical {
                        format!("bottom: {}%", percentage.get())
                    } else {
                        format!("left: {}%", percentage.get())
                    }
                }
                on:mousedown=handle_mousedown
                on:mouseup=handle_mouseup
            />
            <input
                type="range"
                class="weui-slider__input"
                min=move || min.get()
                max=move || max.get()
                step=move || step.get()
                prop:value=value
                disabled=disabled
                on:input=handle_input
            />
            <Show when=move || show_value.get()>
                <span class="weui-slider__value">{value}</span>
            </Show>
        </div>
    }
}
