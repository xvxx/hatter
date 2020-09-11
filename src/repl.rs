use {
    crate::{compiler, VM},
    rustyline::{error::ReadlineError, Editor},
    std::io,
};

/// Start the REPL.
pub fn run() -> Result<(), io::Error> {
    banner();
    let mut vm = VM::new(true);
    let history_file = ".hatter_history";

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_file);
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let _ = compiler::compile(&line)
                    .and_then(|codes| vm.run(codes))
                    .map_err(|e| eprintln!("Error: {:?}", e));
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(history_file).unwrap();
    Ok(())
}

fn banner() {
    let (purple, blue, gold, white, clear) = if std::env::var("NO_COLOR").is_ok() {
        ("", "", "", "", "")
    } else {
        (
            "\x1b[0;95m",
            "\x1b[1;94m",
            "\x1b[1;93m",
            "\x1b[0;1m",
            "\x1b[0m",
        )
    };
    print!(
        r#"{gold}
    ,------------.                ,.--""-._
    |   {purple}WELCOME!{gold}  `.           __/         `.
    |  {purple}TO THE REPL{gold} |     _,**"   "*-.       `.
    | {purple}HATTER v0.0.1{gold}|   ,'            `.       \
    `---------------'  ;    _,.---._    \  ,'\  \
                      :   ,'   ,-.. `.   \'   \ :
                      |  ;_\  (___)`  `-..__  : |
                      ;-'`{purple}*{gold}'"  `{purple}*{gold}'    `--._ ` | ;
                     /,-'/  -.        `---.`  |"
                     /_,'`--='.       `-.._,-" _
                      (/\\,--. \    ___-.`:   //___
                         /\{white}'''{gold}\ '  |   |-`|  ( -__,'
                        '. `--'    ;   ;  ; ;/_/
                          `. `.__,/   /_,' /`.~;
                          _.-._|_/_,'.____/   /
                     ..--" /  =/  \=  \      /
                    /  ;._.\_.-`--'-._/ ____/
                    \ /   /._/|.\     ."
                     `*--'._ "-.:     :
                          :/".A` \    |
                          |   |.  `.  :
                          ;   |.    `. {blue}\SSt
{clear}"#,
        gold = gold,
        purple = purple,
        blue = blue,
        white = white,
        clear = clear
    );
}
