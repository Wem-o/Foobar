use argpars::{Argpars, ArgsObj};

use crate::state::State;

mod state;

#[derive(Debug, Clone)]
enum Mode {
    Launhcer,
    Finder,
    /// Dmenu like operation. First string is the separator, second is
    /// the file to read the elements from.
    Dmenu(Option<String>, Option<String>)
}
impl TryFrom<ArgsObj> for Mode {
    type Error = &'static str;
    
    fn try_from(args: ArgsObj) -> Result<Self, Self::Error> {
        return if args.passed("-d") || args.passed("-dmenu") {            
            let sep = if args.passed("--sep") {
                let sep = args
                    .get_parameter_for("--sep")
                    .to_string();
                
                if !sep.is_empty() {Some(sep)}
                else {
                    return Err("--sep mustn't be empty!");
                }
            } else {
                None
            };

            let file = if args.passed("--from-file") {
                let path = args
                    .get_parameter_for("--from-file")
                    .to_string();

                if !path.trim().is_empty() {Some(path)}
                else {
                    return Err("--from-file passed but no filepath was passed after it!");
                }
            } else {None};
            
            Ok(Mode::Dmenu(sep, file))
        }
        else if args.passed("-l") || args.passed("-launcher") {Ok(Mode::Launhcer)}
        else if args.no_arguments_passed() || args.passed("-f") 
            || args.passed("-finder") {Ok(Mode::Finder)}
        else {
                return Err("Invalid parameters. To see options see --help");
        };
    }
}

fn main() -> Result<(), &'static str> {
    let mut args: ArgsObj = Argpars::new();

    // Info about fooselector
    
    args.help_name = "FooSelector".to_string();
    args.help_description = "Fooselector is Foobar's selector (demnu / rofi like\
        ), application launcher, and general (file, app, text) finder dialog!"
    .to_string();
    args.help_version = format!("{} ({})", 
        env!("CARGO_PKG_VERSION"), 
        if env!("VERGEN_CARGO_DEBUG") == "true" {"Debug"} else {"Release"}
    );
    args.help_usage = "Usage: fooselector [FLAGS] (INPUT IF DMENU MODE)".to_string();

    // Our arguments
    args.add_argument("-finder   -f", "Smart finder mode  (default if no args passed)");
    args.add_argument("-launcher -l", "App launcher mode");
    args.add_argument("-dmenu    -d", "Dmenu like mode");
    args.add_argument("--from-file <FILE>", "If started in dmenu mode \
        specifies the file to read options from instead of stdin");
    args.add_argument("--sep <SEP>", "If started in dmenu mode specifies \
        the separator used to split the input into options.");

    if args.passed("-h") || args.passed("--help") {
        args.display_help_screen();
        return Ok(());
    } else if args.passed("--version") || args.passed("-v") {
        println!("{}", args.help_version);
        return Ok(());
    }

    let mode = Mode::try_from(args)?;

    println!("Mode is: {mode:#?}");

    // iced_layershell::application(
    //     || State::new(mode), 
    //     "Fooselector", 
    //     State::update, 
    //     State::view
    // );

    Ok(())
}
