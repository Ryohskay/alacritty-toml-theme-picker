use std::fs;
use toml::Table;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let selected_theme = "afterglow";

    // open a file
    let config_file_name = "alacritty.toml";

    // load the alacritty config file from toml
    let conf = fs::read_to_string(config_file_name).unwrap();
    let mut config_all = conf.parse::<Table>().unwrap();
    
    // remove and take import section
    let binding = config_all.remove("import").unwrap();
    let imports = binding.as_array().unwrap();
    // initialise a vector to use as a replacement for the import section
    let mut updated_imports: Vec<toml::Value> = Vec::with_capacity(imports.len());

    // iterate over the paths in the import section, find the theme to load
    for f in imports {
        let fpath = f.as_str().unwrap();
        if fpath.contains("themes") {
            // capture the theme directory path and replace the theme file name
            let mut theme_dir = Path::new(fpath).parent().unwrap().to_path_buf();
            theme_dir.push(selected_theme);
            theme_dir.set_extension("toml");
            
            let updated_path_os_str = theme_dir.clone().into_os_string();
            let updated_path = updated_path_os_str.to_str().unwrap();

            let path_for_replace = toml::Value::try_from(updated_path)
                .expect("Converting path into toml::Value failed!");

            // push the created path to the updated imports vector
            updated_imports.push(path_for_replace);
        }
    }

    // replace the import section of the toml
    let replacement_value = toml::Value::try_from(updated_imports).unwrap();
    config_all.insert("import".to_string(), replacement_value);

    // write to the config file
    let data = config_all.to_string();
    fs::write(config_file_name, data.as_bytes())
}
