use anyhow::anyhow;
use id3::Tag;
// use std::env;
use std::path::Path;
use walkdir::WalkDir;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    path: String,
}

fn main() {
    println!("Hello, world!");
    // let _envars = set_env_vars();
    let args = Cli::parse();
    let media_lists = scan_source(args.path);
    let mp3_list = media_lists.0;
    let img_list = media_lists.1;
    println!("media_mp3_lists0: {:#?}", mp3_list);
    // println!("media_img_lists1: {:#?}", img_list);

    for x in img_list {
        if x.contains("Folder.jpg") {
            let path = x.split("Folder.jpg").collect::<Vec<&str>>();

            let psplit = path[0].split("/").collect::<Vec<&str>>();
            let artist = psplit[psplit.len() - 3].to_string();
            let album = psplit[psplit.len() - 2].to_string();
            let newfn = format!("{}{}_-_{}.jpg", path[0], artist, album);
            println!("oldfn: {}", x.clone());
            println!("newfn: {}", newfn);
            std::fs::rename(x, newfn).expect("Failed to rename");
        } else {
            println!("x: {}", x)
        }
    }
    for y in mp3_list {

        let mp3_path = y.split("/").collect::<Vec<&str>>();
        let artist = mp3_path[mp3_path.len() - 3].to_string();
        let album = mp3_path[mp3_path.len() - 2].to_string();
        println!("artist: {:?}", artist);
        println!("album: {:?}", album);

        let ypath = Path::new(&y);
        let yparent = ypath.parent().unwrap();
        println!("parent: {:?}", yparent);
        let mut coverart_path = String::from(yparent.to_str().unwrap());
        coverart_path.push_str("/");
        coverart_path.push_str(&artist);
        coverart_path.push_str("_-_");
        coverart_path.push_str(&album);
        coverart_path.push_str(".jpg");
        println!("coverart_path: {:?}", coverart_path);
        let c_path = Path::new(&coverart_path);
        if !c_path.exists() {
            println!("No cover_art: {:?}", y.clone());
            let _cart = extract_coverart(y, coverart_path).expect("Failed to extract coverart");
        }
    }
}

pub fn extract_coverart(mp3_path: String, cover_art_path: String) -> anyhow::Result<String> {
    let tag = Tag::read_from_path(&mp3_path).expect(&mp3_path);
    let first_picture = tag.pictures().next();
    if let Some(p) = first_picture {
        match image::load_from_memory(&p.data) {
            Ok(image) => {
                image.save(&cover_art_path).map_err(|e| {
                    anyhow!("Couldn't write image file {:?}: {}", cover_art_path, e)
                })?;
            }
            Err(e) => return Err(anyhow!("Couldn't load image: {}", e)),
        };

        Ok(cover_art_path)
    } else {
        // Err(anyhow!("No image found in music file"))
        let no_art_pic = "/media/charliepi/HD/MTVSERVER/rusic/no_art_pic.jpg".to_string();
        Ok(no_art_pic)
    }
}

pub fn walk_additional_dir(apath: String) -> (Vec<String>, Vec<String>) {
    let mut musicvec = Vec::new();
    let mut musicimgvec = Vec::new();

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.contains("Music") {
                if fname.ends_with(".mp3") {
                    musicvec.push(fname.clone());
                } else if fname.ends_with(".jpg") {
                    musicimgvec.push(fname.clone());
                } else if fname.ends_with(".png") {
                    musicimgvec.push(fname.clone());
                } else if fname.ends_with(".webp") {
                    musicimgvec.push(fname.clone());
                } else if fname.ends_with(".jpeg") {
                    musicimgvec.push(fname.clone());
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    (musicimgvec.clone(), musicvec.clone())
}

pub fn scan_source(apath: String) -> (Vec<String>, Vec<String>) {

    let add_media = walk_additional_dir(apath);
    let img_list = add_media.0;
    let music_list = add_media.1;

    println!(
        "this is music_list count: {}",
        music_list.clone().len()
    );
    println!("this is coverart count: {}", img_list.clone().len());

    (music_list, img_list)
}
