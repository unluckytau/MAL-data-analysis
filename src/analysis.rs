use crate::models::Anime;
use std::collections::HashMap;

/// obj 1
/// Filters anime with members > min_members and sorts by lowest score.
pub fn high_viewers_low_acclaim(anime_list: &[Anime], min_members: u32, top_n: usize) -> Vec<Anime> {
    let mut filtered: Vec<Anime> = anime_list
        .iter()
        .filter(|a| a.members > min_members && a.score.is_some())
        .cloned()
        .collect();

    filtered.sort_by(|a, b| a.score.unwrap().partial_cmp(&b.score.unwrap()).unwrap());
    filtered.into_iter().take(top_n).collect()
}

/// obj 2
/// Favourite conversion is modeled as the ratio of `favorites` to `members`.
pub fn high_passion_low_members(anime_list: &[Anime], max_members: u32, top_n: usize) -> Vec<(Anime, f64)> {
    let mut filtered: Vec<(Anime, f64)> = anime_list
        .iter()
        .filter(|a| a.members > 0 && a.members < max_members)
        .map(|a| (a.clone(), a.favorites as f64 / a.members as f64))
        .collect();

    filtered.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    filtered.into_iter().take(top_n).collect()
}

/// obj 3
pub fn engagement_percentage(anime_list: &[Anime]) -> f64 {
    let valid_anime: Vec<&Anime> = anime_list
        .iter()
        .filter(|a| a.members > 0 && a.scored_by.is_some())
        .collect();

    if valid_anime.is_empty() {
        return 0.0;
    }

    let total_ratio: f64 = valid_anime
        .iter()
        .map(|a| a.scored_by.unwrap() / a.members as f64)
        .sum();

    (total_ratio / valid_anime.len() as f64) * 100.0
}

/// obj 4
pub fn studio_performance(anime_list: &[Anime], min_anime: u32) -> (Vec<(String, f64, u32)>, Vec<(String, f64, u32)>) {
    let mut stats: HashMap<String, (f64, u32)> = HashMap::new();

    for anime in anime_list {
        if let (Some(studio_str), Some(score)) = (&anime.studios, anime.score) {
            let entry = stats.entry(studio_str.clone()).or_insert((0.0, 0));
            entry.0 += score;
            entry.1 += 1;
        }
    }

    let all_studios: Vec<(String, f64, u32)> = stats
        .into_iter()
        .map(|(studio, (total_score, count))| (studio, total_score / (count as f64), count))
        .collect();

    let mut by_volume = all_studios.clone();
    by_volume.sort_by(|a, b| b.2.cmp(&a.2));

    let mut by_score: Vec<(String, f64, u32)> = all_studios
        .into_iter()
        .filter(|(_, _, count)| *count >= min_anime)
        .collect();
    by_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    (by_score, by_volume)
}

/// obj 5
pub fn format_roi(anime_list: &[Anime]) -> Vec<(String, f64)> {
    let mut stats: HashMap<String, (f64, u32)> = HashMap::new();

    for anime in anime_list {
        if let (Some(format), Some(episodes)) = (&anime.format, anime.episodes) {
            if episodes > 0.0 {
                let engagement_per_ep = anime.members as f64 / episodes;
                let entry = stats.entry(format.clone()).or_insert((0.0, 0));
                entry.0 += engagement_per_ep;
                entry.1 += 1;
            }
        }
    }

    let mut results: Vec<(String, f64)> = stats
        .into_iter()
        .map(|(format, (total_roi, count))| (format, total_roi / (count as f64)))
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// obj 6
pub fn optimal_episode_count(anime_list: &[Anime]) -> Vec<(String, f64)> {
    let mut buckets: HashMap<String, (f64, u32)> = HashMap::new();

    for anime in anime_list {
        if let Some(score) = anime.score {
            let bucket = match anime.episodes {
                Some(e) if e == 1.0 => "1 (Movie/OVA/Special)",
                Some(e) if e > 1.0 && e <= 13.0 => "2-13 Episodes",
                Some(e) if e > 13.0 && e <= 26.0 => "14-26 Episodes",
                Some(e) if e > 26.0 && e <= 50.0 => "27-50 Episodes",
                Some(e) if e > 50.0 => "50+ Episodes",
                _ => continue,
            };

            let entry = buckets.entry(bucket.to_string()).or_insert((0.0, 0));
            entry.0 += score;
            entry.1 += 1;
        }
    }

    let mut results: Vec<(String, f64)> = buckets
        .into_iter()
        .map(|(bucket, (total_score, count))| (bucket, total_score / (count as f64)))
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// obj 7
pub fn genre_share(anime_list: &[Anime]) -> Vec<(String, f64)> {
    let mut genre_members: HashMap<String, u64> = HashMap::new();
    let mut total_members = 0u64;

    for anime in anime_list {
        if let Some(genres) = &anime.genres {
            for genre in genres.split('|') {
                *genre_members.entry(genre.to_string()).or_insert(0) += anime.members as u64;
            }
            total_members += anime.members as u64;
        }
    }

    let mut results: Vec<(String, f64)> = genre_members
        .into_iter()
        .map(|(genre, members)| (genre, (members as f64 / total_members as f64) * 100.0))
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// obj 8
pub fn demographics_scores(anime_list: &[Anime]) -> Vec<(&'static str, f64)> {
    let targets = vec!["Shounen", "Seinen", "Shoujo", "Josei"];
    let mut results = Vec::new();

    for target in targets {
        let mut total_score = 0.0;
        let mut count = 0;

        for anime in anime_list {
            if let (Some(tags), Some(score)) = (&anime.tags, anime.score) {
                if tags.contains(target) {
                    total_score += score;
                    count += 1;
                }
            }
        }

        let avg = if count > 0 { total_score / count as f64 } else { 0.0 };
        results.push((target, avg));
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// obj 9
pub fn season_performance(anime_list: &[Anime]) -> Vec<(String, f64, f64)> {
    let mut stats: HashMap<String, (f64, f64, u32)> = HashMap::new();

    for anime in anime_list {
        if let (Some(season), Some(score)) = (&anime.season, anime.score) {
            let entry = stats.entry(season.to_lowercase()).or_insert((0.0, 0.0, 0));
            entry.0 += score;
            entry.1 += anime.members as f64;
            entry.2 += 1;
        }
    }

    let mut results: Vec<(String, f64, f64)> = stats
        .into_iter()
        .map(|(season, (tot_score, tot_members, count))| {
            (season, tot_score / count as f64, tot_members / count as f64)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}
