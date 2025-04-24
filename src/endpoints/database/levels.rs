use axum::{Extension, Router, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use serde::{Deserialize, Deserializer};
use sha1::{Digest, Sha1};
use sqlx::PgPool;
use tokio::fs::{read, rename, write};
use tracing::error;

use crate::utilities::{
    self, crypto,
    database::{Level, get_level_by_id},
};

fn deserialize_enum_from_int<'de, D>(deserializer: D) -> Result<Option<QueryType>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i32 = Deserialize::deserialize(deserializer)?;
    match value {
        0 => Ok(Some(QueryType::Search)),
        1 => Ok(Some(QueryType::MostDownloaded)),
        2 => Ok(Some(QueryType::MostLiked)),
        3 => Ok(Some(QueryType::Trending)),
        5 => Ok(Some(QueryType::LevelsPerUser)),
        6 => Ok(Some(QueryType::Featured)),
        7 => Ok(Some(QueryType::Magic)),
        10 => Ok(Some(QueryType::MapPacks)),
        11 => Ok(Some(QueryType::Awarded)),
        12 => Ok(Some(QueryType::Followed)),
        13 => Ok(Some(QueryType::Friends)),
        15 => Ok(Some(QueryType::MostLikedGDW)),
        16 => Ok(Some(QueryType::HallOfFame)),
        17 => Ok(Some(QueryType::FeaturedGDW)),
        19 => Ok(Some(QueryType::Unknown)),
        21 => Ok(Some(QueryType::DailySafe)),
        22 => Ok(Some(QueryType::WeeklySafe)),
        23 => Ok(Some(QueryType::EventSafe)),
        25 => Ok(Some(QueryType::ListLevels)),
        27 => Ok(Some(QueryType::SentLevels)),
        _ => Ok(None),
    }
}

#[derive(Deserialize, Debug)]
enum QueryType {
    Search = 0,
    MostDownloaded = 1,
    MostLiked = 2,
    Trending = 3,
    LevelsPerUser = 5,
    Featured = 6,
    Magic = 7,
    MapPacks = 10,
    Awarded = 11,
    Followed = 12,
    Friends = 13,
    MostLikedGDW = 15,
    HallOfFame = 16,
    FeaturedGDW = 17,
    Unknown = 19,
    DailySafe = 21,
    WeeklySafe = 22,
    EventSafe = 23,
    ListLevels = 25,
    SentLevels = 27,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
struct UploadLevelRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "levelID")]
    pub level_id: i32,
    #[serde(default = "default_level_name", rename = "levelName")]
    pub level_name: String,
    #[serde(default, rename = "levelDesc")]
    pub level_description: String,
    #[serde(rename = "levelLength")]
    pub level_length: i32,
    #[serde(rename = "audioTrack")]
    pub audio_track: i32,
    #[serde(default, rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(default, rename = "auto")]
    pub auto: i32,
    #[serde(default, rename = "original")]
    pub original: i32,
    #[serde(default, rename = "twoPlayer")]
    pub two_player: i32,
    #[serde(default, rename = "songID")]
    pub song_id: i32,
    #[serde(default, rename = "objects")]
    pub objects: i32,
    #[serde(default, rename = "coins")]
    pub coins: i32,
    #[serde(default, rename = "requestedStars")]
    pub requested_stars: i32,
    #[serde(default = "default_extra_string", rename = "extraString")]
    pub extra_string: String,
    #[serde(default, rename = "levelString")]
    pub level_string: String,
    #[serde(default, rename = "levelInfo")]
    pub level_info: String,
    #[serde(default, rename = "unlisted")]
    pub unlisted: i32,
    #[serde(default, rename = "ldm")]
    pub is_ldm: i32,
    #[serde(default, rename = "wt")]
    pub wt: i32,
    #[serde(default, rename = "wt2")]
    pub wt2: i32,
    #[serde(default, rename = "settingsString")]
    pub settings_string: String,
    #[serde(default, rename = "songIDs")]
    pub song_ids: String,
    #[serde(default, rename = "sfxIDs")]
    pub sfx_ids: String,
    #[serde(default, rename = "ts")]
    pub ts: i32,
    #[serde(default, rename = "password")]
    pub password: i32,
    #[serde(rename = "udid")]
    pub id: Option<String>,
    #[serde(rename = "uuid")]
    pub user_id: Option<i32>,
    #[serde(rename = "accountID")]
    pub account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    pub hash: Option<String>,
    #[serde(rename = "userName")]
    pub username: Option<String>,
    #[serde(rename = "levelVersion")]
    pub level_version: Option<i32>,
    #[serde(default)]
    pub seed: String,
    #[serde(default)]
    pub seed2: String,
    #[serde(default)]
    pub secret: String,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
struct GetLevelsRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "levelID")]
    pub level_id: Option<i32>,
    #[serde(rename = "udid")]
    pub id: Option<String>,
    #[serde(rename = "uuid")]
    pub user_id: Option<i32>,
    #[serde(rename = "accountID")]
    pub account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    pub hash: Option<String>,
    #[serde(rename = "type", deserialize_with = "deserialize_enum_from_int")]
    pub query_type: Option<QueryType>,
    #[serde(default, rename = "str")]
    pub query: Option<String>,
    #[serde(rename = "diff")]
    pub difficulty_filter: Vec<String>,
    #[serde(default, rename = "len")]
    pub length_filter: String,
    #[serde(default)]
    pub followed: Option<String>,
    #[serde(default, rename = "demonFilter")]
    pub demon_filter: Option<i32>,
    pub page: Option<i32>,
    pub total: Option<i32>,
    #[serde(default)]
    pub uncompleted: Option<i16>,
    #[serde(default, rename = "onlyCompleted")]
    pub only_completed: Option<i16>,
    #[serde(default, rename = "completedLevels")]
    pub completed_levels: Option<String>,
    #[serde(default)]
    pub featured: Option<i16>,
    #[serde(default)]
    pub original: Option<i16>,
    #[serde(rename = "twoPlayer")]
    pub two_player: Option<i16>,
    #[serde(default)]
    pub coins: Option<i16>,
    #[serde(default)]
    pub epic: Option<i16>,
    #[serde(default)]
    pub legendary: Option<i16>,
    #[serde(default)]
    pub mythic: Option<i16>,
    #[serde(default)]
    pub song: Option<i32>,
    #[serde(rename = "customSong")]
    pub custom_song: Option<i32>,
    #[serde(default)]
    pub star: Option<i16>,
    #[serde(default)]
    pub gauntlet: Option<i32>,
    #[serde(rename = "noStar")]
    pub no_star: Option<i16>,
    #[serde(default)]
    pub secret: Option<String>,
    #[serde(default, rename = "binaryVersion")]
    pub binary_version: Option<i32>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
struct DownloadLevelRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(rename = "udid")]
    pub id: Option<String>,
    #[serde(rename = "uuid")]
    pub user_id: Option<i32>,
    #[serde(rename = "accountID")]
    pub account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    pub hash: Option<String>,
    #[serde(rename = "levelID")]
    pub level_id: Option<i32>,
    #[serde(rename = "inc")]
    pub inc: Option<i32>,
    pub secret: Option<String>,
    #[serde(rename = "rs")]
    pub rs: Option<String>,
    #[serde(rename = "chk")]
    pub checksum: Option<String>,
    pub extras: Option<String>,
}

#[derive(sqlx::FromRow, Deserialize, Debug, Default)]
#[allow(unused)]
struct Song {
    pub id: i32,
    pub name: String,
    pub author_id: i32,
    pub author_name: String,
    pub size: String,
    pub download: String,
    pub hash: String,
    pub is_disabled: i32,
    pub levels_count: i32,
    pub reupload_time: i32,
}

#[derive(Deserialize, Debug, Default)]
struct LevelStats {
    level_id: i32,
    stars: i32,
    coins: i32,
}

fn default_level_name() -> String {
    "Unnamed level".into()
}

fn default_extra_string() -> String {
    "29_29_29_40_29_29_29_29_29_29_29_29_29_29_29_29".into()
}

async fn upload_level(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UploadLevelRequest>,
) -> impl IntoResponse {
    if data.account_id.unwrap_or(0) == 0 || data.user_id.unwrap_or(0) == 0 {
        return "-11".into_response();
    }

    let account_id = data.account_id.unwrap();
    let user_id = data.user_id.unwrap();

    let account = match sqlx::query!(
        "SELECT gjp2 FROM accounts WHERE account_id = $1",
        account_id
    )
    .fetch_optional(&db)
    .await
    .unwrap()
    {
        Some(acc) => acc,
        None => return "-11".into_response(),
    };

    if data.hash != account.gjp2 {
        return "-11".into_response();
    }

    let level = sqlx::query!(
        "SELECT count(*) FROM levels WHERE level_id = $1 AND user_id = $2 AND is_deleted = 0",
        data.level_id,
        user_id
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if level.unwrap().count.unwrap_or_default() != 0 {
        let level_path = format!("./data/levels/{}", data.level_id);

        let level_write_result = write(level_path, &data.level_string).await;
        if level_write_result.is_err() {
            return "-5".into_response();
        }

        sqlx::query!(r#"
            UPDATE levels
                SET game_version = $1, binary_version = $2, level_desc = $3, level_version = level_version + 1, level_length = $4, audio_track = $5, auto = $6, original = $7,
                    two_player = $8, song_id = $9, objects = $10, coins = $11, requested_stars = $12, extra_string = $13, level_string = $14, level_info = $15, unlisted = $16,
                    is_ldm = $17, wt = $18, wt2 = $19, unlisted2 = $20, settings_string = $21, song_ids = $22, sfx_ids = $23, ts = $24, password = $25, update_date = $26
            WHERE level_id = $27
        "#, data.game_version, data.binary_version, data.level_description, data.level_length, data.audio_track, data.auto, data.original, data.two_player, data.song_id,
            data.objects, data.coins, data.requested_stars, data.extra_string, data.level_string, data.level_info, data.unlisted, data.is_ldm, data.wt, data.wt2, 0,
            data.settings_string, data.song_ids, data.sfx_ids, data.ts, data.password, chrono::Utc::now().timestamp() as i32, data.level_id)
            .execute(&db)
            .await
            .unwrap();

        return format!("{}", data.level_id).into_response();
    }

    let timestamp = chrono::Utc::now().timestamp();

    let temporary_level_path = format!("./data/levels/{}_{}", user_id, timestamp);
    let level_path = format!("./data/levels/{}", data.level_id);

    let level_write_result = write(&temporary_level_path, &data.level_string).await;
    if level_write_result.is_err() {
        error!("{level_write_result:?}");
        return "-5".into_response();
    }

    let level_insert = sqlx::query!(
        r#"
            INSERT INTO levels (
                user_id, ext_id, username, game_version, binary_version, level_name, level_desc, level_version, level_length, audio_track,
                auto, original, two_player, song_id, objects, coins, requested_stars, extra_string, level_string, level_info,
                secret, unlisted, is_ldm, wt, wt2, settings_string, song_ids, sfx_ids, ts, password,
                upload_date, update_date, hostname
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                $31, $32, $33
            )
            RETURNING level_id
        "#,
        data.user_id, data.account_id.map(|id| id.to_string()).unwrap_or_default(), data.username.as_deref().unwrap_or(""), data.game_version, data.binary_version,
        data.level_name, data.level_description, 1, data.level_length, data.audio_track, data.auto, data.original,
        data.two_player, data.song_id, data.objects, data.coins, data.requested_stars, data.extra_string, data.level_string, data.level_info,
        data.secret, data.unlisted, data.is_ldm, data.wt, data.wt2, data.settings_string, data.song_ids, data.sfx_ids,
        data.ts, data.password, timestamp, timestamp, ""
    )
    .fetch_one(&db)
    .await
    .unwrap();

    rename(temporary_level_path, level_path).await.unwrap();

    format!("{}", level_insert.level_id).into_response()
}

async fn get_level(
    Extension(db): Extension<PgPool>,
    Form(data): Form<GetLevelsRequest>,
) -> impl IntoResponse {
    let user_id = data.user_id.unwrap_or_default();
    if user_id == 0 {
        return "-11".into_response();
    }

    let game_version = data.game_version;
    let page_offset = data.page.unwrap_or(0) * 10;

    let time = chrono::Utc::now().timestamp();
    let mut order = String::from("upload_date");
    let mut order_sorting = "DESC";
    let mut query_to_join = "";
    let mut is_id_search = false;
    let mut no_limit = false;
    let mut filters: Vec<String> = vec!["(unlisted = 0 AND unlisted2 = 0)".into()];

    let difficulty_filter = data.difficulty_filter;
    let length_filter = &data.length_filter.split(", ").collect::<Vec<_>>();

    let followed = &data.followed.unwrap_or_default();
    let followed = followed.split(", ").collect::<Vec<_>>();
    let completed_levels = &data.completed_levels.unwrap_or_default();
    let completed_levels = completed_levels.split(", ").collect::<Vec<_>>();

    let version_filter = format!("levels.game_version <= '{}'", game_version);
    filters.push(version_filter);

    if data.original.unwrap_or_default() != 0 {
        filters.push("original = 0".into());
    }
    if data.coins.unwrap_or_default() != 0 {
        filters.push("star_coins = 1 AND NOT levels.coins = 0".into());
    }
    if (data.uncompleted.unwrap_or_default() != 0 || data.only_completed.unwrap_or_default() != 0)
        && !completed_levels.is_empty()
    {
        let op = if data.uncompleted.unwrap_or_default() != 0 {
            "NOT "
        } else {
            ""
        };
        filters.push(format!(
            "{}level_id IN ({})",
            op,
            completed_levels.join(",")
        ));
    }
    if let Some(song) = data.song {
        if song > 0 {
            if data.custom_song.is_none() {
                filters.push(format!("audio_track = '{}' AND song_id = 0", song - 1));
            } else {
                filters.push(format!("song_id = '{}'", song));
            }
        }
    }
    if data.two_player.unwrap_or_default() != 0 {
        filters.push("two_player = 1".into());
    }
    if data.star.unwrap_or_default() != 0 {
        filters.push("NOT star_stars = 0".into());
    }
    if data.no_star.unwrap_or_default() != 0 {
        filters.push("star_stars = 0".into());
    }
    if !length_filter.is_empty() && !length_filter.contains(&"-") {
        filters.push(format!("level_length IN ({})", length_filter.join(",")));
    }

    let mut rating_filters: Vec<String> = Vec::new();
    if data.featured.unwrap_or_default() != 0 {
        rating_filters.push("star_featured > 0".into());
    }
    if data.epic.unwrap_or_default() != 0 {
        rating_filters.push("star_epic = 1".into());
    }
    if data.mythic.unwrap_or_default() != 0 {
        rating_filters.push("star_epic = 2".into());
    }
    if data.legendary.unwrap_or_default() != 0 {
        rating_filters.push("star_epic = 3".into());
    }
    if !rating_filters.is_empty() {
        filters.push(format!("({})", rating_filters.join(" OR ")));
    }

    if difficulty_filter.contains(&"-2".to_string()) {
        filters.push("star_demon = 1".into());
        match data.demon_filter.unwrap_or_default() {
            1 => filters.push("star_demon_diff = '3'".into()),
            2 => filters.push("star_demon_diff = '4'".into()),
            3 => filters.push("star_demon_diff = '0'".into()),
            4 => filters.push("star_demon_diff = '5'".into()),
            5 => filters.push("star_demon_diff = '6'".into()),
            _ => (),
        };
    } else if difficulty_filter.contains(&"-1".to_string()) {
        filters.push("star_difficulty = '0'".into());
    } else if difficulty_filter.contains(&"-3".to_string()) {
        filters.push("star_auto = '1'".into());
    } else if !difficulty_filter.is_empty() && !difficulty_filter.contains(&"-".to_string()) {
        let diff_string = difficulty_filter
            .iter()
            .map(|d| format!("{}0", d))
            .collect::<Vec<_>>()
            .join(",");
        filters.push(format!(
            "star_difficulty IN ({}) AND star_auto = '0' AND star_demon = '0'",
            diff_string
        ));
    }

    let query = data.query.unwrap_or_default();
    let query_type = data.query_type.unwrap_or(QueryType::Search);

    match query_type {
        QueryType::Search | QueryType::MostLikedGDW => {
            order = "likes".into();

            if !query.is_empty() {
                if query.parse::<i32>().is_ok() {
                    let level_id = query.parse::<i32>().unwrap_or_default();
                    filters = vec![format!(
                        "level_id = {} AND (unlisted != 1 OR (unlisted = 1 AND (ext_id = '{}')))",
                        level_id,
                        data.account_id.unwrap_or_default()
                    )];
                    is_id_search = true;
                } else {
                    let first_char = query.chars().next().unwrap_or('d');
                    match first_char {
                        'u' => {
                            let potential_user_id = query[1..].parse::<i32>();
                            if let Ok(id) = potential_user_id {
                                filters.push(format!("user_id = {}", id));
                            } else {
                                filters.push(format!("level_name ILIKE '%{}%'", query));
                            }
                        }
                        'a' => {
                            let potential_account_id = query[1..].parse::<i32>();
                            if let Ok(id) = potential_account_id {
                                filters.push(format!("ext_id = '{}'", id));
                            } else {
                                filters.push(format!("level_name ILIKE '%{}%'", query));
                            }
                        }
                        _ => {
                            filters.push(format!("level_name ILIKE '%{}%'", query));
                        }
                    }
                }
            }
        }
        QueryType::MostDownloaded => {
            order = "downloads".into();
        }
        QueryType::MostLiked => {
            order = "likes".into();
        }
        QueryType::LevelsPerUser => {
            if data.user_id == query.parse::<i32>().ok() {
                filters.clear();
                filters.push("(unlisted = 0 AND unlisted2 = 0)".into());
                let version_filter = format!("levels.game_version <= '{}'", game_version);
                filters.push(version_filter);
            }
            filters.push(format!("levels.user_id = '{}'", query));
        }
        QueryType::Featured | QueryType::FeaturedGDW => {
            if game_version > 21 {
                filters.push("(NOT star_featured = 0 OR NOT star_epic = 0)".into());
            } else {
                filters.push("NOT star_featured = 0".into());
            }
            order = "star_featured DESC, rate_date DESC, upload_date".into();
        }
        QueryType::HallOfFame => {
            filters.push("NOT star_epic = 0".into());
            order = "star_featured DESC, rate_date DESC, upload_date".into();
        }
        QueryType::Magic => {
            filters.push("level_desc ILIKE '%#magic%'".into());
            filters.push("objects > 9999".into());
        }
        QueryType::MapPacks | QueryType::Unknown => {
            if !query.is_empty() {
                let levels_array: Vec<&str> = query.split(',').collect();
                let mut levels_text = String::new();

                for (i, level_id) in levels_array.iter().enumerate() {
                    levels_text.push_str(&format!("WHEN level_id = {} THEN {} ", level_id, i + 1));
                }

                order = format!("CASE {}\nEND", levels_text);
                order_sorting = "ASC";

                filters.push(format!(
                    "level_id IN ({}) AND (unlisted != 1 OR (unlisted = 1 AND (ext_id = '{}')))",
                    query,
                    data.account_id.unwrap_or_default()
                ));

                no_limit = true;
            }
        }
        QueryType::Awarded => {
            filters.push("NOT star_stars = 0".into());
            order = "rate_date DESC, upload_date".into();
        }
        QueryType::Followed => {
            if followed.is_empty() {
                filters.push("1 != 1".into());
            } else {
                filters.push(format!("ext_id IN ({})", followed.join(", ")));
            }
        }
        QueryType::Friends => {
            filters.push(format!(
                "ext_id = '{}'",
                data.account_id.unwrap_or_default()
            ));
        }
        QueryType::DailySafe => {
            query_to_join =
                "INNER JOIN daily_features ON levels.level_id = daily_features.level_id";
            filters.push(format!("daily_features.type = 0 AND timestamp < {}", time));
            order = "daily_features.fea_id".into();
        }
        QueryType::WeeklySafe => {
            query_to_join =
                "INNER JOIN daily_features ON levels.level_id = daily_features.level_id";
            filters.push(format!("daily_features.type = 1 AND timestamp < {}", time));
            order = "daily_features.fea_id".into();
        }
        QueryType::EventSafe => {
            query_to_join = "INNER JOIN events ON levels.level_id = events.level_id";
            filters.push(format!("timestamp < {}", time));
            order = "events.fea_id".into();
        }
        QueryType::ListLevels => {
            if !query.is_empty() {
                filters = vec![format!(
                    "level_id IN ({}) AND (unlisted != 1 OR (unlisted = 1 AND (ext_id = '{}')))",
                    query,
                    data.account_id.unwrap_or_default()
                )];
                no_limit = true;
            }
        }
        QueryType::SentLevels => {
            query_to_join = "JOIN (SELECT suggest_level_id as level_id, MAX(suggest.timestamp) AS timestamp FROM suggest GROUP BY level_id) suggest ON levels.level_id = suggest.level_id";
            filters.push("suggest.level_id > 0".into());
            order = "suggest.timestamp".into();
        }
        _ => (),
    };

    let limit_clause = if no_limit {
        "".to_string()
    } else {
        format!("LIMIT 10 OFFSET {}", page_offset)
    };

    let end_query = format!(
        "SELECT * FROM levels {} WHERE ({}) AND is_deleted = 0 ORDER BY {} {} {}",
        query_to_join,
        filters.join(") AND ("),
        order,
        order_sorting,
        limit_clause
    );
    let count_query = format!(
        "SELECT count(*) FROM levels {} WHERE ({}) AND is_deleted = 0",
        query_to_join,
        filters.join(") AND (")
    );

    let levels = sqlx::query_as::<_, Level>(&end_query)
        .fetch_all(&db)
        .await
        .unwrap_or_default();
    let levels_count: (i64,) = sqlx::query_as(&count_query)
        .fetch_one(&db)
        .await
        .unwrap_or((0,));

    let mut level_stats: Vec<LevelStats> = Vec::new();
    let mut result_string = String::new();
    let mut user_string = String::new();
    let mut song_string = String::new();

    for level in levels {
        if is_id_search {
            break;
        }

        level_stats.push(LevelStats {
            level_id: level.level_id,
            coins: level.star_coins,
            stars: level.star_stars,
        });

        let star_demon_str = if level.star_demon == 0 {
            "17:"
        } else {
            &format!("17:{}", level.star_demon)
        };
        let star_auto_str = if level.star_auto == 0 {
            "25:"
        } else {
            &format!("25:{}", level.star_auto)
        };

        result_string += &format!(
            "1:{}:2:{}:5:{}:6:{}:8:10:9:{}:10:{}:12:{}:13:{}:14:{}:{}:43:{}:{}:18:{}:19:{}:42:{}:45:{}:3:{}:15:{}:30:{}:31:{}:37:{}:38:{}:39:{}:46:{}:47:{}:35:{}|",
            level.level_id,
            level.level_name,
            level.level_version,
            level.user_id,
            level.star_difficulty,
            level.downloads,
            level.audio_track,
            level.game_version,
            level.likes,
            star_demon_str,
            level.star_demon_diff,
            star_auto_str,
            level.star_stars,
            level.star_featured,
            level.star_epic,
            level.objects,
            crypto::encode_base64(&level.level_desc),
            level.level_length,
            level.original,
            level.two_player,
            level.coins,
            level.star_coins,
            level.requested_stars,
            level.wt,
            level.wt2,
            level.song_id
        );

        if level.song_id != 0 {
            let song_result = sqlx::query_as::<_, Song>("SELECT * FROM songs WHERE id = $1")
                .bind(level.song_id)
                .fetch_optional(&db)
                .await
                .unwrap_or(None);

            if let Some(song) = song_result {
                song_string += &format!(
                    "1~|~{}~|~2~|~{}~|~3~|~{}~|~4~|~{}~|~5~|~{}~|~6~|~~|~10~|~{}~|~",
                    song.id, song.name, song.author_id, song.author_name, song.size, song.download
                );
            }
        }

        user_string += &format!("{}:{}:{}|", level.user_id, level.username, level.ext_id);
    }

    let result_string_tr = result_string.trim_end_matches('|');
    let user_string_tr = user_string.trim_end_matches('|');
    let song_string_tr = song_string.trim_end_matches("~|~");

    let mut hash_string = String::new();

    for level in level_stats {
        let id = level.level_id.to_string();
        let first = id.as_bytes().first().copied().unwrap_or(b'0') as char;
        let last = id.as_bytes().last().copied().unwrap_or(b'0') as char;

        hash_string.extend([first, last]);
        hash_string.push_str(&format!("{}{}", level.stars, level.coins));
    }

    hash_string.push_str("xI25fpAapCQg");

    let hash_result = {
        let mut hasher = Sha1::new();
        hasher.update(hash_string.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let response_string = format!(
        "{}#{}#{}#{}:{}:10#{}",
        result_string_tr, user_string_tr, song_string_tr, levels_count.0, page_offset, hash_result
    );

    response_string.into_response()
}

async fn download_level(
    Extension(db): Extension<PgPool>,
    Form(data): Form<DownloadLevelRequest>,
) -> impl IntoResponse {
    let account_id = data.account_id.unwrap();

    let account = match sqlx::query!(
        "SELECT gjp2, username, account_id FROM accounts WHERE account_id = $1",
        account_id
    )
    .fetch_optional(&db)
    .await
    .unwrap()
    {
        Some(acc) => acc,
        None => return "-11".into_response(),
    };

    if data.hash != account.gjp2 {
        return "-11".into_response();
    }

    let level = match get_level_by_id(&db, data.level_id.unwrap()).await {
        Ok(level) => level,
        _ => return "-1".into_response(),
    };

    let level_string = match read(format!("./data/levels/{}", level.level_id)).await {
        Ok(content) => String::from_utf8(content).unwrap(),
        _ => level.level_string.unwrap(),
    };

    let mut out: String = format!("1:{}:2:{}:3:{}:4:{}:5:{}:6:{}:8:{}:9:{}:10:{}:12:{}:13:{}:14:{}:17:{}:43:{}:25:{}:18:{}:19:{}:42:{}:45:{}:15:{}:30:{}:31:{}:28:{}:29:{}:35:{}:36:{}:37:{}:38:{}:39:{}:46:{}:47:{}:40:{}:27:{}",
        level.level_id,
        level.level_name,
        level.level_desc,
        level_string,
        level.level_version,
        level.user_id,
        "10",
        level.star_difficulty,
        level.downloads,
        level.song_id,
        level.game_version,
        level.likes,
        level.star_demon,
        level.star_demon_diff,
        level.star_auto as u8,
        level.star_stars,
        level.star_featured,
        level.star_epic as u8,
        level.objects,
        level.level_length,
        level.original,
        level.two_player as u8,
        utilities::make_time(level.upload_date),
        utilities::make_time(level.update_date),
        "",
        level.extra_string,
        level.coins,
        level.star_coins as u8,
        level.requested_stars,
        "",
        "",
        level.is_ldm as u8,
        ""
    ).to_string();

    out = format!("{}#{}", out, crypto::hash_level_string(level_string));
    out = format!(
        "{}#{}",
        out,
        crypto::sha1_salt(
            &format!(
                "{},{},{},{},{},{},{},{}",
                level.user_id,
                level.star_stars,
                level.star_demon as u8,
                level.level_id,
                level.star_coins as u8,
                level.star_featured,
                level.password,
                0
            ),
            "xI25fpAapCQg"
        )
    );

    out.into_response()
}

pub fn init() -> Router {
    Router::new()
        .route("/database/uploadGJLevel21.php", post(upload_level))
        .route("/database/getGJLevels21.php", post(get_level))
        .route("/database/downloadGJLevel22.php", post(download_level))
}
