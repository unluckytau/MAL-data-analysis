use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::models::Anime;

pub fn load_dataset(file_path: &str) -> Result<Vec<Anime>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut anime_list = Vec::new();

    // deserialize into struct
    for result in rdr.deserialize() {
        let record: Anime = result?;
        anime_list.push(record);
    }
    
    Ok(anime_list)
}
