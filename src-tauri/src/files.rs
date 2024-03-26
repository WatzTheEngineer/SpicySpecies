use std::fs::File;

pub fn loadcsv(path: &String) -> Vec<String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return vec!["CSV_ERROR".to_string()],
    };

    let mut csv_reader = csv::Reader::from_reader(file);

    let mut data: Vec<String> = vec![];

    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                data.push(format!("{:?}", record));
            }
            Err(_) => {
                data.push("CSV_ERROR".to_string());
            }
        }
    }

    return data;
}