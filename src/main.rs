mod capture;
mod files;
mod list;
mod tally;

fn main() {
    files::files();
    list::list();
    tally::tally();
    capture::capture();
}
