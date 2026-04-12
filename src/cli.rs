use crate::xpbf::{XPBFDataForRead, XPBFReadResult, XPBFReader, XPBFReaderDataReadError, XPBFReaderError, XPBFReaderFormatReadError, XPBFReaderHeaderReadError, XPBFType, XPBFVersion, XPBFWriter, XPBFWriterDataAddError, XPBFWriterError, XPBFWriterWriteError};
use clap::{Arg, ArgAction, ArgMatches, Command};
use rust_i18n::t;
use spbflib::read::SPBFReader;
use spbflib::SPBFType;
use std::borrow::Cow;
use std::io::{Read, Write};
use std::process::exit;
use std::{fs, io};
use upbflib::read::UPBFReader;
use upbflib::UPBFType;

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
                .arg(i18n_arg!("BUILD_VERSION", "cmd.new.arg.build_version"))
        )
        .subcommand(
            Command::new("write")
                .about(t!("cmd.write.about"))
                .arg(i18n_arg!("FILE", "cmd.write.arg.file"))
                .arg(i18n_arg!("DATA_NAME", "cmd.write.arg.data_name"))
                .arg(i18n_arg!("DATA_FORMAT", "cmd.write.arg.data_format"))
        )
        .subcommand(
            Command::new("remove")
                .about(t!("cmd.remove.about"))
                .arg(i18n_arg!("FILE", "cmd.remove.arg.file"))
                .arg(i18n_arg!("DATA_NAME", "cmd.remove.arg.data_name"))
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

fn cli_reader_new(buff: &'_ [u8]) -> XPBFReader<'_> {
    let reader =
        if spbflib::read::raw::check_magic(buff) {
            SPBFReader::new(buff).try_into()
        } else if upbflib::read::raw::check_magic(buff) {
            UPBFReader::new(buff).try_into()
        } else {
            Err(XPBFReaderError::Header(XPBFReaderHeaderReadError::InvalidMagic))
        };
    match reader {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_reader_error(err)
    }
}

fn cli_reader_read<'a>(reader: &'a XPBFReader<'a>) -> XPBFReadResult<'a> {
    match reader.read() {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_reader_error(err)
    }
}

fn cli_read_data<'a>(read: &'a XPBFReadResult<'a>, name: &String) -> XPBFDataForRead<'a> {
    match read.data().iter().find(|it| it.name() == name) {
        Some(some) => some.clone(),
        None => {
            eprintln!("{}", t!("cmd.err.data_not_found"));
            exit(1);
        }
    }
}

fn cli_writer_from_read(read: &XPBFReadResult) -> XPBFWriter {
    match XPBFWriter::try_from(read) {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_reader_error(err)
    }
}

fn cli_writer_write_data(writer: &mut XPBFWriter, name: &String, format: &String, data: Box<[u8]>) {
    match writer.add_or_overwrite_data(name, format, data) {
        Ok(()) => {}
        Err(err) => cli_exit_with_writer_error(err)
    }
}

fn cli_writer_remove_data(writer: &mut XPBFWriter, name: &String) {
    if !writer.remove_data(name) {
        cli_exit_with_custom_error(t!("cmd.err.remove.not_exits"))
    }
}

fn cli_writer_write_file(writer: &mut XPBFWriter, file: &str) {
    match writer.write() {
        Ok(bytes) => {
            if let Err(err) = fs::write(file, &bytes) {
                eprintln!("{}", t!("cmd.err.file_write", cause = err.to_string()));
            }
        }
        Err(err) => cli_exit_with_writer_error(err)
    }
}

fn cli_writer_convert(writer: XPBFWriter, target: XPBFType) -> XPBFWriter {
    match writer.convert(target) {
        Ok(ok) => ok,
        Err(err) => cli_exit_with_writer_error(err)
    }
}

pub fn cli_exit_with_custom_error(msg: Cow<str>) -> ! {
    eprintln!("{}", msg);
    exit(1);
}

fn cli_exit_with_reader_error(err: XPBFReaderError) -> ! {
    let cause =
        match err {
            XPBFReaderError::InvalidFileLength => t!("xpbf.err.read.invalid_file_length"),
            XPBFReaderError::Header(err) => match err {
                XPBFReaderHeaderReadError::InvalidMagic => t!("xpbf.err.read.header.invalid_magic"),
                XPBFReaderHeaderReadError::InvalidType => t!("xpbf.err.read.header.invalid_type"),
                XPBFReaderHeaderReadError::UnsupportedVersion => t!("xpbf.err.read.header.unsupported_version"),
                XPBFReaderHeaderReadError::InvalidBuildNameLength => t!("xpbf.err.read.header.invalid_build_name_length"),
                XPBFReaderHeaderReadError::InvalidBuildNameString => t!("xpbf.err.read.header.invalid_build_name_string"),
                XPBFReaderHeaderReadError::InvalidBuildVersionLength => t!("xpbf.err.read.header.invalid_build_version_length"),
                XPBFReaderHeaderReadError::InvalidBuildVersionString => t!("xpbf.err.read.header.invalid_build_version_string")
            }
            XPBFReaderError::DataFormat(err) => match err {
                XPBFReaderFormatReadError::InvalidOffset => t!("xpbf.err.read.data_format.invalid_offset"),
                XPBFReaderFormatReadError::InvalidNameLength => t!("xpbf.err.read.data_format.invalid_name_length"),
                XPBFReaderFormatReadError::InvalidNameString => t!("xpbf.err.read.data_format.invalid_name_string")
            }
            XPBFReaderError::Data(err) => match err {
                XPBFReaderDataReadError::InvalidOffset => t!("xpbf.err.read.data.invalid_offset"),
                XPBFReaderDataReadError::InvalidNameLength => t!("xpbf.err.read.data.invalid_name_length"),
                XPBFReaderDataReadError::InvalidNameString => t!("xpbf.err.read.data.invalid_name_string"),
                XPBFReaderDataReadError::InvalidDataLength => t!("xpbf.err.read.data.invalid_data_length"),
                XPBFReaderDataReadError::InvalidDataId => t!("xpbf.err.read.data.invalid_data_id")
            }
        };
    eprintln!("{}", t!("cli.err.reader", cause = cause));
    exit(1);
}

fn cli_exit_with_writer_error(err: XPBFWriterError) -> ! {
    let cause = match err {
        XPBFWriterError::DataAdd(err) => match err {
            XPBFWriterDataAddError::DataAlreadyDefined => t!("xpbf.err.write.data_already_defined"),
            XPBFWriterDataAddError::FormatCounterOverflow => t!("xpbf.err.write.data_counter_overflow"),
        }
        XPBFWriterError::Write(err) => match err {
            XPBFWriterWriteError::UnsupportedVersion => t!("xpbf.err.write.unsupported_version"),
            XPBFWriterWriteError::InvalidBuildNameLength => t!("xpbf.err.write.invalid_build_name_length"),
            XPBFWriterWriteError::InvalidBuildNameString => t!("xpbf.err.write.invalid_build_name_string"),
            XPBFWriterWriteError::InvalidBuildVersionLength => t!("xpbf.err.write.invalid_build_version_length"),
            XPBFWriterWriteError::InvalidBuildVersionString => t!("xpbf.err.write.invalid_build_version_string"),
            XPBFWriterWriteError::InvalidDataFormatNameLength => t!("xpbf.err.write.data_format.invalid_data_format_name_length"),
            XPBFWriterWriteError::InvalidDataFormatNameString => t!("xpbf.err.write.data_format.invalid_data_format_name_string"),
            XPBFWriterWriteError::InvalidDataNameLength => t!("xpbf.err.write.data_format.invalid_data_name_length"),
            XPBFWriterWriteError::InvalidDataNameString => t!("xpbf.err.write.data_format.invalid_data_name_string"),
            XPBFWriterWriteError::InvalidDataLength => t!("xpbf.err.write.data_format.invalid_data_length"),
            XPBFWriterWriteError::InvalidOffset => t!("xpbf.err.write.invalid_offset"),
            XPBFWriterWriteError::IOError(err) => Cow::from(err.to_string())
        }
    };
    eprintln!("{}", t!("cli.err.writer", cause = cause));
    exit(1);
}

fn cli_xpbf_type_parse(r#type: &str) -> XPBFType {
    match r#type {
        "SUL" | "МБМ" => SPBFType::SmallUnalignedLittleEndian.into(),
        "SUB" | "МБС" => SPBFType::SmallUnalignedBigEndian.into(),
        "SAL" | "МВМ" => SPBFType::SmallAlignedLittleEndian.into(),
        "SAB" | "МВС" => SPBFType::SmallAlignedBigEndian.into(),
        "MAL" | "СВМ" => UPBFType::MediumAlignedLittleEndian.into(),
        "MAB" | "СВС" => UPBFType::MediumAlignedBigEndian.into(),
        "BAL" | "БВМ" => UPBFType::BigAlignedLittleEndian.into(),
        "BAB" | "БВС" => UPBFType::BigAlignedBigEndian.into(),
        _ => {
            eprintln!("{}", t!("cmd.err.unknown_type", r#type=r#type));
            eprintln!("{}", cli_xpbf_type_to_str(SPBFType::SmallUnalignedLittleEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(SPBFType::SmallUnalignedBigEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(SPBFType::SmallAlignedLittleEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(SPBFType::SmallAlignedBigEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(UPBFType::MediumAlignedLittleEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(UPBFType::MediumAlignedBigEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(UPBFType::BigAlignedLittleEndian.into()));
            eprintln!("{}", cli_xpbf_type_to_str(UPBFType::BigAlignedBigEndian.into()));
            exit(1);
        }
    }
}

fn cli_xpbf_type_to_str(r#type: XPBFType) -> Cow<'static, str> {
    match r#type {
        XPBFType::SPBF(r#type) => match r#type {
            SPBFType::SmallUnalignedLittleEndian => t!("xpbf.type.sul"),
            SPBFType::SmallUnalignedBigEndian => t!("xpbf.type.sub"),
            SPBFType::SmallAlignedLittleEndian => t!("xpbf.type.sal"),
            SPBFType::SmallAlignedBigEndian => t!("xpbf.type.sab")
        }
        XPBFType::UPBF(r#type) => match r#type {
            UPBFType::MediumAlignedLittleEndian => t!("xpbf.type.mal"),
            UPBFType::MediumAlignedBigEndian => t!("xpbf.type.mab"),
            UPBFType::BigAlignedLittleEndian => t!("xpbf.type.bal"),
            UPBFType::BigAlignedBigEndian => t!("xpbf.type.bab")
        }
    }
}

fn cli_xpbf_version_to_str(version: XPBFVersion) -> String {
    format!("{} ({})", version.as_raw(), t!(if version.is_supported() { "xpbf.version.supported" } else { "xpbf.version.unsupported" }))
}

pub fn cli_process() {
    let matches = cli_commands().get_matches();
    match matches.subcommand() {
        Some(("info", sub_matches)) => cmd_info(sub_matches),
        Some(("new", sub_matches)) => cmd_new(sub_matches),
        Some(("write", sub_matches)) => cmd_write(sub_matches),
        Some(("remove", sub_matches)) => cmd_remove(sub_matches),
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
    println!("{}", t!("cmd.info.exec.file.type",    file_type = cli_xpbf_type_to_str(reader.file_type())));
    println!("{}", t!("cmd.info.exec.file.version", file_version = cli_xpbf_version_to_str(reader.file_version())));

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
                        data_format = data.format().name(),
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

    let file_type = cli_xpbf_type_parse(file_type);
    let mut writer = XPBFWriter::new(build_name.clone(), build_version.clone(), file_type);
    cli_writer_write_file(&mut writer, file_name);
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
    cli_writer_write_file(&mut writer, file_name);
}


fn cmd_remove(args: &ArgMatches) {
    let file_name = cli_get_arg::<String>(args, "FILE");
    let data_name = cli_get_arg::<String>(args, "DATA_NAME");

    let file = cli_read_file(file_name);
    let reader = cli_reader_new(&file);
    let read = cli_reader_read(&reader);
    let mut writer = cli_writer_from_read(&read);
    cli_writer_remove_data(&mut writer, &data_name);
    cli_writer_write_file(&mut writer, file_name);
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
    let file_type = cli_xpbf_type_parse(file_type);
    let reader = cli_reader_new(&file_in);
    let read = cli_reader_read(&reader);
    let writer = cli_writer_from_read(&read);
    let mut writer = cli_writer_convert(writer, file_type);
    cli_writer_write_file(&mut writer, file_out);
}
