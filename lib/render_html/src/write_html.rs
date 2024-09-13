use super::{errors::Error, renderer::PagesHtml};
use log;
use std::io::prelude::Write;
use std::{fs::File, path::PathBuf};

pub fn write_all(html_content: PagesHtml, out_dir: &str) -> Result<(), Error> {
    log::info!("Saving pages html");
    let out_prefix = PathBuf::from(out_dir);
    log::info!("Saving index.html");
    write_html(html_content.index_html, &(out_prefix.join("index.html")))?;
    log::info!("Saving plant_overview.html");
    write_html(
        html_content.plants_overview_html,
        &(out_prefix.join("plant_overview.html")),
    )?;
    log::info!("Saving species_overview.html");
    write_html(
        html_content.species_overview_html,
        &(out_prefix.join("species_overview.html")),
    )?;
    log::info!("Saving gallery.html");
    write_html(
        html_content.gallery_html,
        &(out_prefix.join("gallery.html")),
    )?;
    log::info!("Saving activities.html");
    write_html(
        html_content.activities_html,
        &(out_prefix.join("activities.html")),
    )?;
    log::info!("Saving graveyward.html");
    write_html(
        html_content.graveyard_html,
        &(out_prefix.join("graveyard.html")),
    )?;

    log::info!("Saving plant htmls");
    for plant_html in html_content.plant_htmls.iter() {
        log::info!("saving {}.html", plant_html.page_name);
        write_html(
            plant_html.page_html.clone(),
            &(out_prefix.join(&plant_html.page_name)),
        )?;
    }

    log::info!("Saving species htmls");
    for species_html in html_content.species_htmls.iter() {
        log::info!("Saving {}", species_html.page_name);
        write_html(
            species_html.page_html.clone(),
            &(out_prefix.join(&species_html.page_name)),
        )?;
    }
    Ok(())
}

pub fn write_html(html_content: String, file_path: &PathBuf) -> Result<(), Error> {
    log::info!("Saving file {:?}", file_path);
    let mut out_file = File::create(file_path)?;
    out_file.write_all(html_content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod write_html_tests {
    use super::write_all;
    use crate::{
        renderer::PagesHtml,
        test_common::{example_plant, example_plant2, example_renderer, example_species},
    };
    use std::path::PathBuf;

    const OUT_DIR: &str = "../../testing/html_out/";

    fn example_html() -> PagesHtml {
        PagesHtml {
            index_html: example_renderer().render_index().unwrap(),
            plants_overview_html: example_renderer().render_plant_overview().unwrap(),
            species_overview_html: example_renderer().render_species_overview().unwrap(),
            gallery_html: example_renderer().render_gallery().unwrap(),
            activities_html: example_renderer().render_activities().unwrap(),
            graveyard_html: example_renderer().render_graveyard().unwrap(),
            plant_htmls: example_renderer().render_all_plants().unwrap(),
            species_htmls: example_renderer().render_all_species().unwrap(),
        }
    }

    #[test]
    fn write() {
        let base = PathBuf::from(OUT_DIR);
        if !base.exists() {
            std::fs::create_dir_all(base.clone()).unwrap();
        }
        assert!(base.exists());

        let plants = base.join("plants");
        if !plants.exists() {
            std::fs::create_dir_all(plants.clone()).unwrap();
        }
        assert!(plants.exists());

        let species = base.join("species");
        if !species.exists() {
            std::fs::create_dir_all(species.clone()).unwrap();
        }
        assert!(species.exists());

        write_all(example_html(), OUT_DIR).unwrap();
        let index = base.join("index.html");
        assert!(index.exists());
        let plant_overview = base.join("plant_overview.html");
        assert!(plant_overview.exists());
        let species_overview = base.join("species_overview.html");
        assert!(species_overview.exists());
        let gallery = base.join("gallery.html");
        assert!(gallery.exists());
        let activities = base.join("activities.html");
        assert!(activities.exists());
        let graveyard = base.join("graveyard.html");
        assert!(graveyard.exists());
        let plant1 = base.join(example_plant().get_url("plants"));
        assert!(plant1.exists());
        let plant2 = base.join(example_plant2().get_url("plants"));
        assert!(plant2.exists());
        let species = base.join(example_species().get_url("species"));
        assert!(species.exists());

        std::fs::remove_file(index.clone()).unwrap();
        assert!(!index.exists());
        std::fs::remove_file(plant_overview.clone()).unwrap();
        assert!(!plant_overview.exists());
        std::fs::remove_file(species_overview.clone()).unwrap();
        assert!(!species_overview.exists());
        std::fs::remove_file(gallery.clone()).unwrap();
        assert!(!gallery.exists());
        std::fs::remove_file(activities.clone()).unwrap();
        assert!(!activities.exists());
        std::fs::remove_file(graveyard.clone()).unwrap();
        assert!(!graveyard.exists());
        std::fs::remove_file(plant1.clone()).unwrap();
        assert!(!plant1.exists());
        std::fs::remove_file(plant2.clone()).unwrap();
        assert!(!plant2.exists());
        std::fs::remove_file(species.clone()).unwrap();
        assert!(!species.exists());
    }
}
