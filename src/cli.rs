use clap::{Arg, ArgAction, ArgMatches, Command};
use rust_i18n::t;
use std::borrow::Cow;
use std::{fs, io};
use std::io::{Read, Write};
use std::process::exit;
use upbflib::read::{UPBFDataForRead, UPBFReadResult, UPBFReader, UPBFReaderDataReadError, UPBFReaderError, UPBFReaderFormatReadError, UPBFReaderHeaderReadError, UPBFReaderNameReadError, UPBFReaderVersionReadError};
use upbflib::{UPBFType, UPBFVersion};
use upbflib::write::{UPBFWriter, UPBFWriterDataAddError, UPBFWriterError, UPBFWriterWriteError};

macro_rules! i18n_arg {
    ($value_name:literal, $help_key:literal) => {
        Arg::new($value_name)
            .value_name($value_name)
            .action(ArgAction::Set)
            .help(::rust_i18n::t!($help_key))
    };
}

fn cli_commands() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(t!("cmd.about"))
        .version(env!("CARGO_PKG_VERSION"))

        .disable_help_flag(true)
        .disable_version_flag(true)

        .subcommand_required(true)
        .arg_required_else_help(true)

        .subcommand(
            Command::new("info")
                .about(t!("cmd.info.about"))
                .arg(i18n_arg!("FILE", "cmd.info.arg.file"))
        )
        .subcommand(
            Command::new("new")
                .about(t!("cmd.new.about"))
                .arg(i18n_arg!("FILE", "cmd.new.arg.file"))
                .arg(i18n_arg!("FILE_TYPE", "cmd.new.arg.file_type"))
                .arg(i18n_arg!("BUILD_NAME", "cmd.new.arg.build_name"))
                .arg(i18n_arg!("BUILD_VERSION", "cnd.new.arg.build_version"))
        )
        .subcommand(
            Command::new("write")
                .about(t!("cmd.write.about"))
                .arg(i18n_arg!("FILE", "cmd.write.arg.file"))
                .arg(i18n_arg!("DATA_NAME", "cmd.write.arg.data_name"))
                .arg(i18n_arg!("DATA_FORMAT", "cmd.write.arg.data_format"))
        )
        .subcommand(
            Command::new("read")
                .about(t!("cmd.read.about"))
                .arg(i18n_arg!("FILE", "cmd.read.arg.file"))
                .arg(i18n_arg!("DATA_NAME", "cmd.read.arg.data_name"))
        )
        .subcommand(
            Command::new("convert")
                .about(t!("cmd.convert.about"))
                .arg(i18n_arg!("FILE_IN", "cmd.convert.arg.file_in"))
                .arg(i18n_arg!("FILE_OUT", "cmd.convert.arg.file_out"))
                .arg(i18n_arg!("FILE_TYPE", "cmd.convert.arg.file_type"))
        )

        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help(t!("cmd.help"))
                .action(ArgAction::Help)
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help(t!("cmd.version"))
                .action(ArgAction::Version)
        )
}

fn cli_get_arg<'a, T: Clone + Send + Sync + 'static>(args: &'a ArgMatches, arg: &str) -> &'a T {
    match args.get_one::<T>(arg) {
        Some(some) => some,
        None => {
            eprintln!("{}", t!("cmd.err.arg_get", arg = arg));
            exit(1);
        }
    }
}

fn cli_read_stdin() -> Vec<u8> {
    let mut buff = Vec::<u8>::new();
    let mut lock = io::stdin().lock();
    if let Err(err) = lock.read_to_end(&mut buff) {
        eprintln!("{}", t!("cmd.err.stdin", cause = err.to_string()))
    }
    buff
}

fn cli_write_stdout(buff: &[u8]) {
    let mut lock = io::stdout().lock();
    if let Err(err) = lock.write_all(&buff) {
        eprintln!("{}", t!("cmd.err.stdout", cause = err.to_string()))
    }
}

fn cli_read_file(file: &str) -> Vec<u8> {
    match fs::read(file) {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{}", t!("cmd.err.file_read", cause = err.to_string()));
            exit(1);
        }
    }
}

fn cli_reader_new(buff: &'_ [u8]) -> UPBFReader<'_> {
    match UPBFReader::new(buff) {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_reader_error(err)
    }
}

fn cli_reader_read<'a>(reader: &'a UPBFReader<'a>) -> UPBFReadResult<'a> {
    match reader.read() {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_reader_error(err)
    }
}

fn cli_read_data<'a>(read: &'a UPBFReadResult<'a>, name: &String) -> &'a UPBFDataForRead<'a> {
    match read.data().iter().find(|it| it.name() == name) {
        Some(some) => some,
        None => {
            eprintln!("{}", t!("cmd.err.data_not_found"));
            exit(1);
        }
    }
}

fn cli_writer_from_read(read: &UPBFReadResult) -> UPBFWriter {
    match UPBFWriter::try_from(read) {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_reader_error(err)
    }
}

fn cli_writer_write_data(writer: &mut UPBFWriter, name: &String, format: &String, data: Box<[u8]>) {
    match writer.add_or_overwrite_data(name, format, data) {
        Ok(()) => {}
        Err(err) => cli_exit_with_writer_error(err)
    }
}

fn cli_writer_write_file(writer: &mut UPBFWriter, r#type: UPBFType, version: UPBFVersion, file: &str) {
    match writer.write(r#type, version) {
        Ok(bytes) => {
            if let Err(err) = fs::write(file, &bytes) {
                eprintln!("{}", t!("cmd.err.file_write", cause = err.to_string()));
            }
        }
        Err(err) => cli_exit_with_writer_error(err)
    }
}

fn cli_exit_with_reader_error(err: UPBFReaderError) -> ! {
    let cause = match err {
        UPBFReaderError::InvalidFileLength => "Неверная структура файла",
        UPBFReaderError::Header(err) => match err {
            UPBFReaderHeaderReadError::InvalidMagic => "Неверное магическое число",
            UPBFReaderHeaderReadError::InvalidType => "Неверные тип файла",
            UPBFReaderHeaderReadError::UnsupportedVersion => "Неподдерживаемая версия файла"
        }
        UPBFReaderError::Name(err) => match err {
            UPBFReaderNameReadError::InvalidLength => "Неверная длина имени сборки",
            UPBFReaderNameReadError::InvalidString => "Неверная строка имени сборки"
        }
        UPBFReaderError::Version(err) => match err {
            UPBFReaderVersionReadError::InvalidLength => "Неверная длина версии сборки",
            UPBFReaderVersionReadError::InvalidString => "Неверная строка версии сборки"
        }
        UPBFReaderError::Format(err) => match err {
            UPBFReaderFormatReadError::InvalidOffset => "Неверное смещение блока формата данных",
            UPBFReaderFormatReadError::InvalidNameLength => "Неверная длина имени в блоке формата данных",
            UPBFReaderFormatReadError::InvalidNameString => "Неверная строка имени в блоке формата данных"
        }
        UPBFReaderError::Data(err) => match err {
            UPBFReaderDataReadError::InvalidOffset => "Неверное смещение блока данных",
            UPBFReaderDataReadError::InvalidNameLength => "Неверная длина имени в блоке данных",
            UPBFReaderDataReadError::InvalidNameString => "Неверная строка имени в блоке данных",
            UPBFReaderDataReadError::InvalidDataLength => "Неверный размер данных в блоке данных",
            UPBFReaderDataReadError::InvalidDataId => "Неверный идентификатор данных в блоке данных"
        }
    };
    eprintln!("{}", t!("cli.err.reader", cause = cause));
    exit(1);
}

fn cli_exit_with_writer_error(err: UPBFWriterError) -> ! {
    let cause = match err {
        UPBFWriterError::DataAdd(err) => match err {
            UPBFWriterDataAddError::DataAlreadyDefined => "Блок данных с таким именем уже существует",
            UPBFWriterDataAddError::FormatCounterOverflow => "Переполнение идентификаторов форматов данных"
        }
        UPBFWriterError::Write(err) => match err {
            UPBFWriterWriteError::UnsupportedVersion => "Неподдерживаемая версия файла",
            UPBFWriterWriteError::InvalidNameLength => "Неверная длина имени сборки",
            UPBFWriterWriteError::InvalidVersionLength => "Неверная длина версии сборки",
            UPBFWriterWriteError::InvalidFormatNameLength => "Неверная длина имени в блоке формата данных",
            UPBFWriterWriteError::InvalidDataNameLength => "Неверная длина имени в блоке данных",
            UPBFWriterWriteError::InvalidDataLength => "Неверный размер данных в блоке данных",
            UPBFWriterWriteError::InvalidOffset => "Неверное смещение",
            UPBFWriterWriteError::IOError(err) => err.to_string().leak()
        }
    };
    eprintln!("{}", t!("cli.err.writer", cause = cause));
    exit(1);
}

fn cli_upbf_type_parse(r#type: &str) -> UPBFType {
    match r#type {
        "MAL" | "СВМ" => UPBFType::MediumAlignedLittleEndian,
        "MAB" | "СВС" => UPBFType::MediumAlignedBigEndian,
        "BAL" | "БВМ" => UPBFType::BigAlignedLittleEndian,
        "BAB" | "БВС" => UPBFType::BigAlignedBigEndian,
        _ => {
            eprintln!("{}", t!("cmd.err.unknown_type", r#type=r#type));
            eprintln!("{}", cli_upbf_type_to_str(UPBFType::MediumAlignedLittleEndian));
            eprintln!("{}", cli_upbf_type_to_str(UPBFType::MediumAlignedBigEndian));
            eprintln!("{}", cli_upbf_type_to_str(UPBFType::BigAlignedLittleEndian));
            eprintln!("{}", cli_upbf_type_to_str(UPBFType::BigAlignedBigEndian));
            exit(1);
        }
    }
}

fn cli_upbf_type_to_str(r#type: UPBFType) -> Cow<'static, str> {
    match r#type {
        UPBFType::MediumAlignedLittleEndian => t!("upbf.type.mal"),
        UPBFType::MediumAlignedBigEndian => t!("upbf.type.mab"),
        UPBFType::BigAlignedLittleEndian => t!("upbf.type.bal"),
        UPBFType::BigAlignedBigEndian => t!("upbf.type.bab")
    }
}

fn cli_upbf_version_to_str(version: UPBFVersion) -> String {
    format!("{} ({})", version.as_raw(), t!(if version.is_supported() { "upbf.version.supported" } else { "upbf.version.unsupported" }))
}

pub fn cli_process() {
    let matches = cli_commands().get_matches();
    match matches.subcommand() {
        Some(("info", sub_matches)) => cmd_info(sub_matches),
        Some(("new", sub_matches)) => cmd_new(sub_matches),
        Some(("write", sub_matches)) => cmd_write(sub_matches),
        Some(("read", sub_matches)) => cmd_read(sub_matches),
        Some(("convert", sub_matches)) => cmd_convert(sub_matches),
        _ => unreachable!()
    }
}

fn cmd_info(args: &ArgMatches) {
    let file_name = cli_get_arg::<String>(args, "FILE");

    let file = cli_read_file(file_name);
    let reader = cli_reader_new(&file);

    println!("{}", t!("cmd.info.exec.file.file",    file = file_name));
    println!("{}", t!("cmd.info.exec.file.type",    file_type = cli_upbf_type_to_str(reader.file_type())));
    println!("{}", t!("cmd.info.exec.file.version", file_version = cli_upbf_version_to_str(reader.file_version())));

    if reader.is_read_supported() {
        let read = cli_reader_read(&reader);

        println!();
        println!("{}", t!("cmd.info.exec.build.name",    build_name = read.build_name()));
        println!("{}", t!("cmd.info.exec.build.version", build_version = read.build_version()));

        if !read.data_formats().is_empty() {
            println!();
            println!("{}", t!("cmd.info.exec.data_format_list"));
            for data_format in read.data_formats() {
                println!(
                    "{}",
                    t!("cmd.info.exec.data_format",
                        data_format_name = data_format.name(),
                        data_format_id = data_format.data_id()
                    )
                );
            }
        }

        if !read.data().is_empty() {
            println!();
            println!("{}", t!("cmd.info.exec.data_list"));
            for data in read.data() {
                println!(
                    "{}",
                    t!("cmd.info.exec.data",
                        data_format = data.format(&read).name(),
                        data_name = data.name(),
                        data_length = data.data().len()
                    )
                )
            }
        }
    }
}

fn cmd_new(args: &ArgMatches) {
    let file_name = cli_get_arg::<String>(args, "FILE");
    let file_type = cli_get_arg::<String>(args, "FILE_TYPE");
    let build_name = cli_get_arg::<String>(args, "BUILD_NAME");
    let build_version = cli_get_arg::<String>(args, "BUILD_VERSION");

    let file_type = cli_upbf_type_parse(file_type);
    let mut writer = UPBFWriter::new(build_name.clone(), build_version.clone());
    cli_writer_write_file(&mut writer, file_type, UPBFVersion::LAST_SUPPORTED, file_name);
}

fn cmd_write(args: &ArgMatches) {
    let file_name = cli_get_arg::<String>(args, "FILE");
    let data_name = cli_get_arg::<String>(args, "DATA_NAME");
    let data_format = cli_get_arg::<String>(args, "DATA_FORMAT");
    let data = cli_read_stdin();

    let file = cli_read_file(file_name);
    let reader = cli_reader_new(&file);
    let read = cli_reader_read(&reader);
    let mut writer = cli_writer_from_read(&read);
    cli_writer_write_data(&mut writer, &data_name, &data_format, data.into_boxed_slice());
    cli_writer_write_file(&mut writer, read.file_type(), read.file_version(), file_name);
}

fn cmd_read(args: &ArgMatches) {
    let file_name = cli_get_arg::<String>(args, "FILE");
    let data_name = cli_get_arg::<String>(args, "DATA_NAME");

    let file = cli_read_file(file_name);
    let reader = cli_reader_new(&file);
    let read = cli_reader_read(&reader);
    let data = cli_read_data(&read, data_name);
    cli_write_stdout(data.data());
}

fn cmd_convert(args: &ArgMatches) {
    let file_in = cli_get_arg::<String>(args, "FILE_IN");
    let file_out = cli_get_arg::<String>(args, "FILE_OUT");
    let file_type = cli_get_arg::<String>(args, "FILE_TYPE");

    let file_in = cli_read_file(file_in);
    let file_type = cli_upbf_type_parse(file_type);
    let reader = cli_reader_new(&file_in);
    let read = cli_reader_read(&reader);
    let mut writer = cli_writer_from_read(&read);
    cli_writer_write_file(&mut writer, file_type, read.file_version(), file_out);
}