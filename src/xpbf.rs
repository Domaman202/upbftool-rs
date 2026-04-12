use rust_i18n::t;
use spbflib::read::{SPBFDataForRead, SPBFDataFormatForRead, SPBFReadResult, SPBFReader, SPBFReaderDataFormatReadError, SPBFReaderDataReadError, SPBFReaderError, SPBFReaderHeaderReadError};
use spbflib::write::{SPBFWriter, SPBFWriterDataAddError, SPBFWriterError, SPBFWriterWriteError};
use spbflib::{SPBFType, SPBFVersion};
use upbflib::read::{UPBFDataForRead, UPBFDataFormatForRead, UPBFReadResult, UPBFReader, UPBFReaderDataFormatReadError, UPBFReaderDataReadError, UPBFReaderError, UPBFReaderHeaderReadError};
use upbflib::write::{UPBFWriter, UPBFWriterDataAddError, UPBFWriterError, UPBFWriterWriteError};
use upbflib::{UPBFType, UPBFVersion};
use crate::cli::cli_exit_with_custom_error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XPBFType {
    SPBF(SPBFType),
    UPBF(UPBFType),
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XPBFVersion {
    SPBF(SPBFVersion),
    UPBF(UPBFVersion),
}

#[derive(Debug)]
pub enum XPBFReader<'a> {
    SPBF(SPBFReader<'a>),
    UPBF(UPBFReader<'a>),
}

#[derive(Debug, Clone)]
pub enum XPBFReadResult<'a> {
    SPBF(SPBFReadResult<'a>),
    UPBF(UPBFReadResult<'a>),
}

#[derive(Debug, Clone)]
pub enum XPBFDataFormatForRead<'a> {
    SPBF(&'a SPBFDataFormatForRead),
    UPBF(&'a UPBFDataFormatForRead),
}

#[derive(Debug, Clone)]
pub enum XPBFDataForRead<'a> {
    SPBF(&'a SPBFDataForRead<'a>, &'a SPBFReadResult<'a>),
    UPBF(&'a UPBFDataForRead<'a>, &'a UPBFReadResult<'a>),
}

#[derive(Debug)]
pub enum XPBFWriter {
    SPBF(SPBFWriter, SPBFType, SPBFVersion),
    UPBF(UPBFWriter, UPBFType, UPBFVersion),
}

#[derive(Debug)]
pub enum XPBFReaderError {
    InvalidFileLength,
    Header(XPBFReaderHeaderReadError),
    DataFormat(XPBFReaderFormatReadError),
    Data(XPBFReaderDataReadError)
}

#[derive(Debug)]
pub enum XPBFReaderHeaderReadError {
    InvalidMagic,
    InvalidType,
    UnsupportedVersion,
    InvalidBuildNameLength,
    InvalidBuildNameString,
    InvalidBuildVersionLength,
    InvalidBuildVersionString,
}

#[derive(Debug)]
pub enum XPBFReaderFormatReadError {
    InvalidOffset,
    InvalidNameLength,
    InvalidNameString,
}

#[derive(Debug)]
pub enum XPBFReaderDataReadError {
    InvalidOffset,
    InvalidNameLength,
    InvalidNameString,
    InvalidDataLength,
    InvalidDataId
}

#[derive(Debug)]
pub enum XPBFWriterError {
    DataAdd(XPBFWriterDataAddError),
    Write(XPBFWriterWriteError)
}

#[derive(Debug)]
pub enum XPBFWriterDataAddError {
    DataAlreadyDefined,
    FormatCounterOverflow
}

#[derive(Debug)]
pub enum XPBFWriterWriteError {
    UnsupportedVersion,
    InvalidBuildNameLength,
    InvalidBuildNameString,
    InvalidBuildVersionLength,
    InvalidBuildVersionString,
    InvalidDataFormatNameLength,
    InvalidDataFormatNameString,
    InvalidDataNameLength,
    InvalidDataNameString,
    InvalidDataLength,
    InvalidOffset,
    IOError(std::io::Error),
}

impl From<SPBFType> for XPBFType {
    fn from(value: SPBFType) -> Self {
        Self::SPBF(value)
    }
}

impl From<UPBFType> for XPBFType {
    fn from(value: UPBFType) -> Self {
        Self::UPBF(value)
    }
}

impl Into<SPBFType> for XPBFType {
    fn into(self) -> SPBFType {
        if let Self::SPBF(value) = self {
            value
        } else {
            cli_exit_with_custom_error(t!("xpbf.err.convert.no_spbf_type"))
        }
    }
}

impl From<SPBFVersion> for XPBFVersion {
    fn from(value: SPBFVersion) -> Self {
        Self::SPBF(value)
    }
}

impl From<UPBFVersion> for XPBFVersion {
    fn from(value: UPBFVersion) -> Self {
        Self::UPBF(value)
    }
}

impl Into<UPBFType> for XPBFType {
    fn into(self) -> UPBFType {
        if let Self::UPBF(value) = self {
            value
        } else {
            cli_exit_with_custom_error(t!("xpbf.err.convert.no_upbf_type"))
        }
    }
}

impl Into<SPBFVersion> for XPBFVersion {
    fn into(self) -> SPBFVersion {
        if let Self::SPBF(value) = self {
            value
        } else {
            cli_exit_with_custom_error(t!("xpbf.err.convert.no_spbf_version"))
        }
    }
}

impl Into<UPBFVersion> for XPBFVersion {
    fn into(self) -> UPBFVersion {
        if let Self::UPBF(value) = self {
            value
        } else {
            cli_exit_with_custom_error(t!("xpbf.err.convert.no_upbf_version"))
        }
    }
}

impl XPBFVersion {
    pub fn is_supported(&self) -> bool {
        match self {
            Self::SPBF(value) => value.is_supported(),
            Self::UPBF(value) => value.is_supported()
        }
    }

    pub fn as_raw(self) -> u8 {
        match self {
            Self::SPBF(value) => value.as_raw(),
            Self::UPBF(value) => value.as_raw()
        }
    }
}

impl<'a> TryFrom<Result<SPBFReader<'a>, SPBFReaderError>> for XPBFReader<'a> {
    type Error = XPBFReaderError;
    
    fn try_from(value: Result<SPBFReader<'a>, SPBFReaderError>) -> Result<Self, Self::Error> {
        match value {
            Ok(ok) => Ok(Self::SPBF(ok)),
            Err(err) => Err(err.into())
        }
    }
}

impl<'a> TryFrom<Result<UPBFReader<'a>, UPBFReaderError>> for XPBFReader<'a> {
    type Error = XPBFReaderError;

    fn try_from(value: Result<UPBFReader<'a>, UPBFReaderError>) -> Result<Self, Self::Error> {
        match value {
            Ok(ok) => Ok(Self::UPBF(ok)),
            Err(err) => Err(err.into())
        }
    }
}

impl<'a> XPBFReader<'a> {
    pub fn read(&'_ self) -> Result<XPBFReadResult<'_>, XPBFReaderError> {
        match self {
            Self::SPBF(reader) => reader.read().try_into(),
            Self::UPBF(reader) => reader.read().try_into()
        }
    }

    pub fn is_read_supported(&self) -> bool {
        match self {
            Self::SPBF(reader) => reader.is_read_supported(),
            Self::UPBF(reader) => reader.is_read_supported()
        }
    }

    pub fn file_type(&self) -> XPBFType {
        match self {
            Self::SPBF(reader) => reader.file_type().into(),
            Self::UPBF(reader) => reader.file_type().into()
        }
    }

    pub fn file_version(&self) -> XPBFVersion {
        match self {
            Self::SPBF(reader) => reader.file_version().into(),
            Self::UPBF(reader) => reader.file_version().into()
        }
    }
}

impl<'a> TryFrom<Result<SPBFReadResult<'a>, SPBFReaderError>> for XPBFReadResult<'a> {
    type Error = XPBFReaderError;

    fn try_from(value: Result<SPBFReadResult<'a>, SPBFReaderError>) -> Result<Self, Self::Error> {
        match value {
            Ok(ok) => Ok(Self::SPBF(ok)),
            Err(err) => Err(err.into())
        }
    }
}

impl<'a> TryFrom<Result<UPBFReadResult<'a>, UPBFReaderError>> for XPBFReadResult<'a> {
    type Error = XPBFReaderError;

    fn try_from(value: Result<UPBFReadResult<'a>, UPBFReaderError>) -> Result<Self, Self::Error> {
        match value {
            Ok(ok) => Ok(Self::UPBF(ok)),
            Err(err) => Err(err.into())
        }
    }
}

impl XPBFReadResult<'_> {
    pub fn build_name(&self) -> &String {
        match self {
            Self::SPBF(read) => read.build_name(),
            Self::UPBF(read) => read.build_name()
        }
    }

    pub fn build_version(&self) -> &String {
        match self {
            Self::SPBF(read) => read.build_version(),
            Self::UPBF(read) => read.build_version()
        }
    }

    pub fn data_formats(&self) -> Vec<XPBFDataFormatForRead<'_>> {
        match self {
            Self::SPBF(read) => read.data_formats().iter().map(|it| XPBFDataFormatForRead::SPBF(it)).collect(),
            Self::UPBF(read) => read.data_formats().iter().map(|it| XPBFDataFormatForRead::UPBF(it)).collect()
        }
    }

    pub fn data(&self) -> Vec<XPBFDataForRead<'_>> {
        match self {
            Self::SPBF(read) => read.data().iter().map(|it| XPBFDataForRead::SPBF(it, &read)).collect(),
            Self::UPBF(read) => read.data().iter().map(|it| XPBFDataForRead::UPBF(it, &read)).collect()
        }
    }
}

impl XPBFDataFormatForRead<'_> {
    pub fn name(&self) -> &String {
        match self {
            Self::SPBF(format) => format.name(),
            Self::UPBF(format) => format.name()
        }
    }

    pub fn data_id(&self) -> u32 {
        match self {
            Self::SPBF(format) => format.data_id() as u32,
            Self::UPBF(format) => format.data_id()
        }
    }
}

impl XPBFDataForRead<'_> {
    pub fn name(&self) -> &String {
        match self {
            Self::SPBF(data, _) => data.name(),
            Self::UPBF(data, _) => data.name()
        }
    }

    pub fn format(&self) -> XPBFDataFormatForRead<'_> {
        match self {
            Self::SPBF(data, read) => XPBFDataFormatForRead::SPBF(data.format(*read)),
            Self::UPBF(data, read) => XPBFDataFormatForRead::UPBF(data.format(*read))
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            Self::SPBF(data, _) => data.data(),
            Self::UPBF(data, _) => data.data()
        }
    }
}

impl TryFrom<&XPBFReadResult<'_>> for XPBFWriter {
    type Error = XPBFReaderError;

    fn try_from(value: &XPBFReadResult) -> Result<Self, Self::Error> {
        match value {
            XPBFReadResult::SPBF(read) => Ok(XPBFWriter::SPBF(SPBFWriter::try_from(read)?, read.file_type(), read.file_version())),
            XPBFReadResult::UPBF(read) => Ok(XPBFWriter::UPBF(UPBFWriter::try_from(read)?, read.file_type(), read.file_version())),
        }
    }
}

impl XPBFWriter {
    pub fn new(build_name: String, build_version: String, r#type: XPBFType) -> Self {
        match r#type {
            XPBFType::SPBF(r#type) => Self::SPBF(SPBFWriter::new(build_name, build_version), r#type, SPBFVersion::LAST_SUPPORTED),
            XPBFType::UPBF(r#type) => Self::UPBF(UPBFWriter::new(build_name, build_version), r#type, UPBFVersion::LAST_SUPPORTED),
        }
    }

    pub fn add_data(&mut self, name: String, format: &String, bytes: Box<[u8]>) -> Result<(), XPBFWriterError> {
        match self {
            Self::SPBF(writer, _, _) => writer.add_data(name, format, bytes).map_err(XPBFWriterError::from),
            Self::UPBF(writer, _, _) => writer.add_data(name, format, bytes).map_err(XPBFWriterError::from)
        }
    }

    pub fn add_or_overwrite_data(&mut self, name: &String, format: &String, bytes: Box<[u8]>) -> Result<(), XPBFWriterError> {
        match self {
            Self::SPBF(writer, _, _) => writer.add_or_overwrite_data(name, format, bytes).map_err(XPBFWriterError::from),
            Self::UPBF(writer, _, _) => writer.add_or_overwrite_data(name, format, bytes).map_err(XPBFWriterError::from)
        }
    }

    pub fn remove_data(&mut self, name: &String) -> bool {
        match self {
            Self::SPBF(writer, _, _) => writer.remove_data(name),
            Self::UPBF(writer, _, _) => writer.remove_data(name)
        }
    }

    pub fn write(&mut self) -> Result<Vec<u8>, XPBFWriterError> {
        match self {
            Self::SPBF(writer, r#type, version) => writer.write(*r#type, *version).map_err(XPBFWriterError::from),
            Self::UPBF(writer, r#type, version) => writer.write(*r#type, *version).map_err(XPBFWriterError::from)
        }
    }

    pub fn convert(self, target: XPBFType) -> Result<XPBFWriter, XPBFWriterError> {
        match self {
            Self::SPBF(writer, _, version) => {
                match target {
                    XPBFType::SPBF(target) => Ok(Self::SPBF(writer, target, version)),
                    XPBFType::UPBF(target) => {
                        let mut new = Self::UPBF(UPBFWriter::new(writer.build_name.clone(), writer.build_version.clone()), target, UPBFVersion::LAST_SUPPORTED);
                        for data in writer.data() {
                            let name = data.name().clone();
                            let format = unsafe {
                                writer
                                    .data_formats()
                                    .iter()
                                    .find(|it| it.data_id() == data.data_id())
                                    .unwrap_unchecked()
                            };
                            let format = format.name();
                            let data = Box::from(data.data());
                            new.add_data(name, format, data)?;
                        }
                        Ok(new)
                    }
                }
            },
            Self::UPBF(writer, _, version) => {
                match target {
                    XPBFType::SPBF(target) => {
                        let mut new = Self::SPBF(SPBFWriter::new(writer.build_name.clone(), writer.build_version.clone()), target, SPBFVersion::LAST_SUPPORTED);
                        for data in writer.data() {
                            let name = data.name().clone();
                            let format = unsafe {
                                writer
                                    .data_formats()
                                    .iter()
                                    .find(|it| it.data_id() == data.data_id())
                                    .unwrap_unchecked()
                            };
                            let format = format.name();
                            let data = Box::from(data.data());
                            new.add_data(name, format, data)?;
                        }
                        Ok(new)
                    },
                    XPBFType::UPBF(target) => Ok(Self::UPBF(writer, target, version))
                }
            }
        }
    }
}

impl From<SPBFReaderError> for XPBFReaderError {
    fn from(err: SPBFReaderError) -> XPBFReaderError {
        match err {
            SPBFReaderError::InvalidFileLength => Self::InvalidFileLength,
            SPBFReaderError::Header(err) => match err {
                SPBFReaderHeaderReadError::InvalidMagic => Self::Header(XPBFReaderHeaderReadError::InvalidMagic),
                SPBFReaderHeaderReadError::InvalidType => Self::Header(XPBFReaderHeaderReadError::InvalidType),
                SPBFReaderHeaderReadError::UnsupportedVersion => Self::Header(XPBFReaderHeaderReadError::UnsupportedVersion),
                SPBFReaderHeaderReadError::InvalidBuildNameLength => Self::Header(XPBFReaderHeaderReadError::InvalidBuildNameLength),
                SPBFReaderHeaderReadError::InvalidBuildNameString => Self::Header(XPBFReaderHeaderReadError::InvalidBuildNameString),
                SPBFReaderHeaderReadError::InvalidBuildVersionLength => Self::Header(XPBFReaderHeaderReadError::InvalidBuildVersionLength),
                SPBFReaderHeaderReadError::InvalidBuildVersionString => Self::Header(XPBFReaderHeaderReadError::InvalidBuildVersionString),
            }
            SPBFReaderError::DataFormat(err) => match err {
                SPBFReaderDataFormatReadError::InvalidOffset => Self::DataFormat(XPBFReaderFormatReadError::InvalidOffset),
                SPBFReaderDataFormatReadError::InvalidNameLength => Self::DataFormat(XPBFReaderFormatReadError::InvalidNameLength),
                SPBFReaderDataFormatReadError::InvalidNameString => Self::DataFormat(XPBFReaderFormatReadError::InvalidNameString),
            }
            SPBFReaderError::Data(err) => match err {
                SPBFReaderDataReadError::InvalidOffset => Self::Data(XPBFReaderDataReadError::InvalidOffset),
                SPBFReaderDataReadError::InvalidNameLength => Self::Data(XPBFReaderDataReadError::InvalidNameLength),
                SPBFReaderDataReadError::InvalidNameString => Self::Data(XPBFReaderDataReadError::InvalidNameString),
                SPBFReaderDataReadError::InvalidDataLength => Self::Data(XPBFReaderDataReadError::InvalidDataLength),
                SPBFReaderDataReadError::InvalidDataId => Self::Data(XPBFReaderDataReadError::InvalidDataId)
            }
        }
    }
}

impl From<UPBFReaderError> for XPBFReaderError {
    fn from(err: UPBFReaderError) -> XPBFReaderError {
        match err {
            UPBFReaderError::InvalidFileLength => Self::InvalidFileLength,
            UPBFReaderError::Header(err) => match err {
                UPBFReaderHeaderReadError::InvalidMagic => Self::Header(XPBFReaderHeaderReadError::InvalidMagic),
                UPBFReaderHeaderReadError::InvalidType => Self::Header(XPBFReaderHeaderReadError::InvalidType),
                UPBFReaderHeaderReadError::UnsupportedVersion => Self::Header(XPBFReaderHeaderReadError::UnsupportedVersion),
                UPBFReaderHeaderReadError::InvalidBuildNameLength => Self::Header(XPBFReaderHeaderReadError::InvalidBuildNameLength),
                UPBFReaderHeaderReadError::InvalidBuildNameString => Self::Header(XPBFReaderHeaderReadError::InvalidBuildNameString),
                UPBFReaderHeaderReadError::InvalidBuildVersionLength => Self::Header(XPBFReaderHeaderReadError::InvalidBuildVersionLength),
                UPBFReaderHeaderReadError::InvalidBuildVersionString => Self::Header(XPBFReaderHeaderReadError::InvalidBuildVersionString),
            }
            UPBFReaderError::DataFormat(err) => match err {
                UPBFReaderDataFormatReadError::InvalidOffset => Self::DataFormat(XPBFReaderFormatReadError::InvalidOffset),
                UPBFReaderDataFormatReadError::InvalidNameLength => Self::DataFormat(XPBFReaderFormatReadError::InvalidNameLength),
                UPBFReaderDataFormatReadError::InvalidNameString => Self::DataFormat(XPBFReaderFormatReadError::InvalidNameString),
            }
            UPBFReaderError::Data(err) => match err {
                UPBFReaderDataReadError::InvalidOffset => Self::Data(XPBFReaderDataReadError::InvalidOffset),
                UPBFReaderDataReadError::InvalidNameLength => Self::Data(XPBFReaderDataReadError::InvalidNameLength),
                UPBFReaderDataReadError::InvalidNameString => Self::Data(XPBFReaderDataReadError::InvalidNameString),
                UPBFReaderDataReadError::InvalidDataLength => Self::Data(XPBFReaderDataReadError::InvalidDataLength),
                UPBFReaderDataReadError::InvalidDataId => Self::Data(XPBFReaderDataReadError::InvalidDataId)
            }
        }
    }
}

impl From<SPBFWriterError> for XPBFWriterError {
    fn from(err: SPBFWriterError) -> Self {
        match err {
            SPBFWriterError::DataAdd(err) => match err {
                SPBFWriterDataAddError::DataAlreadyDefined => Self::DataAdd(XPBFWriterDataAddError::DataAlreadyDefined),
                SPBFWriterDataAddError::FormatCounterOverflow => Self::DataAdd(XPBFWriterDataAddError::FormatCounterOverflow),
            }
            SPBFWriterError::Write(err) => match err {
                SPBFWriterWriteError::UnsupportedVersion => Self::Write(XPBFWriterWriteError::UnsupportedVersion),
                SPBFWriterWriteError::InvalidBuildNameLength => Self::Write(XPBFWriterWriteError::InvalidBuildNameLength),
                SPBFWriterWriteError::InvalidBuildNameString => Self::Write(XPBFWriterWriteError::InvalidBuildNameString),
                SPBFWriterWriteError::InvalidBuildVersionLength => Self::Write(XPBFWriterWriteError::InvalidBuildVersionLength),
                SPBFWriterWriteError::InvalidBuildVersionString => Self::Write(XPBFWriterWriteError::InvalidBuildVersionString),
                SPBFWriterWriteError::InvalidDataFormatNameLength => Self::Write(XPBFWriterWriteError::InvalidDataFormatNameLength),
                SPBFWriterWriteError::InvalidDataFormatNameString => Self::Write(XPBFWriterWriteError::InvalidDataFormatNameString),
                SPBFWriterWriteError::InvalidDataNameLength => Self::Write(XPBFWriterWriteError::InvalidDataNameLength),
                SPBFWriterWriteError::InvalidDataNameString => Self::Write(XPBFWriterWriteError::InvalidDataNameString),
                SPBFWriterWriteError::InvalidDataLength => Self::Write(XPBFWriterWriteError::InvalidDataLength),
                SPBFWriterWriteError::InvalidOffset => Self::Write(XPBFWriterWriteError::InvalidOffset),
                SPBFWriterWriteError::IOError(err) => Self::Write(XPBFWriterWriteError::IOError(err))
            }
        }
    }
}

impl From<UPBFWriterError> for XPBFWriterError {
    fn from(err: UPBFWriterError) -> Self {
        match err {
            UPBFWriterError::DataAdd(err) => match err {
                UPBFWriterDataAddError::DataAlreadyDefined => Self::DataAdd(XPBFWriterDataAddError::DataAlreadyDefined),
                UPBFWriterDataAddError::FormatCounterOverflow => Self::DataAdd(XPBFWriterDataAddError::FormatCounterOverflow),
            }
            UPBFWriterError::Write(err) => match err {
                UPBFWriterWriteError::UnsupportedVersion => Self::Write(XPBFWriterWriteError::UnsupportedVersion),
                UPBFWriterWriteError::InvalidBuildNameLength => Self::Write(XPBFWriterWriteError::InvalidBuildNameLength),
                UPBFWriterWriteError::InvalidBuildVersionLength => Self::Write(XPBFWriterWriteError::InvalidBuildVersionLength),
                UPBFWriterWriteError::InvalidDataFormatNameLength => Self::Write(XPBFWriterWriteError::InvalidDataFormatNameLength),
                UPBFWriterWriteError::InvalidDataNameLength => Self::Write(XPBFWriterWriteError::InvalidDataNameLength),
                UPBFWriterWriteError::InvalidDataLength => Self::Write(XPBFWriterWriteError::InvalidDataLength),
                UPBFWriterWriteError::InvalidOffset => Self::Write(XPBFWriterWriteError::InvalidOffset),
                UPBFWriterWriteError::IOError(err) => Self::Write(XPBFWriterWriteError::IOError(err)),
            }
        }
    }
}