use leptos::*;
use leptos::Show;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UploaderStatus {
    Ready,
    Uploading,
    Success,
    Failed,
}

#[derive(Clone)]
pub struct UploadFile {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub status: UploaderStatus,
    pub progress: f64,
    pub size: u64,
    pub mime_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UploaderSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Uploader(
    #[prop(into)] file_list: MaybeSignal<Vec<UploadFile>>,
    #[prop(into)] _on_change: Callback<Vec<UploadFile>>,
    #[prop(into, default = 1.into())] max_count: MaybeSignal<i32>,
    #[prop(into, default = 0.into())] _max_size: MaybeSignal<u64>,
    #[prop(into)] size: MaybeSignal<UploaderSize>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] multiple: MaybeSignal<bool>,
    #[prop(into, default = "".into())] accept: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] _before_read: Option<Callback<UploadFile, bool>>,
    #[prop(into, default = None.into())] after_read: Option<Callback<Vec<UploadFile>, ()>>,
    #[prop(into, default = None.into())] _on_oversize: Option<Callback<Vec<UploadFile>, ()>>,
    #[prop(into, default = None.into())] on_delete: Option<Callback<UploadFile, ()>>,
    #[prop(into, default = None.into())] _on_success: Option<Callback<UploadFile, ()>>,
    #[prop(into, default = None.into())] _on_error: Option<Callback<UploadFile, ()>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();
    let file_list_clone_for_memo = file_list.clone();
    let can_upload = create_memo(move |_| {
        let count = file_list_clone_for_memo.get().len() as i32;
        count < max_count.get() && !disabled.get()
    });
    let size_class = move || match size.get() {
        UploaderSize::Sm => "weui-uploader--sm",
        UploaderSize::Md => "",
        UploaderSize::Lg => "weui-uploader--lg",
    };
    let handle_click = move |_: ev::MouseEvent| {
        if can_upload.get() {
            input_ref.get().map(|el| el.click());
        }
    };
    let after_read_clone = std::rc::Rc::new(after_read.clone());
    let file_list_clone2 = std::rc::Rc::new(file_list.clone());
    let handle_change = move |_ev: ev::Event| {
        let maybe_cb = after_read_clone.clone();
        if let Some(cb) = maybe_cb.as_ref() {
            let value = file_list_clone2.clone().get();
            leptos::Callable::call(cb, value);
        }
    };
    let handle_delete = move |file: UploadFile| {
        if let Some(cb) = &on_delete {
            leptos::Callable::call(cb, file);
        }
    };
    let class_clone = class.clone();
    let file_list_clone = file_list.clone();
    let accept_text = create_memo(move |_| accept.get());
    let multiple_text = create_memo(move |_| multiple.get());
    let disabled_text = create_memo(move |_| disabled.get());
    view! {
        <div
            class=move || format!("weui-uploader {} {}", size_class(), class_clone.get())
            class=("weui-uploader--disabled", move || disabled.get())
        >
            <div class="weui-uploader__files">
                {move || {
                    file_list_clone
                        .get()
                        .into_iter()
                        .map(|file| {
                            let file_clone = file.clone();
                            let file_url = file.url.clone();
                            let file_url_has = file_url.clone();
                            let file_url_src = std::rc::Rc::new(file_url.clone().unwrap_or_default());
                            let file_url_none = file_url.clone();
                            let file_name = file.name.clone();
                            let file_name_alt = std::rc::Rc::new(file_name.clone());
                            let file_name_span = file_name.clone();
                            let file_url_src_closure = {
                                let file_url_src = file_url_src.clone();
                                move || file_url_src.as_ref().clone()
                            };
                            let file_name_alt_closure = {
                                let file_name_alt = file_name_alt.clone();
                                move || file_name_alt.as_ref().clone()
                            };
                            let file_status = file.status;
                            let file_progress = file.progress;
                            view! {
                                <div
                                    class="weui-uploader__file"
                                    class=("weui-uploader__file--uploading", matches!(file_status, UploaderStatus::Uploading))
                                    class=("weui-uploader__file--success", matches!(file_status, UploaderStatus::Success))
                                    class=("weui-uploader__file--failed", matches!(file_status, UploaderStatus::Failed))
                                >
                                    <Show when=move || file_url_has.is_some()>
                                        <img class="weui-uploader__img" src=file_url_src_closure.clone() alt=file_name_alt_closure.clone()/>
                                    </Show>
                                    <Show when=move || file_url_none.is_none()>
                                        <div class="weui-uploader__file-icon"/>
                                    </Show>
                                    <span class="weui-uploader__name">{file_name_span.clone()}</span>
                                    <Show when=move || matches!(file_status, UploaderStatus::Uploading)>
                                        <div class="weui-uploader__progress">
                                            <div
                                                class="weui-uploader__progress-bar"
                                                style=format!("width: {}%", file_progress)
                                            />
                                        </div>
                                    </Show>
                                    <button
                                        class="weui-uploader__delete"
                                        on:click=move |_| handle_delete(file_clone.clone())
                                        type="button"
                                        aria-label="Delete file"
                                    />
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
            <Show when=move || can_upload.get()>
                <div class="weui-uploader__input-wrapper" on:click=handle_click.clone()>
                    <div class="weui-uploader__input">
                        <span class="weui-uploader__input-icon"/>
                    </div>
                    <input
                        node_ref=input_ref
                        type="file"
                        class="weui-uploader__input-native"
                        accept=move || accept_text.get()
                        multiple=move || multiple_text.get()
                        disabled=move || disabled_text.get()
                        on:change=handle_change.clone()
                    />
                </div>
            </Show>
        </div>
    }
}
