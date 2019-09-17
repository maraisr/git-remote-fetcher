use std::path::Path;

extern crate threadpool;

// Try mio
// rayon threads

fn main() {

    let pool = threadpool::ThreadPool::new(20);

    let startPath: &Path = Path::new("C:\\Users\\marais\\Sites\\");

    startPath.read_dir()
        .unwrap()
        .for_each(|path| {
           pool.execute(move || {
               task(path.unwrap().path().as_path());
           })
        });

    pool.join()
}

fn task(path: &Path) {
    println!("Running at {:?}", path);
}