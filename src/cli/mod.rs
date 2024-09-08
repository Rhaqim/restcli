pub mod restcli {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(name = "restcli", version = "0.1.0", author = "Rhaqim")]
    #[clap(about = "Generate RESTful API endpoints")]
    pub struct Opts {
        #[clap(flatten)]
        pub options: RestCLIOptions,
    }

    #[derive(Parser, Debug, Clone)]
    pub struct RestCLIOptions {
        #[clap(help = "Input file containing the request details", required = true)]
        pub file: String,

        #[clap(
            short,
            long = "curl",
            default_value = "true",
            help = "Generate curl requests"
        )]
        pub curl: bool,

        #[clap(
            short,
            long = "postman",
            default_value = "false",
            help = "Create Postman collection"
        )]
        pub postman: bool,

        #[clap(
            short,
            long = "rest_client",
            default_value = "false",
            help = "User rest-client for the requests"
        )]
        pub rest_client: bool,

        #[clap(long = "port", default_value = "8080", help = "Port number")]
        pub port: u16,

        #[clap(long = "url", default_value = "http://localhost", help = "Base URL")]
        pub url: String,

        #[clap(
            short,
            long = "output",
            default_value = "request",
            help = "Output file name"
        )]
        pub output: String,
    }

    pub fn parse() {
        let opts = Opts::parse();

        let client_selected = vec![
            opts.options.curl,
            opts.options.postman,
            opts.options.rest_client,
        ]
        .into_iter()
        .filter(|&x| x)
        .count();

        if client_selected != 1 {
            eprintln!(
                "Error: You must specify exactly one client (curl, postman, or rest-client)."
            );
            std::process::exit(1);
        }

        // Handle input file
        let input_file = opts.options.file;
        println!("Using input file: {}", input_file);

        let mut output_file = format! {"{}.sh", opts.options.output};

        if opts.options.postman {
            output_file = format! {"{}.json", opts.options.output};
        } else if opts.options.rest_client {
            output_file = format! {"{}.http", opts.options.output};
        }

        println!("Using output file: {}", output_file);

        // Display the selected client and URL/Port details
        if opts.options.curl {
            println!("Using curl for the request...");
        } else if opts.options.postman {
            println!("Using Postman for the request...");
        } else if opts.options.rest_client {
            println!("Using rest-client for the request...");
        }

        println!("Base URL: {}", opts.options.url);
        println!("Port: {}", opts.options.port);
    }
}
