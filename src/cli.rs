use structopt::StructOpt;
use drs_cli::apis;


#[derive(Debug, StructOpt)]
#[structopt(name = "Cli", about = "A CLI for interacting with DRS APIs")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    // subcommands
    #[structopt(about = "get service-info")]
    Service,
    #[structopt(about = "get info about drs object through object id")]
    Object{
        #[structopt(about = "object id as argument")]
        obj_id: String,
    },
    #[structopt(about = "get url for fetching data through access id")]
    Access{
        #[structopt(about = "object id as first argument")]
        obj_id: String,
        #[structopt(about = "access id as first argument")]
        access_id: String,
    },
    #[structopt(about = "Get Authorization info about a DrsObject")]
    Authinfo{
        #[structopt(about = "object id as first argument")]
        obj_id: String,
    }
}

// logic for each subcommand
fn execute_command(command: Command) {
    match command {
        Command::Service {} => {    
            let config = apis::configuration::Configuration::default();
            let service_info = apis::service_info_api::get_service_info(&config);
                
            println!("{:#?}", service_info);
        },
        Command::Object {obj_id} => {
            let config = apis::configuration::Configuration::default();
            let obj_id = obj_id;
            let object_info = apis::objects_api::get_object(&config, &obj_id, Some(false));

            println!("{:#?}", object_info);
        },
        Command::Access {obj_id, access_id} => {
            let config = apis::configuration::Configuration::default();
            let obj_id = obj_id;
            let access_id = access_id;
            let data_url = apis::objects_api::get_access_url(&config, &obj_id, &access_id);

            println!("{:#?}", data_url);
        },
        Command::Authinfo {obj_id} => {
            let config = apis::configuration::Configuration::default();
            let obj_id = obj_id;
            let auth_info = apis::objects_api::options_object(&config, &obj_id);
            
            println!("{:#?}",auth_info);
        }
    }
}

pub fn main() {
    let cli = Cli::from_args();

    execute_command(cli.cmd);
}
