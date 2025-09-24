use serde::Deserialize;
use tauri::{command, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[derive(Debug, Deserialize)]
pub struct WindowOptions{
    label: String, // 必须有 label
    url: String,
    #[serde(default)]
    title: Option<String>, // 可选字段
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
    #[serde(default)]
    x: Option<f64>,
    #[serde(default)]
    y: Option<f64>,
    #[serde(default)]
    resizable: Option<bool>,
    #[serde(default)]
    decorations: Option<bool>,
    #[serde(default)]
    shadow: Option<bool>,
    #[serde(default)]
    fullscreen: Option<bool>,
    #[serde(default)]
    maximized: Option<bool>,
}

#[command]
pub async fn create_window(app: AppHandle, opts: WindowOptions) {

    let mut builder =
        WebviewWindowBuilder::new(&app, opts.label, WebviewUrl::External(opts.url.parse().expect("URL解析错误")));

    if let Some(title) = opts.title {
        builder = builder.title(title);
    }
    if let (Some(w), Some(h)) = (opts.width, opts.height) {
        builder = builder.inner_size(w, h);
    }
    if let (Some(x), Some(y)) = (opts.x, opts.y) {
        builder = builder.position(x, y);
    }
    if let Some(r) = opts.resizable {
        builder = builder.resizable(r);
    }
    if let Some(d) = opts.decorations {
        builder = builder.decorations(d);
    }
    if let Some(s) = opts.shadow {
        builder = builder.shadow(s);
    }
    if let Some(f) = opts.fullscreen {
        builder = builder.fullscreen(f);
    }
    if let Some(b) = opts.maximized {
        builder = builder.maximized(b);
    }

    println!("Created window");
    builder.build().expect("创建窗口失败");

}

#[command]
pub fn close_window(app: AppHandle, label: &str) {
    match app.get_webview_window(label) {
        None => println!("Error"),
        Some(window) => window.close().unwrap(),
    };
}

#[command]
pub fn maximize_window(app: AppHandle, label: &str) {

    let window = app.get_webview_window(label).unwrap();

    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }

}

#[command]
pub fn minimize_window(app: AppHandle, label: &str) {
    app.get_webview_window(label).expect("window doesn't exist").minimize().expect("window minimize failed");
}
