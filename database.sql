-- PostgreSQL port of GDPS database schema
-- Converted from MySQL/phpMyAdmin

BEGIN;

--
-- Table structure for table 'acc_comments'
--

CREATE TABLE acc_comments (
  user_id INT NOT NULL,
  username VARCHAR(50) NOT NULL,
  comment TEXT NOT NULL,
  secret VARCHAR(10) NOT NULL DEFAULT 'unused',
  comment_id SERIAL PRIMARY KEY,
  timestamp INT NOT NULL,
  likes INT NOT NULL DEFAULT 0,
  is_spam INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'accounts'
--

CREATE TABLE accounts (
  username VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  gjp2 VARCHAR(255) DEFAULT NULL,
  email VARCHAR(255) NOT NULL,
  account_id SERIAL PRIMARY KEY,
  is_admin INT NOT NULL DEFAULT 0,
  ms INT NOT NULL DEFAULT 0,
  fr_s INT NOT NULL DEFAULT 0,
  cs INT NOT NULL DEFAULT 0,
  youtube_url VARCHAR(255) NOT NULL DEFAULT '',
  twitter VARCHAR(255) NOT NULL DEFAULT '',
  twitch VARCHAR(255) NOT NULL DEFAULT '',
  salt VARCHAR(255) NOT NULL DEFAULT '',
  register_date INT NOT NULL DEFAULT 0,
  friends_count INT NOT NULL DEFAULT 0,
  discord_id BIGINT NOT NULL DEFAULT 0,
  discord_link_req BIGINT NOT NULL DEFAULT 0,
  is_active BOOLEAN NOT NULL DEFAULT FALSE
);

--
-- Table structure for table 'actions'
--

CREATE TABLE actions (
  id SERIAL PRIMARY KEY,
  type INT NOT NULL DEFAULT 0,
  value VARCHAR(255) NOT NULL DEFAULT '0',
  timestamp INT NOT NULL DEFAULT 0,
  value2 VARCHAR(255) NOT NULL DEFAULT '0',
  value3 INT NOT NULL DEFAULT 0,
  value4 INT NOT NULL DEFAULT 0,
  value5 INT NOT NULL DEFAULT 0,
  value6 INT NOT NULL DEFAULT 0,
  account INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'actions_downloads'
--

CREATE TABLE actions_downloads (
  id SERIAL PRIMARY KEY,
  level_id INT NOT NULL,
  ip BYTEA NOT NULL,
  upload_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--
-- Table structure for table 'actions_likes'
--

CREATE TABLE actions_likes (
  id SERIAL PRIMARY KEY,
  item_id INT NOT NULL,
  type INT NOT NULL,
  is_like SMALLINT NOT NULL,
  ip BYTEA NOT NULL,
  upload_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--
-- Table structure for table 'banned_ips'
--

CREATE TABLE banned_ips (
  ip VARCHAR(255) NOT NULL DEFAULT '127.0.0.1',
  id SERIAL PRIMARY KEY
);

--
-- Table structure for table 'blocks'
--

CREATE TABLE blocks (
  id SERIAL PRIMARY KEY,
  person1 INT NOT NULL,
  person2 INT NOT NULL
);

--
-- Table structure for table 'comments'
--

CREATE TABLE comments (
  user_id INT NOT NULL,
  username VARCHAR(50) NOT NULL,
  comment TEXT NOT NULL,
  secret VARCHAR(10) NOT NULL DEFAULT 'none',
  level_id INT NOT NULL,
  comment_id SERIAL PRIMARY KEY,
  timestamp INT NOT NULL,
  likes INT NOT NULL DEFAULT 0,
  percent INT NOT NULL DEFAULT 0,
  is_spam BOOLEAN NOT NULL DEFAULT FALSE
);

--
-- Table structure for table 'cp_shares'
--

CREATE TABLE cp_shares (
  share_id SERIAL PRIMARY KEY,
  level_id INT NOT NULL,
  user_id INT NOT NULL
);

--
-- Table structure for table 'daily_features'
--

CREATE TABLE daily_features (
  fea_id SERIAL PRIMARY KEY,
  level_id INT NOT NULL,
  timestamp INT NOT NULL,
  type INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'friend_requests'
--

CREATE TABLE friend_requests (
  account_id INT NOT NULL,
  to_account_id INT NOT NULL,
  comment VARCHAR(1000) NOT NULL,
  upload_date INT NOT NULL,
  id SERIAL PRIMARY KEY,
  is_new BOOLEAN NOT NULL DEFAULT TRUE
);

--
-- Table structure for table 'friendships'
--

CREATE TABLE friendships (
  id SERIAL PRIMARY KEY,
  person1 INT NOT NULL,
  person2 INT NOT NULL,
  is_new1 INT NOT NULL,
  is_new2 INT NOT NULL
);

--
-- Table structure for table 'gauntlets'
--

CREATE TABLE gauntlets (
  id SERIAL PRIMARY KEY,
  level1 INT NOT NULL,
  level2 INT NOT NULL,
  level3 INT NOT NULL,
  level4 INT NOT NULL,
  level5 INT NOT NULL
);

--
-- Table structure for table 'levels'
--

CREATE TABLE levels (
  game_version INT NOT NULL,
  binary_version INT NOT NULL DEFAULT 0,
  username TEXT NOT NULL,
  level_id SERIAL PRIMARY KEY,
  level_name VARCHAR(255) NOT NULL,
  level_desc TEXT NOT NULL,
  level_version INT NOT NULL,
  level_length INT NOT NULL DEFAULT 0,
  audio_track INT NOT NULL,
  auto INT NOT NULL,
  password INT NOT NULL,
  original INT NOT NULL,
  two_player INT NOT NULL DEFAULT 0,
  song_id INT NOT NULL DEFAULT 0,
  song_ids VARCHAR(2048) DEFAULT '',
  sfx_ids VARCHAR(2048) DEFAULT '',
  objects INT NOT NULL DEFAULT 0,
  coins INT NOT NULL DEFAULT 0,
  requested_stars INT NOT NULL DEFAULT 0,
  extra_string TEXT NOT NULL,
  level_string TEXT,
  level_info TEXT NOT NULL,
  secret TEXT NOT NULL,
  star_difficulty INT NOT NULL DEFAULT 0, -- 0=N/A 10=EASY 20=NORMAL 30=HARD 40=HARDER 50=INSANE 50=AUTO 50=DEMON
  downloads INT NOT NULL DEFAULT 300,
  likes INT NOT NULL DEFAULT 100,
  star_demon INT NOT NULL DEFAULT 0,
  star_auto SMALLINT NOT NULL DEFAULT 0,
  star_stars INT NOT NULL DEFAULT 0,
  upload_date BIGINT NOT NULL,
  update_date BIGINT NOT NULL,
  rate_date BIGINT NOT NULL DEFAULT 0,
  star_coins INT NOT NULL DEFAULT 0,
  star_featured INT NOT NULL DEFAULT 0,
  star_hall INT NOT NULL DEFAULT 0,
  star_epic INT NOT NULL DEFAULT 0,
  star_demon_diff INT NOT NULL DEFAULT 0,
  user_id INT NOT NULL,
  ext_id VARCHAR(255) NOT NULL,
  unlisted INT NOT NULL,
  original_reup INT NOT NULL DEFAULT 0, -- used for levelReupload.php
  hostname VARCHAR(255) NOT NULL,
  is_cp_shared INT NOT NULL DEFAULT 0,
  is_deleted INT NOT NULL DEFAULT 0,
  is_ldm INT NOT NULL DEFAULT 0,
  unlisted2 INT NOT NULL DEFAULT 0,
  wt INT NOT NULL DEFAULT 0,
  wt2 INT NOT NULL DEFAULT 0,
  ts INT NOT NULL DEFAULT 0,
  settings_string TEXT NOT NULL
);

--
-- Table structure for table 'level_scores'
--

CREATE TABLE level_scores (
  score_id SERIAL PRIMARY KEY,
  account_id INT NOT NULL,
  level_id INT NOT NULL,
  percent INT NOT NULL,
  upload_date INT NOT NULL,
  attempts INT NOT NULL DEFAULT 0,
  coins INT NOT NULL DEFAULT 0,
  clicks INT NOT NULL DEFAULT 0,
  time INT NOT NULL DEFAULT 0,
  progresses TEXT NOT NULL,
  daily_id INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'links'
--

CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL,
  target_account_id INT NOT NULL,
  server VARCHAR(255) NOT NULL,
  timestamp INT NOT NULL,
  user_id INT NOT NULL,
  target_user_id INT NOT NULL
);

--
-- Table structure for table 'lists'
--

CREATE TABLE lists (
  list_id SERIAL PRIMARY KEY,
  list_name VARCHAR(2048) NOT NULL,
  list_desc VARCHAR(2048) NOT NULL,
  list_version INT NOT NULL DEFAULT 1,
  account_id INT NOT NULL,
  downloads INT NOT NULL DEFAULT 0,
  star_difficulty INT NOT NULL,
  likes INT NOT NULL DEFAULT 0,
  star_featured INT NOT NULL DEFAULT 0,
  star_stars INT NOT NULL DEFAULT 0,
  list_levels VARCHAR(2048) NOT NULL,
  count_for_reward INT NOT NULL DEFAULT 0,
  upload_date INT NOT NULL DEFAULT 0,
  update_date INT NOT NULL DEFAULT 0,
  original INT NOT NULL DEFAULT 0,
  unlisted INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'map_packs'
--

CREATE TABLE map_packs (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  levels VARCHAR(512) NOT NULL, -- entered as "ID of level 1, ID of level 2, ID of level 3" for example "13,14,15" (without the "s)
  stars INT NOT NULL,
  coins INT NOT NULL,
  difficulty INT NOT NULL,
  rgb_colors VARCHAR(11) NOT NULL, -- entered as R,G,B
  colors2 VARCHAR(11) NOT NULL DEFAULT 'none'
);

--
-- Table structure for table 'messages'
--

CREATE TABLE messages (
  user_id INT NOT NULL,
  username VARCHAR(50) NOT NULL,
  body TEXT NOT NULL,
  subject TEXT NOT NULL,
  acc_id INT NOT NULL,
  message_id SERIAL PRIMARY KEY,
  to_account_id INT NOT NULL,
  timestamp INT NOT NULL,
  secret VARCHAR(25) NOT NULL DEFAULT 'unused',
  is_new INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'mod_actions'
--

CREATE TABLE mod_actions (
  id SERIAL PRIMARY KEY,
  type INT NOT NULL DEFAULT 0,
  value VARCHAR(255) NOT NULL DEFAULT '0',
  timestamp INT NOT NULL DEFAULT 0,
  value2 VARCHAR(255) NOT NULL DEFAULT '0',
  value3 INT NOT NULL DEFAULT 0,
  value4 VARCHAR(255) NOT NULL DEFAULT '0',
  value5 INT NOT NULL DEFAULT 0,
  value6 INT NOT NULL DEFAULT 0,
  account INT NOT NULL DEFAULT 0,
  value7 VARCHAR(255) NOT NULL DEFAULT '0'
);

--
-- Table structure for table 'mod_ip_perms'
--

CREATE TABLE mod_ip_perms (
  category_id SERIAL PRIMARY KEY,
  action_free_copy INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'mod_ips'
--

CREATE TABLE mod_ips (
  id SERIAL PRIMARY KEY,
  ip VARCHAR(69) NOT NULL,
  is_mod INT NOT NULL,
  account_id INT NOT NULL,
  modip_category INT NOT NULL
);

--
-- Table structure for table 'plat_scores'
--

CREATE TABLE plat_scores (
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL DEFAULT 0,
  level_id INT NOT NULL DEFAULT 0,
  time INT NOT NULL DEFAULT 0,
  points INT NOT NULL DEFAULT 0,
  timestamp INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'quests'
--

CREATE TABLE quests (
  id SERIAL PRIMARY KEY,
  type INT NOT NULL,
  amount INT NOT NULL,
  reward INT NOT NULL,
  name VARCHAR(255) NOT NULL
);

--
-- Table structure for table 'reports'
--

CREATE TABLE reports (
  id SERIAL PRIMARY KEY,
  level_id INT NOT NULL,
  hostname VARCHAR(255) NOT NULL
);

--
-- Table structure for table 'role_assign'
--

CREATE TABLE role_assign (
  assign_id BIGSERIAL PRIMARY KEY,
  role_id BIGINT NOT NULL,
  account_id BIGINT NOT NULL
);

--
-- Table structure for table 'roles'
--

CREATE TABLE roles (
  role_id BIGSERIAL PRIMARY KEY,
  priority INT NOT NULL DEFAULT 0,
  role_name VARCHAR(255) NOT NULL,
  command_rate INT NOT NULL DEFAULT 0,
  command_feature INT NOT NULL DEFAULT 0,
  command_epic INT NOT NULL DEFAULT 0,
  command_unepic INT NOT NULL DEFAULT 0,
  command_verifycoins INT NOT NULL DEFAULT 0,
  command_daily INT NOT NULL DEFAULT 0,
  command_weekly INT NOT NULL DEFAULT 0,
  command_delete INT NOT NULL DEFAULT 0,
  command_setacc INT NOT NULL DEFAULT 0,
  command_rename_own INT NOT NULL DEFAULT 1,
  command_rename_all INT NOT NULL DEFAULT 0,
  command_pass_own INT NOT NULL DEFAULT 1,
  command_pass_all INT NOT NULL DEFAULT 0,
  command_description_own INT NOT NULL DEFAULT 1,
  command_description_all INT NOT NULL DEFAULT 0,
  command_public_own INT NOT NULL DEFAULT 1,
  command_public_all INT NOT NULL DEFAULT 0,
  command_unlist_own INT NOT NULL DEFAULT 1,
  command_unlist_all INT NOT NULL DEFAULT 0,
  command_sharecp_own INT NOT NULL DEFAULT 1,
  command_sharecp_all INT NOT NULL DEFAULT 0,
  command_song_own INT NOT NULL DEFAULT 1,
  command_song_all INT NOT NULL DEFAULT 0,
  profilecommand_discord INT NOT NULL DEFAULT 1,
  action_rate_demon INT NOT NULL DEFAULT 0,
  action_rate_stars INT NOT NULL DEFAULT 0,
  action_rate_difficulty INT NOT NULL DEFAULT 0,
  action_request_mod INT NOT NULL DEFAULT 0,
  action_suggest_rating INT NOT NULL DEFAULT 0,
  action_delete_comment INT NOT NULL DEFAULT 0,
  tool_leaderboardsban INT NOT NULL DEFAULT 0,
  tool_packcreate INT NOT NULL DEFAULT 0,
  tool_quests_create INT NOT NULL DEFAULT 0,
  tool_modactions INT NOT NULL DEFAULT 0,
  tool_suggestlist INT NOT NULL DEFAULT 0,
  dashboard_mod_tools INT NOT NULL DEFAULT 0,
  modip_category INT NOT NULL DEFAULT 0,
  is_default INT NOT NULL DEFAULT 0,
  comment_color VARCHAR(11) NOT NULL DEFAULT '000,000,000',
  mod_badge_level INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'songs'
--

CREATE TABLE songs (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  author_id INT NOT NULL,
  author_name VARCHAR(100) NOT NULL,
  size VARCHAR(100) NOT NULL,
  download VARCHAR(1337) NOT NULL,
  hash VARCHAR(256) NOT NULL DEFAULT '',
  is_disabled INT NOT NULL DEFAULT 0,
  levels_count INT NOT NULL DEFAULT 0,
  reupload_time INT NOT NULL DEFAULT 0
);

--
-- Table structure for table 'suggest'
--

CREATE TABLE suggest (
  id SERIAL PRIMARY KEY,
  suggest_by INT NOT NULL DEFAULT 0,
  suggest_level_id INT NOT NULL DEFAULT 0,
  suggest_difficulty INT NOT NULL DEFAULT 0, -- 0 - NA 10 - Easy 20 - Normal 30 - Hard 40 - Harder 50 - Insane/Demon/Auto
  suggest_stars INT NOT NULL DEFAULT 0,
  suggest_featured INT NOT NULL DEFAULT 0,
  suggest_auto INT NOT NULL DEFAULT 0,
  suggest_demon INT NOT NULL DEFAULT 0,
  timestamp INT NOT NULL
);

--
-- Table structure for table 'users'
--

CREATE TABLE users (
  is_registered INT NOT NULL,
  user_id SERIAL PRIMARY KEY,
  ext_id VARCHAR(100) NOT NULL,
  username VARCHAR(69) NOT NULL DEFAULT 'undefined',
  stars INT NOT NULL DEFAULT 0,
  demons INT NOT NULL DEFAULT 0,
  icon INT NOT NULL DEFAULT 0,
  color1 INT NOT NULL DEFAULT 0,
  color2 INT NOT NULL DEFAULT 3,
  color3 INT NOT NULL DEFAULT 0,
  icon_type INT NOT NULL DEFAULT 0,
  coins INT NOT NULL DEFAULT 0,
  user_coins INT NOT NULL DEFAULT 0,
  special INT NOT NULL DEFAULT 0,
  game_version INT NOT NULL DEFAULT 0,
  secret VARCHAR(69) NOT NULL DEFAULT 'none',
  acc_icon INT NOT NULL DEFAULT 0,
  acc_ship INT NOT NULL DEFAULT 0,
  acc_ball INT NOT NULL DEFAULT 0,
  acc_bird INT NOT NULL DEFAULT 0,
  acc_dart INT NOT NULL DEFAULT 0,
  acc_robot INT DEFAULT 0,
  acc_glow INT NOT NULL DEFAULT 0,
  acc_swing INT NOT NULL DEFAULT 0,
  acc_jetpack INT NOT NULL DEFAULT 0,
  dinfo VARCHAR(100) DEFAULT '',
  sinfo VARCHAR(100) DEFAULT '',
  pinfo VARCHAR(100) DEFAULT '',
  creator_points DOUBLE PRECISION NOT NULL DEFAULT 0,
  ip VARCHAR(69) NOT NULL DEFAULT '127.0.0.1',
  last_played INT NOT NULL DEFAULT 0,
  diamonds INT NOT NULL DEFAULT 0,
  moons INT NOT NULL DEFAULT 0,
  orbs INT NOT NULL DEFAULT 0,
  completed_lvls INT NOT NULL DEFAULT 0,
  acc_spider INT NOT NULL DEFAULT 0,
  acc_explosion INT NOT NULL DEFAULT 0,
  chest1_time INT NOT NULL DEFAULT 0,
  chest2_time INT NOT NULL DEFAULT 0,
  chest1_count INT NOT NULL DEFAULT 0,
  chest2_count INT NOT NULL DEFAULT 0,
  is_banned INT NOT NULL DEFAULT 0,
  is_creator_banned INT NOT NULL DEFAULT 0
);

-- Create indexes

-- acc_comments
CREATE INDEX idx_acc_comments_userid ON acc_comments(user_id);
CREATE INDEX idx_acc_comments_timestamp ON acc_comments(timestamp);

-- accounts
CREATE INDEX idx_accounts_isadmin ON accounts(is_admin);
CREATE INDEX idx_accounts_frs ON accounts(fr_s);
CREATE INDEX idx_accounts_discordid ON accounts(discord_id);
CREATE INDEX idx_accounts_discordlinkreq ON accounts(discord_link_req);
CREATE INDEX idx_accounts_friendscount ON accounts(friends_count);
CREATE INDEX idx_accounts_isactive ON accounts(is_active);

-- actions
CREATE INDEX idx_actions_type ON actions(type);
CREATE INDEX idx_actions_value ON actions(value);
CREATE INDEX idx_actions_value2 ON actions(value2);
CREATE INDEX idx_actions_timestamp ON actions(timestamp);

-- actions_downloads
CREATE INDEX idx_actions_downloads_levelid_ip_date ON actions_downloads(level_id, ip, upload_date);

-- actions_likes
CREATE INDEX idx_actions_likes_item_type_islike_ip_date ON actions_likes(item_id, type, is_like, ip, upload_date);

-- blocks
CREATE INDEX idx_blocks_person1 ON blocks(person1);
CREATE INDEX idx_blocks_person2 ON blocks(person2);

-- comments
CREATE INDEX idx_comments_levelid ON comments(level_id);
CREATE INDEX idx_comments_userid ON comments(user_id);
CREATE INDEX idx_comments_likes ON comments(likes);

-- cp_shares
CREATE INDEX idx_cp_shares_levelid ON cp_shares(level_id);

-- daily_features
CREATE INDEX idx_daily_features_type ON daily_features(type);
CREATE INDEX idx_daily_features_timestamp ON daily_features(timestamp);

-- friend_requests
CREATE INDEX idx_friend_requests_toaccount ON friend_requests(to_account_id);
CREATE INDEX idx_friend_requests_account ON friend_requests(account_id);
CREATE INDEX idx_friend_requests_uploaddate ON friend_requests(upload_date);

-- friendships
CREATE INDEX idx_friendships_person1 ON friendships(person1);
CREATE INDEX idx_friendships_person2 ON friendships(person2);
CREATE INDEX idx_friendships_isnew1 ON friendships(is_new1);
CREATE INDEX idx_friendships_isnew2 ON friendships(is_new2);

-- gauntlets
CREATE INDEX idx_gauntlets_level5 ON gauntlets(level5);

-- levels
CREATE INDEX idx_levels_levelname ON levels(level_name);
CREATE INDEX idx_levels_stardifficulty ON levels(star_difficulty);
CREATE INDEX idx_levels_starfeatured ON levels(star_featured);
CREATE INDEX idx_levels_starepic ON levels(star_epic);
CREATE INDEX idx_levels_stardemondiff ON levels(star_demon_diff);
CREATE INDEX idx_levels_userid ON levels(user_id);
CREATE INDEX idx_levels_likes ON levels(likes);
CREATE INDEX idx_levels_downloads ON levels(downloads);
CREATE INDEX idx_levels_starstars ON levels(star_stars);
CREATE INDEX idx_levels_songid ON levels(song_id);
CREATE INDEX idx_levels_audiotrack ON levels(audio_track);
CREATE INDEX idx_levels_levellength ON levels(level_length);
CREATE INDEX idx_levels_twoplayer ON levels(two_player);
CREATE INDEX idx_levels_stardemon ON levels(star_demon);
CREATE INDEX idx_levels_starauto ON levels(star_auto);
CREATE INDEX idx_levels_extid ON levels(ext_id);
CREATE INDEX idx_levels_uploaddate ON levels(upload_date);
CREATE INDEX idx_levels_updatedate ON levels(update_date);
CREATE INDEX idx_levels_starcoins ON levels(star_coins);
CREATE INDEX idx_levels_coins ON levels(coins);
CREATE INDEX idx_levels_password ON levels(password);
CREATE INDEX idx_levels_originalreup ON levels(original_reup);
CREATE INDEX idx_levels_original ON levels(original);
CREATE INDEX idx_levels_unlisted ON levels(unlisted);
CREATE INDEX idx_levels_iscpshared ON levels(is_cp_shared);
CREATE INDEX idx_levels_gameversion ON levels(game_version);
CREATE INDEX idx_levels_ratedate ON levels(rate_date);
CREATE INDEX idx_levels_objects ON levels(objects);
CREATE INDEX idx_levels_unlisted2 ON levels(unlisted2);

-- level_scores
CREATE INDEX idx_level_scores_levelid ON level_scores(level_id);
CREATE INDEX idx_level_scores_accountid ON level_scores(account_id);

-- links
CREATE INDEX idx_links_targetuserid ON links(target_user_id);
CREATE INDEX idx_links_targetaccountid ON links(target_account_id);
CREATE INDEX idx_links_server ON links(server);

-- messages
CREATE INDEX idx_messages_toaccount ON messages(to_account_id);
CREATE INDEX idx_messages_acc ON messages(acc_id);

-- mod_actions
CREATE INDEX idx_mod_actions_account ON mod_actions(account);
CREATE INDEX idx_mod_actions_type ON mod_actions(type);
CREATE INDEX idx_mod_actions_value3 ON mod_actions(value3);

-- mod_ips
CREATE INDEX idx_mod_ips_accountid ON mod_ips(account_id);
CREATE INDEX idx_mod_ips_ip ON mod_ips(ip);

-- reports
CREATE INDEX idx_reports_levelid ON reports(level_id);
CREATE INDEX idx_reports_hostname ON reports(hostname);

-- role_assign
CREATE INDEX idx_role_assign_roleid ON role_assign(role_id);
CREATE INDEX idx_role_assign_accountid ON role_assign(account_id);

-- roles
CREATE INDEX idx_roles_priority ON roles(priority);
CREATE INDEX idx_roles_toolmodactions ON roles(tool_modactions);

-- songs
CREATE INDEX idx_songs_name ON songs(name);
CREATE INDEX idx_songs_authorname ON songs(author_name);

-- suggest
CREATE INDEX idx_suggest_timestamp ON suggest(timestamp);

-- users
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_stars ON users(stars);
CREATE INDEX idx_users_demons ON users(demons);
CREATE INDEX idx_users_coins ON users(coins);
CREATE INDEX idx_users_usercoins ON users(user_coins);
CREATE INDEX idx_users_gameversion ON users(game_version);
CREATE INDEX idx_users_creatorpoints ON users(creator_points);
CREATE INDEX idx_users_diamonds ON users(diamonds);
CREATE INDEX idx_users_orbs ON users(orbs);
CREATE INDEX idx_users_completedlvls ON users(completed_lvls);
CREATE INDEX idx_users_isbanned ON users(is_banned);
CREATE INDEX idx_users_iscreatorbanned ON users(is_creator_banned);
CREATE INDEX idx_users_extid ON users(ext_id);
CREATE INDEX idx_users_ip ON users(ip);
CREATE INDEX idx_users_isregistered ON users(is_registered);

COMMIT;