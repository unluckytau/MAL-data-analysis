mod models;
mod io;
mod analysis;

fn main() {
    let file_path = "data/anime_dataset.csv";
    
    // dataset
    let dataset = match io::load_dataset(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read dataset: {}", e);
            return;
        }
    };
    println!("dataset loaded {} \n", dataset.len());

    // obj 1
    println!("HIGH VIEWERBASE, LOW CRITICAL ACCLAIM\n");
    let popular_but_disliked = analysis::high_viewers_low_acclaim(&dataset, 500_000, 5);
    for anime in popular_but_disliked {
        println!("Title: {} | Members: {} | Score: {:.2}", 
            anime.display_title(), anime.members, anime.score.unwrap());
    }
    println!();

    // obj 2
    println!("HIGH FAVOURITES, LOWER VIEWERBASE (<100k)\n");
    let cult_classics = analysis::high_passion_low_members(&dataset, 100_000, 5);
    for (anime, ratio) in cult_classics {
        println!("Title: {} | Members: {} | Favorites: {} | Favourites Conversion: {:.2}%", 
            anime.display_title(), anime.members, anime.favorites, ratio * 100.0);
    }
    println!();

    // obj 3
    println!("RATING ENGAGEMENT PERCENTAGE\n");
    let engagement = analysis::engagement_percentage(&dataset);
    println!("On average, {:.2}% of listed members actually score/rate a title.", engagement);

    // obj 4
    println!("\nSTUDIO CONSISTENCY\n");
    let (top_by_score, top_by_volume) = analysis::studio_performance(&dataset, 5); // Min 5 anime for score
    println!("Highest Scoring (Min 5 Anime):");
    for (studio, score, count) in top_by_score.iter().take(3) {
        println!("  {} | Avg Score: {:.2} | Titles: {}", studio, score, count);
    }
    println!("Highest Volume Producers:");
    for (studio, score, count) in top_by_volume.iter().take(3) {
        println!("  {} | Titles: {} | Avg Score: {:.2}", studio, count, score);
    }
    println!();

    // obj 5
    println!("FORMAT ROI (MEMBERS PER EPISODE)\n");
    let format_roi = analysis::format_roi(&dataset);
    for (format, roi) in format_roi.iter().take(4) { // Show top 4 formats
        println!("  {}: {:.0} avg members per episode", format, roi);
    }
    println!();

    // obj 6
    println!("OPTIMAL EPISODE COUNT\n");
    let ep_ranges = analysis::optimal_episode_count(&dataset);
    for (range, score) in ep_ranges {
        println!("  {}: {:.2} Avg Score", range, score);
    }
    println!();

    // obj 7
    println!("GENRE AUDIENCE SHARE\n");
    let genre_shares = analysis::genre_share(&dataset);
    for (genre, percentage) in genre_shares.iter().take(5) {
        println!("  {}: {:.2}% of total market attention", genre, percentage);
    }
    println!();

    // obj 8
    println!("DEMOGRAPHICS SCORES\n");
    let demographics = analysis::demographics_scores(&dataset);
    for (target, score) in demographics {
        println!("  {}: {:.2}", target, score);
    }
    println!();

    // obj 9
    println!("SEASON PERFORMANCE (SPRING/FALL VS WINTER/SUMMER)\n");
    let seasons = analysis::season_performance(&dataset);
    for (season, score, members) in seasons {
        println!("  {:<6} | Avg Score: {:.2} | Avg Members: {:.0}", season, score, members);
    }
}
