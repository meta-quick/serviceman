use clap::{*,Parser, Subcommand};

use service_manager::*;
use std::{ffi::OsString, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        //service_name: String,
        #[clap(short, long, value_parser)]
        service: String,

        //executable binary,
        #[clap(short, long, value_parser)]
        executable: String,

        #[clap(short, long, value_parser)]
        args: String,
    },
    Remove {
        //service_name: String,
        #[clap(short, long, value_parser)]
        service: String,
    },
    Start {
        //service_name: String,
        #[clap(short, long, value_parser)]
        service: String,
    },
    Stop {
        //service_name: String,
        #[clap(short, long, value_parser)]
        service: String,
    },
}


struct Service {
    manager: Box<dyn ServiceManager>,
}

impl Service {
    fn new() -> Box<Self> {
        let mut manager = <dyn ServiceManager>::native()
            .expect("Failed to detect management platform");
        // manager.set_level(ServiceLevel::User).expect("failed to set service level");
        // manager.config.install.keep_alive = false;
        Box::new(Self { manager })
    }

    fn install(&self, service_name: String, executable: String, args: Vec<OsString>) {
        self.manager.install(ServiceInstallCtx {
            label: service_name.parse().unwrap(),
            program: PathBuf::from(executable),
            args,
        }).expect("Failed to install");
    }

    fn uninstall(&self, service_name: String) {
        self.manager.uninstall(ServiceUninstallCtx {
            label:  service_name.parse().unwrap(),
        }).expect("Failed to stop");
    }

    fn start(&self, service_name: String) {
        self.manager.start(ServiceStartCtx {
            label:  service_name.parse().unwrap(),
        }).expect("Failed to start");
    }

    fn stop(&self, service_name: String) {
        self.manager.stop(ServiceStopCtx {
            label:  service_name.parse().unwrap(),
        }).expect("Failed to stop");
    }

    fn run(&self, args: Vec<OsString>) {
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Install { service, executable, args } => {
                println!("Installing service: {}", service);
                println!("Executable: {}", executable);

                let os_args: Vec<OsString> = args.split(" ").map(|x| OsString::from(x)).collect();

                self.install(service, executable, os_args);
            },
            Commands::Remove { service } => {
                println!("Removing service: {}", service);
                self.uninstall(service);
            },
            Commands::Start { service } => {
                println!("Starting service: {}", service);
                self.start(service);
            },
            Commands::Stop { service } => {
                println!("Stopping service: {}", service);
                self.stop(service);
            },
        }
    }
}


fn main() {
    let service = Service::new();
    service.run(std::env::args_os().collect());
}
