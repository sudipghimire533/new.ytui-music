use std::borrow::Cow;

pub enum Query {
    SearchAll(String),
    SearchMusic(String),
    SearchPlaylist(String),
    Local,
    Trending,
    FollowingArtist,
    SavedPlaylist,
    LikedMusic,
}

pub struct FinalQuery(Cow<'static, str>);

impl Query {
    pub fn get_final_query(self) -> FinalQuery {
        match self {
            Query::SearchAll(mut search_query) => {
                search_query.push_str(":all");
                FinalQuery(search_query.into())
            }
            Query::SearchMusic(mut search_query) => {
                search_query.push_str(":music");
                FinalQuery(search_query.into())
            }
            Query::SearchPlaylist(mut search_query) => {
                search_query.push_str(":playlist");
                FinalQuery(search_query.into())
            }
            Query::Trending => FinalQuery(":trending".into()),
            Query::LikedMusic => FinalQuery(":liked_music".into()),
            Query::SavedPlaylist => FinalQuery(":saved_playlist".into()),
            Query::FollowingArtist => FinalQuery(":following_artist".into()),
            Query::Local => FinalQuery(":local".into()),
        }
    }
}
