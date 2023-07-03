use reqwest::blocking;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

/*
Data formation:
- The spec list is within the <div id="specs-list"> which contains multiple <table> elements.
- Every <table> is a section (Network, Launch, Body ... etc)
- In every section, there are multiple <tr> which are subsections. E.g.(within Display section) Type, Size...
- The first <tr> within a section has a <th> with the encompassing title (Network, Launch, Body... etc)
- Within every <tr> there are two <td> elements. Representing subsection title and subsection value respectively.
*/

pub fn fetch_source(gsm_arena_id: String) -> String {
    let url = format!("https://www.gsmarena.com/{}.php", gsm_arena_id);
    let response = blocking::get(url).expect("Could not load url");
    let body = response.text().unwrap();
    body
}

type SingleSpecification = [String; 2];

#[derive(Serialize, Deserialize)]
struct Category {
    category_title: String,
    category_spec: Vec<SingleSpecification>,
}
#[derive(Serialize, Deserialize)]
struct DeviceSpecification {
    name: String,
    specification: Vec<Category>,
}

impl Category {
    fn new() -> Self {
        Self {
            category_title: String::new(),
            category_spec: Vec::new(),
        }
    }
    fn add_specification(&mut self, new_specification: SingleSpecification) {
        self.category_spec.push(new_specification);
    }
}

impl DeviceSpecification {
    fn new(name: String) -> Self {
        Self {
            name,
            specification: Vec::new(),
        }
    }
    fn add_category(&mut self, new_category: Category) {
        self.specification.push(new_category);
    }
}

pub fn get_specification(gsm_arena_id: String)-> String{
    let mut device_specification = DeviceSpecification::new(gsm_arena_id.clone());

    let body = fetch_source(gsm_arena_id);
    let document = Html::parse_document(&body);

    let specs_list_table_selector = Selector::parse("#specs-list table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let th_selector = Selector::parse("th").unwrap();

    let mut check_title = true;

    for table in document.select(&specs_list_table_selector) {
        let subsection_vec = table.select(&tr_selector).collect::<Vec<_>>();

        let mut category_specification = Category::new();

        for subsection in subsection_vec {
            if check_title {
                let th_vec = table.select(&th_selector).collect::<Vec<_>>();
                for th in th_vec {
                    let section_title = th.text().collect::<Vec<_>>()[0].to_string();
                    category_specification.category_title = section_title.clone();
                }
            }
            check_title = false;

            let tr_vec = subsection.select(&td_selector).collect::<Vec<_>>();
            let key = tr_vec[0].text().collect::<Vec<_>>()[0].to_string();
            let value = tr_vec[1].text().collect::<Vec<_>>()[0].to_string();

            let new_specification: SingleSpecification = [key, value];
            category_specification.add_specification(new_specification)
        }
        device_specification.add_category(category_specification);
        check_title = true;
    }

    let json_format = serde_json::to_string(&device_specification).unwrap();
    return json_format;
}
