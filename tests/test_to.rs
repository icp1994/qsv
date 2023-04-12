use std::path::Path;

use rusqlite::Connection;

use crate::workdir::{is_same_file, Workdir};

#[test]
fn to_xlsx_roundtrip() {
    let wrk = Workdir::new("to_xlsx");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "Mary had a little lamb"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "I am a leaf on the wind."],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "Bazinga!"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let xlsx_file = wrk.path("testxlsx.xlsx").to_string_lossy().to_string();
    log::info!("xlsx_file: {}", xlsx_file);

    let mut cmd = wrk.command("to");
    cmd.arg("xlsx").arg(xlsx_file.clone()).arg("in.csv");

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("excel");
    cmd2.arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd2);
}

#[test]
fn to_xlsx_dir() {
    let wrk = Workdir::new("to_xlsx_dir");

    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    wrk.create("cities.csv", cities.clone());
    wrk.create("places.csv", places.clone());

    let xlsx_file = wrk.path("testxlsx.xlsx").to_string_lossy().to_string();
    log::info!("xlsx_file: {}", xlsx_file);

    let mut cmd = wrk.command("to");
    cmd.arg("xlsx").arg(xlsx_file.clone()).arg(wrk.path(""));

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("excel");
    cmd2.arg(xlsx_file.clone()).args(&["--sheet", "cities"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);
    assert_eq!(got, cities);

    wrk.assert_success(&mut cmd2);

    let mut cmd2 = wrk.command("excel");
    cmd2.arg(xlsx_file).args(&["--sheet", "places"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);
    assert_eq!(got, places);

    wrk.assert_success(&mut cmd2);
}

#[test]
fn to_datapackage() {
    let wrk = Workdir::new("to_datapackage");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "Mary had a little lamb"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "I am a leaf on the wind."],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "Bazinga!"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let testdp_json_file = wrk.path("testdp.json");
    let testdp_json_filename = testdp_json_file.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("datapackage")
        .arg(testdp_json_filename.clone())
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected: String = r#"Table 'in' (8 rows)

Field Name   Field Type  Field Format
Col1         integer     integer
Description  string      string"#
        .to_string();
    assert_eq!(got, expected);

    let expected = wrk.load_test_file("testdp.json");
    let expected_path = Path::new(&expected);

    assert!(is_same_file(&testdp_json_file, expected_path).unwrap());
}

#[test]
fn to_datapackage_dir() {
    let wrk = Workdir::new("to_datapackage_dir");

    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    wrk.create("cities.csv", cities.clone());
    wrk.create("places.csv", places.clone());

    let dp_file = wrk.path("dpdir.json");
    let dp_file_filename = dp_file.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("datapackage")
        .arg(dp_file_filename.clone())
        .arg(wrk.path(""));

    let got: String = wrk.stdout(&mut cmd);
    let expected: String = r#"Table 'places' (4 rows)

Field Name  Field Type  Field Format
city        string      string
place       string      string

Table 'cities' (4 rows)

Field Name  Field Type  Field Format
city        string      string
state       string      string"#
        .to_string();
    assert_eq!(got, expected);

    let expected = wrk.load_test_file("dpdir.json");
    let expected_path = Path::new(&expected);

    assert!(is_same_file(&dp_file, expected_path).unwrap());
}

#[test]
fn to_sqlite_dir() {
    let wrk = Workdir::new("to_sqlite_dir");

    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    wrk.create("cities.csv", cities.clone());
    wrk.create("places.csv", places.clone());

    let sqlite_file = wrk.path("test_to_sqlite.db");
    let sqlite_file_filename = sqlite_file.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg(sqlite_file_filename.clone())
        .arg(wrk.path(""));

    let got: String = wrk.stdout(&mut cmd);
    let expected: String = r#"Table 'places' (4 rows)

Field Name  Field Type  Field Format
city        string      string
place       string      string

Table 'cities' (4 rows)

Field Name  Field Type  Field Format
city        string      string
state       string      string"#
        .to_string();
    assert_eq!(got, expected);

    let db = Connection::open(sqlite_file_filename).unwrap();
    let mut stmt = db.prepare("SELECT * FROM cities ORDER BY city").unwrap();
    let cities_iter = stmt
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap();
    let cities: Vec<(String, String)> = cities_iter.map(|r| r.unwrap()).collect();
    assert_eq!(
        cities,
        vec![
            (String::from("Boston"), String::from("MA")),
            (String::from("Buffalo"), String::from("NY")),
            (String::from("New York"), String::from("NY")),
            (String::from("San Francisco"), String::from("CA")),
        ]
    );

    let mut stmt = db.prepare("SELECT * FROM places ORDER BY city, place").unwrap();
    let places_iter = stmt
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap();
    let places: Vec<(String, String)> = places_iter.map(|r| r.unwrap()).collect();
    assert_eq!(
        places,
        vec![
            (String::from("Boston"), String::from("Boston Garden")),
            (String::from("Boston"), String::from("Logan Airport")),
            (
                String::from("Buffalo"),
                String::from("Ralph Wilson Stadium")
            ),
            (String::from("Orlando"), String::from("Disney World")),
        ]
    );
}
