use slint::Weak;

slint::include_modules!();

const TAXPER: f64 = 0.3;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPERXPER: f64 = 0.1;
fn main() -> Result<(), slint::PlatformError> {
    // weak pointer
    let ui: AppWindow = AppWindow::new()?;

    let ui_handle: Weak<AppWindow> = ui.as_weak();

    ui.on_divide_income(move |string| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        // calculation
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPERXPER;

        let result: String = format!(
            "Texes: {:.2}\nOwener: {:.2}\nProfit: {:.2}\nOpEx: {:.2}",
            tax,
            owner,
            profit,
            opex
        );
        ui.set_results(result.into())
    });
    ui.run()
}
