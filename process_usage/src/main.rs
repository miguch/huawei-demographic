use std::collections::{HashMap, HashSet};
use std::fs::File;

struct Usage {
    pub category_usage: Vec<u32>,
    pub category_times: Vec<f32>,
    pub category_duration: Vec<f64>,
}

impl Usage {
    pub fn new(category_counts: usize) -> Self {
        Self {
            category_usage: vec![0; category_counts],
            category_times: vec![0.0; category_counts],
            category_duration: vec![0.0; category_counts],
        }
    }
}

fn main() {
    let mut categories = HashSet::new();
    let app_info_map = {
        let mut map = HashMap::new();
        let info_file = File::open("../data/app_info.csv").unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(info_file);
        for result in reader.records() {
            let records = result.unwrap();
            let app_id = records[0].to_string();
            let cate = records[1].to_string();
            if !map.contains_key(&app_id) {
                map.insert(app_id.clone(), vec![]);
            }
            map.get_mut(&app_id).unwrap().push(cate);
            categories.insert(records[1].to_string());
        }
        map
    };
    categories.insert("unknown".to_string());

    println!("Categories count: {}", categories.len());
    println!("Known apps count: {}", app_info_map.len());

    let (category_name, category_code) = {
        let mut name = HashMap::new();
        let mut code = HashMap::new();
        let mut i = 0;
        for cat in &categories {
            name.insert(i, cat);
            code.insert(cat, i);
            i += 1;
        }
        (name, code)
    };

    let uid_list = {
        let mut li = vec![];
        let info_file = File::open("../data/user_basic_info.csv").unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(info_file);
        for result in reader.records() {
            let records = result.unwrap();
            li.push(records[0].parse::<u32>().unwrap());
        }
        li
    };

    let mut usages = {
        let mut map = HashMap::new();
        for id in uid_list {
            map.insert(id, Usage::new(category_name.len()));
        }
        map
    };

    let unknown_str = vec!["unknown".to_string()];
    let sty = indicatif::ProgressStyle::default_bar()
        .template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})",
        )
        .progress_chars("##-");
    let bar = indicatif::ProgressBar::new(651007710);
    bar.set_style(sty.clone());
    let file_path = format!("/Volumes/datum/data/user_app_usage.csv");
    let file = File::open(file_path).unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(std::io::BufReader::new(file));
    let mut i = 0;
    for result in reader.records() {
        let records = result.unwrap();
        let uid = records[0].parse::<u32>().unwrap();
        let app_id = &records[1];
        let categories = app_info_map.get(app_id).unwrap_or(&unknown_str);
        for cate in categories {
            let cate_id = category_code[cate];
            let u_usage = usages.get_mut(&uid).unwrap();
            u_usage.category_usage[cate_id] += 1;
            u_usage.category_times[cate_id] += records[3].parse::<f32>().unwrap();
            u_usage.category_duration[cate_id] += records[2].parse::<f64>().unwrap();
        }

        i += 1;
        // The progress bar has become a huge overhead
        if i % 10000 == 0 {
            bar.inc(10000);
            i = 0;
        }
    }

    println!("Writing to file");

    // Write to file
    let output = File::create("../data/usages_summary.csv").unwrap();
    let mut output_writer = std::io::BufWriter::new(output);
    use std::io::Write;
    output_writer.write(b"uid").unwrap();
    for (_code, cate_name) in &category_name {
        output_writer
            .write(format!(",usage_{}", cate_name).as_bytes())
            .unwrap();
        output_writer
            .write(format!(",times_{}", cate_name).as_bytes())
            .unwrap();
        output_writer
            .write(format!(",duration_{}", cate_name).as_bytes())
            .unwrap();
    }
    output_writer.write(b"\n").unwrap();

    for (id, usage) in usages {
        output_writer.write(id.to_string().as_bytes()).unwrap();
        for i in 0..category_name.len() {
            output_writer
                .write(format!(",{}", usage.category_usage[i]).as_bytes())
                .unwrap();
            output_writer
                .write(format!(",{}", usage.category_times[i]).as_bytes())
                .unwrap();
            output_writer
                .write(format!(",{}", usage.category_duration[i]).as_bytes())
                .unwrap();
        }
        output_writer.write(b"\n").unwrap();
    }
}
