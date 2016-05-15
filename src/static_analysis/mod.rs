mod manifest;
mod code;

use self::manifest::*;
use self::code::*;
use results::Results;

pub fn static_analysis(app_id: &str,
                       verbose: bool,
                       quiet: bool,
                       force: bool,
                       results: &mut Results) {
    if verbose {
        println!("It's time to analyse the application. First, a static analysis will be \
                  performed, starting with the AndroidManifest.xml file and then going through \
                  the actual code. Let's start!");
    }

    manifest_analysis(app_id, verbose, quiet, force, results);
    // TODO Code analysis
}