use std::{
    io::{stderr, stdout, Error as IOError, Write},
    path::PathBuf,
    process::{Command, Output},
};

use tempfile::NamedTempFile;

use super::DOT;
use crate::{io::File, prelude::Plot};

/// Layout engine.
#[derive(Clone, Copy, Debug, Default)]
pub enum Layout {
    /// Hierarchical or layered drawings of directed graphs.
    #[default]
    Dot,
    /// "Spring model" layouts.
    Neato,
    /// Force-Directed Placement.
    Fdp,
    /// Scalable Force-Directed Placement.
    Sfdp,
    /// Circular layout.
    Circo,
    /// Radial layout.
    Twopi,
    /// Pretty-print DOT graph file.
    Nop,
    /// Pretty-print DOT graph file, assuming positions already known.
    Nop2,
    /// Draws clustered graphs.
    Osage,
    /// Draws map of clustered graph using a squarified treemap layout.
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

/// Output format.
#[derive(Clone, Copy, Debug, Default)]
pub enum Format {
    /// Windows Bitmap.
    Bmp,
    /// Graphviz Language.
    Canon,
    /// Apple Core Graphics.
    Cgimage,
    /// Image Map: Server-side and client-side.
    Cmap,
    /// Image Map: Server-side and client-side.
    Cmapx,
    /// Image Map: Server-side and client-side.
    CmapxNp,
    /// Graphviz Language.
    Dot,
    /// JavaScript Object Notation.
    DotJson,
    /// Encapsulated PostScript.
    Eps,
    /// OpenEXR.
    Exr,
    /// Xfig.
    Fig,
    /// LibGD.
    Gd,
    /// LibGD.
    Gd2,
    /// Graphics Interchange Format.
    Gif,
    /// Formerly GTK+ / GIMP ToolKit.
    Gtk,
    /// Graphviz Language.
    Gv,
    /// Windows Icon.
    Ico,
    /// Image Map: Server-side and client-side.
    Imap,
    /// Image Map: Server-side and client-side.
    ImapNp,
    /// Image Map: Server-side and client-side.
    Ismap,
    /// nan.
    Jp2,
    /// Joint Photographic Experts Group.
    Jpe,
    /// Joint Photographic Experts Group.
    Jpeg,
    /// Joint Photographic Experts Group.
    Jpg,
    /// JavaScript Object Notation.
    Json,
    /// JavaScript Object Notation.
    Json0,
    /// Apple PICT.
    Pct,
    /// Portable Document Format.
    #[default]
    Pdf,
    /// Brian Kernighan's Diagram Language.
    Pic,
    /// Apple PICT.
    Pict,
    /// Simple, line-based language.
    Plain,
    /// Simple, line-based language.
    PlainExt,
    /// Portable Network Graphics.
    Png,
    /// Persistence of Vision Raytracer (prototype).
    Pov,
    /// Adobe PostScript.
    Ps,
    /// Adobe PostScript for Portable Document Format.
    Ps2,
    /// Photoshop.
    Psd,
    /// Silicon Graphics Image.
    Sgi,
    /// Scalable Vector Graphics.
    Svg,
    /// Scalable Vector Graphics.
    Svgz,
    /// Truevision TARGA.
    Tga,
    /// Tag Image File Format.
    Tif,
    /// Tag Image File Format.
    Tiff,
    /// Tcl/Tk.
    Tk,
    /// Vector Markup Language..
    Vml,
    /// Vector Markup Language..
    Vmlz,
    /// Virtual Reality Modeling Language.
    Vrml,
    /// Wireless Bitmap.
    Wbmp,
    /// WebP.
    Webp,
    /// X11 Window.
    X11,
    /// Graphviz Language.
    Xdot,
    /// Graphviz Language.
    Xdot12,
    /// Graphviz Language.
    Xdot14,
    /// JavaScript Object Notation.
    XdotJson,
    /// X11 Window.
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
    /// Set [`Layout`] engine.
    pub fn with_layout(mut self, layout: Layout) -> Self {
        // Set layout engine.
        self.layout = layout;

        self
    }

    /// Set output [`Format`].
    pub fn with_format(mut self, format: Format) -> Self {
        // Set output format.
        self.format = format;

        self
    }
}

impl Plot for DOT {
    type Success = Output;

    type Error = IOError;

    /// Plot to path with given [`Layout`] and [`Format`].
    ///
    /// # Panics
    ///
    /// Require <a href = "https://graphviz.org/" target = "_blank">Graphviz</a> to work.
    ///
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
