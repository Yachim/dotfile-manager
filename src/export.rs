use crate::lib;
use dotfile_manager::question_yes_no;
use lib::{get_existing_templates, process_template_to_struct};
use std::{fs, path::Path};

pub fn export_templates(export_file: String) {
    // Get all templates
    let templates = get_existing_templates();

    let mut toml = String::new();
    for template in templates {
        let template = process_template_to_struct(&template);
        toml.push_str(&format!(
            "[{}]\nname = \"{}\"\npath = \"{}\"\ngit_path = \"{}\"\n\n",
            template.name, template.name, template.path, template.git_path
        ));
    }

    // Remove last newline
    toml.pop();

    // Check if file already exists
    let export_file_path = Path::new(&export_file);
    if export_file_path.exists() {
        println!("File already exists");
        question_yes_no!("Do you want to overwrite it?");
    }

    // Write to file
    fs::write(&export_file, toml).unwrap();

    println!("Exported templates to \"{export_file}\"");
}
