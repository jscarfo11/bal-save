use crate::ui::MyApp;

#[cfg(feature = "dev")]
pub fn draw_dev(app: &mut MyApp, ui: &mut egui::Ui) {
    let lua = &mut app.dev.lua;

    lua.lua
        .load(
            r#"
                function print_nested_table(tbl, indent)
                    indent = indent or 0
                    local prefix = string.rep("  ", indent)
                    for k, v in pairs(tbl) do
                        if type(v) == "table" then
                            print(prefix .. tostring(k) .. ":")
                            print_nested_table(v, indent + 1)
                        else
                            print(prefix .. tostring(k) .. ": " .. tostring(v))
                        end
                    end
                end
    "#,
        )
        .exec()
        .unwrap();
    lua.lua
        .load(
            r#"
        function print_table(tbl)
            for k, v in pairs(tbl) do
                print(k .. ": " .. tostring(v))
            end
        end
    "#,
        )
        .exec()
        .unwrap();
    if app.dev.table.is_none() {
        ui.label("No table loaded");
        return;
    }
    ui.columns(2, |columns| {
        columns[0].with_layout(
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.vertical(|ui| {
                    let label = ui.label("Save Data");
                    ui.text_edit_multiline(&mut app.dev.lua_string)
                        .labelled_by(label.id);

                    if ui.button("Run").clicked() {
                        lua.lua
                            .load(
                                r#"
            Output = ""
            function print(...)
                local args = {...}
                for i, v in ipairs(args) do
                    Output = Output .. tostring(v) .. " "
                end
                Output = Output .. "\n"
            end
            "#,
                            )
                            .exec()
                            .unwrap();
                        let run_result =
                            lua.lua.load(&app.dev.lua_string).exec();
                        if let Err(e) = run_result {
                            app.dev.output = format!("Error: {}", e);
                        } else if let Ok(output) =
                            lua.lua.globals().get("Output")
                            && output != "".to_string()
                        {
                            app.dev.output = output;
                        } else {
                            app.dev.output = "No output".to_string();
                        }
                        println!("{}", app.dev.output);
                    }

                    if ui.button("Print Table").clicked() {
                        if let Some(table) = &app.dev.table {
                            lua.lua.globals().set("dev_table", table).unwrap();
                            lua.lua
                                .load("print_nested_table(dev_table)")
                                .exec()
                                .unwrap();
                        }
                    }
                    if ui.button("Clear").clicked() {
                        print!("\n\n\n\n\n\n\n\n\n\n\n");
                    }
                });
            },
        );
    });
}
