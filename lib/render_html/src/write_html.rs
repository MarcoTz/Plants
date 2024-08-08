use super::{errors::Error, renderer::PagesHtml};
use std::fs::File;
use std::io::prelude::Write;

pub fn write_all(html_content: PagesHtml, out_dir: &str) -> Result<(), Error> {
    let out_prefix = out_dir.to_owned();
    write_html(
        html_content.index_html,
        &(out_prefix.clone() + "index.html"),
    )?;
    write_html(
        html_content.plants_overview_html,
        &(out_prefix.clone() + "plant_overview.html"),
    )?;
    write_html(
        html_content.species_overview_html,
        &(out_prefix.clone() + "species_overview.html"),
    )?;
    write_html(
        html_content.gallery_html,
        &(out_prefix.clone() + "gallery.html"),
    )?;
    write_html(
        html_content.activities_html,
        &(out_prefix.clone() + "activities.html"),
    )?;
    write_html(
        html_content.graveyard_html,
        &(out_prefix.clone() + "graveyard.html"),
    )?;

    for plant_html in html_content.plant_htmls.iter() {
        write_html(
            plant_html.page_html.clone(),
            &(out_prefix.clone() + &plant_html.page_name),
        )?;
    }

    for species_html in html_content.species_htmls.iter() {
        write_html(
            species_html.page_html.clone(),
            &(out_prefix.clone() + &species_html.page_name),
        )?;
    }
    Ok(())
}

pub fn write_html(html_content: String, file_path: &str) -> Result<(), Error> {
    let mut out_file = File::create(file_path)?;
    out_file.write_all(html_content.as_bytes())?;
    Ok(())
}
