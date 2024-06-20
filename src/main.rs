fn main() {
    let path = get_file_path();
    let columns = read_file(path);
    transform_to_json(columns);
}
/// get file path from command line
fn get_file_path() -> Result<String, String> {
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        Result::Ok(args[1].clone())
    } else {
        Result::Err("file path is required".to_string())
    }
}

/// read excel file and return column name
fn read_file(path: Result<String, String>) -> Vec<String> {
    use calamine::{open_workbook, Data, Reader, Xlsx};
    let path = path.unwrap();
    // open excel file, default get first sheet
    let mut workbook: Xlsx<_> = open_workbook(path).expect("open file fail");
    let sheets = workbook.worksheets();
    let default_sheet = &sheets[0].clone();
    let (_, sheet_data) = default_sheet;
    let row_names: Vec<String> = sheet_data
        .rows()
        .next()
        .expect("sheet is null")
        .iter()
        .map(|cell| match cell {
            Data::String(s) => s.clone(),
            _ => "".to_string(),
        })
        .collect();
    row_names
}

/// transform excel file to json file
fn transform_to_json(columns: Vec<String>) {
    use serde::Serialize;
    use std::fs;

    #[derive(Serialize)]
    struct ColumnInfo {
        label: String,
        prop: String,
    }

    let column_infos: Vec<ColumnInfo> = columns
        .into_iter()
        .map(|name| ColumnInfo {
            label: name,
            prop: String::new(),
        })
        .collect();

    let json_res = serde_json::to_string(&column_infos).unwrap();
    fs::write("output.json", json_res).expect("write in json file fail")
}
