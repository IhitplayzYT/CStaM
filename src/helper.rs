pub mod Helper {
    pub const OK: i32 = 0;
    pub const ERR: i32 = 1;
    pub const DBG_STR: &str = "";

    struct CLI {
        targets: Vec<String>,
        dbg: bool,
    }

    impl CLI {
        pub fn new() -> CLI {
            Self {
                targets: vec![],
                dbg: false,
            }
        }
    }

    pub fn Help() {
        println!("{DBG_STR}");
        std::process::exit(OK);
    }

    pub fn Parse_Args() -> CLI {
        let mut ret = CLI::new();
        let clargs: Vec<String> = std::env::args().collect::<Vec<String>>();
        for i in &clargs {
            if &i[..] == "-h" || &i[..] == "--help" || &i[..] == "-H" || &i[..] == "--HELP" {
                Help();
            } else if &i[..] == "-d" || &i[..] == "--debug" || &i[..] == "-D" || &i[..] == "--DEBUG"
            {
                ret.dbg = true;
            } else {
                ret.targets.push(i.to_string());
            }
        }
        ret
    }
}
