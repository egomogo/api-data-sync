pub enum Const {
    TestKey,
    KakaoRestApiKey,
    KakaoRestApiUrl,
}

impl Const {
    pub fn value(&self) -> String {
        let prefix = self.get_env_value("API_DATA_SYNC");
        let key = match self {
            Self::TestKey => "TEST_KEY",
            Self::KakaoRestApiKey => "KAKAO_REST_API_KEY",
            Self::KakaoRestApiUrl => "KAKAO_REST_API_URL",
        };
        self.get_env_value(format!("{prefix}_{key}").as_str())
    }

    fn get_env_value(&self, key: &str) -> String {
        match std::env::var(key) {
            Ok(v) => v,
            Err(e) => panic!("{key}\n{e:?}"),
        }
    }
}
