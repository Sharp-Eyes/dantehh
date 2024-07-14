use anyhow::{Context, Result};
use imageproc::template_matching::{match_template_parallel, MatchTemplateMethod};
use rand::Rng;
use serde::Deserialize;
use std::process::Command;

use image::{open, GrayImage};


#[derive(Deserialize)]
pub struct Config {
    pub adb: ADB,
}

#[derive(Deserialize)]
pub struct ADB {
    pub path: String,
    pub ip: String,
}


pub fn open_template(fp: &str) -> Result<GrayImage> {
    Ok(
        open(format!("resources/{}.png", fp))
            .with_context(|| format!("Failed to open template '{}'", fp))?
            .to_luma8()
    )
}


pub fn screencap(config: &Config) -> Result<GrayImage> {
    let img = image::load_from_memory(
        &Command::new(&config.adb.path)
            .arg("exec-out")
            .arg("screencap")
            .arg("-p")
            .output()
            .context("Failed to screencap")?
            .stdout
    )?;

    // TODO: yeet when done debugging
    img.save("test.png").ok();

    Ok(
        image::imageops::grayscale(
            &img
        )
    )
}


pub fn find(config: &Config, template: &str) -> Result<Option<(u32, u32)>> {
    let img = screencap(&config)?;
    let template = open_template(template)?;

    let extremes = imageproc::template_matching::find_extremes(
        &match_template_parallel(
            &img,
            &template,
            MatchTemplateMethod::CrossCorrelationNormalized,
        )
    );

    println!("confidence: {}", extremes.max_value);

    if extremes.max_value < 0.5 {
        return Ok(None)
    } 

    Ok(Some(extremes.max_value_location))
}


pub fn tap(config: &Config, x: u32, y: u32) -> Result<()> {
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(10..20);

    println!("tapping at ({}, {})", x+offset, y+offset);

    Command::new(&config.adb.path)
        .arg("shell")
        .arg("input")
        .arg("tap")
        .arg((x+offset).to_string())
        .arg((y+offset).to_string())
        .output()
        .context(format!("Failed to send tap event (x={}, y={})", x+offset, y+offset))?;

    Ok(())
}


pub fn find_and_tap(config: &Config, template: &str) -> Result<Option<()>> {
    let result = find(&config, template)?;
    Ok(
        if let Some((x, y)) = result {
            tap(&config, x, y)?;
            Some(())
        } else {
            None
        }
    )
}