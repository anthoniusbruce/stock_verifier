use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use html_editor::{
    operation::{Editable, Htmlifiable, Queryable, Selector},
    parse, Node,
};
use structopt::StructOpt;
use time::{Date, Month, OffsetDateTime, Time};
use tokio::runtime::Builder;
use yahoo_finance_api::Quote;

mod tests;

#[derive(StructOpt)]
#[structopt(
    name = "stock_verifier",
    about = "modifies html produced by stock_simulator to add actual results to compare historical runs of stock_simulator. Dates are acceptable as MM/DD/YYY or MM-DD-YYYY."
)]
struct Opt {
    #[structopt(parse(from_os_str), required(true))]
    html_file: PathBuf,
    #[structopt(required(true))]
    start_date: String,
    #[structopt(required(true))]
    end_date: String,
}

fn main() {
    let opt = Opt::from_args_safe();

    match opt {
        Ok(args) => {
            let dates = validate_args(&args.html_file, args.start_date, args.end_date);
            modify_html(args.html_file, dates.0, dates.1);
        }
        Err(e) => println!("{e}"),
    }
}

fn modify_html(path: PathBuf, start_date: OffsetDateTime, end_date: OffsetDateTime) {
    let result: Result<String, std::io::Error>;
    {
        result = fs::read_to_string(path.to_owned());
    }
    match result {
        Ok(s) => {
            let mut dom = parse(&s).expect("html could not be parsed");
            let immutable_dom = dom.to_owned();
            let symbols = immutable_dom.query_all(&Selector::from(".item-header"));
            for symbol in symbols {
                for node in symbol.children.clone() {
                    if let Node::Text(sym) = node {
                        let percent_change = get_percent_change(&sym, start_date, end_date);
                        let css_query = &format!("div#{sym}")[..];
                        let percent = Node::new_element(
                            "span",
                            vec![("class", "primary blue")],
                            vec![Node::Text(format!("{percent_change}"))],
                        );

                        let node_option = dom.query_mut(&Selector::from(css_query));
                        match node_option {
                            None => println!("{sym} node not found"),
                            Some(node) => {
                                node.children.push(Node::Text(String::from(": ")));
                                node.children.push(percent);
                            }
                        }

                        break;
                    }
                }
            }
            let html = dom.trim().html();
            let mut new_file = path.to_owned();
            new_file.set_extension("updated.html");

            let file_result = File::create(new_file);
            match file_result {
                Err(e) => println!("{e}"),
                Ok(mut file) => match file.write_all(html.as_bytes()) {
                    Err(e) => println!("{e}"),
                    Ok(_) => (),
                },
            }
        }
        Err(e) => panic!("{e}"),
    }
}

fn get_percent_change(symbol: &str, start_date: OffsetDateTime, end_date: OffsetDateTime) -> i32 {
    let mut quotes = get_quotes(&symbol, start_date, end_date);

    if quotes.len() < 1 {
        return 0;
    }

    let start_quote = quotes[0].clone();
    let end_quote = quotes.pop().unwrap();

    if start_quote.open == 0.0 {
        return 0;
    }

    let gain = (end_quote.close - start_quote.open) / start_quote.open;
    (gain * 100.0).round() as i32
}

fn get_quotes(symbol: &str, start_date: OffsetDateTime, end_date: OffsetDateTime) -> Vec<Quote> {
    let quotes = Vec::new();
    let provider_result = yahoo_finance_api::YahooConnector::new();
    match provider_result {
        Err(e) => println!("{e}"),
        Ok(provider) => {
            let builder_result = Builder::new_current_thread().enable_all().build();
            match builder_result {
                Err(e) => println!("{e}"),
                Ok(builder) => {
                    let resp_result =
                        builder.block_on(provider.get_quote_history(symbol, start_date, end_date));
                    match resp_result {
                        Err(e) => println!("{e}"),
                        Ok(resp) => match resp.quotes() {
                            Err(e) => println!("{e}"),
                            Ok(quotes) => {
                                return quotes;
                            }
                        },
                    }
                }
            }
        }
    }

    quotes
}

fn validate_args(
    html: &PathBuf,
    start_date_string: String,
    end_date_string: String,
) -> (OffsetDateTime, OffsetDateTime) {
    let file_exists = Path::exists(html);
    if !file_exists {
        panic!("file does not exist");
    }

    (
        validate_date(start_date_string),
        validate_date(end_date_string),
    )
}

fn validate_date(date_string: String) -> OffsetDateTime {
    let mut parts: Vec<&str> = date_string.split('-').collect();
    if parts.len() != 3 {
        parts = date_string.split('/').collect();
    }
    if parts.len() != 3 {
        panic!("not valid date");
    }
    let month: u8 = parts[0].parse().expect("not valid date");
    let day: u8 = parts[1].parse().expect("not valid date");
    let year: u16 = parts[2].parse().expect("not valid date");

    let mnth = Month::try_from(month).expect("not valid date");
    let date = OffsetDateTime::new_utc(
        Date::from_calendar_date(year.into(), mnth, day).expect("not valid date"),
        Time::from_hms_nano(0, 0, 0, 0).expect("not valid date"),
    );

    date
}
