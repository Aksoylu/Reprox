use std::path::PathBuf;

use tokio::sync::watch;

use crate::{
    constants::Constants,
    core::router::Router,
    models::{config::Config, rpc_session::RpcSession},
    utils::{crypt_utility::generate_hash, file_utility::{get_runtime_path, load_json}},
};

pub struct Runtime {
    pub config: Config,
    pub rpc_session: RpcSession,
    pub router: Router,

    config_path: PathBuf,
    rpc_session_path: PathBuf,
    route_path: PathBuf,
}

impl Runtime {
    pub fn init() -> Self {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Runtime::get_rpc_session_path(&runtime_path);

        let config_path = Runtime::get_config_path(&runtime_path);
        let config = Runtime::load_config(config_path.clone());
        if config == Config::default() {
            // TODO: WRITE FILE BACK
        }

        let rpc_session = Runtime::load_rpc_session(rpc_session_path.clone());
        if rpc_session == RpcSession::default() {
            // TODO: WRITE FILE BACK
        }
        println!("rpc session hash >> {}", rpc_session.hash.clone());


        let route_path = Runtime::get_route_path(&runtime_path);
        let router = Router::load(route_path.clone());

        Self {
            config,
            rpc_session,
            router,

            config_path,
            rpc_session_path,
            route_path,
        }
    }

    pub fn update_config(&mut self, new_config: Config, runtime_sub: watch::Sender<()>) {
        self.config = new_config;
        let _ = runtime_sub.send(());
    }

    fn load_config(config_path: PathBuf) -> Config {
        let read_config_operation = load_json::<Config>(&config_path);
        if read_config_operation.is_err() {
            eprintln!(
                "Could not load default config file on path: {}.\nUsing default",
                config_path.to_str().unwrap_or_default()
            );
            return Config::default();
        }

        let config = read_config_operation.unwrap();
        let validation = config.validate();

        if validation.is_ok() {
            return config;
        } else {
            let error_text = validation.err().unwrap_or_default();
            eprintln!("Invalid configuration: {}", error_text);
            return Config::default();
        }
    }

    fn load_rpc_session(rpc_session_path: PathBuf) -> RpcSession {
        let read_rpc_session_operation = load_json::<RpcSession>(&rpc_session_path);
        if read_rpc_session_operation.is_err() {
            eprintln!(
                "Could not load RpcSession file on Path: {}.\nUsing default",
                rpc_session_path.to_str().unwrap_or_default()
            );
            return RpcSession::default();
        }

        let mut rpc_session = read_rpc_session_operation.unwrap();
        rpc_session.hash = generate_hash(rpc_session.private_key.clone());

        let validation = rpc_session.clone().validate();
        
        if validation.is_ok() {
            return rpc_session;
        } else {
            let error_text = validation.err().unwrap_or_default();
            eprintln!("Invalid Rpc Session: {}\nUsing default", error_text);
            return RpcSession::default();
        }
    }

    fn get_config_path(runtime_path: &PathBuf) -> PathBuf {
        let mut config_path = runtime_path.clone();
        config_path.push(Constants::SETTINGS_FILENAME);
        config_path
    }

    fn get_route_path(runtime_path: &PathBuf) -> PathBuf {
        let mut runtime_path = runtime_path.clone();
        runtime_path.push(Constants::ROUTER_FILENAME);
        runtime_path
    }

    fn get_rpc_session_path(runtime_path: &PathBuf) -> PathBuf {
        let mut session_path = runtime_path.clone();
        session_path.push(Constants::SESSION_FILENAME);
        session_path
    }
}
