use rbatis::rbatis::Rbatis;
use crate::config::config::AppConfig;

pub struct ServiceContext {
    pub config: AppConfig,
    pub r_batis: Rbatis,
}

impl ServiceContext {
    pub async fn link_db(&self) {
        //连接数据库
        println!("连接数据库，数据库地址： ({})...", self.config.database_url);
        let result = self.r_batis.link(&*self.config.database_url).await;
        if let Err(e) = result {
            println!("--------------------------数据库连接失败--------------------------");
            println!("{}", e);
            return;
        }
        println!("--------------------------数据库连接成功--------------------------");
        log::info!(
            " - Local：http:// {}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            config: AppConfig::default(),
            r_batis: Rbatis::new(),
        }
    }
}
lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}