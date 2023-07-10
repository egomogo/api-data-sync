pub enum Const {
    TestKey,
    KakaoRestApiKey,
    KakaoRestApiUrl,
    DB_URL,
}

impl Const {
    pub fn value(&self) -> String {
        let prefix = self.get_env_value("API_DATA_SYNC");
        let key = match self {
            Self::TestKey => "TEST_KEY",
            Self::KakaoRestApiKey => "KAKAO_REST_API_KEY",
            Self::KakaoRestApiUrl => "KAKAO_REST_API_URL",
            Self::DB_URL => "EGOMOGO_DATABASE_URL",
        };
        self.get_env_value(format!("{prefix}_{key}").as_str())
    }

    fn get_env_value(&self, key: &str) -> String {
        match std::env::var(key) {
            Ok(v) => v,
            Err(e) => panic!("{key} {e:?}"),
        }
    }
}

pub mod geo {
    use crate::error;

    pub fn is_lat(v: f64) -> bool {
        v <= 90.0 && v >= -90.0
    }
    pub fn is_long(v: f64) -> bool {
        v <= 180.0 && v >= -180.0
    }
    pub fn assert_lat_range(v: f64) -> Result<(), error::Error> {
        if !is_lat(v) {
            return Err(error::Error::InvalidLatitudeRange);
        }
        Ok(())
    }
    pub fn assert_long_range(v: f64) -> Result<(), error::Error> {
        if !is_long(v) {
            return Err(error::Error::InvalidLongitudeRange);
        }
        Ok(())
    }
}
