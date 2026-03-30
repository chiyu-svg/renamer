use anyhow::{Result, anyhow};
use clap::Parser;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let args = Args::parse();
    if !args.directory.is_dir() {
        return Err(anyhow!(
            "提供的路径 '{}' 不是一个有效的目录。",
            args.directory.display()
        ));
    }
    // 开始遍历和处理文件
    process_directory(
        &args.directory,
        &args.old_extension,
        &args.new_extension,
        args.recursive,
        args.dry_run,
    )?;
    if args.dry_run {
        println!("模拟运行完成。请使用 ");
    } else {
        println!("文件后缀修改完成！");
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    directory: PathBuf,
    old_extension: String,
    new_extension: String,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long)]
    dry_run: bool,
}

fn process_directory(
    dir: &Path,
    old_ext: &str,
    new_ext: &str,
    recursive: bool,
    dry_run: bool,
) -> Result<()> {
    let entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    for entry in entries {
        if entry.is_dir() && recursive {
            process_directory(&entry, old_ext, new_ext, recursive, dry_run)?;
        }
        if entry.is_file() {
            try_rename_file(&entry, old_ext, new_ext, dry_run)?;
        }
    }
    Ok(())
}

fn try_rename_file(file_path: &Path, old_ext: &str, new_ext: &str, dry_run: bool) -> Result<()> {
    if let Some(txt) = file_path.extension().and_then(|e| e.to_str()) { 
        if txt == old_ext {
            let mut new_path = file_path.to_path_buf();
            new_path.set_extension(new_ext);
            println!(
                "将重命名: '{}' -> '{}'",
                file_path.display(),
                new_path.file_name().unwrap().to_str().unwrap()
            );
            if !dry_run {
                fs::rename(file_path, &new_path)?;
            }
        }
    }
    Ok(())
}
