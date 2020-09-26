use {
    crate::{Args, Env, Result, Value},
    rustyline::{error::ReadlineError, Editor},
    std::io,
};

/// Start the REPL.
pub fn run() -> io::Result<()> {
    banner();
    let mut env = Env::new();
    env.set("help", help);
    env.set("vars", vars);
    env.set("fns", fns);
    let history_file = ".hatter_history";
    let (red, clear) = if std::env::var("NO_COLOR").is_ok() {
        ("", "")
    } else {
        ("\x1b[0;91m", "\x1b[0m")
    };

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(history_file);
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "help" {
                    help(env.empty_args())?;
                    continue;
                }
                match env.render(&line) {
                    Ok(out) => {
                        if !out.trim().is_empty() {
                            print!("{}", out);
                        }
                    }
                    Err(e) => eprintln!("{}{}{}", red, e, clear),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            }
            Err(err) => {
                println!("{}Error: {:?}{}", red, err, clear);
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
                      ;-'`{blue}*{gold}'"  `{blue}*{gold}'    `--._ ` | ;
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

fn help(_: Args) -> Result<Value> {
    println!("REPL commands:\n");
    println!("  - help()");
    println!("  - vars()");
    print!("  - fns()");
    Value::None.ok()
}

fn vars(args: Args) -> Result<Value> {
    for scope in args.env.scopes() {
        for (k, v) in scope {
            if matches!(v, Value::Fn(..)) {
                continue;
            }
            println!("{}: {:?}", k, v);
        }
    }
    Value::None.ok()
}

fn fns(args: Args) -> Result<Value> {
    let mut out = vec![];
    for scope in args.env.scopes() {
        for (name, v) in scope {
            if matches!(v, Value::Fn(..)) {
                out.push(name.into());
            }
        }
    }
    Value::new_list(out).ok()
}
