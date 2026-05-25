use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        // 画面の初期サイズを設定
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 350.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "場合の数・確率計算シミュレーター",
        options,
        Box::new(|cc| {
            // 日本語フォントの設定を呼び出す
            setup_custom_fonts(&cc.egui_ctx);
            Box::<NumApp>::default()
        }),
    )
}

// GUIの状態を管理する構造体
struct NumApp {
    n: u32,
    r: u32,
}

impl Default for NumApp {
    fn default() -> Self {
        Self { n: 5, r: 2 }
    }
}

// --- 計算用ロジック (u128でオーバーフロー対策) ---
fn factorial(n: u32) -> u128 {
    (1..=n as u128).product()
}

fn permutation(n: u32, r: u32) -> u128 {
    if r > n { return 0; }
    ((n - r + 1) as u128..=n as u128).product()
}

fn combination(n: u32, r: u32) -> u128 {
    if r > n { return 0; }
    let r = std::cmp::min(r, n - r);
    let mut num: u128 = 1;
    let mut den: u128 = 1;
    for i in 1..=r as u128 {
        num *= n as u128 - i + 1;
        den *= i;
    }
    num / den
}

// --- GUIの描画ロジック ---
impl eframe::App for NumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("📊 場合の数 計算シミュレーター");
            });
            ui.separator();

            // パラメータ調整エリア
            ui.group(|ui| {
                ui.label("【パラメータ設定】");
                ui.add_space(5.0);
                
                ui.horizontal(|ui| {
                    ui.label("全体の要素数 (n):");
                    ui.add(egui::Slider::new(&mut self.n, 0..=30)); 
                });

                ui.horizontal(|ui| {
                    ui.label("選択する数   (r):");
                    // r が n を超えないように動的に制限
                    ui.add(egui::Slider::new(&mut self.r, 0..=self.n)); 
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // 計算結果表示エリア
            ui.heading("🔢 計算結果");
            ui.add_space(5.0);

            egui::Grid::new("result_grid")
                .num_columns(2)
                .spacing([40.0, 15.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("階乗 (n!):");
                    ui.label(format!("{}! = {}", self.n, factorial(self.n)));
                    ui.end_row();

                    ui.label("順列 (nPr):");
                    ui.label(format!("_{}P{} = {} 通り", self.n, self.r, permutation(self.n, self.r)));
                    ui.end_row();

                    ui.label("組合せ (nCr):");
                    ui.label(format!("_{}C{} = {} 通り", self.n, self.r, combination(self.n, self.r)));
                    ui.end_row();
                });

            ui.add_space(20.0);
            
            // 注意書き（nが大きくなってきた場合）
            if self.n > 25 {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 165, 0), 
                    "⚠️ n の値が大きいため、階乗の桁数が非常に大きくなっています。"
                );
            }
        });
    }
}

// --- 日本語文字化け対策のフォント設定 ---
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // OS標準の日本語フォントのパスを設定（環境に合わせて自動フォールバック）
    #[cfg(target_os = "windows")]
    let font_path = "C:\\Windows\\Fonts\\msjh.ttc"; // 微軟正黑體 or 游ゴシック等
    #[cfg(target_os = "macos")]
    let font_path = "/System/Library/Fonts/Hiragino Sans GB.ttc";
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let font_path = "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf"; // Linux用

    // フォントファイルの読み込みを試みる
    if let Ok(font_data) = std::fs::read(font_path) {
        fonts.font_data.insert(
            "jp_font".to_owned(),
            egui::FontData::from_owned(font_data),
        );

        // 最優先フォントとして登録
        fonts.families.get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "jp_font".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "jp_font".to_owned());
    }

    ctx.set_fonts(fonts);
}