use crate::colors;

const FAT_RSS: &'static str = "";
const THIN_RSS: &'static str = "";
const MED_RSS: &'static str = "索";

pub fn print_rss_things(clrs: colors::Colors) {
    println!("{} {}{}", FAT_RSS, MED_RSS, THIN_RSS);
}
