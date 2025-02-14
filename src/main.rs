use filesize::file_real_size_fast;
use std::env::args;
use std::path::{Path};
use jwalk;
use tabled::{Table, Tabled};
use tabled::settings::{Alignment, Style};
use tabled::settings::object::Columns;

const Byte: u64 = 1 << (0 * 10);
const KiByte: u64 = 1 << (1 * 10);
const MiByte: u64 = 1 << (2 * 10);
const GiByte: u64 = 1 << (3 * 10);
const TiByte: u64 = 1 << (4 * 10);
const PiByte: u64 = 1 << (5 * 10);
const EiByte: u64 = 1 << (6 * 10);

type WalkDir = jwalk::WalkDirGeneric<((), Option<Result<std::fs::Metadata, jwalk::Error>>)>;
#[derive(Tabled)]
struct ShowInfo {
    #[tabled(rename = "PATH")]
    path: String,
    #[tabled(skip)]
    file_size: String,
    #[tabled(rename = "UsedDiskSize")]
    file_size_string: String,
    #[tabled(rename = "UsedDiskSizeType")]
    file_size_type: String,
}

fn format_filesize_type(size: u64) -> String {
    match size {
        size if size > EiByte => format!("EiB"),
        size if size > PiByte => format!("PiB"),
        size if size > TiByte => format!("TiB"),
        size if size > GiByte => format!("GiB"),
        size if size > MiByte => format!("MiB"),
        size if size > KiByte => format!("KiB"),
        _ => format!("B"),
    }
}

async fn iter_from_path(root: &Path) -> WalkDir {
    WalkDir::new(root)
        .follow_links(false)
        .min_depth(0)
        .sort(false)
        .skip_hidden(false)
        .process_read_dir({
            move |_, _, _, dir_entry_results| {
                dir_entry_results.iter_mut().for_each(|dir_entry_result| {
                    if let Ok(dir_entry) = dir_entry_result {
                        let metadata = dir_entry.metadata();
                        dir_entry.client_state = Some(metadata);
                    }
                })
            }
        })
        .parallelism(jwalk::Parallelism::RayonExistingPool {
            pool: jwalk::rayon::ThreadPoolBuilder::new()
                .stack_size(128 * 1024)
                .num_threads(num_cpus::get())
                .thread_name(|idx| format!("dua-fs-walk-{idx}"))
                .build()
                .expect("fields we set cannot fail")
                .into(),
            busy_timeout: None,
        })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if args().len() < 2 {
        println!("arg len need >= 2");
        return Ok(());
    }
    let paths = args()
        .skip(1)
        .into_iter()
        .map(|path| std::fs::canonicalize(path).expect("can't canonicalize path"))
        .collect::<Vec<_>>();
    // println!("begin to scan paths:{:?}", paths);
    let mut tasks = Vec::new();
    let (tx, mut rx) = tokio::sync::mpsc::channel(64);
    for path in paths {
        let tx_clone = tx.clone();
        let task = tokio::spawn(async move {
            let total_file_size = iter_from_path(path.as_path()).await
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| {
                    file_real_size_fast(entry.path(), &entry.client_state.unwrap().unwrap())
                        .unwrap_or(0)
                })
                .sum::<u64>();
            let file_size_string = humansize::format_size(total_file_size, humansize::BINARY);
            let file_size_type = format_filesize_type(total_file_size);

            tx_clone.send(ShowInfo {
                path: path.display().to_string(),
                file_size: total_file_size.to_string(),
                file_size_string: file_size_string,
                file_size_type: file_size_type,
            }).await
        });
        tasks.push(task);
    }

    futures::future::join_all(tasks).await;
    drop(tx);
    let mut show_infos = Vec::new();
    while let Some(info) = rx.recv().await {
        show_infos.push(info);
    }
    let mut table = Table::new(show_infos);
    table.with(Style::psql());
    table.with(Alignment::center());
    table.modify(Columns::first(), Alignment::left());
    println!("{}",table);
    Ok(())
}
