use leptos::*;

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
    #[prop(into)] on_change: Callback<Vec<UploadFile>>,
    #[prop(into, default = 1.into())] max_count: MaybeSignal<i32>,
    #[prop(into, default = 0.into())] max_size: MaybeSignal<u64>,
    #[prop(into)] size: MaybeSignal<UploaderSize>,
    #[prop(into, default = false.into())] disabled: MaybeSignal<bool>,
    #[prop(into, default = false.into())] multiple: MaybeSignal<bool>,
    #[prop(into, default = "".into())] accept: MaybeSignal<String>,
    #[prop(into, default = "".into())] class: MaybeSignal<String>,
    #[prop(into, default = None.into())] before_read: Option<Callback<UploadFile, bool>>,
    #[prop(into, default = None.into())] after_read: Option<Callback<Vec<UploadFile>>>,
    #[prop(into, default = None.into())] on_oversize: Option<Callback<Vec<UploadFile>>>,
    #[prop(into, default = None.into())] on_delete: Option<Callback<UploadFile>>,
    #[prop(into, default = None.into())] on_success: Option<Callback<UploadFile>>,
    #[prop(into, default = None.into())] on_error: Option<Callback<UploadFile>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();
    let can_upload = create_memo(move |_| {
        let count = file_list.get().len() as i32;
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
    let handle_change = move |ev: ev::Event| {
        if let Some(cb) = &after_read {
            cb.call(file_list.get());
        }
    };
    let handle_delete = move |file: UploadFile| {
        if let Some(cb) = &on_delete {
            cb.call(file);
        }
    };
    view! {
        <div
            class=move || format!("weui-uploader {} {}", size_class(), class.get())
            class:weui-uploader--disabled=move || disabled.get()
        >
            <div class="weui-uploader__files">
                {move || {
                    file_list
                        .get()
                        .into_iter()
                        .map(|file| {
                            let file_clone = file.clone();
                            view! {
                                <div
                                    class="weui-uploader__file"
                                    class:weui-uploader__file--uploading=move || matches!(file.status, UploaderStatus::Uploading)
                                    class:weui-uploader__file--success=move || matches!(file.status, UploaderStatus::Success)
                                    class:weui-uploader__file--failed=move || matches!(file.status, UploaderStatus::Failed)
                                >
                                    {move || {
                                        if let Some(ref url) = file.url {
                                            view! { <img class="weui-uploader__img" src=url alt=file.name.clone()/> }
                                        } else {
                                            view! { <div class="weui-uploader__file-icon"/> }
                                        }
                                    }}
                                    <span class="weui-uploader__name">{file.name}</span>
                                    {move || {
                                        if matches!(file.status, UploaderStatus::Uploading) {
                                            view! {
                                                <div class="weui-uploader__progress">
                                                    <div
                                                        class="weui-uploader__progress-bar"
                                                        style=move || format!("width: {}%", file.progress)
                                                    />
                                                </div>
                                            }
                                        } else {
                                            view! {}
                                        }
                                    }}
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
            {move || {
                if can_upload.get() {
                    view! {
                        <div class="weui-uploader__input-wrapper" on:click=handle_click>
                            <div class="weui-uploader__input">
                                <span class="weui-uploader__input-icon"/>
                            </div>
                            <input
                                node_ref=input_ref
                                type="file"
                                class="weui-uploader__input-native"
                                accept=accept
                                multiple=multiple
                                disabled=disabled
                                on:change=handle_change
                            />
                        </div>
                    }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}
