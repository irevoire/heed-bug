use heed::EnvOpenOptions;

fn main() {
    const NBR_THREADS: usize = 11;
    const NBR_DB: u32 = 100;

    let mut handles = vec![];
    for _i in 0..NBR_THREADS {
        let h = std::thread::spawn(|| {
            let dir = tempfile::tempdir_in(".").unwrap();

            let mut options = EnvOpenOptions::new();
            options.max_dbs(NBR_DB);

            let env = options.open(dir.path()).unwrap();
            for i in 0..NBR_DB {
                env.create_poly_database(Some(&format!("db{i}"))).unwrap();
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("ok!");
}
