use std::{
    io::{stderr, stdout, Error as IOError, Write},
    path::PathBuf,
    process::{Command, Output},
};

use tempfile::NamedTempFile;

use super::DOT;
use crate::{io::File, prelude::Plot};

#[derive(Clone, Copy, Debug, Default)]
pub enum Layout {
    #[default]
    Dot,

    Neato,

    Fdp,

    Sfdp,

    Circo,

    Twopi,

    Nop,

    Nop2,

    Osage,

    Patchwork,
}

impl From<Layout> for String {
    fn from(layout: Layout) -> Self {
        let layout = match layout {
            Layout::Dot => "dot",
            Layout::Neato => "neato",
            Layout::Fdp => "fpd",
            Layout::Sfdp => "sfdp",
            Layout::Circo => "circo",
            Layout::Twopi => "twopi",
            Layout::Nop => "nop",
            Layout::Nop2 => "nop2",
            Layout::Osage => "osage",
            Layout::Patchwork => "patchwork",
        };

        layout.into()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Format {
    Bmp,

    Canon,

    Cgimage,

    Cmap,

    Cmapx,

    CmapxNp,

    Dot,

    DotJson,

    Eps,

    Exr,

    Fig,

    Gd,

    Gd2,

    Gif,

    Gtk,

    Gv,

    Ico,

    Imap,

    ImapNp,

    Ismap,

    Jp2,

    Jpe,

    Jpeg,

    Jpg,

    Json,

    Json0,

    Pct,

    #[default]
    Pdf,

    Pic,

    Pict,

    Plain,

    PlainExt,

    Png,

    Pov,

    Ps,

    Ps2,

    Psd,

    Sgi,

    Svg,

    Svgz,

    Tga,

    Tif,

    Tiff,

    Tk,

    Vml,

    Vmlz,

    Vrml,

    Wbmp,

    Webp,

    X11,

    Xdot,

    Xdot12,

    Xdot14,

    XdotJson,

    Xlib,
}

impl From<Format> for String {
    fn from(format: Format) -> Self {
        let format = match format {
            Format::Bmp => "bmp",
            Format::Canon => "canon",
            Format::Cgimage => "cgimage",
            Format::Cmap => "cmap",
            Format::Cmapx => "cmapx",
            Format::CmapxNp => "cmapx_np",
            Format::Dot => "dot",
            Format::DotJson => "dot_json",
            Format::Eps => "eps",
            Format::Exr => "exr",
            Format::Fig => "fig",
            Format::Gd => "gd",
            Format::Gd2 => "gd2",
            Format::Gif => "gif",
            Format::Gtk => "gtk",
            Format::Gv => "gv",
            Format::Ico => "ico",
            Format::Imap => "imap",
            Format::ImapNp => "imap_np",
            Format::Ismap => "ismap",
            Format::Jp2 => "jp2",
            Format::Jpe => "jpe",
            Format::Jpeg => "jpeg",
            Format::Jpg => "jpg",
            Format::Json => "json",
            Format::Json0 => "json0",
            Format::Pct => "pct",
            Format::Pdf => "pdf",
            Format::Pic => "pic",
            Format::Pict => "pict",
            Format::Plain => "plain",
            Format::PlainExt => "plain-ext",
            Format::Png => "png",
            Format::Pov => "pov",
            Format::Ps => "ps",
            Format::Ps2 => "ps2",
            Format::Psd => "psd",
            Format::Sgi => "sgi",
            Format::Svg => "svg",
            Format::Svgz => "svgz",
            Format::Tga => "tga",
            Format::Tif => "tif",
            Format::Tiff => "tiff",
            Format::Tk => "tk",
            Format::Vml => "vml",
            Format::Vmlz => "vmlz",
            Format::Vrml => "vrml",
            Format::Wbmp => "wbmp",
            Format::Webp => "webp",
            Format::X11 => "x11",
            Format::Xdot => "xdot",
            Format::Xdot12 => "xdot1.2",
            Format::Xdot14 => "xdot1.4",
            Format::XdotJson => "xdot_json",
            Format::Xlib => "xlib",
        };

        format.into()
    }
}

impl DOT {
    pub fn with_layout(mut self, layout: Layout) -> Self {
        // Set layout engine.
        self.layout = layout;

        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        // Set output format.
        self.format = format;

        self
    }
}

impl Plot for DOT {
    type Success = Output;

    type Error = IOError;

    fn plot<P>(self, path: P) -> Result<Self::Success, Self::Error>
    where
        P: Into<PathBuf>,
    {
        // Create new temp file.
        let input = NamedTempFile::new().expect("Failed to create tempfile");

        // Get layout engine.
        let layout = String::from(self.layout);
        // Get output format.
        let format = String::from(self.format);

        // Initialize plot command.
        let mut plot = Command::new("dot");
        // Add command arguments
        let plot = plot
            // Set layout engine.
            .arg(format!("-K{layout}"))
            // Set output format.
            .arg(format!("-T{format}"))
            // Set output path.
            .arg(format!("-o{}", path.into().display()))
            // Set input path.
            .arg(input.path());

        // Write DOT to temporary file.
        self.write(input.path()).expect("Failed to write tempfile");

        // Execute command and get output.
        let output = plot.output().expect("Failed to execute the plot command");

        // Log plot status and output.
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();

        Ok(output)
    }
}
