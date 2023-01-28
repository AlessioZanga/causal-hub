// Automatically generated on: 2023-01-28 12:37:25.340358 .

/// A string in the xdot format specifying an arbitrary background.
#[derive(Clone, Debug)]
struct Background(pub(crate) String);

impl Background {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_background.in"))
    }
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_background = \"{}\"", self.0)
    }
}

/// Indicates the preferred area for a node or empty cluster. patchwork only.
#[derive(Clone, Debug)]
struct Area(pub(crate) String);

impl Area {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_area.in"))
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "area = \"{}\"", self.0)
    }
}

/// Style of arrowhead on the head node of an edge.
#[derive(Clone, Debug)]
struct Arrowhead(pub(crate) String);

impl Arrowhead {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_arrowhead.in"))
    }
}

impl std::fmt::Display for Arrowhead {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "arrowhead = \"{}\"", self.0)
    }
}

/// Multiplicative scale factor for arrowheads.
#[derive(Clone, Debug)]
struct Arrowsize(pub(crate) String);

impl Arrowsize {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_arrowsize.in"))
    }
}

impl std::fmt::Display for Arrowsize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "arrowsize = \"{}\"", self.0)
    }
}

/// Style of arrowhead on the tail node of an edge.
#[derive(Clone, Debug)]
struct Arrowtail(pub(crate) String);

impl Arrowtail {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_arrowtail.in"))
    }
}

impl std::fmt::Display for Arrowtail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "arrowtail = \"{}\"", self.0)
    }
}

/// Bounding box of drawing in points. write only.
#[derive(Clone, Debug)]
struct Bb(pub(crate) String);

impl Bb {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_bb.in"))
    }
}

impl std::fmt::Display for Bb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bb = \"{}\"", self.0)
    }
}

/// Whether to draw leaf nodes uniformly in a circle around the root node in sfdp.. sfdp only.
#[derive(Clone, Debug)]
struct Beautify(pub(crate) String);

impl Beautify {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_beautify.in"))
    }
}

impl std::fmt::Display for Beautify {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "beautify = \"{}\"", self.0)
    }
}

/// Canvas background color.
#[derive(Clone, Debug)]
struct Bgcolor(pub(crate) String);

impl Bgcolor {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_bgcolor.in"))
    }
}

impl std::fmt::Display for Bgcolor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bgcolor = \"{}\"", self.0)
    }
}

/// Whether to center the drawing in the output canvas.
#[derive(Clone, Debug)]
struct Center(pub(crate) String);

impl Center {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_center.in"))
    }
}

impl std::fmt::Display for Center {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "center = \"{}\"", self.0)
    }
}

/// Character encoding used when interpreting string input as a text label..
#[derive(Clone, Debug)]
struct Charset(pub(crate) String);

impl Charset {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_charset.in"))
    }
}

impl std::fmt::Display for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "charset = \"{}\"", self.0)
    }
}

/// Classnames to attach to the node, edge, graph, or cluster's SVG element. svg only.
#[derive(Clone, Debug)]
struct Class(pub(crate) String);

impl Class {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_class.in"))
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "class = \"{}\"", self.0)
    }
}

/// Whether the subgraph is a cluster.
#[derive(Clone, Debug)]
struct Cluster(pub(crate) String);

impl Cluster {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_cluster.in"))
    }
}

impl std::fmt::Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "cluster = \"{}\"", self.0)
    }
}

/// Mode used for handling clusters. dot only.
#[derive(Clone, Debug)]
struct Clusterrank(pub(crate) String);

impl Clusterrank {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_clusterrank.in"))
    }
}

impl std::fmt::Display for Clusterrank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "clusterrank = \"{}\"", self.0)
    }
}

/// Basic drawing color for graphics, not text.
#[derive(Clone, Debug)]
struct Color(pub(crate) String);

impl Color {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_color.in"))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "color = \"{}\"", self.0)
    }
}

/// A color scheme namespace: the context for interpreting color names.
#[derive(Clone, Debug)]
struct Colorscheme(pub(crate) String);

impl Colorscheme {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_colorscheme.in"))
    }
}

impl std::fmt::Display for Colorscheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "colorscheme = \"{}\"", self.0)
    }
}

/// Comments are inserted into output.
#[derive(Clone, Debug)]
struct Comment(pub(crate) String);

impl Comment {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_comment.in"))
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "comment = \"{}\"", self.0)
    }
}

/// If true, allow edges between clusters. dot only.
#[derive(Clone, Debug)]
struct Compound(pub(crate) String);

impl Compound {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_compound.in"))
    }
}

impl std::fmt::Display for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "compound = \"{}\"", self.0)
    }
}

/// If true, use edge concentrators.
#[derive(Clone, Debug)]
struct Concentrate(pub(crate) String);

impl Concentrate {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_concentrate.in"))
    }
}

impl std::fmt::Display for Concentrate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "concentrate = \"{}\"", self.0)
    }
}

/// If false, the edge is not used in ranking the nodes. dot only.
#[derive(Clone, Debug)]
struct Constraint(pub(crate) String);

impl Constraint {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_constraint.in"))
    }
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "constraint = \"{}\"", self.0)
    }
}

/// Factor damping force motions.. neato only.
#[derive(Clone, Debug)]
struct Damping(pub(crate) String);

impl Damping {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_damping.in"))
    }
}

impl std::fmt::Display for Damping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Damping = \"{}\"", self.0)
    }
}

/// Whether to connect the edge label to the edge with a line.
#[derive(Clone, Debug)]
struct Decorate(pub(crate) String);

impl Decorate {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_decorate.in"))
    }
}

impl std::fmt::Display for Decorate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "decorate = \"{}\"", self.0)
    }
}

/// The distance between nodes in separate connected components. neato only.
#[derive(Clone, Debug)]
struct Defaultdist(pub(crate) String);

impl Defaultdist {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_defaultdist.in"))
    }
}

impl std::fmt::Display for Defaultdist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "defaultdist = \"{}\"", self.0)
    }
}

/// Set the number of dimensions used for the layout. neato, fdp, sfdp only.
#[derive(Clone, Debug)]
struct Dim(pub(crate) String);

impl Dim {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_dim.in"))
    }
}

impl std::fmt::Display for Dim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dim = \"{}\"", self.0)
    }
}

/// Set the number of dimensions used for rendering. neato, fdp, sfdp only.
#[derive(Clone, Debug)]
struct Dimen(pub(crate) String);

impl Dimen {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_dimen.in"))
    }
}

impl std::fmt::Display for Dimen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dimen = \"{}\"", self.0)
    }
}

/// Edge type for drawing arrowheads.
#[derive(Clone, Debug)]
struct Dir(pub(crate) String);

impl Dir {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_dir.in"))
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dir = \"{}\"", self.0)
    }
}

/// Whether to constrain most edges to point downwards. neato only.
#[derive(Clone, Debug)]
struct Diredgeconstraints(pub(crate) String);

impl Diredgeconstraints {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_diredgeconstraints.in"))
    }
}

impl std::fmt::Display for Diredgeconstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "diredgeconstraints = \"{}\"", self.0)
    }
}

/// Distortion factor for shape=polygon.
#[derive(Clone, Debug)]
struct Distortion(pub(crate) String);

impl Distortion {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_distortion.in"))
    }
}

impl std::fmt::Display for Distortion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "distortion = \"{}\"", self.0)
    }
}

/// Specifies the expected number of pixels per inch on a display device. bitmap output, svg only.
#[derive(Clone, Debug)]
struct Dpi(pub(crate) String);

impl Dpi {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_dpi.in"))
    }
}

impl std::fmt::Display for Dpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dpi = \"{}\"", self.0)
    }
}

/// Synonym for edgeURL. map, svg only.
#[derive(Clone, Debug)]
struct Edgehref(pub(crate) String);

impl Edgehref {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_edgehref.in"))
    }
}

impl std::fmt::Display for Edgehref {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "edgehref = \"{}\"", self.0)
    }
}

/// Browser window to use for the edgeURL link. map, svg only.
#[derive(Clone, Debug)]
struct Edgetarget(pub(crate) String);

impl Edgetarget {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_edgetarget.in"))
    }
}

impl std::fmt::Display for Edgetarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "edgetarget = \"{}\"", self.0)
    }
}

/// Tooltip annotation attached to the non-label part of an edge. cmap, svg only.
#[derive(Clone, Debug)]
struct Edgetooltip(pub(crate) String);

impl Edgetooltip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_edgetooltip.in"))
    }
}

impl std::fmt::Display for Edgetooltip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "edgetooltip = \"{}\"", self.0)
    }
}

/// The link for the non-label parts of an edge. map, svg only.
#[derive(Clone, Debug)]
struct Edgeurl(pub(crate) String);

impl Edgeurl {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_edgeurl.in"))
    }
}

impl std::fmt::Display for Edgeurl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "edgeURL = \"{}\"", self.0)
    }
}

/// Terminating condition. neato only.
#[derive(Clone, Debug)]
struct Epsilon(pub(crate) String);

impl Epsilon {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_epsilon.in"))
    }
}

impl std::fmt::Display for Epsilon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "epsilon = \"{}\"", self.0)
    }
}

/// Margin used around polygons for purposes of spline edge routing. neato only.
#[derive(Clone, Debug)]
struct Esep(pub(crate) String);

impl Esep {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_esep.in"))
    }
}

impl std::fmt::Display for Esep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "esep = \"{}\"", self.0)
    }
}

/// Color used to fill the background of a node or cluster.
#[derive(Clone, Debug)]
struct Fillcolor(pub(crate) String);

impl Fillcolor {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fillcolor.in"))
    }
}

impl std::fmt::Display for Fillcolor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fillcolor = \"{}\"", self.0)
    }
}

/// Whether to use the specified width and height attributes to choose node size (rather than sizing to fit the node contents).
#[derive(Clone, Debug)]
struct Fixedsize(pub(crate) String);

impl Fixedsize {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fixedsize.in"))
    }
}

impl std::fmt::Display for Fixedsize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fixedsize = \"{}\"", self.0)
    }
}

/// Color used for text.
#[derive(Clone, Debug)]
struct Fontcolor(pub(crate) String);

impl Fontcolor {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fontcolor.in"))
    }
}

impl std::fmt::Display for Fontcolor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fontcolor = \"{}\"", self.0)
    }
}

/// Font used for text.
#[derive(Clone, Debug)]
struct Fontname(pub(crate) String);

impl Fontname {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fontname.in"))
    }
}

impl std::fmt::Display for Fontname {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fontname = \"{}\"", self.0)
    }
}

/// Allows user control of how basic fontnames are represented in SVG output. svg only.
#[derive(Clone, Debug)]
struct Fontnames(pub(crate) String);

impl Fontnames {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fontnames.in"))
    }
}

impl std::fmt::Display for Fontnames {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fontnames = \"{}\"", self.0)
    }
}

/// Directory list used by libgd to search for bitmap fonts.
#[derive(Clone, Debug)]
struct Fontpath(pub(crate) String);

impl Fontpath {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fontpath.in"))
    }
}

impl std::fmt::Display for Fontpath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fontpath = \"{}\"", self.0)
    }
}

/// Font size, in points, used for text.
#[derive(Clone, Debug)]
struct Fontsize(pub(crate) String);

impl Fontsize {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_fontsize.in"))
    }
}

impl std::fmt::Display for Fontsize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fontsize = \"{}\"", self.0)
    }
}

/// Whether to force placement of all xlabels, even if overlapping.
#[derive(Clone, Debug)]
struct Forcelabels(pub(crate) String);

impl Forcelabels {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_forcelabels.in"))
    }
}

impl std::fmt::Display for Forcelabels {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "forcelabels = \"{}\"", self.0)
    }
}

/// If a gradient fill is being used, this determines the angle of the fill.
#[derive(Clone, Debug)]
struct Gradientangle(pub(crate) String);

impl Gradientangle {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_gradientangle.in"))
    }
}

impl std::fmt::Display for Gradientangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "gradientangle = \"{}\"", self.0)
    }
}

/// Name for a group of nodes, for bundling edges avoiding crossings.. dot only.
#[derive(Clone, Debug)]
struct Group(pub(crate) String);

impl Group {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_group.in"))
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "group = \"{}\"", self.0)
    }
}

/// Center position of an edge's head label. write only.
#[derive(Clone, Debug)]
struct HeadLp(pub(crate) String);

impl HeadLp {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_head_lp.in"))
    }
}

impl std::fmt::Display for HeadLp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "head_lp = \"{}\"", self.0)
    }
}

/// If true, the head of an edge is clipped to the boundary of the head node.
#[derive(Clone, Debug)]
struct Headclip(pub(crate) String);

impl Headclip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headclip.in"))
    }
}

impl std::fmt::Display for Headclip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headclip = \"{}\"", self.0)
    }
}

/// Synonym for headURL. map, svg only.
#[derive(Clone, Debug)]
struct Headhref(pub(crate) String);

impl Headhref {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headhref.in"))
    }
}

impl std::fmt::Display for Headhref {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headhref = \"{}\"", self.0)
    }
}

/// Text label to be placed near head of edge.
#[derive(Clone, Debug)]
struct Headlabel(pub(crate) String);

impl Headlabel {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headlabel.in"))
    }
}

impl std::fmt::Display for Headlabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headlabel = \"{}\"", self.0)
    }
}

/// Indicates where on the head node to attach the head of the edge.
#[derive(Clone, Debug)]
struct Headport(pub(crate) String);

impl Headport {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headport.in"))
    }
}

impl std::fmt::Display for Headport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headport = \"{}\"", self.0)
    }
}

/// Browser window to use for the headURL link. map, svg only.
#[derive(Clone, Debug)]
struct Headtarget(pub(crate) String);

impl Headtarget {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headtarget.in"))
    }
}

impl std::fmt::Display for Headtarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headtarget = \"{}\"", self.0)
    }
}

/// Tooltip annotation attached to the head of an edge. cmap, svg only.
#[derive(Clone, Debug)]
struct Headtooltip(pub(crate) String);

impl Headtooltip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headtooltip.in"))
    }
}

impl std::fmt::Display for Headtooltip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headtooltip = \"{}\"", self.0)
    }
}

/// If defined, headURL is output as part of the head label of the edge. map, svg only.
#[derive(Clone, Debug)]
struct Headurl(pub(crate) String);

impl Headurl {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_headurl.in"))
    }
}

impl std::fmt::Display for Headurl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "headURL = \"{}\"", self.0)
    }
}

/// Height of node, in inches.
#[derive(Clone, Debug)]
struct Height(pub(crate) String);

impl Height {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_height.in"))
    }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "height = \"{}\"", self.0)
    }
}

/// Synonym for URL. map, postscript, svg only.
#[derive(Clone, Debug)]
struct Href(pub(crate) String);

impl Href {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_href.in"))
    }
}

impl std::fmt::Display for Href {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "href = \"{}\"", self.0)
    }
}

/// Identifier for graph objects. map, postscript, svg only.
#[derive(Clone, Debug)]
struct Id(pub(crate) String);

impl Id {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_id.in"))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id = \"{}\"", self.0)
    }
}

/// Gives the name of a file containing an image to be displayed inside a node.
#[derive(Clone, Debug)]
struct Image(pub(crate) String);

impl Image {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_image.in"))
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "image = \"{}\"", self.0)
    }
}

/// A list of directories in which to look for image files.
#[derive(Clone, Debug)]
struct Imagepath(pub(crate) String);

impl Imagepath {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_imagepath.in"))
    }
}

impl std::fmt::Display for Imagepath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "imagepath = \"{}\"", self.0)
    }
}

/// Controls how an image is positioned within its containing node.
#[derive(Clone, Debug)]
struct Imagepos(pub(crate) String);

impl Imagepos {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_imagepos.in"))
    }
}

impl std::fmt::Display for Imagepos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "imagepos = \"{}\"", self.0)
    }
}

/// Controls how an image fills its containing node.
#[derive(Clone, Debug)]
struct Imagescale(pub(crate) String);

impl Imagescale {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_imagescale.in"))
    }
}

impl std::fmt::Display for Imagescale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "imagescale = \"{}\"", self.0)
    }
}

/// Scales the input positions to convert between length units. neato, fdp only.
#[derive(Clone, Debug)]
struct Inputscale(pub(crate) String);

impl Inputscale {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_inputscale.in"))
    }
}

impl std::fmt::Display for Inputscale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "inputscale = \"{}\"", self.0)
    }
}

/// Spring constant used in virtual physical model. fdp, sfdp only.
#[derive(Clone, Debug)]
struct K(pub(crate) String);

impl K {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_k.in"))
    }
}

impl std::fmt::Display for K {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "K = \"{}\"", self.0)
    }
}

/// Text label attached to objects.
#[derive(Clone, Debug)]
struct Label(pub(crate) String);

impl Label {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_label.in"))
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "label = \"{}\"", self.0)
    }
}

/// Whether to treat a node whose name has the form |edgelabel|* as a special node representing an edge label.. sfdp only.
#[derive(Clone, Debug)]
struct LabelScheme(pub(crate) String);

impl LabelScheme {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_label_scheme.in"))
    }
}

impl std::fmt::Display for LabelScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "label_scheme = \"{}\"", self.0)
    }
}

/// The angle (in degrees) in polar coordinates of the head & tail edge labels..
#[derive(Clone, Debug)]
struct Labelangle(pub(crate) String);

impl Labelangle {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelangle.in"))
    }
}

impl std::fmt::Display for Labelangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelangle = \"{}\"", self.0)
    }
}

/// Scaling factor for the distance of headlabel / taillabel from the head / tail nodes..
#[derive(Clone, Debug)]
struct Labeldistance(pub(crate) String);

impl Labeldistance {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labeldistance.in"))
    }
}

impl std::fmt::Display for Labeldistance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labeldistance = \"{}\"", self.0)
    }
}

/// If true, allows edge labels to be less constrained in position.
#[derive(Clone, Debug)]
struct Labelfloat(pub(crate) String);

impl Labelfloat {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelfloat.in"))
    }
}

impl std::fmt::Display for Labelfloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelfloat = \"{}\"", self.0)
    }
}

/// Color used for headlabel and taillabel..
#[derive(Clone, Debug)]
struct Labelfontcolor(pub(crate) String);

impl Labelfontcolor {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelfontcolor.in"))
    }
}

impl std::fmt::Display for Labelfontcolor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelfontcolor = \"{}\"", self.0)
    }
}

/// Font for headlabel and taillabel.
#[derive(Clone, Debug)]
struct Labelfontname(pub(crate) String);

impl Labelfontname {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelfontname.in"))
    }
}

impl std::fmt::Display for Labelfontname {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelfontname = \"{}\"", self.0)
    }
}

/// Font size of headlabel and taillabel.
#[derive(Clone, Debug)]
struct Labelfontsize(pub(crate) String);

impl Labelfontsize {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelfontsize.in"))
    }
}

impl std::fmt::Display for Labelfontsize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelfontsize = \"{}\"", self.0)
    }
}

/// Synonym for labelURL. map, svg only.
#[derive(Clone, Debug)]
struct Labelhref(pub(crate) String);

impl Labelhref {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelhref.in"))
    }
}

impl std::fmt::Display for Labelhref {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelhref = \"{}\"", self.0)
    }
}

/// Justification for graph & cluster labels.
#[derive(Clone, Debug)]
struct Labeljust(pub(crate) String);

impl Labeljust {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labeljust.in"))
    }
}

impl std::fmt::Display for Labeljust {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labeljust = \"{}\"", self.0)
    }
}

/// Vertical placement of labels for nodes, root graphs and clusters.
#[derive(Clone, Debug)]
struct Labelloc(pub(crate) String);

impl Labelloc {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelloc.in"))
    }
}

impl std::fmt::Display for Labelloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelloc = \"{}\"", self.0)
    }
}

/// Browser window to open labelURL links in. map, svg only.
#[derive(Clone, Debug)]
struct Labeltarget(pub(crate) String);

impl Labeltarget {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labeltarget.in"))
    }
}

impl std::fmt::Display for Labeltarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labeltarget = \"{}\"", self.0)
    }
}

/// Tooltip annotation attached to label of an edge. cmap, svg only.
#[derive(Clone, Debug)]
struct Labeltooltip(pub(crate) String);

impl Labeltooltip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labeltooltip.in"))
    }
}

impl std::fmt::Display for Labeltooltip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labeltooltip = \"{}\"", self.0)
    }
}

/// If defined, labelURL is the link used for the label of an edge. map, svg only.
#[derive(Clone, Debug)]
struct Labelurl(pub(crate) String);

impl Labelurl {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_labelurl.in"))
    }
}

impl std::fmt::Display for Labelurl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "labelURL = \"{}\"", self.0)
    }
}

/// If true, the graph is rendered in landscape mode.
#[derive(Clone, Debug)]
struct Landscape(pub(crate) String);

impl Landscape {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_landscape.in"))
    }
}

impl std::fmt::Display for Landscape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "landscape = \"{}\"", self.0)
    }
}

/// Specifies layers in which the node, edge or cluster is present.
#[derive(Clone, Debug)]
struct Layer(pub(crate) String);

impl Layer {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_layer.in"))
    }
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "layer = \"{}\"", self.0)
    }
}

/// The separator characters used to split attributes of type layerRange into a list of ranges..
#[derive(Clone, Debug)]
struct Layerlistsep(pub(crate) String);

impl Layerlistsep {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_layerlistsep.in"))
    }
}

impl std::fmt::Display for Layerlistsep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "layerlistsep = \"{}\"", self.0)
    }
}

/// A linearly ordered list of layer names attached to the graph.
#[derive(Clone, Debug)]
struct Layers(pub(crate) String);

impl Layers {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_layers.in"))
    }
}

impl std::fmt::Display for Layers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "layers = \"{}\"", self.0)
    }
}

/// Selects a list of layers to be emitted.
#[derive(Clone, Debug)]
struct Layerselect(pub(crate) String);

impl Layerselect {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_layerselect.in"))
    }
}

impl std::fmt::Display for Layerselect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "layerselect = \"{}\"", self.0)
    }
}

/// The separator characters for splitting the layers attribute into a list of layer names..
#[derive(Clone, Debug)]
struct Layersep(pub(crate) String);

impl Layersep {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_layersep.in"))
    }
}

impl std::fmt::Display for Layersep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "layersep = \"{}\"", self.0)
    }
}

/// Which layout engine to use.
#[derive(Clone, Debug)]
struct Layout(pub(crate) String);

impl Layout {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_layout.in"))
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "layout = \"{}\"", self.0)
    }
}

/// Preferred edge length, in inches. neato, fdp only.
#[derive(Clone, Debug)]
struct Len(pub(crate) String);

impl Len {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_len.in"))
    }
}

impl std::fmt::Display for Len {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "len = \"{}\"", self.0)
    }
}

/// Number of levels allowed in the multilevel scheme. sfdp only.
#[derive(Clone, Debug)]
struct Levels(pub(crate) String);

impl Levels {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_levels.in"))
    }
}

impl std::fmt::Display for Levels {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "levels = \"{}\"", self.0)
    }
}

/// strictness of neato level constraints. neato only.
#[derive(Clone, Debug)]
struct Levelsgap(pub(crate) String);

impl Levelsgap {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_levelsgap.in"))
    }
}

impl std::fmt::Display for Levelsgap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "levelsgap = \"{}\"", self.0)
    }
}

/// Logical head of an edge. dot only.
#[derive(Clone, Debug)]
struct Lhead(pub(crate) String);

impl Lhead {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_lhead.in"))
    }
}

impl std::fmt::Display for Lhead {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lhead = \"{}\"", self.0)
    }
}

/// Height of graph or cluster label, in inches. write only.
#[derive(Clone, Debug)]
struct Lheight(pub(crate) String);

impl Lheight {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_lheight.in"))
    }
}

impl std::fmt::Display for Lheight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lheight = \"{}\"", self.0)
    }
}

/// How long strings should get before overflowing to next line, for text output..
#[derive(Clone, Debug)]
struct Linelength(pub(crate) String);

impl Linelength {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_linelength.in"))
    }
}

impl std::fmt::Display for Linelength {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "linelength = \"{}\"", self.0)
    }
}

/// Label center position. write only.
#[derive(Clone, Debug)]
struct Lp(pub(crate) String);

impl Lp {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_lp.in"))
    }
}

impl std::fmt::Display for Lp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lp = \"{}\"", self.0)
    }
}

/// Logical tail of an edge. dot only.
#[derive(Clone, Debug)]
struct Ltail(pub(crate) String);

impl Ltail {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_ltail.in"))
    }
}

impl std::fmt::Display for Ltail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ltail = \"{}\"", self.0)
    }
}

/// Width of graph or cluster label, in inches. write only.
#[derive(Clone, Debug)]
struct Lwidth(pub(crate) String);

impl Lwidth {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_lwidth.in"))
    }
}

impl std::fmt::Display for Lwidth {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lwidth = \"{}\"", self.0)
    }
}

/// For graphs, this sets x and y margins of canvas, in inches.
#[derive(Clone, Debug)]
struct Margin(pub(crate) String);

impl Margin {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_margin.in"))
    }
}

impl std::fmt::Display for Margin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "margin = \"{}\"", self.0)
    }
}

/// Sets the number of iterations used. neato, fdp only.
#[derive(Clone, Debug)]
struct Maxiter(pub(crate) String);

impl Maxiter {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_maxiter.in"))
    }
}

impl std::fmt::Display for Maxiter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "maxiter = \"{}\"", self.0)
    }
}

/// Scale factor for mincross (mc) edge crossing minimiser parameters. dot only.
#[derive(Clone, Debug)]
struct Mclimit(pub(crate) String);

impl Mclimit {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_mclimit.in"))
    }
}

impl std::fmt::Display for Mclimit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mclimit = \"{}\"", self.0)
    }
}

/// Specifies the minimum separation between all nodes. circo only.
#[derive(Clone, Debug)]
struct Mindist(pub(crate) String);

impl Mindist {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_mindist.in"))
    }
}

impl std::fmt::Display for Mindist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mindist = \"{}\"", self.0)
    }
}

/// Minimum edge length (rank difference between head and tail). dot only.
#[derive(Clone, Debug)]
struct Minlen(pub(crate) String);

impl Minlen {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_minlen.in"))
    }
}

impl std::fmt::Display for Minlen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "minlen = \"{}\"", self.0)
    }
}

/// Technique for optimizing the layout. neato only.
#[derive(Clone, Debug)]
struct Mode(pub(crate) String);

impl Mode {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_mode.in"))
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mode = \"{}\"", self.0)
    }
}

/// Specifies how the distance matrix is computed for the input graph. neato only.
#[derive(Clone, Debug)]
struct Model(pub(crate) String);

impl Model {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_model.in"))
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "model = \"{}\"", self.0)
    }
}

/// Whether to use a single global ranking, ignoring clusters. dot only.
#[derive(Clone, Debug)]
struct Newrank(pub(crate) String);

impl Newrank {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_newrank.in"))
    }
}

impl std::fmt::Display for Newrank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "newrank = \"{}\"", self.0)
    }
}

/// In dot, nodesep specifies the minimum space between two adjacent nodes in the same rank, in inches.
#[derive(Clone, Debug)]
struct Nodesep(pub(crate) String);

impl Nodesep {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_nodesep.in"))
    }
}

impl std::fmt::Display for Nodesep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "nodesep = \"{}\"", self.0)
    }
}

/// Whether to justify multiline text vs the previous text line (rather than the side of the container)..
#[derive(Clone, Debug)]
struct Nojustify(pub(crate) String);

impl Nojustify {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_nojustify.in"))
    }
}

impl std::fmt::Display for Nojustify {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "nojustify = \"{}\"", self.0)
    }
}

/// normalizes coordinates of final layout. neato, fdp, sfdp, twopi, circo only.
#[derive(Clone, Debug)]
struct Normalize(pub(crate) String);

impl Normalize {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_normalize.in"))
    }
}

impl std::fmt::Display for Normalize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "normalize = \"{}\"", self.0)
    }
}

/// Whether to avoid translating layout to the origin point. neato only.
#[derive(Clone, Debug)]
struct Notranslate(pub(crate) String);

impl Notranslate {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_notranslate.in"))
    }
}

impl std::fmt::Display for Notranslate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "notranslate = \"{}\"", self.0)
    }
}

/// Sets number of iterations in network simplex applications. dot only.
#[derive(Clone, Debug)]
struct Nslimit(pub(crate) String);

impl Nslimit {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_nslimit.in"))
    }
}

impl std::fmt::Display for Nslimit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "nslimit = \"{}\"", self.0)
    }
}

/// Sets number of iterations in network simplex applications. dot only.
#[derive(Clone, Debug)]
struct Nslimit1(pub(crate) String);

impl Nslimit1 {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_nslimit1.in"))
    }
}

impl std::fmt::Display for Nslimit1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "nslimit1 = \"{}\"", self.0)
    }
}

/// Whether to draw circo graphs around one circle.. circo only.
#[derive(Clone, Debug)]
struct Oneblock(pub(crate) String);

impl Oneblock {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_oneblock.in"))
    }
}

impl std::fmt::Display for Oneblock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "oneblock = \"{}\"", self.0)
    }
}

/// Constrains the left-to-right ordering of node edges.. dot only.
#[derive(Clone, Debug)]
struct Ordering(pub(crate) String);

impl Ordering {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_ordering.in"))
    }
}

impl std::fmt::Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ordering = \"{}\"", self.0)
    }
}

/// node shape rotation angle, or graph orientation.
#[derive(Clone, Debug)]
struct Orientation(pub(crate) String);

impl Orientation {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_orientation.in"))
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "orientation = \"{}\"", self.0)
    }
}

/// Specify order in which nodes and edges are drawn.
#[derive(Clone, Debug)]
struct Outputorder(pub(crate) String);

impl Outputorder {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_outputorder.in"))
    }
}

impl std::fmt::Display for Outputorder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "outputorder = \"{}\"", self.0)
    }
}

/// Determines if and how node overlaps should be removed. fdp, neato only.
#[derive(Clone, Debug)]
struct Overlap(pub(crate) String);

impl Overlap {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_overlap.in"))
    }
}

impl std::fmt::Display for Overlap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "overlap = \"{}\"", self.0)
    }
}

/// Scale layout by factor, to reduce node overlap.. prism, neato, sfdp, fdp, circo, twopi only.
#[derive(Clone, Debug)]
struct OverlapScaling(pub(crate) String);

impl OverlapScaling {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_overlap_scaling.in"))
    }
}

impl std::fmt::Display for OverlapScaling {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "overlap_scaling = \"{}\"", self.0)
    }
}

/// Whether the overlap removal algorithm should perform a compression pass to reduce the size of the layout. prism only.
#[derive(Clone, Debug)]
struct OverlapShrink(pub(crate) String);

impl OverlapShrink {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_overlap_shrink.in"))
    }
}

impl std::fmt::Display for OverlapShrink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "overlap_shrink = \"{}\"", self.0)
    }
}

/// Whether each connected component of the graph should be laid out separately, and then the graphs packed together..
#[derive(Clone, Debug)]
struct Pack(pub(crate) String);

impl Pack {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_pack.in"))
    }
}

impl std::fmt::Display for Pack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pack = \"{}\"", self.0)
    }
}

/// How connected components should be packed.
#[derive(Clone, Debug)]
struct Packmode(pub(crate) String);

impl Packmode {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_packmode.in"))
    }
}

impl std::fmt::Display for Packmode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "packmode = \"{}\"", self.0)
    }
}

/// Inches to extend the drawing area around the minimal area needed to draw the graph.
#[derive(Clone, Debug)]
struct Pad(pub(crate) String);

impl Pad {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_pad.in"))
    }
}

impl std::fmt::Display for Pad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pad = \"{}\"", self.0)
    }
}

/// Width and height of output pages, in inches.
#[derive(Clone, Debug)]
struct Page(pub(crate) String);

impl Page {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_page.in"))
    }
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "page = \"{}\"", self.0)
    }
}

/// The order in which pages are emitted.
#[derive(Clone, Debug)]
struct Pagedir(pub(crate) String);

impl Pagedir {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_pagedir.in"))
    }
}

impl std::fmt::Display for Pagedir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pagedir = \"{}\"", self.0)
    }
}

/// Color used to draw the bounding box around a cluster.
#[derive(Clone, Debug)]
struct Pencolor(pub(crate) String);

impl Pencolor {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_pencolor.in"))
    }
}

impl std::fmt::Display for Pencolor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pencolor = \"{}\"", self.0)
    }
}

/// Specifies the width of the pen, in points, used to draw lines and curves.
#[derive(Clone, Debug)]
struct Penwidth(pub(crate) String);

impl Penwidth {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_penwidth.in"))
    }
}

impl std::fmt::Display for Penwidth {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "penwidth = \"{}\"", self.0)
    }
}

/// Set number of peripheries used in polygonal shapes and cluster boundaries.
#[derive(Clone, Debug)]
struct Peripheries(pub(crate) String);

impl Peripheries {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_peripheries.in"))
    }
}

impl std::fmt::Display for Peripheries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "peripheries = \"{}\"", self.0)
    }
}

/// Keeps the node at the node's given input position. neato, fdp only.
#[derive(Clone, Debug)]
struct Pin(pub(crate) String);

impl Pin {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_pin.in"))
    }
}

impl std::fmt::Display for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pin = \"{}\"", self.0)
    }
}

/// Position of node, or spline control points. neato, fdp only.
#[derive(Clone, Debug)]
struct Pos(pub(crate) String);

impl Pos {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_pos.in"))
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos = \"{}\"", self.0)
    }
}

/// Quadtree scheme to use. sfdp only.
#[derive(Clone, Debug)]
struct Quadtree(pub(crate) String);

impl Quadtree {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_quadtree.in"))
    }
}

impl std::fmt::Display for Quadtree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "quadtree = \"{}\"", self.0)
    }
}

/// If quantum > 0.0, node label dimensions will be rounded to integral multiples of the quantum.
#[derive(Clone, Debug)]
struct Quantum(pub(crate) String);

impl Quantum {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_quantum.in"))
    }
}

impl std::fmt::Display for Quantum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "quantum = \"{}\"", self.0)
    }
}

/// Rank constraints on the nodes in a subgraph. dot only.
#[derive(Clone, Debug)]
struct Rank(pub(crate) String);

impl Rank {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_rank.in"))
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rank = \"{}\"", self.0)
    }
}

/// Sets direction of graph layout. dot only.
#[derive(Clone, Debug)]
struct Rankdir(pub(crate) String);

impl Rankdir {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_rankdir.in"))
    }
}

impl std::fmt::Display for Rankdir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rankdir = \"{}\"", self.0)
    }
}

/// Specifies separation between ranks. dot, twopi only.
#[derive(Clone, Debug)]
struct Ranksep(pub(crate) String);

impl Ranksep {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_ranksep.in"))
    }
}

impl std::fmt::Display for Ranksep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ranksep = \"{}\"", self.0)
    }
}

/// Sets the aspect ratio (drawing height/drawing width) for the drawing.
#[derive(Clone, Debug)]
struct Ratio(pub(crate) String);

impl Ratio {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_ratio.in"))
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ratio = \"{}\"", self.0)
    }
}

/// Rectangles for fields of records, in points. write only.
#[derive(Clone, Debug)]
struct Rects(pub(crate) String);

impl Rects {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_rects.in"))
    }
}

impl std::fmt::Display for Rects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rects = \"{}\"", self.0)
    }
}

/// If true, force polygon to be regular, i.e., the vertices of th.
#[derive(Clone, Debug)]
struct Regular(pub(crate) String);

impl Regular {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_regular.in"))
    }
}

impl std::fmt::Display for Regular {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "regular = \"{}\"", self.0)
    }
}

/// If there are multiple clusters, whether to run edge crossing minimization a second time.. dot only.
#[derive(Clone, Debug)]
struct Remincross(pub(crate) String);

impl Remincross {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_remincross.in"))
    }
}

impl std::fmt::Display for Remincross {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "remincross = \"{}\"", self.0)
    }
}

/// The power of the repulsive force used in an extended Fruchterman-Reingold. sfdp only.
#[derive(Clone, Debug)]
struct Repulsiveforce(pub(crate) String);

impl Repulsiveforce {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_repulsiveforce.in"))
    }
}

impl std::fmt::Display for Repulsiveforce {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "repulsiveforce = \"{}\"", self.0)
    }
}

/// Synonym for dpi.. bitmap output, svg only.
#[derive(Clone, Debug)]
struct Resolution(pub(crate) String);

impl Resolution {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_resolution.in"))
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "resolution = \"{}\"", self.0)
    }
}

/// Specifies nodes to be used as the center of the layout. twopi, circo only.
#[derive(Clone, Debug)]
struct Root(pub(crate) String);

impl Root {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_root.in"))
    }
}

impl std::fmt::Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "root = \"{}\"", self.0)
    }
}

/// If rotate=90, sets drawing orientation to landscape.
#[derive(Clone, Debug)]
struct Rotate(pub(crate) String);

impl Rotate {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_rotate.in"))
    }
}

impl std::fmt::Display for Rotate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rotate = \"{}\"", self.0)
    }
}

/// Rotates the final layout counter-clockwise by the specified number of degrees. sfdp only.
#[derive(Clone, Debug)]
struct Rotation(pub(crate) String);

impl Rotation {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_rotation.in"))
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rotation = \"{}\"", self.0)
    }
}

/// Edges with the same head and the same samehead value are aimed at the same point on the head. dot only.
#[derive(Clone, Debug)]
struct Samehead(pub(crate) String);

impl Samehead {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_samehead.in"))
    }
}

impl std::fmt::Display for Samehead {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "samehead = \"{}\"", self.0)
    }
}

/// Edges with the same tail and the same sametail value are aimed at th. dot only.
#[derive(Clone, Debug)]
struct Sametail(pub(crate) String);

impl Sametail {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_sametail.in"))
    }
}

impl std::fmt::Display for Sametail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sametail = \"{}\"", self.0)
    }
}

/// Gives the number of points used for a circle/ellipse node.
#[derive(Clone, Debug)]
struct Samplepoints(pub(crate) String);

impl Samplepoints {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_samplepoints.in"))
    }
}

impl std::fmt::Display for Samplepoints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "samplepoints = \"{}\"", self.0)
    }
}

/// Scales layout by the given factor after the initial layout. neato, twopi only.
#[derive(Clone, Debug)]
struct Scale(pub(crate) String);

impl Scale {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_scale.in"))
    }
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "scale = \"{}\"", self.0)
    }
}

/// During network simplex, the maximum number of edges with negative cut values to search when looking for an edge with minimum cut value.. dot only.
#[derive(Clone, Debug)]
struct Searchsize(pub(crate) String);

impl Searchsize {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_searchsize.in"))
    }
}

impl std::fmt::Display for Searchsize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "searchsize = \"{}\"", self.0)
    }
}

/// Margin to leave around nodes when removing node overlap. fdp, neato only.
#[derive(Clone, Debug)]
struct Sep(pub(crate) String);

impl Sep {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_sep.in"))
    }
}

impl std::fmt::Display for Sep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sep = \"{}\"", self.0)
    }
}

/// Sets the shape of a node.
#[derive(Clone, Debug)]
struct Shape(pub(crate) String);

impl Shape {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_shape.in"))
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "shape = \"{}\"", self.0)
    }
}

/// A file containing user-supplied node content.
#[derive(Clone, Debug)]
struct Shapefile(pub(crate) String);

impl Shapefile {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_shapefile.in"))
    }
}

impl std::fmt::Display for Shapefile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "shapefile = \"{}\"", self.0)
    }
}

/// Print guide boxes for debugging. dot only.
#[derive(Clone, Debug)]
struct Showboxes(pub(crate) String);

impl Showboxes {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_showboxes.in"))
    }
}

impl std::fmt::Display for Showboxes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "showboxes = \"{}\"", self.0)
    }
}

/// Number of sides when shape=polygon.
#[derive(Clone, Debug)]
struct Sides(pub(crate) String);

impl Sides {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_sides.in"))
    }
}

impl std::fmt::Display for Sides {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sides = \"{}\"", self.0)
    }
}

/// Maximum width and height of drawing, in inches.
#[derive(Clone, Debug)]
struct Size(pub(crate) String);

impl Size {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_size.in"))
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "size = \"{}\"", self.0)
    }
}

/// Skew factor for shape=polygon.
#[derive(Clone, Debug)]
struct Skew(pub(crate) String);

impl Skew {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_skew.in"))
    }
}

impl std::fmt::Display for Skew {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "skew = \"{}\"", self.0)
    }
}

/// Specifies a post-processing step used to smooth out an uneven distribution of nodes.. sfdp only.
#[derive(Clone, Debug)]
struct Smoothing(pub(crate) String);

impl Smoothing {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_smoothing.in"))
    }
}

impl std::fmt::Display for Smoothing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "smoothing = \"{}\"", self.0)
    }
}

/// Sort order of graph components for ordering packmode packing..
#[derive(Clone, Debug)]
struct Sortv(pub(crate) String);

impl Sortv {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_sortv.in"))
    }
}

impl std::fmt::Display for Sortv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sortv = \"{}\"", self.0)
    }
}

/// Controls how, and if, edges are represented.
#[derive(Clone, Debug)]
struct Splines(pub(crate) String);

impl Splines {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_splines.in"))
    }
}

impl std::fmt::Display for Splines {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "splines = \"{}\"", self.0)
    }
}

/// Parameter used to determine the initial layout of nodes. neato, fdp, sfdp only.
#[derive(Clone, Debug)]
struct Start(pub(crate) String);

impl Start {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_start.in"))
    }
}

impl std::fmt::Display for Start {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start = \"{}\"", self.0)
    }
}

/// Set style information for components of the graph.
#[derive(Clone, Debug)]
struct Style(pub(crate) String);

impl Style {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_style.in"))
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "style = \"{}\"", self.0)
    }
}

/// A URL or pathname specifying an XML style sheet, used in SVG output. svg only.
#[derive(Clone, Debug)]
struct Stylesheet(pub(crate) String);

impl Stylesheet {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_stylesheet.in"))
    }
}

impl std::fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "stylesheet = \"{}\"", self.0)
    }
}

/// Position of an edge's tail label, in points.. write only.
#[derive(Clone, Debug)]
struct TailLp(pub(crate) String);

impl TailLp {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tail_lp.in"))
    }
}

impl std::fmt::Display for TailLp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tail_lp = \"{}\"", self.0)
    }
}

/// If true, the tail of an edge is clipped to the boundary of the tail node.
#[derive(Clone, Debug)]
struct Tailclip(pub(crate) String);

impl Tailclip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tailclip.in"))
    }
}

impl std::fmt::Display for Tailclip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tailclip = \"{}\"", self.0)
    }
}

/// Synonym for tailURL.. map, svg only.
#[derive(Clone, Debug)]
struct Tailhref(pub(crate) String);

impl Tailhref {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tailhref.in"))
    }
}

impl std::fmt::Display for Tailhref {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tailhref = \"{}\"", self.0)
    }
}

/// Text label to be placed near tail of edge.
#[derive(Clone, Debug)]
struct Taillabel(pub(crate) String);

impl Taillabel {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_taillabel.in"))
    }
}

impl std::fmt::Display for Taillabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "taillabel = \"{}\"", self.0)
    }
}

/// Indicates where on the tail node to attach the tail of the edge.
#[derive(Clone, Debug)]
struct Tailport(pub(crate) String);

impl Tailport {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tailport.in"))
    }
}

impl std::fmt::Display for Tailport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tailport = \"{}\"", self.0)
    }
}

/// Browser window to use for the tailURL link. map, svg only.
#[derive(Clone, Debug)]
struct Tailtarget(pub(crate) String);

impl Tailtarget {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tailtarget.in"))
    }
}

impl std::fmt::Display for Tailtarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tailtarget = \"{}\"", self.0)
    }
}

/// Tooltip annotation attached to the tail of an edge. cmap, svg only.
#[derive(Clone, Debug)]
struct Tailtooltip(pub(crate) String);

impl Tailtooltip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tailtooltip.in"))
    }
}

impl std::fmt::Display for Tailtooltip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tailtooltip = \"{}\"", self.0)
    }
}

/// If defined, tailURL is output as part of the tail label of th. map, svg only.
#[derive(Clone, Debug)]
struct Tailurl(pub(crate) String);

impl Tailurl {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tailurl.in"))
    }
}

impl std::fmt::Display for Tailurl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tailURL = \"{}\"", self.0)
    }
}

/// If the object has a URL, this attribute determines which window of the browser is used for the URL.. map, svg only.
#[derive(Clone, Debug)]
struct Target(pub(crate) String);

impl Target {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_target.in"))
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "target = \"{}\"", self.0)
    }
}

/// Which rank to move floating (loose) nodes to. dot only.
#[derive(Clone, Debug)]
struct Tbbalance(pub(crate) String);

impl Tbbalance {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tbbalance.in"))
    }
}

impl std::fmt::Display for Tbbalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TBbalance = \"{}\"", self.0)
    }
}

/// Tooltip (mouse hover text) attached to the node, edge, cluster, or graph. cmap, svg only.
#[derive(Clone, Debug)]
struct Tooltip(pub(crate) String);

impl Tooltip {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_tooltip.in"))
    }
}

impl std::fmt::Display for Tooltip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tooltip = \"{}\"", self.0)
    }
}

/// Whether internal bitmap rendering relies on a truecolor color model or uses. bitmap output only.
#[derive(Clone, Debug)]
struct Truecolor(pub(crate) String);

impl Truecolor {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_truecolor.in"))
    }
}

impl std::fmt::Display for Truecolor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "truecolor = \"{}\"", self.0)
    }
}

/// Hyperlinks incorporated into device-dependent output. map, postscript, svg only.
#[derive(Clone, Debug)]
struct Url(pub(crate) String);

impl Url {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_url.in"))
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "URL = \"{}\"", self.0)
    }
}

/// Sets the coordinates of the vertices of the node's polygon, in inches. write only.
#[derive(Clone, Debug)]
struct Vertices(pub(crate) String);

impl Vertices {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_vertices.in"))
    }
}

impl std::fmt::Display for Vertices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "vertices = \"{}\"", self.0)
    }
}

/// Clipping window on final drawing.
#[derive(Clone, Debug)]
struct Viewport(pub(crate) String);

impl Viewport {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_viewport.in"))
    }
}

impl std::fmt::Display for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "viewport = \"{}\"", self.0)
    }
}

/// Tuning margin of Voronoi technique. neato, fdp, sfdp, twopi, circo only.
#[derive(Clone, Debug)]
struct VoroMargin(pub(crate) String);

impl VoroMargin {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_voro_margin.in"))
    }
}

impl std::fmt::Display for VoroMargin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "voro_margin = \"{}\"", self.0)
    }
}

/// Weight of edge.
#[derive(Clone, Debug)]
struct Weight(pub(crate) String);

impl Weight {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_weight.in"))
    }
}

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "weight = \"{}\"", self.0)
    }
}

/// Width of node, in inches.
#[derive(Clone, Debug)]
struct Width(pub(crate) String);

impl Width {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_width.in"))
    }
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "width = \"{}\"", self.0)
    }
}

/// Determines the version of xdot used in output. xdot only.
#[derive(Clone, Debug)]
struct Xdotversion(pub(crate) String);

impl Xdotversion {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_xdotversion.in"))
    }
}

impl std::fmt::Display for Xdotversion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "xdotversion = \"{}\"", self.0)
    }
}

/// External label for a node or edge.
#[derive(Clone, Debug)]
struct Xlabel(pub(crate) String);

impl Xlabel {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_xlabel.in"))
    }
}

impl std::fmt::Display for Xlabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "xlabel = \"{}\"", self.0)
    }
}

/// Position of an exterior label, in points. write only.
#[derive(Clone, Debug)]
struct Xlp(pub(crate) String);

impl Xlp {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_xlp.in"))
    }
}

impl std::fmt::Display for Xlp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "xlp = \"{}\"", self.0)
    }
}

/// Z-coordinate value for 3D layouts and displays.
#[derive(Clone, Debug)]
struct Z(pub(crate) String);

impl Z {
    pub fn new(s: &str) -> Self {
        Self(include!("./validate_z.in"))
    }
}

impl std::fmt::Display for Z {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "z = \"{}\"", self.0)
    }
}

/// Graph attributes.
#[derive(Clone, Debug, Default)]
pub struct GraphAttrs {
    _background: Option<Background>,
    bb: Option<Bb>,
    beautify: Option<Beautify>,
    bgcolor: Option<Bgcolor>,
    center: Option<Center>,
    charset: Option<Charset>,
    class: Option<Class>,
    clusterrank: Option<Clusterrank>,
    colorscheme: Option<Colorscheme>,
    comment: Option<Comment>,
    compound: Option<Compound>,
    concentrate: Option<Concentrate>,
    damping: Option<Damping>,
    defaultdist: Option<Defaultdist>,
    dim: Option<Dim>,
    dimen: Option<Dimen>,
    diredgeconstraints: Option<Diredgeconstraints>,
    dpi: Option<Dpi>,
    epsilon: Option<Epsilon>,
    esep: Option<Esep>,
    fontcolor: Option<Fontcolor>,
    fontname: Option<Fontname>,
    fontnames: Option<Fontnames>,
    fontpath: Option<Fontpath>,
    fontsize: Option<Fontsize>,
    forcelabels: Option<Forcelabels>,
    gradientangle: Option<Gradientangle>,
    href: Option<Href>,
    id: Option<Id>,
    imagepath: Option<Imagepath>,
    inputscale: Option<Inputscale>,
    k: Option<K>,
    label: Option<Label>,
    label_scheme: Option<LabelScheme>,
    labeljust: Option<Labeljust>,
    labelloc: Option<Labelloc>,
    landscape: Option<Landscape>,
    layerlistsep: Option<Layerlistsep>,
    layers: Option<Layers>,
    layerselect: Option<Layerselect>,
    layersep: Option<Layersep>,
    layout: Option<Layout>,
    levels: Option<Levels>,
    levelsgap: Option<Levelsgap>,
    lheight: Option<Lheight>,
    linelength: Option<Linelength>,
    lp: Option<Lp>,
    lwidth: Option<Lwidth>,
    margin: Option<Margin>,
    maxiter: Option<Maxiter>,
    mclimit: Option<Mclimit>,
    mindist: Option<Mindist>,
    mode: Option<Mode>,
    model: Option<Model>,
    newrank: Option<Newrank>,
    nodesep: Option<Nodesep>,
    nojustify: Option<Nojustify>,
    normalize: Option<Normalize>,
    notranslate: Option<Notranslate>,
    nslimit: Option<Nslimit>,
    nslimit1: Option<Nslimit1>,
    oneblock: Option<Oneblock>,
    ordering: Option<Ordering>,
    orientation: Option<Orientation>,
    outputorder: Option<Outputorder>,
    overlap: Option<Overlap>,
    overlap_scaling: Option<OverlapScaling>,
    overlap_shrink: Option<OverlapShrink>,
    pack: Option<Pack>,
    packmode: Option<Packmode>,
    pad: Option<Pad>,
    page: Option<Page>,
    pagedir: Option<Pagedir>,
    quadtree: Option<Quadtree>,
    quantum: Option<Quantum>,
    rankdir: Option<Rankdir>,
    ranksep: Option<Ranksep>,
    ratio: Option<Ratio>,
    remincross: Option<Remincross>,
    repulsiveforce: Option<Repulsiveforce>,
    resolution: Option<Resolution>,
    root: Option<Root>,
    rotate: Option<Rotate>,
    rotation: Option<Rotation>,
    scale: Option<Scale>,
    searchsize: Option<Searchsize>,
    sep: Option<Sep>,
    showboxes: Option<Showboxes>,
    size: Option<Size>,
    smoothing: Option<Smoothing>,
    sortv: Option<Sortv>,
    splines: Option<Splines>,
    start: Option<Start>,
    style: Option<Style>,
    stylesheet: Option<Stylesheet>,
    target: Option<Target>,
    tbbalance: Option<Tbbalance>,
    tooltip: Option<Tooltip>,
    truecolor: Option<Truecolor>,
    url: Option<Url>,
    viewport: Option<Viewport>,
    voro_margin: Option<VoroMargin>,
    xdotversion: Option<Xdotversion>,
}

impl GraphAttrs {
    /// A string in the xdot format specifying an arbitrary background. [Read more](https://graphviz.org/docs/attrs/background/).
    pub fn get_background(&self) -> Option<&str> {
        self._background.as_ref().map(|x| x.0.as_str())
    }

    /// Set `_background` attribute. [Read more](https://graphviz.org/docs/attrs/background/).
    pub fn set_background(&mut self, s: &str) {
        self._background = Some(Background::new(s));
    }

    /// Unset `_background` attribute. [Read more](https://graphviz.org/docs/attrs/background/).
    pub fn unset_background(&mut self) {
        self._background = None;
    }

    /// Bounding box of drawing in points. write only. [Read more](https://graphviz.org/docs/attrs/bb/).
    pub fn get_bb(&self) -> Option<&str> {
        self.bb.as_ref().map(|x| x.0.as_str())
    }

    /// Set `bb` attribute. [Read more](https://graphviz.org/docs/attrs/bb/).
    pub fn set_bb(&mut self, s: &str) {
        self.bb = Some(Bb::new(s));
    }

    /// Unset `bb` attribute. [Read more](https://graphviz.org/docs/attrs/bb/).
    pub fn unset_bb(&mut self) {
        self.bb = None;
    }

    /// Whether to draw leaf nodes uniformly in a circle around the root node in sfdp.. sfdp only. [Read more](https://graphviz.org/docs/attrs/beautify/).
    pub fn get_beautify(&self) -> Option<&str> {
        self.beautify.as_ref().map(|x| x.0.as_str())
    }

    /// Set `beautify` attribute. [Read more](https://graphviz.org/docs/attrs/beautify/).
    pub fn set_beautify(&mut self, s: &str) {
        self.beautify = Some(Beautify::new(s));
    }

    /// Unset `beautify` attribute. [Read more](https://graphviz.org/docs/attrs/beautify/).
    pub fn unset_beautify(&mut self) {
        self.beautify = None;
    }

    /// Canvas background color. [Read more](https://graphviz.org/docs/attrs/bgcolor/).
    pub fn get_bgcolor(&self) -> Option<&str> {
        self.bgcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `bgcolor` attribute. [Read more](https://graphviz.org/docs/attrs/bgcolor/).
    pub fn set_bgcolor(&mut self, s: &str) {
        self.bgcolor = Some(Bgcolor::new(s));
    }

    /// Unset `bgcolor` attribute. [Read more](https://graphviz.org/docs/attrs/bgcolor/).
    pub fn unset_bgcolor(&mut self) {
        self.bgcolor = None;
    }

    /// Whether to center the drawing in the output canvas. [Read more](https://graphviz.org/docs/attrs/center/).
    pub fn get_center(&self) -> Option<&str> {
        self.center.as_ref().map(|x| x.0.as_str())
    }

    /// Set `center` attribute. [Read more](https://graphviz.org/docs/attrs/center/).
    pub fn set_center(&mut self, s: &str) {
        self.center = Some(Center::new(s));
    }

    /// Unset `center` attribute. [Read more](https://graphviz.org/docs/attrs/center/).
    pub fn unset_center(&mut self) {
        self.center = None;
    }

    /// Character encoding used when interpreting string input as a text label.. [Read more](https://graphviz.org/docs/attrs/charset/).
    pub fn get_charset(&self) -> Option<&str> {
        self.charset.as_ref().map(|x| x.0.as_str())
    }

    /// Set `charset` attribute. [Read more](https://graphviz.org/docs/attrs/charset/).
    pub fn set_charset(&mut self, s: &str) {
        self.charset = Some(Charset::new(s));
    }

    /// Unset `charset` attribute. [Read more](https://graphviz.org/docs/attrs/charset/).
    pub fn unset_charset(&mut self) {
        self.charset = None;
    }

    /// Classnames to attach to the node, edge, graph, or cluster's SVG element. svg only. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn get_class(&self) -> Option<&str> {
        self.class.as_ref().map(|x| x.0.as_str())
    }

    /// Set `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn set_class(&mut self, s: &str) {
        self.class = Some(Class::new(s));
    }

    /// Unset `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn unset_class(&mut self) {
        self.class = None;
    }

    /// Mode used for handling clusters. dot only. [Read more](https://graphviz.org/docs/attrs/clusterrank/).
    pub fn get_clusterrank(&self) -> Option<&str> {
        self.clusterrank.as_ref().map(|x| x.0.as_str())
    }

    /// Set `clusterrank` attribute. [Read more](https://graphviz.org/docs/attrs/clusterrank/).
    pub fn set_clusterrank(&mut self, s: &str) {
        self.clusterrank = Some(Clusterrank::new(s));
    }

    /// Unset `clusterrank` attribute. [Read more](https://graphviz.org/docs/attrs/clusterrank/).
    pub fn unset_clusterrank(&mut self) {
        self.clusterrank = None;
    }

    /// A color scheme namespace: the context for interpreting color names. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn get_colorscheme(&self) -> Option<&str> {
        self.colorscheme.as_ref().map(|x| x.0.as_str())
    }

    /// Set `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn set_colorscheme(&mut self, s: &str) {
        self.colorscheme = Some(Colorscheme::new(s));
    }

    /// Unset `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn unset_colorscheme(&mut self) {
        self.colorscheme = None;
    }

    /// Comments are inserted into output. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|x| x.0.as_str())
    }

    /// Set `comment` attribute. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn set_comment(&mut self, s: &str) {
        self.comment = Some(Comment::new(s));
    }

    /// Unset `comment` attribute. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn unset_comment(&mut self) {
        self.comment = None;
    }

    /// If true, allow edges between clusters. dot only. [Read more](https://graphviz.org/docs/attrs/compound/).
    pub fn get_compound(&self) -> Option<&str> {
        self.compound.as_ref().map(|x| x.0.as_str())
    }

    /// Set `compound` attribute. [Read more](https://graphviz.org/docs/attrs/compound/).
    pub fn set_compound(&mut self, s: &str) {
        self.compound = Some(Compound::new(s));
    }

    /// Unset `compound` attribute. [Read more](https://graphviz.org/docs/attrs/compound/).
    pub fn unset_compound(&mut self) {
        self.compound = None;
    }

    /// If true, use edge concentrators. [Read more](https://graphviz.org/docs/attrs/concentrate/).
    pub fn get_concentrate(&self) -> Option<&str> {
        self.concentrate.as_ref().map(|x| x.0.as_str())
    }

    /// Set `concentrate` attribute. [Read more](https://graphviz.org/docs/attrs/concentrate/).
    pub fn set_concentrate(&mut self, s: &str) {
        self.concentrate = Some(Concentrate::new(s));
    }

    /// Unset `concentrate` attribute. [Read more](https://graphviz.org/docs/attrs/concentrate/).
    pub fn unset_concentrate(&mut self) {
        self.concentrate = None;
    }

    /// Factor damping force motions.. neato only. [Read more](https://graphviz.org/docs/attrs/Damping/).
    pub fn get_damping(&self) -> Option<&str> {
        self.damping.as_ref().map(|x| x.0.as_str())
    }

    /// Set `damping` attribute. [Read more](https://graphviz.org/docs/attrs/Damping/).
    pub fn set_damping(&mut self, s: &str) {
        self.damping = Some(Damping::new(s));
    }

    /// Unset `damping` attribute. [Read more](https://graphviz.org/docs/attrs/Damping/).
    pub fn unset_damping(&mut self) {
        self.damping = None;
    }

    /// The distance between nodes in separate connected components. neato only. [Read more](https://graphviz.org/docs/attrs/defaultdist/).
    pub fn get_defaultdist(&self) -> Option<&str> {
        self.defaultdist.as_ref().map(|x| x.0.as_str())
    }

    /// Set `defaultdist` attribute. [Read more](https://graphviz.org/docs/attrs/defaultdist/).
    pub fn set_defaultdist(&mut self, s: &str) {
        self.defaultdist = Some(Defaultdist::new(s));
    }

    /// Unset `defaultdist` attribute. [Read more](https://graphviz.org/docs/attrs/defaultdist/).
    pub fn unset_defaultdist(&mut self) {
        self.defaultdist = None;
    }

    /// Set the number of dimensions used for the layout. neato, fdp, sfdp only. [Read more](https://graphviz.org/docs/attrs/dim/).
    pub fn get_dim(&self) -> Option<&str> {
        self.dim.as_ref().map(|x| x.0.as_str())
    }

    /// Set `dim` attribute. [Read more](https://graphviz.org/docs/attrs/dim/).
    pub fn set_dim(&mut self, s: &str) {
        self.dim = Some(Dim::new(s));
    }

    /// Unset `dim` attribute. [Read more](https://graphviz.org/docs/attrs/dim/).
    pub fn unset_dim(&mut self) {
        self.dim = None;
    }

    /// Set the number of dimensions used for rendering. neato, fdp, sfdp only. [Read more](https://graphviz.org/docs/attrs/dimen/).
    pub fn get_dimen(&self) -> Option<&str> {
        self.dimen.as_ref().map(|x| x.0.as_str())
    }

    /// Set `dimen` attribute. [Read more](https://graphviz.org/docs/attrs/dimen/).
    pub fn set_dimen(&mut self, s: &str) {
        self.dimen = Some(Dimen::new(s));
    }

    /// Unset `dimen` attribute. [Read more](https://graphviz.org/docs/attrs/dimen/).
    pub fn unset_dimen(&mut self) {
        self.dimen = None;
    }

    /// Whether to constrain most edges to point downwards. neato only. [Read more](https://graphviz.org/docs/attrs/diredgeconstraints/).
    pub fn get_diredgeconstraints(&self) -> Option<&str> {
        self.diredgeconstraints.as_ref().map(|x| x.0.as_str())
    }

    /// Set `diredgeconstraints` attribute. [Read more](https://graphviz.org/docs/attrs/diredgeconstraints/).
    pub fn set_diredgeconstraints(&mut self, s: &str) {
        self.diredgeconstraints = Some(Diredgeconstraints::new(s));
    }

    /// Unset `diredgeconstraints` attribute. [Read more](https://graphviz.org/docs/attrs/diredgeconstraints/).
    pub fn unset_diredgeconstraints(&mut self) {
        self.diredgeconstraints = None;
    }

    /// Specifies the expected number of pixels per inch on a display device. bitmap output, svg only. [Read more](https://graphviz.org/docs/attrs/dpi/).
    pub fn get_dpi(&self) -> Option<&str> {
        self.dpi.as_ref().map(|x| x.0.as_str())
    }

    /// Set `dpi` attribute. [Read more](https://graphviz.org/docs/attrs/dpi/).
    pub fn set_dpi(&mut self, s: &str) {
        self.dpi = Some(Dpi::new(s));
    }

    /// Unset `dpi` attribute. [Read more](https://graphviz.org/docs/attrs/dpi/).
    pub fn unset_dpi(&mut self) {
        self.dpi = None;
    }

    /// Terminating condition. neato only. [Read more](https://graphviz.org/docs/attrs/epsilon/).
    pub fn get_epsilon(&self) -> Option<&str> {
        self.epsilon.as_ref().map(|x| x.0.as_str())
    }

    /// Set `epsilon` attribute. [Read more](https://graphviz.org/docs/attrs/epsilon/).
    pub fn set_epsilon(&mut self, s: &str) {
        self.epsilon = Some(Epsilon::new(s));
    }

    /// Unset `epsilon` attribute. [Read more](https://graphviz.org/docs/attrs/epsilon/).
    pub fn unset_epsilon(&mut self) {
        self.epsilon = None;
    }

    /// Margin used around polygons for purposes of spline edge routing. neato only. [Read more](https://graphviz.org/docs/attrs/esep/).
    pub fn get_esep(&self) -> Option<&str> {
        self.esep.as_ref().map(|x| x.0.as_str())
    }

    /// Set `esep` attribute. [Read more](https://graphviz.org/docs/attrs/esep/).
    pub fn set_esep(&mut self, s: &str) {
        self.esep = Some(Esep::new(s));
    }

    /// Unset `esep` attribute. [Read more](https://graphviz.org/docs/attrs/esep/).
    pub fn unset_esep(&mut self) {
        self.esep = None;
    }

    /// Color used for text. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn get_fontcolor(&self) -> Option<&str> {
        self.fontcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn set_fontcolor(&mut self, s: &str) {
        self.fontcolor = Some(Fontcolor::new(s));
    }

    /// Unset `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn unset_fontcolor(&mut self) {
        self.fontcolor = None;
    }

    /// Font used for text. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn get_fontname(&self) -> Option<&str> {
        self.fontname.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn set_fontname(&mut self, s: &str) {
        self.fontname = Some(Fontname::new(s));
    }

    /// Unset `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn unset_fontname(&mut self) {
        self.fontname = None;
    }

    /// Allows user control of how basic fontnames are represented in SVG output. svg only. [Read more](https://graphviz.org/docs/attrs/fontnames/).
    pub fn get_fontnames(&self) -> Option<&str> {
        self.fontnames.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontnames` attribute. [Read more](https://graphviz.org/docs/attrs/fontnames/).
    pub fn set_fontnames(&mut self, s: &str) {
        self.fontnames = Some(Fontnames::new(s));
    }

    /// Unset `fontnames` attribute. [Read more](https://graphviz.org/docs/attrs/fontnames/).
    pub fn unset_fontnames(&mut self) {
        self.fontnames = None;
    }

    /// Directory list used by libgd to search for bitmap fonts. [Read more](https://graphviz.org/docs/attrs/fontpath/).
    pub fn get_fontpath(&self) -> Option<&str> {
        self.fontpath.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontpath` attribute. [Read more](https://graphviz.org/docs/attrs/fontpath/).
    pub fn set_fontpath(&mut self, s: &str) {
        self.fontpath = Some(Fontpath::new(s));
    }

    /// Unset `fontpath` attribute. [Read more](https://graphviz.org/docs/attrs/fontpath/).
    pub fn unset_fontpath(&mut self) {
        self.fontpath = None;
    }

    /// Font size, in points, used for text. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn get_fontsize(&self) -> Option<&str> {
        self.fontsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn set_fontsize(&mut self, s: &str) {
        self.fontsize = Some(Fontsize::new(s));
    }

    /// Unset `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn unset_fontsize(&mut self) {
        self.fontsize = None;
    }

    /// Whether to force placement of all xlabels, even if overlapping. [Read more](https://graphviz.org/docs/attrs/forcelabels/).
    pub fn get_forcelabels(&self) -> Option<&str> {
        self.forcelabels.as_ref().map(|x| x.0.as_str())
    }

    /// Set `forcelabels` attribute. [Read more](https://graphviz.org/docs/attrs/forcelabels/).
    pub fn set_forcelabels(&mut self, s: &str) {
        self.forcelabels = Some(Forcelabels::new(s));
    }

    /// Unset `forcelabels` attribute. [Read more](https://graphviz.org/docs/attrs/forcelabels/).
    pub fn unset_forcelabels(&mut self) {
        self.forcelabels = None;
    }

    /// If a gradient fill is being used, this determines the angle of the fill. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn get_gradientangle(&self) -> Option<&str> {
        self.gradientangle.as_ref().map(|x| x.0.as_str())
    }

    /// Set `gradientangle` attribute. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn set_gradientangle(&mut self, s: &str) {
        self.gradientangle = Some(Gradientangle::new(s));
    }

    /// Unset `gradientangle` attribute. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn unset_gradientangle(&mut self) {
        self.gradientangle = None;
    }

    /// Synonym for URL. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn get_href(&self) -> Option<&str> {
        self.href.as_ref().map(|x| x.0.as_str())
    }

    /// Set `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn set_href(&mut self, s: &str) {
        self.href = Some(Href::new(s));
    }

    /// Unset `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn unset_href(&mut self) {
        self.href = None;
    }

    /// Identifier for graph objects. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.0.as_str())
    }

    /// Set `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn set_id(&mut self, s: &str) {
        self.id = Some(Id::new(s));
    }

    /// Unset `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn unset_id(&mut self) {
        self.id = None;
    }

    /// A list of directories in which to look for image files. [Read more](https://graphviz.org/docs/attrs/imagepath/).
    pub fn get_imagepath(&self) -> Option<&str> {
        self.imagepath.as_ref().map(|x| x.0.as_str())
    }

    /// Set `imagepath` attribute. [Read more](https://graphviz.org/docs/attrs/imagepath/).
    pub fn set_imagepath(&mut self, s: &str) {
        self.imagepath = Some(Imagepath::new(s));
    }

    /// Unset `imagepath` attribute. [Read more](https://graphviz.org/docs/attrs/imagepath/).
    pub fn unset_imagepath(&mut self) {
        self.imagepath = None;
    }

    /// Scales the input positions to convert between length units. neato, fdp only. [Read more](https://graphviz.org/docs/attrs/inputscale/).
    pub fn get_inputscale(&self) -> Option<&str> {
        self.inputscale.as_ref().map(|x| x.0.as_str())
    }

    /// Set `inputscale` attribute. [Read more](https://graphviz.org/docs/attrs/inputscale/).
    pub fn set_inputscale(&mut self, s: &str) {
        self.inputscale = Some(Inputscale::new(s));
    }

    /// Unset `inputscale` attribute. [Read more](https://graphviz.org/docs/attrs/inputscale/).
    pub fn unset_inputscale(&mut self) {
        self.inputscale = None;
    }

    /// Spring constant used in virtual physical model. fdp, sfdp only. [Read more](https://graphviz.org/docs/attrs/K/).
    pub fn get_k(&self) -> Option<&str> {
        self.k.as_ref().map(|x| x.0.as_str())
    }

    /// Set `k` attribute. [Read more](https://graphviz.org/docs/attrs/K/).
    pub fn set_k(&mut self, s: &str) {
        self.k = Some(K::new(s));
    }

    /// Unset `k` attribute. [Read more](https://graphviz.org/docs/attrs/K/).
    pub fn unset_k(&mut self) {
        self.k = None;
    }

    /// Text label attached to objects. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_ref().map(|x| x.0.as_str())
    }

    /// Set `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn set_label(&mut self, s: &str) {
        self.label = Some(Label::new(s));
    }

    /// Unset `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn unset_label(&mut self) {
        self.label = None;
    }

    /// Whether to treat a node whose name has the form |edgelabel|* as a special node representing an edge label.. sfdp only. [Read more](https://graphviz.org/docs/attrs/label_scheme/).
    pub fn get_label_scheme(&self) -> Option<&str> {
        self.label_scheme.as_ref().map(|x| x.0.as_str())
    }

    /// Set `label_scheme` attribute. [Read more](https://graphviz.org/docs/attrs/label_scheme/).
    pub fn set_label_scheme(&mut self, s: &str) {
        self.label_scheme = Some(LabelScheme::new(s));
    }

    /// Unset `label_scheme` attribute. [Read more](https://graphviz.org/docs/attrs/label_scheme/).
    pub fn unset_label_scheme(&mut self) {
        self.label_scheme = None;
    }

    /// Justification for graph & cluster labels. [Read more](https://graphviz.org/docs/attrs/labeljust/).
    pub fn get_labeljust(&self) -> Option<&str> {
        self.labeljust.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labeljust` attribute. [Read more](https://graphviz.org/docs/attrs/labeljust/).
    pub fn set_labeljust(&mut self, s: &str) {
        self.labeljust = Some(Labeljust::new(s));
    }

    /// Unset `labeljust` attribute. [Read more](https://graphviz.org/docs/attrs/labeljust/).
    pub fn unset_labeljust(&mut self) {
        self.labeljust = None;
    }

    /// Vertical placement of labels for nodes, root graphs and clusters. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn get_labelloc(&self) -> Option<&str> {
        self.labelloc.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelloc` attribute. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn set_labelloc(&mut self, s: &str) {
        self.labelloc = Some(Labelloc::new(s));
    }

    /// Unset `labelloc` attribute. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn unset_labelloc(&mut self) {
        self.labelloc = None;
    }

    /// If true, the graph is rendered in landscape mode. [Read more](https://graphviz.org/docs/attrs/landscape/).
    pub fn get_landscape(&self) -> Option<&str> {
        self.landscape.as_ref().map(|x| x.0.as_str())
    }

    /// Set `landscape` attribute. [Read more](https://graphviz.org/docs/attrs/landscape/).
    pub fn set_landscape(&mut self, s: &str) {
        self.landscape = Some(Landscape::new(s));
    }

    /// Unset `landscape` attribute. [Read more](https://graphviz.org/docs/attrs/landscape/).
    pub fn unset_landscape(&mut self) {
        self.landscape = None;
    }

    /// The separator characters used to split attributes of type layerRange into a list of ranges.. [Read more](https://graphviz.org/docs/attrs/layerlistsep/).
    pub fn get_layerlistsep(&self) -> Option<&str> {
        self.layerlistsep.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layerlistsep` attribute. [Read more](https://graphviz.org/docs/attrs/layerlistsep/).
    pub fn set_layerlistsep(&mut self, s: &str) {
        self.layerlistsep = Some(Layerlistsep::new(s));
    }

    /// Unset `layerlistsep` attribute. [Read more](https://graphviz.org/docs/attrs/layerlistsep/).
    pub fn unset_layerlistsep(&mut self) {
        self.layerlistsep = None;
    }

    /// A linearly ordered list of layer names attached to the graph. [Read more](https://graphviz.org/docs/attrs/layers/).
    pub fn get_layers(&self) -> Option<&str> {
        self.layers.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layers` attribute. [Read more](https://graphviz.org/docs/attrs/layers/).
    pub fn set_layers(&mut self, s: &str) {
        self.layers = Some(Layers::new(s));
    }

    /// Unset `layers` attribute. [Read more](https://graphviz.org/docs/attrs/layers/).
    pub fn unset_layers(&mut self) {
        self.layers = None;
    }

    /// Selects a list of layers to be emitted. [Read more](https://graphviz.org/docs/attrs/layerselect/).
    pub fn get_layerselect(&self) -> Option<&str> {
        self.layerselect.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layerselect` attribute. [Read more](https://graphviz.org/docs/attrs/layerselect/).
    pub fn set_layerselect(&mut self, s: &str) {
        self.layerselect = Some(Layerselect::new(s));
    }

    /// Unset `layerselect` attribute. [Read more](https://graphviz.org/docs/attrs/layerselect/).
    pub fn unset_layerselect(&mut self) {
        self.layerselect = None;
    }

    /// The separator characters for splitting the layers attribute into a list of layer names.. [Read more](https://graphviz.org/docs/attrs/layersep/).
    pub fn get_layersep(&self) -> Option<&str> {
        self.layersep.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layersep` attribute. [Read more](https://graphviz.org/docs/attrs/layersep/).
    pub fn set_layersep(&mut self, s: &str) {
        self.layersep = Some(Layersep::new(s));
    }

    /// Unset `layersep` attribute. [Read more](https://graphviz.org/docs/attrs/layersep/).
    pub fn unset_layersep(&mut self) {
        self.layersep = None;
    }

    /// Which layout engine to use. [Read more](https://graphviz.org/docs/attrs/layout/).
    pub fn get_layout(&self) -> Option<&str> {
        self.layout.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layout` attribute. [Read more](https://graphviz.org/docs/attrs/layout/).
    pub fn set_layout(&mut self, s: &str) {
        self.layout = Some(Layout::new(s));
    }

    /// Unset `layout` attribute. [Read more](https://graphviz.org/docs/attrs/layout/).
    pub fn unset_layout(&mut self) {
        self.layout = None;
    }

    /// Number of levels allowed in the multilevel scheme. sfdp only. [Read more](https://graphviz.org/docs/attrs/levels/).
    pub fn get_levels(&self) -> Option<&str> {
        self.levels.as_ref().map(|x| x.0.as_str())
    }

    /// Set `levels` attribute. [Read more](https://graphviz.org/docs/attrs/levels/).
    pub fn set_levels(&mut self, s: &str) {
        self.levels = Some(Levels::new(s));
    }

    /// Unset `levels` attribute. [Read more](https://graphviz.org/docs/attrs/levels/).
    pub fn unset_levels(&mut self) {
        self.levels = None;
    }

    /// strictness of neato level constraints. neato only. [Read more](https://graphviz.org/docs/attrs/levelsgap/).
    pub fn get_levelsgap(&self) -> Option<&str> {
        self.levelsgap.as_ref().map(|x| x.0.as_str())
    }

    /// Set `levelsgap` attribute. [Read more](https://graphviz.org/docs/attrs/levelsgap/).
    pub fn set_levelsgap(&mut self, s: &str) {
        self.levelsgap = Some(Levelsgap::new(s));
    }

    /// Unset `levelsgap` attribute. [Read more](https://graphviz.org/docs/attrs/levelsgap/).
    pub fn unset_levelsgap(&mut self) {
        self.levelsgap = None;
    }

    /// Height of graph or cluster label, in inches. write only. [Read more](https://graphviz.org/docs/attrs/lheight/).
    pub fn get_lheight(&self) -> Option<&str> {
        self.lheight.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lheight` attribute. [Read more](https://graphviz.org/docs/attrs/lheight/).
    pub fn set_lheight(&mut self, s: &str) {
        self.lheight = Some(Lheight::new(s));
    }

    /// Unset `lheight` attribute. [Read more](https://graphviz.org/docs/attrs/lheight/).
    pub fn unset_lheight(&mut self) {
        self.lheight = None;
    }

    /// How long strings should get before overflowing to next line, for text output.. [Read more](https://graphviz.org/docs/attrs/linelength/).
    pub fn get_linelength(&self) -> Option<&str> {
        self.linelength.as_ref().map(|x| x.0.as_str())
    }

    /// Set `linelength` attribute. [Read more](https://graphviz.org/docs/attrs/linelength/).
    pub fn set_linelength(&mut self, s: &str) {
        self.linelength = Some(Linelength::new(s));
    }

    /// Unset `linelength` attribute. [Read more](https://graphviz.org/docs/attrs/linelength/).
    pub fn unset_linelength(&mut self) {
        self.linelength = None;
    }

    /// Label center position. write only. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn get_lp(&self) -> Option<&str> {
        self.lp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lp` attribute. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn set_lp(&mut self, s: &str) {
        self.lp = Some(Lp::new(s));
    }

    /// Unset `lp` attribute. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn unset_lp(&mut self) {
        self.lp = None;
    }

    /// Width of graph or cluster label, in inches. write only. [Read more](https://graphviz.org/docs/attrs/lwidth/).
    pub fn get_lwidth(&self) -> Option<&str> {
        self.lwidth.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lwidth` attribute. [Read more](https://graphviz.org/docs/attrs/lwidth/).
    pub fn set_lwidth(&mut self, s: &str) {
        self.lwidth = Some(Lwidth::new(s));
    }

    /// Unset `lwidth` attribute. [Read more](https://graphviz.org/docs/attrs/lwidth/).
    pub fn unset_lwidth(&mut self) {
        self.lwidth = None;
    }

    /// For graphs, this sets x and y margins of canvas, in inches. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn get_margin(&self) -> Option<&str> {
        self.margin.as_ref().map(|x| x.0.as_str())
    }

    /// Set `margin` attribute. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn set_margin(&mut self, s: &str) {
        self.margin = Some(Margin::new(s));
    }

    /// Unset `margin` attribute. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn unset_margin(&mut self) {
        self.margin = None;
    }

    /// Sets the number of iterations used. neato, fdp only. [Read more](https://graphviz.org/docs/attrs/maxiter/).
    pub fn get_maxiter(&self) -> Option<&str> {
        self.maxiter.as_ref().map(|x| x.0.as_str())
    }

    /// Set `maxiter` attribute. [Read more](https://graphviz.org/docs/attrs/maxiter/).
    pub fn set_maxiter(&mut self, s: &str) {
        self.maxiter = Some(Maxiter::new(s));
    }

    /// Unset `maxiter` attribute. [Read more](https://graphviz.org/docs/attrs/maxiter/).
    pub fn unset_maxiter(&mut self) {
        self.maxiter = None;
    }

    /// Scale factor for mincross (mc) edge crossing minimiser parameters. dot only. [Read more](https://graphviz.org/docs/attrs/mclimit/).
    pub fn get_mclimit(&self) -> Option<&str> {
        self.mclimit.as_ref().map(|x| x.0.as_str())
    }

    /// Set `mclimit` attribute. [Read more](https://graphviz.org/docs/attrs/mclimit/).
    pub fn set_mclimit(&mut self, s: &str) {
        self.mclimit = Some(Mclimit::new(s));
    }

    /// Unset `mclimit` attribute. [Read more](https://graphviz.org/docs/attrs/mclimit/).
    pub fn unset_mclimit(&mut self) {
        self.mclimit = None;
    }

    /// Specifies the minimum separation between all nodes. circo only. [Read more](https://graphviz.org/docs/attrs/mindist/).
    pub fn get_mindist(&self) -> Option<&str> {
        self.mindist.as_ref().map(|x| x.0.as_str())
    }

    /// Set `mindist` attribute. [Read more](https://graphviz.org/docs/attrs/mindist/).
    pub fn set_mindist(&mut self, s: &str) {
        self.mindist = Some(Mindist::new(s));
    }

    /// Unset `mindist` attribute. [Read more](https://graphviz.org/docs/attrs/mindist/).
    pub fn unset_mindist(&mut self) {
        self.mindist = None;
    }

    /// Technique for optimizing the layout. neato only. [Read more](https://graphviz.org/docs/attrs/mode/).
    pub fn get_mode(&self) -> Option<&str> {
        self.mode.as_ref().map(|x| x.0.as_str())
    }

    /// Set `mode` attribute. [Read more](https://graphviz.org/docs/attrs/mode/).
    pub fn set_mode(&mut self, s: &str) {
        self.mode = Some(Mode::new(s));
    }

    /// Unset `mode` attribute. [Read more](https://graphviz.org/docs/attrs/mode/).
    pub fn unset_mode(&mut self) {
        self.mode = None;
    }

    /// Specifies how the distance matrix is computed for the input graph. neato only. [Read more](https://graphviz.org/docs/attrs/model/).
    pub fn get_model(&self) -> Option<&str> {
        self.model.as_ref().map(|x| x.0.as_str())
    }

    /// Set `model` attribute. [Read more](https://graphviz.org/docs/attrs/model/).
    pub fn set_model(&mut self, s: &str) {
        self.model = Some(Model::new(s));
    }

    /// Unset `model` attribute. [Read more](https://graphviz.org/docs/attrs/model/).
    pub fn unset_model(&mut self) {
        self.model = None;
    }

    /// Whether to use a single global ranking, ignoring clusters. dot only. [Read more](https://graphviz.org/docs/attrs/newrank/).
    pub fn get_newrank(&self) -> Option<&str> {
        self.newrank.as_ref().map(|x| x.0.as_str())
    }

    /// Set `newrank` attribute. [Read more](https://graphviz.org/docs/attrs/newrank/).
    pub fn set_newrank(&mut self, s: &str) {
        self.newrank = Some(Newrank::new(s));
    }

    /// Unset `newrank` attribute. [Read more](https://graphviz.org/docs/attrs/newrank/).
    pub fn unset_newrank(&mut self) {
        self.newrank = None;
    }

    /// In dot, nodesep specifies the minimum space between two adjacent nodes in the same rank, in inches. [Read more](https://graphviz.org/docs/attrs/nodesep/).
    pub fn get_nodesep(&self) -> Option<&str> {
        self.nodesep.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nodesep` attribute. [Read more](https://graphviz.org/docs/attrs/nodesep/).
    pub fn set_nodesep(&mut self, s: &str) {
        self.nodesep = Some(Nodesep::new(s));
    }

    /// Unset `nodesep` attribute. [Read more](https://graphviz.org/docs/attrs/nodesep/).
    pub fn unset_nodesep(&mut self) {
        self.nodesep = None;
    }

    /// Whether to justify multiline text vs the previous text line (rather than the side of the container).. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn get_nojustify(&self) -> Option<&str> {
        self.nojustify.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn set_nojustify(&mut self, s: &str) {
        self.nojustify = Some(Nojustify::new(s));
    }

    /// Unset `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn unset_nojustify(&mut self) {
        self.nojustify = None;
    }

    /// normalizes coordinates of final layout. neato, fdp, sfdp, twopi, circo only. [Read more](https://graphviz.org/docs/attrs/normalize/).
    pub fn get_normalize(&self) -> Option<&str> {
        self.normalize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `normalize` attribute. [Read more](https://graphviz.org/docs/attrs/normalize/).
    pub fn set_normalize(&mut self, s: &str) {
        self.normalize = Some(Normalize::new(s));
    }

    /// Unset `normalize` attribute. [Read more](https://graphviz.org/docs/attrs/normalize/).
    pub fn unset_normalize(&mut self) {
        self.normalize = None;
    }

    /// Whether to avoid translating layout to the origin point. neato only. [Read more](https://graphviz.org/docs/attrs/notranslate/).
    pub fn get_notranslate(&self) -> Option<&str> {
        self.notranslate.as_ref().map(|x| x.0.as_str())
    }

    /// Set `notranslate` attribute. [Read more](https://graphviz.org/docs/attrs/notranslate/).
    pub fn set_notranslate(&mut self, s: &str) {
        self.notranslate = Some(Notranslate::new(s));
    }

    /// Unset `notranslate` attribute. [Read more](https://graphviz.org/docs/attrs/notranslate/).
    pub fn unset_notranslate(&mut self) {
        self.notranslate = None;
    }

    /// Sets number of iterations in network simplex applications. dot only. [Read more](https://graphviz.org/docs/attrs/nslimit/).
    pub fn get_nslimit(&self) -> Option<&str> {
        self.nslimit.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nslimit` attribute. [Read more](https://graphviz.org/docs/attrs/nslimit/).
    pub fn set_nslimit(&mut self, s: &str) {
        self.nslimit = Some(Nslimit::new(s));
    }

    /// Unset `nslimit` attribute. [Read more](https://graphviz.org/docs/attrs/nslimit/).
    pub fn unset_nslimit(&mut self) {
        self.nslimit = None;
    }

    /// Sets number of iterations in network simplex applications. dot only. [Read more](https://graphviz.org/docs/attrs/nslimit1/).
    pub fn get_nslimit1(&self) -> Option<&str> {
        self.nslimit1.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nslimit1` attribute. [Read more](https://graphviz.org/docs/attrs/nslimit1/).
    pub fn set_nslimit1(&mut self, s: &str) {
        self.nslimit1 = Some(Nslimit1::new(s));
    }

    /// Unset `nslimit1` attribute. [Read more](https://graphviz.org/docs/attrs/nslimit1/).
    pub fn unset_nslimit1(&mut self) {
        self.nslimit1 = None;
    }

    /// Whether to draw circo graphs around one circle.. circo only. [Read more](https://graphviz.org/docs/attrs/oneblock/).
    pub fn get_oneblock(&self) -> Option<&str> {
        self.oneblock.as_ref().map(|x| x.0.as_str())
    }

    /// Set `oneblock` attribute. [Read more](https://graphviz.org/docs/attrs/oneblock/).
    pub fn set_oneblock(&mut self, s: &str) {
        self.oneblock = Some(Oneblock::new(s));
    }

    /// Unset `oneblock` attribute. [Read more](https://graphviz.org/docs/attrs/oneblock/).
    pub fn unset_oneblock(&mut self) {
        self.oneblock = None;
    }

    /// Constrains the left-to-right ordering of node edges.. dot only. [Read more](https://graphviz.org/docs/attrs/ordering/).
    pub fn get_ordering(&self) -> Option<&str> {
        self.ordering.as_ref().map(|x| x.0.as_str())
    }

    /// Set `ordering` attribute. [Read more](https://graphviz.org/docs/attrs/ordering/).
    pub fn set_ordering(&mut self, s: &str) {
        self.ordering = Some(Ordering::new(s));
    }

    /// Unset `ordering` attribute. [Read more](https://graphviz.org/docs/attrs/ordering/).
    pub fn unset_ordering(&mut self) {
        self.ordering = None;
    }

    /// node shape rotation angle, or graph orientation. [Read more](https://graphviz.org/docs/attrs/orientation/).
    pub fn get_orientation(&self) -> Option<&str> {
        self.orientation.as_ref().map(|x| x.0.as_str())
    }

    /// Set `orientation` attribute. [Read more](https://graphviz.org/docs/attrs/orientation/).
    pub fn set_orientation(&mut self, s: &str) {
        self.orientation = Some(Orientation::new(s));
    }

    /// Unset `orientation` attribute. [Read more](https://graphviz.org/docs/attrs/orientation/).
    pub fn unset_orientation(&mut self) {
        self.orientation = None;
    }

    /// Specify order in which nodes and edges are drawn. [Read more](https://graphviz.org/docs/attrs/outputorder/).
    pub fn get_outputorder(&self) -> Option<&str> {
        self.outputorder.as_ref().map(|x| x.0.as_str())
    }

    /// Set `outputorder` attribute. [Read more](https://graphviz.org/docs/attrs/outputorder/).
    pub fn set_outputorder(&mut self, s: &str) {
        self.outputorder = Some(Outputorder::new(s));
    }

    /// Unset `outputorder` attribute. [Read more](https://graphviz.org/docs/attrs/outputorder/).
    pub fn unset_outputorder(&mut self) {
        self.outputorder = None;
    }

    /// Determines if and how node overlaps should be removed. fdp, neato only. [Read more](https://graphviz.org/docs/attrs/overlap/).
    pub fn get_overlap(&self) -> Option<&str> {
        self.overlap.as_ref().map(|x| x.0.as_str())
    }

    /// Set `overlap` attribute. [Read more](https://graphviz.org/docs/attrs/overlap/).
    pub fn set_overlap(&mut self, s: &str) {
        self.overlap = Some(Overlap::new(s));
    }

    /// Unset `overlap` attribute. [Read more](https://graphviz.org/docs/attrs/overlap/).
    pub fn unset_overlap(&mut self) {
        self.overlap = None;
    }

    /// Scale layout by factor, to reduce node overlap.. prism, neato, sfdp, fdp, circo, twopi only. [Read more](https://graphviz.org/docs/attrs/overlap_scaling/).
    pub fn get_overlap_scaling(&self) -> Option<&str> {
        self.overlap_scaling.as_ref().map(|x| x.0.as_str())
    }

    /// Set `overlap_scaling` attribute. [Read more](https://graphviz.org/docs/attrs/overlap_scaling/).
    pub fn set_overlap_scaling(&mut self, s: &str) {
        self.overlap_scaling = Some(OverlapScaling::new(s));
    }

    /// Unset `overlap_scaling` attribute. [Read more](https://graphviz.org/docs/attrs/overlap_scaling/).
    pub fn unset_overlap_scaling(&mut self) {
        self.overlap_scaling = None;
    }

    /// Whether the overlap removal algorithm should perform a compression pass to reduce the size of the layout. prism only. [Read more](https://graphviz.org/docs/attrs/overlap_shrink/).
    pub fn get_overlap_shrink(&self) -> Option<&str> {
        self.overlap_shrink.as_ref().map(|x| x.0.as_str())
    }

    /// Set `overlap_shrink` attribute. [Read more](https://graphviz.org/docs/attrs/overlap_shrink/).
    pub fn set_overlap_shrink(&mut self, s: &str) {
        self.overlap_shrink = Some(OverlapShrink::new(s));
    }

    /// Unset `overlap_shrink` attribute. [Read more](https://graphviz.org/docs/attrs/overlap_shrink/).
    pub fn unset_overlap_shrink(&mut self) {
        self.overlap_shrink = None;
    }

    /// Whether each connected component of the graph should be laid out separately, and then the graphs packed together.. [Read more](https://graphviz.org/docs/attrs/pack/).
    pub fn get_pack(&self) -> Option<&str> {
        self.pack.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pack` attribute. [Read more](https://graphviz.org/docs/attrs/pack/).
    pub fn set_pack(&mut self, s: &str) {
        self.pack = Some(Pack::new(s));
    }

    /// Unset `pack` attribute. [Read more](https://graphviz.org/docs/attrs/pack/).
    pub fn unset_pack(&mut self) {
        self.pack = None;
    }

    /// How connected components should be packed. [Read more](https://graphviz.org/docs/attrs/packmode/).
    pub fn get_packmode(&self) -> Option<&str> {
        self.packmode.as_ref().map(|x| x.0.as_str())
    }

    /// Set `packmode` attribute. [Read more](https://graphviz.org/docs/attrs/packmode/).
    pub fn set_packmode(&mut self, s: &str) {
        self.packmode = Some(Packmode::new(s));
    }

    /// Unset `packmode` attribute. [Read more](https://graphviz.org/docs/attrs/packmode/).
    pub fn unset_packmode(&mut self) {
        self.packmode = None;
    }

    /// Inches to extend the drawing area around the minimal area needed to draw the graph. [Read more](https://graphviz.org/docs/attrs/pad/).
    pub fn get_pad(&self) -> Option<&str> {
        self.pad.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pad` attribute. [Read more](https://graphviz.org/docs/attrs/pad/).
    pub fn set_pad(&mut self, s: &str) {
        self.pad = Some(Pad::new(s));
    }

    /// Unset `pad` attribute. [Read more](https://graphviz.org/docs/attrs/pad/).
    pub fn unset_pad(&mut self) {
        self.pad = None;
    }

    /// Width and height of output pages, in inches. [Read more](https://graphviz.org/docs/attrs/page/).
    pub fn get_page(&self) -> Option<&str> {
        self.page.as_ref().map(|x| x.0.as_str())
    }

    /// Set `page` attribute. [Read more](https://graphviz.org/docs/attrs/page/).
    pub fn set_page(&mut self, s: &str) {
        self.page = Some(Page::new(s));
    }

    /// Unset `page` attribute. [Read more](https://graphviz.org/docs/attrs/page/).
    pub fn unset_page(&mut self) {
        self.page = None;
    }

    /// The order in which pages are emitted. [Read more](https://graphviz.org/docs/attrs/pagedir/).
    pub fn get_pagedir(&self) -> Option<&str> {
        self.pagedir.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pagedir` attribute. [Read more](https://graphviz.org/docs/attrs/pagedir/).
    pub fn set_pagedir(&mut self, s: &str) {
        self.pagedir = Some(Pagedir::new(s));
    }

    /// Unset `pagedir` attribute. [Read more](https://graphviz.org/docs/attrs/pagedir/).
    pub fn unset_pagedir(&mut self) {
        self.pagedir = None;
    }

    /// Quadtree scheme to use. sfdp only. [Read more](https://graphviz.org/docs/attrs/quadtree/).
    pub fn get_quadtree(&self) -> Option<&str> {
        self.quadtree.as_ref().map(|x| x.0.as_str())
    }

    /// Set `quadtree` attribute. [Read more](https://graphviz.org/docs/attrs/quadtree/).
    pub fn set_quadtree(&mut self, s: &str) {
        self.quadtree = Some(Quadtree::new(s));
    }

    /// Unset `quadtree` attribute. [Read more](https://graphviz.org/docs/attrs/quadtree/).
    pub fn unset_quadtree(&mut self) {
        self.quadtree = None;
    }

    /// If quantum > 0.0, node label dimensions will be rounded to integral multiples of the quantum. [Read more](https://graphviz.org/docs/attrs/quantum/).
    pub fn get_quantum(&self) -> Option<&str> {
        self.quantum.as_ref().map(|x| x.0.as_str())
    }

    /// Set `quantum` attribute. [Read more](https://graphviz.org/docs/attrs/quantum/).
    pub fn set_quantum(&mut self, s: &str) {
        self.quantum = Some(Quantum::new(s));
    }

    /// Unset `quantum` attribute. [Read more](https://graphviz.org/docs/attrs/quantum/).
    pub fn unset_quantum(&mut self) {
        self.quantum = None;
    }

    /// Sets direction of graph layout. dot only. [Read more](https://graphviz.org/docs/attrs/rankdir/).
    pub fn get_rankdir(&self) -> Option<&str> {
        self.rankdir.as_ref().map(|x| x.0.as_str())
    }

    /// Set `rankdir` attribute. [Read more](https://graphviz.org/docs/attrs/rankdir/).
    pub fn set_rankdir(&mut self, s: &str) {
        self.rankdir = Some(Rankdir::new(s));
    }

    /// Unset `rankdir` attribute. [Read more](https://graphviz.org/docs/attrs/rankdir/).
    pub fn unset_rankdir(&mut self) {
        self.rankdir = None;
    }

    /// Specifies separation between ranks. dot, twopi only. [Read more](https://graphviz.org/docs/attrs/ranksep/).
    pub fn get_ranksep(&self) -> Option<&str> {
        self.ranksep.as_ref().map(|x| x.0.as_str())
    }

    /// Set `ranksep` attribute. [Read more](https://graphviz.org/docs/attrs/ranksep/).
    pub fn set_ranksep(&mut self, s: &str) {
        self.ranksep = Some(Ranksep::new(s));
    }

    /// Unset `ranksep` attribute. [Read more](https://graphviz.org/docs/attrs/ranksep/).
    pub fn unset_ranksep(&mut self) {
        self.ranksep = None;
    }

    /// Sets the aspect ratio (drawing height/drawing width) for the drawing. [Read more](https://graphviz.org/docs/attrs/ratio/).
    pub fn get_ratio(&self) -> Option<&str> {
        self.ratio.as_ref().map(|x| x.0.as_str())
    }

    /// Set `ratio` attribute. [Read more](https://graphviz.org/docs/attrs/ratio/).
    pub fn set_ratio(&mut self, s: &str) {
        self.ratio = Some(Ratio::new(s));
    }

    /// Unset `ratio` attribute. [Read more](https://graphviz.org/docs/attrs/ratio/).
    pub fn unset_ratio(&mut self) {
        self.ratio = None;
    }

    /// If there are multiple clusters, whether to run edge crossing minimization a second time.. dot only. [Read more](https://graphviz.org/docs/attrs/remincross/).
    pub fn get_remincross(&self) -> Option<&str> {
        self.remincross.as_ref().map(|x| x.0.as_str())
    }

    /// Set `remincross` attribute. [Read more](https://graphviz.org/docs/attrs/remincross/).
    pub fn set_remincross(&mut self, s: &str) {
        self.remincross = Some(Remincross::new(s));
    }

    /// Unset `remincross` attribute. [Read more](https://graphviz.org/docs/attrs/remincross/).
    pub fn unset_remincross(&mut self) {
        self.remincross = None;
    }

    /// The power of the repulsive force used in an extended Fruchterman-Reingold. sfdp only. [Read more](https://graphviz.org/docs/attrs/repulsiveforce/).
    pub fn get_repulsiveforce(&self) -> Option<&str> {
        self.repulsiveforce.as_ref().map(|x| x.0.as_str())
    }

    /// Set `repulsiveforce` attribute. [Read more](https://graphviz.org/docs/attrs/repulsiveforce/).
    pub fn set_repulsiveforce(&mut self, s: &str) {
        self.repulsiveforce = Some(Repulsiveforce::new(s));
    }

    /// Unset `repulsiveforce` attribute. [Read more](https://graphviz.org/docs/attrs/repulsiveforce/).
    pub fn unset_repulsiveforce(&mut self) {
        self.repulsiveforce = None;
    }

    /// Synonym for dpi.. bitmap output, svg only. [Read more](https://graphviz.org/docs/attrs/resolution/).
    pub fn get_resolution(&self) -> Option<&str> {
        self.resolution.as_ref().map(|x| x.0.as_str())
    }

    /// Set `resolution` attribute. [Read more](https://graphviz.org/docs/attrs/resolution/).
    pub fn set_resolution(&mut self, s: &str) {
        self.resolution = Some(Resolution::new(s));
    }

    /// Unset `resolution` attribute. [Read more](https://graphviz.org/docs/attrs/resolution/).
    pub fn unset_resolution(&mut self) {
        self.resolution = None;
    }

    /// Specifies nodes to be used as the center of the layout. twopi, circo only. [Read more](https://graphviz.org/docs/attrs/root/).
    pub fn get_root(&self) -> Option<&str> {
        self.root.as_ref().map(|x| x.0.as_str())
    }

    /// Set `root` attribute. [Read more](https://graphviz.org/docs/attrs/root/).
    pub fn set_root(&mut self, s: &str) {
        self.root = Some(Root::new(s));
    }

    /// Unset `root` attribute. [Read more](https://graphviz.org/docs/attrs/root/).
    pub fn unset_root(&mut self) {
        self.root = None;
    }

    /// If rotate=90, sets drawing orientation to landscape. [Read more](https://graphviz.org/docs/attrs/rotate/).
    pub fn get_rotate(&self) -> Option<&str> {
        self.rotate.as_ref().map(|x| x.0.as_str())
    }

    /// Set `rotate` attribute. [Read more](https://graphviz.org/docs/attrs/rotate/).
    pub fn set_rotate(&mut self, s: &str) {
        self.rotate = Some(Rotate::new(s));
    }

    /// Unset `rotate` attribute. [Read more](https://graphviz.org/docs/attrs/rotate/).
    pub fn unset_rotate(&mut self) {
        self.rotate = None;
    }

    /// Rotates the final layout counter-clockwise by the specified number of degrees. sfdp only. [Read more](https://graphviz.org/docs/attrs/rotation/).
    pub fn get_rotation(&self) -> Option<&str> {
        self.rotation.as_ref().map(|x| x.0.as_str())
    }

    /// Set `rotation` attribute. [Read more](https://graphviz.org/docs/attrs/rotation/).
    pub fn set_rotation(&mut self, s: &str) {
        self.rotation = Some(Rotation::new(s));
    }

    /// Unset `rotation` attribute. [Read more](https://graphviz.org/docs/attrs/rotation/).
    pub fn unset_rotation(&mut self) {
        self.rotation = None;
    }

    /// Scales layout by the given factor after the initial layout. neato, twopi only. [Read more](https://graphviz.org/docs/attrs/scale/).
    pub fn get_scale(&self) -> Option<&str> {
        self.scale.as_ref().map(|x| x.0.as_str())
    }

    /// Set `scale` attribute. [Read more](https://graphviz.org/docs/attrs/scale/).
    pub fn set_scale(&mut self, s: &str) {
        self.scale = Some(Scale::new(s));
    }

    /// Unset `scale` attribute. [Read more](https://graphviz.org/docs/attrs/scale/).
    pub fn unset_scale(&mut self) {
        self.scale = None;
    }

    /// During network simplex, the maximum number of edges with negative cut values to search when looking for an edge with minimum cut value.. dot only. [Read more](https://graphviz.org/docs/attrs/searchsize/).
    pub fn get_searchsize(&self) -> Option<&str> {
        self.searchsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `searchsize` attribute. [Read more](https://graphviz.org/docs/attrs/searchsize/).
    pub fn set_searchsize(&mut self, s: &str) {
        self.searchsize = Some(Searchsize::new(s));
    }

    /// Unset `searchsize` attribute. [Read more](https://graphviz.org/docs/attrs/searchsize/).
    pub fn unset_searchsize(&mut self) {
        self.searchsize = None;
    }

    /// Margin to leave around nodes when removing node overlap. fdp, neato only. [Read more](https://graphviz.org/docs/attrs/sep/).
    pub fn get_sep(&self) -> Option<&str> {
        self.sep.as_ref().map(|x| x.0.as_str())
    }

    /// Set `sep` attribute. [Read more](https://graphviz.org/docs/attrs/sep/).
    pub fn set_sep(&mut self, s: &str) {
        self.sep = Some(Sep::new(s));
    }

    /// Unset `sep` attribute. [Read more](https://graphviz.org/docs/attrs/sep/).
    pub fn unset_sep(&mut self) {
        self.sep = None;
    }

    /// Print guide boxes for debugging. dot only. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn get_showboxes(&self) -> Option<&str> {
        self.showboxes.as_ref().map(|x| x.0.as_str())
    }

    /// Set `showboxes` attribute. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn set_showboxes(&mut self, s: &str) {
        self.showboxes = Some(Showboxes::new(s));
    }

    /// Unset `showboxes` attribute. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn unset_showboxes(&mut self) {
        self.showboxes = None;
    }

    /// Maximum width and height of drawing, in inches. [Read more](https://graphviz.org/docs/attrs/size/).
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_ref().map(|x| x.0.as_str())
    }

    /// Set `size` attribute. [Read more](https://graphviz.org/docs/attrs/size/).
    pub fn set_size(&mut self, s: &str) {
        self.size = Some(Size::new(s));
    }

    /// Unset `size` attribute. [Read more](https://graphviz.org/docs/attrs/size/).
    pub fn unset_size(&mut self) {
        self.size = None;
    }

    /// Specifies a post-processing step used to smooth out an uneven distribution of nodes.. sfdp only. [Read more](https://graphviz.org/docs/attrs/smoothing/).
    pub fn get_smoothing(&self) -> Option<&str> {
        self.smoothing.as_ref().map(|x| x.0.as_str())
    }

    /// Set `smoothing` attribute. [Read more](https://graphviz.org/docs/attrs/smoothing/).
    pub fn set_smoothing(&mut self, s: &str) {
        self.smoothing = Some(Smoothing::new(s));
    }

    /// Unset `smoothing` attribute. [Read more](https://graphviz.org/docs/attrs/smoothing/).
    pub fn unset_smoothing(&mut self) {
        self.smoothing = None;
    }

    /// Sort order of graph components for ordering packmode packing.. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn get_sortv(&self) -> Option<&str> {
        self.sortv.as_ref().map(|x| x.0.as_str())
    }

    /// Set `sortv` attribute. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn set_sortv(&mut self, s: &str) {
        self.sortv = Some(Sortv::new(s));
    }

    /// Unset `sortv` attribute. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn unset_sortv(&mut self) {
        self.sortv = None;
    }

    /// Controls how, and if, edges are represented. [Read more](https://graphviz.org/docs/attrs/splines/).
    pub fn get_splines(&self) -> Option<&str> {
        self.splines.as_ref().map(|x| x.0.as_str())
    }

    /// Set `splines` attribute. [Read more](https://graphviz.org/docs/attrs/splines/).
    pub fn set_splines(&mut self, s: &str) {
        self.splines = Some(Splines::new(s));
    }

    /// Unset `splines` attribute. [Read more](https://graphviz.org/docs/attrs/splines/).
    pub fn unset_splines(&mut self) {
        self.splines = None;
    }

    /// Parameter used to determine the initial layout of nodes. neato, fdp, sfdp only. [Read more](https://graphviz.org/docs/attrs/start/).
    pub fn get_start(&self) -> Option<&str> {
        self.start.as_ref().map(|x| x.0.as_str())
    }

    /// Set `start` attribute. [Read more](https://graphviz.org/docs/attrs/start/).
    pub fn set_start(&mut self, s: &str) {
        self.start = Some(Start::new(s));
    }

    /// Unset `start` attribute. [Read more](https://graphviz.org/docs/attrs/start/).
    pub fn unset_start(&mut self) {
        self.start = None;
    }

    /// Set style information for components of the graph. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn get_style(&self) -> Option<&str> {
        self.style.as_ref().map(|x| x.0.as_str())
    }

    /// Set `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn set_style(&mut self, s: &str) {
        self.style = Some(Style::new(s));
    }

    /// Unset `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn unset_style(&mut self) {
        self.style = None;
    }

    /// A URL or pathname specifying an XML style sheet, used in SVG output. svg only. [Read more](https://graphviz.org/docs/attrs/stylesheet/).
    pub fn get_stylesheet(&self) -> Option<&str> {
        self.stylesheet.as_ref().map(|x| x.0.as_str())
    }

    /// Set `stylesheet` attribute. [Read more](https://graphviz.org/docs/attrs/stylesheet/).
    pub fn set_stylesheet(&mut self, s: &str) {
        self.stylesheet = Some(Stylesheet::new(s));
    }

    /// Unset `stylesheet` attribute. [Read more](https://graphviz.org/docs/attrs/stylesheet/).
    pub fn unset_stylesheet(&mut self) {
        self.stylesheet = None;
    }

    /// If the object has a URL, this attribute determines which window of the browser is used for the URL.. map, svg only. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn get_target(&self) -> Option<&str> {
        self.target.as_ref().map(|x| x.0.as_str())
    }

    /// Set `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn set_target(&mut self, s: &str) {
        self.target = Some(Target::new(s));
    }

    /// Unset `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn unset_target(&mut self) {
        self.target = None;
    }

    /// Which rank to move floating (loose) nodes to. dot only. [Read more](https://graphviz.org/docs/attrs/TBbalance/).
    pub fn get_tbbalance(&self) -> Option<&str> {
        self.tbbalance.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tbbalance` attribute. [Read more](https://graphviz.org/docs/attrs/TBbalance/).
    pub fn set_tbbalance(&mut self, s: &str) {
        self.tbbalance = Some(Tbbalance::new(s));
    }

    /// Unset `tbbalance` attribute. [Read more](https://graphviz.org/docs/attrs/TBbalance/).
    pub fn unset_tbbalance(&mut self) {
        self.tbbalance = None;
    }

    /// Tooltip (mouse hover text) attached to the node, edge, cluster, or graph. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn get_tooltip(&self) -> Option<&str> {
        self.tooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn set_tooltip(&mut self, s: &str) {
        self.tooltip = Some(Tooltip::new(s));
    }

    /// Unset `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn unset_tooltip(&mut self) {
        self.tooltip = None;
    }

    /// Whether internal bitmap rendering relies on a truecolor color model or uses. bitmap output only. [Read more](https://graphviz.org/docs/attrs/truecolor/).
    pub fn get_truecolor(&self) -> Option<&str> {
        self.truecolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `truecolor` attribute. [Read more](https://graphviz.org/docs/attrs/truecolor/).
    pub fn set_truecolor(&mut self, s: &str) {
        self.truecolor = Some(Truecolor::new(s));
    }

    /// Unset `truecolor` attribute. [Read more](https://graphviz.org/docs/attrs/truecolor/).
    pub fn unset_truecolor(&mut self) {
        self.truecolor = None;
    }

    /// Hyperlinks incorporated into device-dependent output. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_ref().map(|x| x.0.as_str())
    }

    /// Set `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn set_url(&mut self, s: &str) {
        self.url = Some(Url::new(s));
    }

    /// Unset `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn unset_url(&mut self) {
        self.url = None;
    }

    /// Clipping window on final drawing. [Read more](https://graphviz.org/docs/attrs/viewport/).
    pub fn get_viewport(&self) -> Option<&str> {
        self.viewport.as_ref().map(|x| x.0.as_str())
    }

    /// Set `viewport` attribute. [Read more](https://graphviz.org/docs/attrs/viewport/).
    pub fn set_viewport(&mut self, s: &str) {
        self.viewport = Some(Viewport::new(s));
    }

    /// Unset `viewport` attribute. [Read more](https://graphviz.org/docs/attrs/viewport/).
    pub fn unset_viewport(&mut self) {
        self.viewport = None;
    }

    /// Tuning margin of Voronoi technique. neato, fdp, sfdp, twopi, circo only. [Read more](https://graphviz.org/docs/attrs/voro_margin/).
    pub fn get_voro_margin(&self) -> Option<&str> {
        self.voro_margin.as_ref().map(|x| x.0.as_str())
    }

    /// Set `voro_margin` attribute. [Read more](https://graphviz.org/docs/attrs/voro_margin/).
    pub fn set_voro_margin(&mut self, s: &str) {
        self.voro_margin = Some(VoroMargin::new(s));
    }

    /// Unset `voro_margin` attribute. [Read more](https://graphviz.org/docs/attrs/voro_margin/).
    pub fn unset_voro_margin(&mut self) {
        self.voro_margin = None;
    }

    /// Determines the version of xdot used in output. xdot only. [Read more](https://graphviz.org/docs/attrs/xdotversion/).
    pub fn get_xdotversion(&self) -> Option<&str> {
        self.xdotversion.as_ref().map(|x| x.0.as_str())
    }

    /// Set `xdotversion` attribute. [Read more](https://graphviz.org/docs/attrs/xdotversion/).
    pub fn set_xdotversion(&mut self, s: &str) {
        self.xdotversion = Some(Xdotversion::new(s));
    }

    /// Unset `xdotversion` attribute. [Read more](https://graphviz.org/docs/attrs/xdotversion/).
    pub fn unset_xdotversion(&mut self) {
        self.xdotversion = None;
    }
}

impl IntoIterator for GraphAttrs {
    type Item = String;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Self::Item>, 103>>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self._background.map(|x| x.to_string()),
            self.bb.map(|x| x.to_string()),
            self.beautify.map(|x| x.to_string()),
            self.bgcolor.map(|x| x.to_string()),
            self.center.map(|x| x.to_string()),
            self.charset.map(|x| x.to_string()),
            self.class.map(|x| x.to_string()),
            self.clusterrank.map(|x| x.to_string()),
            self.colorscheme.map(|x| x.to_string()),
            self.comment.map(|x| x.to_string()),
            self.compound.map(|x| x.to_string()),
            self.concentrate.map(|x| x.to_string()),
            self.damping.map(|x| x.to_string()),
            self.defaultdist.map(|x| x.to_string()),
            self.dim.map(|x| x.to_string()),
            self.dimen.map(|x| x.to_string()),
            self.diredgeconstraints.map(|x| x.to_string()),
            self.dpi.map(|x| x.to_string()),
            self.epsilon.map(|x| x.to_string()),
            self.esep.map(|x| x.to_string()),
            self.fontcolor.map(|x| x.to_string()),
            self.fontname.map(|x| x.to_string()),
            self.fontnames.map(|x| x.to_string()),
            self.fontpath.map(|x| x.to_string()),
            self.fontsize.map(|x| x.to_string()),
            self.forcelabels.map(|x| x.to_string()),
            self.gradientangle.map(|x| x.to_string()),
            self.href.map(|x| x.to_string()),
            self.id.map(|x| x.to_string()),
            self.imagepath.map(|x| x.to_string()),
            self.inputscale.map(|x| x.to_string()),
            self.k.map(|x| x.to_string()),
            self.label.map(|x| x.to_string()),
            self.label_scheme.map(|x| x.to_string()),
            self.labeljust.map(|x| x.to_string()),
            self.labelloc.map(|x| x.to_string()),
            self.landscape.map(|x| x.to_string()),
            self.layerlistsep.map(|x| x.to_string()),
            self.layers.map(|x| x.to_string()),
            self.layerselect.map(|x| x.to_string()),
            self.layersep.map(|x| x.to_string()),
            self.layout.map(|x| x.to_string()),
            self.levels.map(|x| x.to_string()),
            self.levelsgap.map(|x| x.to_string()),
            self.lheight.map(|x| x.to_string()),
            self.linelength.map(|x| x.to_string()),
            self.lp.map(|x| x.to_string()),
            self.lwidth.map(|x| x.to_string()),
            self.margin.map(|x| x.to_string()),
            self.maxiter.map(|x| x.to_string()),
            self.mclimit.map(|x| x.to_string()),
            self.mindist.map(|x| x.to_string()),
            self.mode.map(|x| x.to_string()),
            self.model.map(|x| x.to_string()),
            self.newrank.map(|x| x.to_string()),
            self.nodesep.map(|x| x.to_string()),
            self.nojustify.map(|x| x.to_string()),
            self.normalize.map(|x| x.to_string()),
            self.notranslate.map(|x| x.to_string()),
            self.nslimit.map(|x| x.to_string()),
            self.nslimit1.map(|x| x.to_string()),
            self.oneblock.map(|x| x.to_string()),
            self.ordering.map(|x| x.to_string()),
            self.orientation.map(|x| x.to_string()),
            self.outputorder.map(|x| x.to_string()),
            self.overlap.map(|x| x.to_string()),
            self.overlap_scaling.map(|x| x.to_string()),
            self.overlap_shrink.map(|x| x.to_string()),
            self.pack.map(|x| x.to_string()),
            self.packmode.map(|x| x.to_string()),
            self.pad.map(|x| x.to_string()),
            self.page.map(|x| x.to_string()),
            self.pagedir.map(|x| x.to_string()),
            self.quadtree.map(|x| x.to_string()),
            self.quantum.map(|x| x.to_string()),
            self.rankdir.map(|x| x.to_string()),
            self.ranksep.map(|x| x.to_string()),
            self.ratio.map(|x| x.to_string()),
            self.remincross.map(|x| x.to_string()),
            self.repulsiveforce.map(|x| x.to_string()),
            self.resolution.map(|x| x.to_string()),
            self.root.map(|x| x.to_string()),
            self.rotate.map(|x| x.to_string()),
            self.rotation.map(|x| x.to_string()),
            self.scale.map(|x| x.to_string()),
            self.searchsize.map(|x| x.to_string()),
            self.sep.map(|x| x.to_string()),
            self.showboxes.map(|x| x.to_string()),
            self.size.map(|x| x.to_string()),
            self.smoothing.map(|x| x.to_string()),
            self.sortv.map(|x| x.to_string()),
            self.splines.map(|x| x.to_string()),
            self.start.map(|x| x.to_string()),
            self.style.map(|x| x.to_string()),
            self.stylesheet.map(|x| x.to_string()),
            self.target.map(|x| x.to_string()),
            self.tbbalance.map(|x| x.to_string()),
            self.tooltip.map(|x| x.to_string()),
            self.truecolor.map(|x| x.to_string()),
            self.url.map(|x| x.to_string()),
            self.viewport.map(|x| x.to_string()),
            self.voro_margin.map(|x| x.to_string()),
            self.xdotversion.map(|x| x.to_string()),
        ]
        .into_iter()
        .flatten()
    }
}

/// Node attributes.
#[derive(Clone, Debug, Default)]
pub struct NodeAttrs {
    area: Option<Area>,
    class: Option<Class>,
    color: Option<Color>,
    colorscheme: Option<Colorscheme>,
    comment: Option<Comment>,
    distortion: Option<Distortion>,
    fillcolor: Option<Fillcolor>,
    fixedsize: Option<Fixedsize>,
    fontcolor: Option<Fontcolor>,
    fontname: Option<Fontname>,
    fontsize: Option<Fontsize>,
    gradientangle: Option<Gradientangle>,
    group: Option<Group>,
    height: Option<Height>,
    href: Option<Href>,
    id: Option<Id>,
    image: Option<Image>,
    imagepos: Option<Imagepos>,
    imagescale: Option<Imagescale>,
    label: Option<Label>,
    labelloc: Option<Labelloc>,
    layer: Option<Layer>,
    margin: Option<Margin>,
    nojustify: Option<Nojustify>,
    ordering: Option<Ordering>,
    orientation: Option<Orientation>,
    penwidth: Option<Penwidth>,
    peripheries: Option<Peripheries>,
    pin: Option<Pin>,
    pos: Option<Pos>,
    rects: Option<Rects>,
    regular: Option<Regular>,
    root: Option<Root>,
    samplepoints: Option<Samplepoints>,
    shape: Option<Shape>,
    shapefile: Option<Shapefile>,
    showboxes: Option<Showboxes>,
    sides: Option<Sides>,
    skew: Option<Skew>,
    sortv: Option<Sortv>,
    style: Option<Style>,
    target: Option<Target>,
    tooltip: Option<Tooltip>,
    url: Option<Url>,
    vertices: Option<Vertices>,
    width: Option<Width>,
    xlabel: Option<Xlabel>,
    xlp: Option<Xlp>,
    z: Option<Z>,
}

impl NodeAttrs {
    /// Indicates the preferred area for a node or empty cluster. patchwork only. [Read more](https://graphviz.org/docs/attrs/area/).
    pub fn get_area(&self) -> Option<&str> {
        self.area.as_ref().map(|x| x.0.as_str())
    }

    /// Set `area` attribute. [Read more](https://graphviz.org/docs/attrs/area/).
    pub fn set_area(&mut self, s: &str) {
        self.area = Some(Area::new(s));
    }

    /// Unset `area` attribute. [Read more](https://graphviz.org/docs/attrs/area/).
    pub fn unset_area(&mut self) {
        self.area = None;
    }

    /// Classnames to attach to the node, edge, graph, or cluster's SVG element. svg only. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn get_class(&self) -> Option<&str> {
        self.class.as_ref().map(|x| x.0.as_str())
    }

    /// Set `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn set_class(&mut self, s: &str) {
        self.class = Some(Class::new(s));
    }

    /// Unset `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn unset_class(&mut self) {
        self.class = None;
    }

    /// Basic drawing color for graphics, not text. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_ref().map(|x| x.0.as_str())
    }

    /// Set `color` attribute. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn set_color(&mut self, s: &str) {
        self.color = Some(Color::new(s));
    }

    /// Unset `color` attribute. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn unset_color(&mut self) {
        self.color = None;
    }

    /// A color scheme namespace: the context for interpreting color names. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn get_colorscheme(&self) -> Option<&str> {
        self.colorscheme.as_ref().map(|x| x.0.as_str())
    }

    /// Set `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn set_colorscheme(&mut self, s: &str) {
        self.colorscheme = Some(Colorscheme::new(s));
    }

    /// Unset `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn unset_colorscheme(&mut self) {
        self.colorscheme = None;
    }

    /// Comments are inserted into output. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|x| x.0.as_str())
    }

    /// Set `comment` attribute. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn set_comment(&mut self, s: &str) {
        self.comment = Some(Comment::new(s));
    }

    /// Unset `comment` attribute. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn unset_comment(&mut self) {
        self.comment = None;
    }

    /// Distortion factor for shape=polygon. [Read more](https://graphviz.org/docs/attrs/distortion/).
    pub fn get_distortion(&self) -> Option<&str> {
        self.distortion.as_ref().map(|x| x.0.as_str())
    }

    /// Set `distortion` attribute. [Read more](https://graphviz.org/docs/attrs/distortion/).
    pub fn set_distortion(&mut self, s: &str) {
        self.distortion = Some(Distortion::new(s));
    }

    /// Unset `distortion` attribute. [Read more](https://graphviz.org/docs/attrs/distortion/).
    pub fn unset_distortion(&mut self) {
        self.distortion = None;
    }

    /// Color used to fill the background of a node or cluster. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn get_fillcolor(&self) -> Option<&str> {
        self.fillcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fillcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn set_fillcolor(&mut self, s: &str) {
        self.fillcolor = Some(Fillcolor::new(s));
    }

    /// Unset `fillcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn unset_fillcolor(&mut self) {
        self.fillcolor = None;
    }

    /// Whether to use the specified width and height attributes to choose node size (rather than sizing to fit the node contents). [Read more](https://graphviz.org/docs/attrs/fixedsize/).
    pub fn get_fixedsize(&self) -> Option<&str> {
        self.fixedsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fixedsize` attribute. [Read more](https://graphviz.org/docs/attrs/fixedsize/).
    pub fn set_fixedsize(&mut self, s: &str) {
        self.fixedsize = Some(Fixedsize::new(s));
    }

    /// Unset `fixedsize` attribute. [Read more](https://graphviz.org/docs/attrs/fixedsize/).
    pub fn unset_fixedsize(&mut self) {
        self.fixedsize = None;
    }

    /// Color used for text. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn get_fontcolor(&self) -> Option<&str> {
        self.fontcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn set_fontcolor(&mut self, s: &str) {
        self.fontcolor = Some(Fontcolor::new(s));
    }

    /// Unset `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn unset_fontcolor(&mut self) {
        self.fontcolor = None;
    }

    /// Font used for text. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn get_fontname(&self) -> Option<&str> {
        self.fontname.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn set_fontname(&mut self, s: &str) {
        self.fontname = Some(Fontname::new(s));
    }

    /// Unset `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn unset_fontname(&mut self) {
        self.fontname = None;
    }

    /// Font size, in points, used for text. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn get_fontsize(&self) -> Option<&str> {
        self.fontsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn set_fontsize(&mut self, s: &str) {
        self.fontsize = Some(Fontsize::new(s));
    }

    /// Unset `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn unset_fontsize(&mut self) {
        self.fontsize = None;
    }

    /// If a gradient fill is being used, this determines the angle of the fill. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn get_gradientangle(&self) -> Option<&str> {
        self.gradientangle.as_ref().map(|x| x.0.as_str())
    }

    /// Set `gradientangle` attribute. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn set_gradientangle(&mut self, s: &str) {
        self.gradientangle = Some(Gradientangle::new(s));
    }

    /// Unset `gradientangle` attribute. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn unset_gradientangle(&mut self) {
        self.gradientangle = None;
    }

    /// Name for a group of nodes, for bundling edges avoiding crossings.. dot only. [Read more](https://graphviz.org/docs/attrs/group/).
    pub fn get_group(&self) -> Option<&str> {
        self.group.as_ref().map(|x| x.0.as_str())
    }

    /// Set `group` attribute. [Read more](https://graphviz.org/docs/attrs/group/).
    pub fn set_group(&mut self, s: &str) {
        self.group = Some(Group::new(s));
    }

    /// Unset `group` attribute. [Read more](https://graphviz.org/docs/attrs/group/).
    pub fn unset_group(&mut self) {
        self.group = None;
    }

    /// Height of node, in inches. [Read more](https://graphviz.org/docs/attrs/height/).
    pub fn get_height(&self) -> Option<&str> {
        self.height.as_ref().map(|x| x.0.as_str())
    }

    /// Set `height` attribute. [Read more](https://graphviz.org/docs/attrs/height/).
    pub fn set_height(&mut self, s: &str) {
        self.height = Some(Height::new(s));
    }

    /// Unset `height` attribute. [Read more](https://graphviz.org/docs/attrs/height/).
    pub fn unset_height(&mut self) {
        self.height = None;
    }

    /// Synonym for URL. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn get_href(&self) -> Option<&str> {
        self.href.as_ref().map(|x| x.0.as_str())
    }

    /// Set `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn set_href(&mut self, s: &str) {
        self.href = Some(Href::new(s));
    }

    /// Unset `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn unset_href(&mut self) {
        self.href = None;
    }

    /// Identifier for graph objects. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.0.as_str())
    }

    /// Set `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn set_id(&mut self, s: &str) {
        self.id = Some(Id::new(s));
    }

    /// Unset `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn unset_id(&mut self) {
        self.id = None;
    }

    /// Gives the name of a file containing an image to be displayed inside a node. [Read more](https://graphviz.org/docs/attrs/image/).
    pub fn get_image(&self) -> Option<&str> {
        self.image.as_ref().map(|x| x.0.as_str())
    }

    /// Set `image` attribute. [Read more](https://graphviz.org/docs/attrs/image/).
    pub fn set_image(&mut self, s: &str) {
        self.image = Some(Image::new(s));
    }

    /// Unset `image` attribute. [Read more](https://graphviz.org/docs/attrs/image/).
    pub fn unset_image(&mut self) {
        self.image = None;
    }

    /// Controls how an image is positioned within its containing node. [Read more](https://graphviz.org/docs/attrs/imagepos/).
    pub fn get_imagepos(&self) -> Option<&str> {
        self.imagepos.as_ref().map(|x| x.0.as_str())
    }

    /// Set `imagepos` attribute. [Read more](https://graphviz.org/docs/attrs/imagepos/).
    pub fn set_imagepos(&mut self, s: &str) {
        self.imagepos = Some(Imagepos::new(s));
    }

    /// Unset `imagepos` attribute. [Read more](https://graphviz.org/docs/attrs/imagepos/).
    pub fn unset_imagepos(&mut self) {
        self.imagepos = None;
    }

    /// Controls how an image fills its containing node. [Read more](https://graphviz.org/docs/attrs/imagescale/).
    pub fn get_imagescale(&self) -> Option<&str> {
        self.imagescale.as_ref().map(|x| x.0.as_str())
    }

    /// Set `imagescale` attribute. [Read more](https://graphviz.org/docs/attrs/imagescale/).
    pub fn set_imagescale(&mut self, s: &str) {
        self.imagescale = Some(Imagescale::new(s));
    }

    /// Unset `imagescale` attribute. [Read more](https://graphviz.org/docs/attrs/imagescale/).
    pub fn unset_imagescale(&mut self) {
        self.imagescale = None;
    }

    /// Text label attached to objects. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_ref().map(|x| x.0.as_str())
    }

    /// Set `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn set_label(&mut self, s: &str) {
        self.label = Some(Label::new(s));
    }

    /// Unset `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn unset_label(&mut self) {
        self.label = None;
    }

    /// Vertical placement of labels for nodes, root graphs and clusters. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn get_labelloc(&self) -> Option<&str> {
        self.labelloc.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelloc` attribute. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn set_labelloc(&mut self, s: &str) {
        self.labelloc = Some(Labelloc::new(s));
    }

    /// Unset `labelloc` attribute. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn unset_labelloc(&mut self) {
        self.labelloc = None;
    }

    /// Specifies layers in which the node, edge or cluster is present. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn get_layer(&self) -> Option<&str> {
        self.layer.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layer` attribute. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn set_layer(&mut self, s: &str) {
        self.layer = Some(Layer::new(s));
    }

    /// Unset `layer` attribute. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn unset_layer(&mut self) {
        self.layer = None;
    }

    /// For graphs, this sets x and y margins of canvas, in inches. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn get_margin(&self) -> Option<&str> {
        self.margin.as_ref().map(|x| x.0.as_str())
    }

    /// Set `margin` attribute. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn set_margin(&mut self, s: &str) {
        self.margin = Some(Margin::new(s));
    }

    /// Unset `margin` attribute. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn unset_margin(&mut self) {
        self.margin = None;
    }

    /// Whether to justify multiline text vs the previous text line (rather than the side of the container).. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn get_nojustify(&self) -> Option<&str> {
        self.nojustify.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn set_nojustify(&mut self, s: &str) {
        self.nojustify = Some(Nojustify::new(s));
    }

    /// Unset `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn unset_nojustify(&mut self) {
        self.nojustify = None;
    }

    /// Constrains the left-to-right ordering of node edges.. dot only. [Read more](https://graphviz.org/docs/attrs/ordering/).
    pub fn get_ordering(&self) -> Option<&str> {
        self.ordering.as_ref().map(|x| x.0.as_str())
    }

    /// Set `ordering` attribute. [Read more](https://graphviz.org/docs/attrs/ordering/).
    pub fn set_ordering(&mut self, s: &str) {
        self.ordering = Some(Ordering::new(s));
    }

    /// Unset `ordering` attribute. [Read more](https://graphviz.org/docs/attrs/ordering/).
    pub fn unset_ordering(&mut self) {
        self.ordering = None;
    }

    /// node shape rotation angle, or graph orientation. [Read more](https://graphviz.org/docs/attrs/orientation/).
    pub fn get_orientation(&self) -> Option<&str> {
        self.orientation.as_ref().map(|x| x.0.as_str())
    }

    /// Set `orientation` attribute. [Read more](https://graphviz.org/docs/attrs/orientation/).
    pub fn set_orientation(&mut self, s: &str) {
        self.orientation = Some(Orientation::new(s));
    }

    /// Unset `orientation` attribute. [Read more](https://graphviz.org/docs/attrs/orientation/).
    pub fn unset_orientation(&mut self) {
        self.orientation = None;
    }

    /// Specifies the width of the pen, in points, used to draw lines and curves. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn get_penwidth(&self) -> Option<&str> {
        self.penwidth.as_ref().map(|x| x.0.as_str())
    }

    /// Set `penwidth` attribute. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn set_penwidth(&mut self, s: &str) {
        self.penwidth = Some(Penwidth::new(s));
    }

    /// Unset `penwidth` attribute. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn unset_penwidth(&mut self) {
        self.penwidth = None;
    }

    /// Set number of peripheries used in polygonal shapes and cluster boundaries. [Read more](https://graphviz.org/docs/attrs/peripheries/).
    pub fn get_peripheries(&self) -> Option<&str> {
        self.peripheries.as_ref().map(|x| x.0.as_str())
    }

    /// Set `peripheries` attribute. [Read more](https://graphviz.org/docs/attrs/peripheries/).
    pub fn set_peripheries(&mut self, s: &str) {
        self.peripheries = Some(Peripheries::new(s));
    }

    /// Unset `peripheries` attribute. [Read more](https://graphviz.org/docs/attrs/peripheries/).
    pub fn unset_peripheries(&mut self) {
        self.peripheries = None;
    }

    /// Keeps the node at the node's given input position. neato, fdp only. [Read more](https://graphviz.org/docs/attrs/pin/).
    pub fn get_pin(&self) -> Option<&str> {
        self.pin.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pin` attribute. [Read more](https://graphviz.org/docs/attrs/pin/).
    pub fn set_pin(&mut self, s: &str) {
        self.pin = Some(Pin::new(s));
    }

    /// Unset `pin` attribute. [Read more](https://graphviz.org/docs/attrs/pin/).
    pub fn unset_pin(&mut self) {
        self.pin = None;
    }

    /// Position of node, or spline control points. neato, fdp only. [Read more](https://graphviz.org/docs/attrs/pos/).
    pub fn get_pos(&self) -> Option<&str> {
        self.pos.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pos` attribute. [Read more](https://graphviz.org/docs/attrs/pos/).
    pub fn set_pos(&mut self, s: &str) {
        self.pos = Some(Pos::new(s));
    }

    /// Unset `pos` attribute. [Read more](https://graphviz.org/docs/attrs/pos/).
    pub fn unset_pos(&mut self) {
        self.pos = None;
    }

    /// Rectangles for fields of records, in points. write only. [Read more](https://graphviz.org/docs/attrs/rects/).
    pub fn get_rects(&self) -> Option<&str> {
        self.rects.as_ref().map(|x| x.0.as_str())
    }

    /// Set `rects` attribute. [Read more](https://graphviz.org/docs/attrs/rects/).
    pub fn set_rects(&mut self, s: &str) {
        self.rects = Some(Rects::new(s));
    }

    /// Unset `rects` attribute. [Read more](https://graphviz.org/docs/attrs/rects/).
    pub fn unset_rects(&mut self) {
        self.rects = None;
    }

    /// If true, force polygon to be regular, i.e., the vertices of th. [Read more](https://graphviz.org/docs/attrs/regular/).
    pub fn get_regular(&self) -> Option<&str> {
        self.regular.as_ref().map(|x| x.0.as_str())
    }

    /// Set `regular` attribute. [Read more](https://graphviz.org/docs/attrs/regular/).
    pub fn set_regular(&mut self, s: &str) {
        self.regular = Some(Regular::new(s));
    }

    /// Unset `regular` attribute. [Read more](https://graphviz.org/docs/attrs/regular/).
    pub fn unset_regular(&mut self) {
        self.regular = None;
    }

    /// Specifies nodes to be used as the center of the layout. twopi, circo only. [Read more](https://graphviz.org/docs/attrs/root/).
    pub fn get_root(&self) -> Option<&str> {
        self.root.as_ref().map(|x| x.0.as_str())
    }

    /// Set `root` attribute. [Read more](https://graphviz.org/docs/attrs/root/).
    pub fn set_root(&mut self, s: &str) {
        self.root = Some(Root::new(s));
    }

    /// Unset `root` attribute. [Read more](https://graphviz.org/docs/attrs/root/).
    pub fn unset_root(&mut self) {
        self.root = None;
    }

    /// Gives the number of points used for a circle/ellipse node. [Read more](https://graphviz.org/docs/attrs/samplepoints/).
    pub fn get_samplepoints(&self) -> Option<&str> {
        self.samplepoints.as_ref().map(|x| x.0.as_str())
    }

    /// Set `samplepoints` attribute. [Read more](https://graphviz.org/docs/attrs/samplepoints/).
    pub fn set_samplepoints(&mut self, s: &str) {
        self.samplepoints = Some(Samplepoints::new(s));
    }

    /// Unset `samplepoints` attribute. [Read more](https://graphviz.org/docs/attrs/samplepoints/).
    pub fn unset_samplepoints(&mut self) {
        self.samplepoints = None;
    }

    /// Sets the shape of a node. [Read more](https://graphviz.org/docs/attrs/shape/).
    pub fn get_shape(&self) -> Option<&str> {
        self.shape.as_ref().map(|x| x.0.as_str())
    }

    /// Set `shape` attribute. [Read more](https://graphviz.org/docs/attrs/shape/).
    pub fn set_shape(&mut self, s: &str) {
        self.shape = Some(Shape::new(s));
    }

    /// Unset `shape` attribute. [Read more](https://graphviz.org/docs/attrs/shape/).
    pub fn unset_shape(&mut self) {
        self.shape = None;
    }

    /// A file containing user-supplied node content. [Read more](https://graphviz.org/docs/attrs/shapefile/).
    pub fn get_shapefile(&self) -> Option<&str> {
        self.shapefile.as_ref().map(|x| x.0.as_str())
    }

    /// Set `shapefile` attribute. [Read more](https://graphviz.org/docs/attrs/shapefile/).
    pub fn set_shapefile(&mut self, s: &str) {
        self.shapefile = Some(Shapefile::new(s));
    }

    /// Unset `shapefile` attribute. [Read more](https://graphviz.org/docs/attrs/shapefile/).
    pub fn unset_shapefile(&mut self) {
        self.shapefile = None;
    }

    /// Print guide boxes for debugging. dot only. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn get_showboxes(&self) -> Option<&str> {
        self.showboxes.as_ref().map(|x| x.0.as_str())
    }

    /// Set `showboxes` attribute. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn set_showboxes(&mut self, s: &str) {
        self.showboxes = Some(Showboxes::new(s));
    }

    /// Unset `showboxes` attribute. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn unset_showboxes(&mut self) {
        self.showboxes = None;
    }

    /// Number of sides when shape=polygon. [Read more](https://graphviz.org/docs/attrs/sides/).
    pub fn get_sides(&self) -> Option<&str> {
        self.sides.as_ref().map(|x| x.0.as_str())
    }

    /// Set `sides` attribute. [Read more](https://graphviz.org/docs/attrs/sides/).
    pub fn set_sides(&mut self, s: &str) {
        self.sides = Some(Sides::new(s));
    }

    /// Unset `sides` attribute. [Read more](https://graphviz.org/docs/attrs/sides/).
    pub fn unset_sides(&mut self) {
        self.sides = None;
    }

    /// Skew factor for shape=polygon. [Read more](https://graphviz.org/docs/attrs/skew/).
    pub fn get_skew(&self) -> Option<&str> {
        self.skew.as_ref().map(|x| x.0.as_str())
    }

    /// Set `skew` attribute. [Read more](https://graphviz.org/docs/attrs/skew/).
    pub fn set_skew(&mut self, s: &str) {
        self.skew = Some(Skew::new(s));
    }

    /// Unset `skew` attribute. [Read more](https://graphviz.org/docs/attrs/skew/).
    pub fn unset_skew(&mut self) {
        self.skew = None;
    }

    /// Sort order of graph components for ordering packmode packing.. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn get_sortv(&self) -> Option<&str> {
        self.sortv.as_ref().map(|x| x.0.as_str())
    }

    /// Set `sortv` attribute. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn set_sortv(&mut self, s: &str) {
        self.sortv = Some(Sortv::new(s));
    }

    /// Unset `sortv` attribute. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn unset_sortv(&mut self) {
        self.sortv = None;
    }

    /// Set style information for components of the graph. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn get_style(&self) -> Option<&str> {
        self.style.as_ref().map(|x| x.0.as_str())
    }

    /// Set `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn set_style(&mut self, s: &str) {
        self.style = Some(Style::new(s));
    }

    /// Unset `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn unset_style(&mut self) {
        self.style = None;
    }

    /// If the object has a URL, this attribute determines which window of the browser is used for the URL.. map, svg only. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn get_target(&self) -> Option<&str> {
        self.target.as_ref().map(|x| x.0.as_str())
    }

    /// Set `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn set_target(&mut self, s: &str) {
        self.target = Some(Target::new(s));
    }

    /// Unset `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn unset_target(&mut self) {
        self.target = None;
    }

    /// Tooltip (mouse hover text) attached to the node, edge, cluster, or graph. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn get_tooltip(&self) -> Option<&str> {
        self.tooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn set_tooltip(&mut self, s: &str) {
        self.tooltip = Some(Tooltip::new(s));
    }

    /// Unset `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn unset_tooltip(&mut self) {
        self.tooltip = None;
    }

    /// Hyperlinks incorporated into device-dependent output. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_ref().map(|x| x.0.as_str())
    }

    /// Set `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn set_url(&mut self, s: &str) {
        self.url = Some(Url::new(s));
    }

    /// Unset `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn unset_url(&mut self) {
        self.url = None;
    }

    /// Sets the coordinates of the vertices of the node's polygon, in inches. write only. [Read more](https://graphviz.org/docs/attrs/vertices/).
    pub fn get_vertices(&self) -> Option<&str> {
        self.vertices.as_ref().map(|x| x.0.as_str())
    }

    /// Set `vertices` attribute. [Read more](https://graphviz.org/docs/attrs/vertices/).
    pub fn set_vertices(&mut self, s: &str) {
        self.vertices = Some(Vertices::new(s));
    }

    /// Unset `vertices` attribute. [Read more](https://graphviz.org/docs/attrs/vertices/).
    pub fn unset_vertices(&mut self) {
        self.vertices = None;
    }

    /// Width of node, in inches. [Read more](https://graphviz.org/docs/attrs/width/).
    pub fn get_width(&self) -> Option<&str> {
        self.width.as_ref().map(|x| x.0.as_str())
    }

    /// Set `width` attribute. [Read more](https://graphviz.org/docs/attrs/width/).
    pub fn set_width(&mut self, s: &str) {
        self.width = Some(Width::new(s));
    }

    /// Unset `width` attribute. [Read more](https://graphviz.org/docs/attrs/width/).
    pub fn unset_width(&mut self) {
        self.width = None;
    }

    /// External label for a node or edge. [Read more](https://graphviz.org/docs/attrs/xlabel/).
    pub fn get_xlabel(&self) -> Option<&str> {
        self.xlabel.as_ref().map(|x| x.0.as_str())
    }

    /// Set `xlabel` attribute. [Read more](https://graphviz.org/docs/attrs/xlabel/).
    pub fn set_xlabel(&mut self, s: &str) {
        self.xlabel = Some(Xlabel::new(s));
    }

    /// Unset `xlabel` attribute. [Read more](https://graphviz.org/docs/attrs/xlabel/).
    pub fn unset_xlabel(&mut self) {
        self.xlabel = None;
    }

    /// Position of an exterior label, in points. write only. [Read more](https://graphviz.org/docs/attrs/xlp/).
    pub fn get_xlp(&self) -> Option<&str> {
        self.xlp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `xlp` attribute. [Read more](https://graphviz.org/docs/attrs/xlp/).
    pub fn set_xlp(&mut self, s: &str) {
        self.xlp = Some(Xlp::new(s));
    }

    /// Unset `xlp` attribute. [Read more](https://graphviz.org/docs/attrs/xlp/).
    pub fn unset_xlp(&mut self) {
        self.xlp = None;
    }

    /// Z-coordinate value for 3D layouts and displays. [Read more](https://graphviz.org/docs/attrs/z/).
    pub fn get_z(&self) -> Option<&str> {
        self.z.as_ref().map(|x| x.0.as_str())
    }

    /// Set `z` attribute. [Read more](https://graphviz.org/docs/attrs/z/).
    pub fn set_z(&mut self, s: &str) {
        self.z = Some(Z::new(s));
    }

    /// Unset `z` attribute. [Read more](https://graphviz.org/docs/attrs/z/).
    pub fn unset_z(&mut self) {
        self.z = None;
    }
}

impl IntoIterator for NodeAttrs {
    type Item = String;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Self::Item>, 49>>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.area.map(|x| x.to_string()),
            self.class.map(|x| x.to_string()),
            self.color.map(|x| x.to_string()),
            self.colorscheme.map(|x| x.to_string()),
            self.comment.map(|x| x.to_string()),
            self.distortion.map(|x| x.to_string()),
            self.fillcolor.map(|x| x.to_string()),
            self.fixedsize.map(|x| x.to_string()),
            self.fontcolor.map(|x| x.to_string()),
            self.fontname.map(|x| x.to_string()),
            self.fontsize.map(|x| x.to_string()),
            self.gradientangle.map(|x| x.to_string()),
            self.group.map(|x| x.to_string()),
            self.height.map(|x| x.to_string()),
            self.href.map(|x| x.to_string()),
            self.id.map(|x| x.to_string()),
            self.image.map(|x| x.to_string()),
            self.imagepos.map(|x| x.to_string()),
            self.imagescale.map(|x| x.to_string()),
            self.label.map(|x| x.to_string()),
            self.labelloc.map(|x| x.to_string()),
            self.layer.map(|x| x.to_string()),
            self.margin.map(|x| x.to_string()),
            self.nojustify.map(|x| x.to_string()),
            self.ordering.map(|x| x.to_string()),
            self.orientation.map(|x| x.to_string()),
            self.penwidth.map(|x| x.to_string()),
            self.peripheries.map(|x| x.to_string()),
            self.pin.map(|x| x.to_string()),
            self.pos.map(|x| x.to_string()),
            self.rects.map(|x| x.to_string()),
            self.regular.map(|x| x.to_string()),
            self.root.map(|x| x.to_string()),
            self.samplepoints.map(|x| x.to_string()),
            self.shape.map(|x| x.to_string()),
            self.shapefile.map(|x| x.to_string()),
            self.showboxes.map(|x| x.to_string()),
            self.sides.map(|x| x.to_string()),
            self.skew.map(|x| x.to_string()),
            self.sortv.map(|x| x.to_string()),
            self.style.map(|x| x.to_string()),
            self.target.map(|x| x.to_string()),
            self.tooltip.map(|x| x.to_string()),
            self.url.map(|x| x.to_string()),
            self.vertices.map(|x| x.to_string()),
            self.width.map(|x| x.to_string()),
            self.xlabel.map(|x| x.to_string()),
            self.xlp.map(|x| x.to_string()),
            self.z.map(|x| x.to_string()),
        ]
        .into_iter()
        .flatten()
    }
}

/// Cluster attributes.
#[derive(Clone, Debug, Default)]
pub struct ClusterAttrs {
    area: Option<Area>,
    bgcolor: Option<Bgcolor>,
    class: Option<Class>,
    cluster: Option<Cluster>,
    color: Option<Color>,
    colorscheme: Option<Colorscheme>,
    fillcolor: Option<Fillcolor>,
    fontcolor: Option<Fontcolor>,
    fontname: Option<Fontname>,
    fontsize: Option<Fontsize>,
    gradientangle: Option<Gradientangle>,
    href: Option<Href>,
    id: Option<Id>,
    k: Option<K>,
    label: Option<Label>,
    labeljust: Option<Labeljust>,
    labelloc: Option<Labelloc>,
    layer: Option<Layer>,
    lheight: Option<Lheight>,
    lp: Option<Lp>,
    lwidth: Option<Lwidth>,
    margin: Option<Margin>,
    nojustify: Option<Nojustify>,
    pencolor: Option<Pencolor>,
    penwidth: Option<Penwidth>,
    peripheries: Option<Peripheries>,
    sortv: Option<Sortv>,
    style: Option<Style>,
    target: Option<Target>,
    tooltip: Option<Tooltip>,
    url: Option<Url>,
}

impl ClusterAttrs {
    /// Indicates the preferred area for a node or empty cluster. patchwork only. [Read more](https://graphviz.org/docs/attrs/area/).
    pub fn get_area(&self) -> Option<&str> {
        self.area.as_ref().map(|x| x.0.as_str())
    }

    /// Set `area` attribute. [Read more](https://graphviz.org/docs/attrs/area/).
    pub fn set_area(&mut self, s: &str) {
        self.area = Some(Area::new(s));
    }

    /// Unset `area` attribute. [Read more](https://graphviz.org/docs/attrs/area/).
    pub fn unset_area(&mut self) {
        self.area = None;
    }

    /// Canvas background color. [Read more](https://graphviz.org/docs/attrs/bgcolor/).
    pub fn get_bgcolor(&self) -> Option<&str> {
        self.bgcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `bgcolor` attribute. [Read more](https://graphviz.org/docs/attrs/bgcolor/).
    pub fn set_bgcolor(&mut self, s: &str) {
        self.bgcolor = Some(Bgcolor::new(s));
    }

    /// Unset `bgcolor` attribute. [Read more](https://graphviz.org/docs/attrs/bgcolor/).
    pub fn unset_bgcolor(&mut self) {
        self.bgcolor = None;
    }

    /// Classnames to attach to the node, edge, graph, or cluster's SVG element. svg only. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn get_class(&self) -> Option<&str> {
        self.class.as_ref().map(|x| x.0.as_str())
    }

    /// Set `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn set_class(&mut self, s: &str) {
        self.class = Some(Class::new(s));
    }

    /// Unset `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn unset_class(&mut self) {
        self.class = None;
    }

    /// Whether the subgraph is a cluster. [Read more](https://graphviz.org/docs/attrs/cluster/).
    pub fn get_cluster(&self) -> Option<&str> {
        self.cluster.as_ref().map(|x| x.0.as_str())
    }

    /// Set `cluster` attribute. [Read more](https://graphviz.org/docs/attrs/cluster/).
    pub fn set_cluster(&mut self, s: &str) {
        self.cluster = Some(Cluster::new(s));
    }

    /// Unset `cluster` attribute. [Read more](https://graphviz.org/docs/attrs/cluster/).
    pub fn unset_cluster(&mut self) {
        self.cluster = None;
    }

    /// Basic drawing color for graphics, not text. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_ref().map(|x| x.0.as_str())
    }

    /// Set `color` attribute. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn set_color(&mut self, s: &str) {
        self.color = Some(Color::new(s));
    }

    /// Unset `color` attribute. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn unset_color(&mut self) {
        self.color = None;
    }

    /// A color scheme namespace: the context for interpreting color names. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn get_colorscheme(&self) -> Option<&str> {
        self.colorscheme.as_ref().map(|x| x.0.as_str())
    }

    /// Set `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn set_colorscheme(&mut self, s: &str) {
        self.colorscheme = Some(Colorscheme::new(s));
    }

    /// Unset `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn unset_colorscheme(&mut self) {
        self.colorscheme = None;
    }

    /// Color used to fill the background of a node or cluster. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn get_fillcolor(&self) -> Option<&str> {
        self.fillcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fillcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn set_fillcolor(&mut self, s: &str) {
        self.fillcolor = Some(Fillcolor::new(s));
    }

    /// Unset `fillcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn unset_fillcolor(&mut self) {
        self.fillcolor = None;
    }

    /// Color used for text. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn get_fontcolor(&self) -> Option<&str> {
        self.fontcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn set_fontcolor(&mut self, s: &str) {
        self.fontcolor = Some(Fontcolor::new(s));
    }

    /// Unset `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn unset_fontcolor(&mut self) {
        self.fontcolor = None;
    }

    /// Font used for text. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn get_fontname(&self) -> Option<&str> {
        self.fontname.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn set_fontname(&mut self, s: &str) {
        self.fontname = Some(Fontname::new(s));
    }

    /// Unset `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn unset_fontname(&mut self) {
        self.fontname = None;
    }

    /// Font size, in points, used for text. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn get_fontsize(&self) -> Option<&str> {
        self.fontsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn set_fontsize(&mut self, s: &str) {
        self.fontsize = Some(Fontsize::new(s));
    }

    /// Unset `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn unset_fontsize(&mut self) {
        self.fontsize = None;
    }

    /// If a gradient fill is being used, this determines the angle of the fill. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn get_gradientangle(&self) -> Option<&str> {
        self.gradientangle.as_ref().map(|x| x.0.as_str())
    }

    /// Set `gradientangle` attribute. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn set_gradientangle(&mut self, s: &str) {
        self.gradientangle = Some(Gradientangle::new(s));
    }

    /// Unset `gradientangle` attribute. [Read more](https://graphviz.org/docs/attrs/gradientangle/).
    pub fn unset_gradientangle(&mut self) {
        self.gradientangle = None;
    }

    /// Synonym for URL. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn get_href(&self) -> Option<&str> {
        self.href.as_ref().map(|x| x.0.as_str())
    }

    /// Set `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn set_href(&mut self, s: &str) {
        self.href = Some(Href::new(s));
    }

    /// Unset `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn unset_href(&mut self) {
        self.href = None;
    }

    /// Identifier for graph objects. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.0.as_str())
    }

    /// Set `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn set_id(&mut self, s: &str) {
        self.id = Some(Id::new(s));
    }

    /// Unset `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn unset_id(&mut self) {
        self.id = None;
    }

    /// Spring constant used in virtual physical model. fdp, sfdp only. [Read more](https://graphviz.org/docs/attrs/K/).
    pub fn get_k(&self) -> Option<&str> {
        self.k.as_ref().map(|x| x.0.as_str())
    }

    /// Set `k` attribute. [Read more](https://graphviz.org/docs/attrs/K/).
    pub fn set_k(&mut self, s: &str) {
        self.k = Some(K::new(s));
    }

    /// Unset `k` attribute. [Read more](https://graphviz.org/docs/attrs/K/).
    pub fn unset_k(&mut self) {
        self.k = None;
    }

    /// Text label attached to objects. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_ref().map(|x| x.0.as_str())
    }

    /// Set `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn set_label(&mut self, s: &str) {
        self.label = Some(Label::new(s));
    }

    /// Unset `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn unset_label(&mut self) {
        self.label = None;
    }

    /// Justification for graph & cluster labels. [Read more](https://graphviz.org/docs/attrs/labeljust/).
    pub fn get_labeljust(&self) -> Option<&str> {
        self.labeljust.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labeljust` attribute. [Read more](https://graphviz.org/docs/attrs/labeljust/).
    pub fn set_labeljust(&mut self, s: &str) {
        self.labeljust = Some(Labeljust::new(s));
    }

    /// Unset `labeljust` attribute. [Read more](https://graphviz.org/docs/attrs/labeljust/).
    pub fn unset_labeljust(&mut self) {
        self.labeljust = None;
    }

    /// Vertical placement of labels for nodes, root graphs and clusters. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn get_labelloc(&self) -> Option<&str> {
        self.labelloc.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelloc` attribute. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn set_labelloc(&mut self, s: &str) {
        self.labelloc = Some(Labelloc::new(s));
    }

    /// Unset `labelloc` attribute. [Read more](https://graphviz.org/docs/attrs/labelloc/).
    pub fn unset_labelloc(&mut self) {
        self.labelloc = None;
    }

    /// Specifies layers in which the node, edge or cluster is present. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn get_layer(&self) -> Option<&str> {
        self.layer.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layer` attribute. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn set_layer(&mut self, s: &str) {
        self.layer = Some(Layer::new(s));
    }

    /// Unset `layer` attribute. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn unset_layer(&mut self) {
        self.layer = None;
    }

    /// Height of graph or cluster label, in inches. write only. [Read more](https://graphviz.org/docs/attrs/lheight/).
    pub fn get_lheight(&self) -> Option<&str> {
        self.lheight.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lheight` attribute. [Read more](https://graphviz.org/docs/attrs/lheight/).
    pub fn set_lheight(&mut self, s: &str) {
        self.lheight = Some(Lheight::new(s));
    }

    /// Unset `lheight` attribute. [Read more](https://graphviz.org/docs/attrs/lheight/).
    pub fn unset_lheight(&mut self) {
        self.lheight = None;
    }

    /// Label center position. write only. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn get_lp(&self) -> Option<&str> {
        self.lp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lp` attribute. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn set_lp(&mut self, s: &str) {
        self.lp = Some(Lp::new(s));
    }

    /// Unset `lp` attribute. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn unset_lp(&mut self) {
        self.lp = None;
    }

    /// Width of graph or cluster label, in inches. write only. [Read more](https://graphviz.org/docs/attrs/lwidth/).
    pub fn get_lwidth(&self) -> Option<&str> {
        self.lwidth.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lwidth` attribute. [Read more](https://graphviz.org/docs/attrs/lwidth/).
    pub fn set_lwidth(&mut self, s: &str) {
        self.lwidth = Some(Lwidth::new(s));
    }

    /// Unset `lwidth` attribute. [Read more](https://graphviz.org/docs/attrs/lwidth/).
    pub fn unset_lwidth(&mut self) {
        self.lwidth = None;
    }

    /// For graphs, this sets x and y margins of canvas, in inches. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn get_margin(&self) -> Option<&str> {
        self.margin.as_ref().map(|x| x.0.as_str())
    }

    /// Set `margin` attribute. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn set_margin(&mut self, s: &str) {
        self.margin = Some(Margin::new(s));
    }

    /// Unset `margin` attribute. [Read more](https://graphviz.org/docs/attrs/margin/).
    pub fn unset_margin(&mut self) {
        self.margin = None;
    }

    /// Whether to justify multiline text vs the previous text line (rather than the side of the container).. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn get_nojustify(&self) -> Option<&str> {
        self.nojustify.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn set_nojustify(&mut self, s: &str) {
        self.nojustify = Some(Nojustify::new(s));
    }

    /// Unset `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn unset_nojustify(&mut self) {
        self.nojustify = None;
    }

    /// Color used to draw the bounding box around a cluster. [Read more](https://graphviz.org/docs/attrs/pencolor/).
    pub fn get_pencolor(&self) -> Option<&str> {
        self.pencolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pencolor` attribute. [Read more](https://graphviz.org/docs/attrs/pencolor/).
    pub fn set_pencolor(&mut self, s: &str) {
        self.pencolor = Some(Pencolor::new(s));
    }

    /// Unset `pencolor` attribute. [Read more](https://graphviz.org/docs/attrs/pencolor/).
    pub fn unset_pencolor(&mut self) {
        self.pencolor = None;
    }

    /// Specifies the width of the pen, in points, used to draw lines and curves. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn get_penwidth(&self) -> Option<&str> {
        self.penwidth.as_ref().map(|x| x.0.as_str())
    }

    /// Set `penwidth` attribute. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn set_penwidth(&mut self, s: &str) {
        self.penwidth = Some(Penwidth::new(s));
    }

    /// Unset `penwidth` attribute. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn unset_penwidth(&mut self) {
        self.penwidth = None;
    }

    /// Set number of peripheries used in polygonal shapes and cluster boundaries. [Read more](https://graphviz.org/docs/attrs/peripheries/).
    pub fn get_peripheries(&self) -> Option<&str> {
        self.peripheries.as_ref().map(|x| x.0.as_str())
    }

    /// Set `peripheries` attribute. [Read more](https://graphviz.org/docs/attrs/peripheries/).
    pub fn set_peripheries(&mut self, s: &str) {
        self.peripheries = Some(Peripheries::new(s));
    }

    /// Unset `peripheries` attribute. [Read more](https://graphviz.org/docs/attrs/peripheries/).
    pub fn unset_peripheries(&mut self) {
        self.peripheries = None;
    }

    /// Sort order of graph components for ordering packmode packing.. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn get_sortv(&self) -> Option<&str> {
        self.sortv.as_ref().map(|x| x.0.as_str())
    }

    /// Set `sortv` attribute. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn set_sortv(&mut self, s: &str) {
        self.sortv = Some(Sortv::new(s));
    }

    /// Unset `sortv` attribute. [Read more](https://graphviz.org/docs/attrs/sortv/).
    pub fn unset_sortv(&mut self) {
        self.sortv = None;
    }

    /// Set style information for components of the graph. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn get_style(&self) -> Option<&str> {
        self.style.as_ref().map(|x| x.0.as_str())
    }

    /// Set `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn set_style(&mut self, s: &str) {
        self.style = Some(Style::new(s));
    }

    /// Unset `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn unset_style(&mut self) {
        self.style = None;
    }

    /// If the object has a URL, this attribute determines which window of the browser is used for the URL.. map, svg only. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn get_target(&self) -> Option<&str> {
        self.target.as_ref().map(|x| x.0.as_str())
    }

    /// Set `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn set_target(&mut self, s: &str) {
        self.target = Some(Target::new(s));
    }

    /// Unset `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn unset_target(&mut self) {
        self.target = None;
    }

    /// Tooltip (mouse hover text) attached to the node, edge, cluster, or graph. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn get_tooltip(&self) -> Option<&str> {
        self.tooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn set_tooltip(&mut self, s: &str) {
        self.tooltip = Some(Tooltip::new(s));
    }

    /// Unset `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn unset_tooltip(&mut self) {
        self.tooltip = None;
    }

    /// Hyperlinks incorporated into device-dependent output. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_ref().map(|x| x.0.as_str())
    }

    /// Set `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn set_url(&mut self, s: &str) {
        self.url = Some(Url::new(s));
    }

    /// Unset `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn unset_url(&mut self) {
        self.url = None;
    }
}

impl IntoIterator for ClusterAttrs {
    type Item = String;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Self::Item>, 31>>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.area.map(|x| x.to_string()),
            self.bgcolor.map(|x| x.to_string()),
            self.class.map(|x| x.to_string()),
            self.cluster.map(|x| x.to_string()),
            self.color.map(|x| x.to_string()),
            self.colorscheme.map(|x| x.to_string()),
            self.fillcolor.map(|x| x.to_string()),
            self.fontcolor.map(|x| x.to_string()),
            self.fontname.map(|x| x.to_string()),
            self.fontsize.map(|x| x.to_string()),
            self.gradientangle.map(|x| x.to_string()),
            self.href.map(|x| x.to_string()),
            self.id.map(|x| x.to_string()),
            self.k.map(|x| x.to_string()),
            self.label.map(|x| x.to_string()),
            self.labeljust.map(|x| x.to_string()),
            self.labelloc.map(|x| x.to_string()),
            self.layer.map(|x| x.to_string()),
            self.lheight.map(|x| x.to_string()),
            self.lp.map(|x| x.to_string()),
            self.lwidth.map(|x| x.to_string()),
            self.margin.map(|x| x.to_string()),
            self.nojustify.map(|x| x.to_string()),
            self.pencolor.map(|x| x.to_string()),
            self.penwidth.map(|x| x.to_string()),
            self.peripheries.map(|x| x.to_string()),
            self.sortv.map(|x| x.to_string()),
            self.style.map(|x| x.to_string()),
            self.target.map(|x| x.to_string()),
            self.tooltip.map(|x| x.to_string()),
            self.url.map(|x| x.to_string()),
        ]
        .into_iter()
        .flatten()
    }
}

/// Edge attributes.
#[derive(Clone, Debug, Default)]
pub struct EdgeAttrs {
    arrowhead: Option<Arrowhead>,
    arrowsize: Option<Arrowsize>,
    arrowtail: Option<Arrowtail>,
    class: Option<Class>,
    color: Option<Color>,
    colorscheme: Option<Colorscheme>,
    comment: Option<Comment>,
    constraint: Option<Constraint>,
    decorate: Option<Decorate>,
    dir: Option<Dir>,
    edgehref: Option<Edgehref>,
    edgetarget: Option<Edgetarget>,
    edgetooltip: Option<Edgetooltip>,
    edgeurl: Option<Edgeurl>,
    fillcolor: Option<Fillcolor>,
    fontcolor: Option<Fontcolor>,
    fontname: Option<Fontname>,
    fontsize: Option<Fontsize>,
    head_lp: Option<HeadLp>,
    headclip: Option<Headclip>,
    headhref: Option<Headhref>,
    headlabel: Option<Headlabel>,
    headport: Option<Headport>,
    headtarget: Option<Headtarget>,
    headtooltip: Option<Headtooltip>,
    headurl: Option<Headurl>,
    href: Option<Href>,
    id: Option<Id>,
    label: Option<Label>,
    labelangle: Option<Labelangle>,
    labeldistance: Option<Labeldistance>,
    labelfloat: Option<Labelfloat>,
    labelfontcolor: Option<Labelfontcolor>,
    labelfontname: Option<Labelfontname>,
    labelfontsize: Option<Labelfontsize>,
    labelhref: Option<Labelhref>,
    labeltarget: Option<Labeltarget>,
    labeltooltip: Option<Labeltooltip>,
    labelurl: Option<Labelurl>,
    layer: Option<Layer>,
    len: Option<Len>,
    lhead: Option<Lhead>,
    lp: Option<Lp>,
    ltail: Option<Ltail>,
    minlen: Option<Minlen>,
    nojustify: Option<Nojustify>,
    penwidth: Option<Penwidth>,
    pos: Option<Pos>,
    samehead: Option<Samehead>,
    sametail: Option<Sametail>,
    showboxes: Option<Showboxes>,
    style: Option<Style>,
    tail_lp: Option<TailLp>,
    tailclip: Option<Tailclip>,
    tailhref: Option<Tailhref>,
    taillabel: Option<Taillabel>,
    tailport: Option<Tailport>,
    tailtarget: Option<Tailtarget>,
    tailtooltip: Option<Tailtooltip>,
    tailurl: Option<Tailurl>,
    target: Option<Target>,
    tooltip: Option<Tooltip>,
    url: Option<Url>,
    weight: Option<Weight>,
    xlabel: Option<Xlabel>,
    xlp: Option<Xlp>,
}

impl EdgeAttrs {
    /// Style of arrowhead on the head node of an edge. [Read more](https://graphviz.org/docs/attrs/arrowhead/).
    pub fn get_arrowhead(&self) -> Option<&str> {
        self.arrowhead.as_ref().map(|x| x.0.as_str())
    }

    /// Set `arrowhead` attribute. [Read more](https://graphviz.org/docs/attrs/arrowhead/).
    pub fn set_arrowhead(&mut self, s: &str) {
        self.arrowhead = Some(Arrowhead::new(s));
    }

    /// Unset `arrowhead` attribute. [Read more](https://graphviz.org/docs/attrs/arrowhead/).
    pub fn unset_arrowhead(&mut self) {
        self.arrowhead = None;
    }

    /// Multiplicative scale factor for arrowheads. [Read more](https://graphviz.org/docs/attrs/arrowsize/).
    pub fn get_arrowsize(&self) -> Option<&str> {
        self.arrowsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `arrowsize` attribute. [Read more](https://graphviz.org/docs/attrs/arrowsize/).
    pub fn set_arrowsize(&mut self, s: &str) {
        self.arrowsize = Some(Arrowsize::new(s));
    }

    /// Unset `arrowsize` attribute. [Read more](https://graphviz.org/docs/attrs/arrowsize/).
    pub fn unset_arrowsize(&mut self) {
        self.arrowsize = None;
    }

    /// Style of arrowhead on the tail node of an edge. [Read more](https://graphviz.org/docs/attrs/arrowtail/).
    pub fn get_arrowtail(&self) -> Option<&str> {
        self.arrowtail.as_ref().map(|x| x.0.as_str())
    }

    /// Set `arrowtail` attribute. [Read more](https://graphviz.org/docs/attrs/arrowtail/).
    pub fn set_arrowtail(&mut self, s: &str) {
        self.arrowtail = Some(Arrowtail::new(s));
    }

    /// Unset `arrowtail` attribute. [Read more](https://graphviz.org/docs/attrs/arrowtail/).
    pub fn unset_arrowtail(&mut self) {
        self.arrowtail = None;
    }

    /// Classnames to attach to the node, edge, graph, or cluster's SVG element. svg only. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn get_class(&self) -> Option<&str> {
        self.class.as_ref().map(|x| x.0.as_str())
    }

    /// Set `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn set_class(&mut self, s: &str) {
        self.class = Some(Class::new(s));
    }

    /// Unset `class` attribute. [Read more](https://graphviz.org/docs/attrs/class/).
    pub fn unset_class(&mut self) {
        self.class = None;
    }

    /// Basic drawing color for graphics, not text. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_ref().map(|x| x.0.as_str())
    }

    /// Set `color` attribute. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn set_color(&mut self, s: &str) {
        self.color = Some(Color::new(s));
    }

    /// Unset `color` attribute. [Read more](https://graphviz.org/docs/attrs/color/).
    pub fn unset_color(&mut self) {
        self.color = None;
    }

    /// A color scheme namespace: the context for interpreting color names. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn get_colorscheme(&self) -> Option<&str> {
        self.colorscheme.as_ref().map(|x| x.0.as_str())
    }

    /// Set `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn set_colorscheme(&mut self, s: &str) {
        self.colorscheme = Some(Colorscheme::new(s));
    }

    /// Unset `colorscheme` attribute. [Read more](https://graphviz.org/docs/attrs/colorscheme/).
    pub fn unset_colorscheme(&mut self) {
        self.colorscheme = None;
    }

    /// Comments are inserted into output. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|x| x.0.as_str())
    }

    /// Set `comment` attribute. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn set_comment(&mut self, s: &str) {
        self.comment = Some(Comment::new(s));
    }

    /// Unset `comment` attribute. [Read more](https://graphviz.org/docs/attrs/comment/).
    pub fn unset_comment(&mut self) {
        self.comment = None;
    }

    /// If false, the edge is not used in ranking the nodes. dot only. [Read more](https://graphviz.org/docs/attrs/constraint/).
    pub fn get_constraint(&self) -> Option<&str> {
        self.constraint.as_ref().map(|x| x.0.as_str())
    }

    /// Set `constraint` attribute. [Read more](https://graphviz.org/docs/attrs/constraint/).
    pub fn set_constraint(&mut self, s: &str) {
        self.constraint = Some(Constraint::new(s));
    }

    /// Unset `constraint` attribute. [Read more](https://graphviz.org/docs/attrs/constraint/).
    pub fn unset_constraint(&mut self) {
        self.constraint = None;
    }

    /// Whether to connect the edge label to the edge with a line. [Read more](https://graphviz.org/docs/attrs/decorate/).
    pub fn get_decorate(&self) -> Option<&str> {
        self.decorate.as_ref().map(|x| x.0.as_str())
    }

    /// Set `decorate` attribute. [Read more](https://graphviz.org/docs/attrs/decorate/).
    pub fn set_decorate(&mut self, s: &str) {
        self.decorate = Some(Decorate::new(s));
    }

    /// Unset `decorate` attribute. [Read more](https://graphviz.org/docs/attrs/decorate/).
    pub fn unset_decorate(&mut self) {
        self.decorate = None;
    }

    /// Edge type for drawing arrowheads. [Read more](https://graphviz.org/docs/attrs/dir/).
    pub fn get_dir(&self) -> Option<&str> {
        self.dir.as_ref().map(|x| x.0.as_str())
    }

    /// Set `dir` attribute. [Read more](https://graphviz.org/docs/attrs/dir/).
    pub fn set_dir(&mut self, s: &str) {
        self.dir = Some(Dir::new(s));
    }

    /// Unset `dir` attribute. [Read more](https://graphviz.org/docs/attrs/dir/).
    pub fn unset_dir(&mut self) {
        self.dir = None;
    }

    /// Synonym for edgeURL. map, svg only. [Read more](https://graphviz.org/docs/attrs/edgehref/).
    pub fn get_edgehref(&self) -> Option<&str> {
        self.edgehref.as_ref().map(|x| x.0.as_str())
    }

    /// Set `edgehref` attribute. [Read more](https://graphviz.org/docs/attrs/edgehref/).
    pub fn set_edgehref(&mut self, s: &str) {
        self.edgehref = Some(Edgehref::new(s));
    }

    /// Unset `edgehref` attribute. [Read more](https://graphviz.org/docs/attrs/edgehref/).
    pub fn unset_edgehref(&mut self) {
        self.edgehref = None;
    }

    /// Browser window to use for the edgeURL link. map, svg only. [Read more](https://graphviz.org/docs/attrs/edgetarget/).
    pub fn get_edgetarget(&self) -> Option<&str> {
        self.edgetarget.as_ref().map(|x| x.0.as_str())
    }

    /// Set `edgetarget` attribute. [Read more](https://graphviz.org/docs/attrs/edgetarget/).
    pub fn set_edgetarget(&mut self, s: &str) {
        self.edgetarget = Some(Edgetarget::new(s));
    }

    /// Unset `edgetarget` attribute. [Read more](https://graphviz.org/docs/attrs/edgetarget/).
    pub fn unset_edgetarget(&mut self) {
        self.edgetarget = None;
    }

    /// Tooltip annotation attached to the non-label part of an edge. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/edgetooltip/).
    pub fn get_edgetooltip(&self) -> Option<&str> {
        self.edgetooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `edgetooltip` attribute. [Read more](https://graphviz.org/docs/attrs/edgetooltip/).
    pub fn set_edgetooltip(&mut self, s: &str) {
        self.edgetooltip = Some(Edgetooltip::new(s));
    }

    /// Unset `edgetooltip` attribute. [Read more](https://graphviz.org/docs/attrs/edgetooltip/).
    pub fn unset_edgetooltip(&mut self) {
        self.edgetooltip = None;
    }

    /// The link for the non-label parts of an edge. map, svg only. [Read more](https://graphviz.org/docs/attrs/edgeURL/).
    pub fn get_edgeurl(&self) -> Option<&str> {
        self.edgeurl.as_ref().map(|x| x.0.as_str())
    }

    /// Set `edgeurl` attribute. [Read more](https://graphviz.org/docs/attrs/edgeURL/).
    pub fn set_edgeurl(&mut self, s: &str) {
        self.edgeurl = Some(Edgeurl::new(s));
    }

    /// Unset `edgeurl` attribute. [Read more](https://graphviz.org/docs/attrs/edgeURL/).
    pub fn unset_edgeurl(&mut self) {
        self.edgeurl = None;
    }

    /// Color used to fill the background of a node or cluster. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn get_fillcolor(&self) -> Option<&str> {
        self.fillcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fillcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn set_fillcolor(&mut self, s: &str) {
        self.fillcolor = Some(Fillcolor::new(s));
    }

    /// Unset `fillcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fillcolor/).
    pub fn unset_fillcolor(&mut self) {
        self.fillcolor = None;
    }

    /// Color used for text. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn get_fontcolor(&self) -> Option<&str> {
        self.fontcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn set_fontcolor(&mut self, s: &str) {
        self.fontcolor = Some(Fontcolor::new(s));
    }

    /// Unset `fontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/fontcolor/).
    pub fn unset_fontcolor(&mut self) {
        self.fontcolor = None;
    }

    /// Font used for text. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn get_fontname(&self) -> Option<&str> {
        self.fontname.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn set_fontname(&mut self, s: &str) {
        self.fontname = Some(Fontname::new(s));
    }

    /// Unset `fontname` attribute. [Read more](https://graphviz.org/docs/attrs/fontname/).
    pub fn unset_fontname(&mut self) {
        self.fontname = None;
    }

    /// Font size, in points, used for text. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn get_fontsize(&self) -> Option<&str> {
        self.fontsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn set_fontsize(&mut self, s: &str) {
        self.fontsize = Some(Fontsize::new(s));
    }

    /// Unset `fontsize` attribute. [Read more](https://graphviz.org/docs/attrs/fontsize/).
    pub fn unset_fontsize(&mut self) {
        self.fontsize = None;
    }

    /// Center position of an edge's head label. write only. [Read more](https://graphviz.org/docs/attrs/head_lp/).
    pub fn get_head_lp(&self) -> Option<&str> {
        self.head_lp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `head_lp` attribute. [Read more](https://graphviz.org/docs/attrs/head_lp/).
    pub fn set_head_lp(&mut self, s: &str) {
        self.head_lp = Some(HeadLp::new(s));
    }

    /// Unset `head_lp` attribute. [Read more](https://graphviz.org/docs/attrs/head_lp/).
    pub fn unset_head_lp(&mut self) {
        self.head_lp = None;
    }

    /// If true, the head of an edge is clipped to the boundary of the head node. [Read more](https://graphviz.org/docs/attrs/headclip/).
    pub fn get_headclip(&self) -> Option<&str> {
        self.headclip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headclip` attribute. [Read more](https://graphviz.org/docs/attrs/headclip/).
    pub fn set_headclip(&mut self, s: &str) {
        self.headclip = Some(Headclip::new(s));
    }

    /// Unset `headclip` attribute. [Read more](https://graphviz.org/docs/attrs/headclip/).
    pub fn unset_headclip(&mut self) {
        self.headclip = None;
    }

    /// Synonym for headURL. map, svg only. [Read more](https://graphviz.org/docs/attrs/headhref/).
    pub fn get_headhref(&self) -> Option<&str> {
        self.headhref.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headhref` attribute. [Read more](https://graphviz.org/docs/attrs/headhref/).
    pub fn set_headhref(&mut self, s: &str) {
        self.headhref = Some(Headhref::new(s));
    }

    /// Unset `headhref` attribute. [Read more](https://graphviz.org/docs/attrs/headhref/).
    pub fn unset_headhref(&mut self) {
        self.headhref = None;
    }

    /// Text label to be placed near head of edge. [Read more](https://graphviz.org/docs/attrs/headlabel/).
    pub fn get_headlabel(&self) -> Option<&str> {
        self.headlabel.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headlabel` attribute. [Read more](https://graphviz.org/docs/attrs/headlabel/).
    pub fn set_headlabel(&mut self, s: &str) {
        self.headlabel = Some(Headlabel::new(s));
    }

    /// Unset `headlabel` attribute. [Read more](https://graphviz.org/docs/attrs/headlabel/).
    pub fn unset_headlabel(&mut self) {
        self.headlabel = None;
    }

    /// Indicates where on the head node to attach the head of the edge. [Read more](https://graphviz.org/docs/attrs/headport/).
    pub fn get_headport(&self) -> Option<&str> {
        self.headport.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headport` attribute. [Read more](https://graphviz.org/docs/attrs/headport/).
    pub fn set_headport(&mut self, s: &str) {
        self.headport = Some(Headport::new(s));
    }

    /// Unset `headport` attribute. [Read more](https://graphviz.org/docs/attrs/headport/).
    pub fn unset_headport(&mut self) {
        self.headport = None;
    }

    /// Browser window to use for the headURL link. map, svg only. [Read more](https://graphviz.org/docs/attrs/headtarget/).
    pub fn get_headtarget(&self) -> Option<&str> {
        self.headtarget.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headtarget` attribute. [Read more](https://graphviz.org/docs/attrs/headtarget/).
    pub fn set_headtarget(&mut self, s: &str) {
        self.headtarget = Some(Headtarget::new(s));
    }

    /// Unset `headtarget` attribute. [Read more](https://graphviz.org/docs/attrs/headtarget/).
    pub fn unset_headtarget(&mut self) {
        self.headtarget = None;
    }

    /// Tooltip annotation attached to the head of an edge. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/headtooltip/).
    pub fn get_headtooltip(&self) -> Option<&str> {
        self.headtooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headtooltip` attribute. [Read more](https://graphviz.org/docs/attrs/headtooltip/).
    pub fn set_headtooltip(&mut self, s: &str) {
        self.headtooltip = Some(Headtooltip::new(s));
    }

    /// Unset `headtooltip` attribute. [Read more](https://graphviz.org/docs/attrs/headtooltip/).
    pub fn unset_headtooltip(&mut self) {
        self.headtooltip = None;
    }

    /// If defined, headURL is output as part of the head label of the edge. map, svg only. [Read more](https://graphviz.org/docs/attrs/headURL/).
    pub fn get_headurl(&self) -> Option<&str> {
        self.headurl.as_ref().map(|x| x.0.as_str())
    }

    /// Set `headurl` attribute. [Read more](https://graphviz.org/docs/attrs/headURL/).
    pub fn set_headurl(&mut self, s: &str) {
        self.headurl = Some(Headurl::new(s));
    }

    /// Unset `headurl` attribute. [Read more](https://graphviz.org/docs/attrs/headURL/).
    pub fn unset_headurl(&mut self) {
        self.headurl = None;
    }

    /// Synonym for URL. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn get_href(&self) -> Option<&str> {
        self.href.as_ref().map(|x| x.0.as_str())
    }

    /// Set `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn set_href(&mut self, s: &str) {
        self.href = Some(Href::new(s));
    }

    /// Unset `href` attribute. [Read more](https://graphviz.org/docs/attrs/href/).
    pub fn unset_href(&mut self) {
        self.href = None;
    }

    /// Identifier for graph objects. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.0.as_str())
    }

    /// Set `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn set_id(&mut self, s: &str) {
        self.id = Some(Id::new(s));
    }

    /// Unset `id` attribute. [Read more](https://graphviz.org/docs/attrs/id/).
    pub fn unset_id(&mut self) {
        self.id = None;
    }

    /// Text label attached to objects. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_ref().map(|x| x.0.as_str())
    }

    /// Set `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn set_label(&mut self, s: &str) {
        self.label = Some(Label::new(s));
    }

    /// Unset `label` attribute. [Read more](https://graphviz.org/docs/attrs/label/).
    pub fn unset_label(&mut self) {
        self.label = None;
    }

    /// The angle (in degrees) in polar coordinates of the head & tail edge labels.. [Read more](https://graphviz.org/docs/attrs/labelangle/).
    pub fn get_labelangle(&self) -> Option<&str> {
        self.labelangle.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelangle` attribute. [Read more](https://graphviz.org/docs/attrs/labelangle/).
    pub fn set_labelangle(&mut self, s: &str) {
        self.labelangle = Some(Labelangle::new(s));
    }

    /// Unset `labelangle` attribute. [Read more](https://graphviz.org/docs/attrs/labelangle/).
    pub fn unset_labelangle(&mut self) {
        self.labelangle = None;
    }

    /// Scaling factor for the distance of headlabel / taillabel from the head / tail nodes.. [Read more](https://graphviz.org/docs/attrs/labeldistance/).
    pub fn get_labeldistance(&self) -> Option<&str> {
        self.labeldistance.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labeldistance` attribute. [Read more](https://graphviz.org/docs/attrs/labeldistance/).
    pub fn set_labeldistance(&mut self, s: &str) {
        self.labeldistance = Some(Labeldistance::new(s));
    }

    /// Unset `labeldistance` attribute. [Read more](https://graphviz.org/docs/attrs/labeldistance/).
    pub fn unset_labeldistance(&mut self) {
        self.labeldistance = None;
    }

    /// If true, allows edge labels to be less constrained in position. [Read more](https://graphviz.org/docs/attrs/labelfloat/).
    pub fn get_labelfloat(&self) -> Option<&str> {
        self.labelfloat.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelfloat` attribute. [Read more](https://graphviz.org/docs/attrs/labelfloat/).
    pub fn set_labelfloat(&mut self, s: &str) {
        self.labelfloat = Some(Labelfloat::new(s));
    }

    /// Unset `labelfloat` attribute. [Read more](https://graphviz.org/docs/attrs/labelfloat/).
    pub fn unset_labelfloat(&mut self) {
        self.labelfloat = None;
    }

    /// Color used for headlabel and taillabel.. [Read more](https://graphviz.org/docs/attrs/labelfontcolor/).
    pub fn get_labelfontcolor(&self) -> Option<&str> {
        self.labelfontcolor.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelfontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/labelfontcolor/).
    pub fn set_labelfontcolor(&mut self, s: &str) {
        self.labelfontcolor = Some(Labelfontcolor::new(s));
    }

    /// Unset `labelfontcolor` attribute. [Read more](https://graphviz.org/docs/attrs/labelfontcolor/).
    pub fn unset_labelfontcolor(&mut self) {
        self.labelfontcolor = None;
    }

    /// Font for headlabel and taillabel. [Read more](https://graphviz.org/docs/attrs/labelfontname/).
    pub fn get_labelfontname(&self) -> Option<&str> {
        self.labelfontname.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelfontname` attribute. [Read more](https://graphviz.org/docs/attrs/labelfontname/).
    pub fn set_labelfontname(&mut self, s: &str) {
        self.labelfontname = Some(Labelfontname::new(s));
    }

    /// Unset `labelfontname` attribute. [Read more](https://graphviz.org/docs/attrs/labelfontname/).
    pub fn unset_labelfontname(&mut self) {
        self.labelfontname = None;
    }

    /// Font size of headlabel and taillabel. [Read more](https://graphviz.org/docs/attrs/labelfontsize/).
    pub fn get_labelfontsize(&self) -> Option<&str> {
        self.labelfontsize.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelfontsize` attribute. [Read more](https://graphviz.org/docs/attrs/labelfontsize/).
    pub fn set_labelfontsize(&mut self, s: &str) {
        self.labelfontsize = Some(Labelfontsize::new(s));
    }

    /// Unset `labelfontsize` attribute. [Read more](https://graphviz.org/docs/attrs/labelfontsize/).
    pub fn unset_labelfontsize(&mut self) {
        self.labelfontsize = None;
    }

    /// Synonym for labelURL. map, svg only. [Read more](https://graphviz.org/docs/attrs/labelhref/).
    pub fn get_labelhref(&self) -> Option<&str> {
        self.labelhref.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelhref` attribute. [Read more](https://graphviz.org/docs/attrs/labelhref/).
    pub fn set_labelhref(&mut self, s: &str) {
        self.labelhref = Some(Labelhref::new(s));
    }

    /// Unset `labelhref` attribute. [Read more](https://graphviz.org/docs/attrs/labelhref/).
    pub fn unset_labelhref(&mut self) {
        self.labelhref = None;
    }

    /// Browser window to open labelURL links in. map, svg only. [Read more](https://graphviz.org/docs/attrs/labeltarget/).
    pub fn get_labeltarget(&self) -> Option<&str> {
        self.labeltarget.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labeltarget` attribute. [Read more](https://graphviz.org/docs/attrs/labeltarget/).
    pub fn set_labeltarget(&mut self, s: &str) {
        self.labeltarget = Some(Labeltarget::new(s));
    }

    /// Unset `labeltarget` attribute. [Read more](https://graphviz.org/docs/attrs/labeltarget/).
    pub fn unset_labeltarget(&mut self) {
        self.labeltarget = None;
    }

    /// Tooltip annotation attached to label of an edge. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/labeltooltip/).
    pub fn get_labeltooltip(&self) -> Option<&str> {
        self.labeltooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labeltooltip` attribute. [Read more](https://graphviz.org/docs/attrs/labeltooltip/).
    pub fn set_labeltooltip(&mut self, s: &str) {
        self.labeltooltip = Some(Labeltooltip::new(s));
    }

    /// Unset `labeltooltip` attribute. [Read more](https://graphviz.org/docs/attrs/labeltooltip/).
    pub fn unset_labeltooltip(&mut self) {
        self.labeltooltip = None;
    }

    /// If defined, labelURL is the link used for the label of an edge. map, svg only. [Read more](https://graphviz.org/docs/attrs/labelURL/).
    pub fn get_labelurl(&self) -> Option<&str> {
        self.labelurl.as_ref().map(|x| x.0.as_str())
    }

    /// Set `labelurl` attribute. [Read more](https://graphviz.org/docs/attrs/labelURL/).
    pub fn set_labelurl(&mut self, s: &str) {
        self.labelurl = Some(Labelurl::new(s));
    }

    /// Unset `labelurl` attribute. [Read more](https://graphviz.org/docs/attrs/labelURL/).
    pub fn unset_labelurl(&mut self) {
        self.labelurl = None;
    }

    /// Specifies layers in which the node, edge or cluster is present. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn get_layer(&self) -> Option<&str> {
        self.layer.as_ref().map(|x| x.0.as_str())
    }

    /// Set `layer` attribute. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn set_layer(&mut self, s: &str) {
        self.layer = Some(Layer::new(s));
    }

    /// Unset `layer` attribute. [Read more](https://graphviz.org/docs/attrs/layer/).
    pub fn unset_layer(&mut self) {
        self.layer = None;
    }

    /// Preferred edge length, in inches. neato, fdp only. [Read more](https://graphviz.org/docs/attrs/len/).
    pub fn get_len(&self) -> Option<&str> {
        self.len.as_ref().map(|x| x.0.as_str())
    }

    /// Set `len` attribute. [Read more](https://graphviz.org/docs/attrs/len/).
    pub fn set_len(&mut self, s: &str) {
        self.len = Some(Len::new(s));
    }

    /// Unset `len` attribute. [Read more](https://graphviz.org/docs/attrs/len/).
    pub fn unset_len(&mut self) {
        self.len = None;
    }

    /// Logical head of an edge. dot only. [Read more](https://graphviz.org/docs/attrs/lhead/).
    pub fn get_lhead(&self) -> Option<&str> {
        self.lhead.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lhead` attribute. [Read more](https://graphviz.org/docs/attrs/lhead/).
    pub fn set_lhead(&mut self, s: &str) {
        self.lhead = Some(Lhead::new(s));
    }

    /// Unset `lhead` attribute. [Read more](https://graphviz.org/docs/attrs/lhead/).
    pub fn unset_lhead(&mut self) {
        self.lhead = None;
    }

    /// Label center position. write only. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn get_lp(&self) -> Option<&str> {
        self.lp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `lp` attribute. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn set_lp(&mut self, s: &str) {
        self.lp = Some(Lp::new(s));
    }

    /// Unset `lp` attribute. [Read more](https://graphviz.org/docs/attrs/lp/).
    pub fn unset_lp(&mut self) {
        self.lp = None;
    }

    /// Logical tail of an edge. dot only. [Read more](https://graphviz.org/docs/attrs/ltail/).
    pub fn get_ltail(&self) -> Option<&str> {
        self.ltail.as_ref().map(|x| x.0.as_str())
    }

    /// Set `ltail` attribute. [Read more](https://graphviz.org/docs/attrs/ltail/).
    pub fn set_ltail(&mut self, s: &str) {
        self.ltail = Some(Ltail::new(s));
    }

    /// Unset `ltail` attribute. [Read more](https://graphviz.org/docs/attrs/ltail/).
    pub fn unset_ltail(&mut self) {
        self.ltail = None;
    }

    /// Minimum edge length (rank difference between head and tail). dot only. [Read more](https://graphviz.org/docs/attrs/minlen/).
    pub fn get_minlen(&self) -> Option<&str> {
        self.minlen.as_ref().map(|x| x.0.as_str())
    }

    /// Set `minlen` attribute. [Read more](https://graphviz.org/docs/attrs/minlen/).
    pub fn set_minlen(&mut self, s: &str) {
        self.minlen = Some(Minlen::new(s));
    }

    /// Unset `minlen` attribute. [Read more](https://graphviz.org/docs/attrs/minlen/).
    pub fn unset_minlen(&mut self) {
        self.minlen = None;
    }

    /// Whether to justify multiline text vs the previous text line (rather than the side of the container).. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn get_nojustify(&self) -> Option<&str> {
        self.nojustify.as_ref().map(|x| x.0.as_str())
    }

    /// Set `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn set_nojustify(&mut self, s: &str) {
        self.nojustify = Some(Nojustify::new(s));
    }

    /// Unset `nojustify` attribute. [Read more](https://graphviz.org/docs/attrs/nojustify/).
    pub fn unset_nojustify(&mut self) {
        self.nojustify = None;
    }

    /// Specifies the width of the pen, in points, used to draw lines and curves. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn get_penwidth(&self) -> Option<&str> {
        self.penwidth.as_ref().map(|x| x.0.as_str())
    }

    /// Set `penwidth` attribute. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn set_penwidth(&mut self, s: &str) {
        self.penwidth = Some(Penwidth::new(s));
    }

    /// Unset `penwidth` attribute. [Read more](https://graphviz.org/docs/attrs/penwidth/).
    pub fn unset_penwidth(&mut self) {
        self.penwidth = None;
    }

    /// Position of node, or spline control points. neato, fdp only. [Read more](https://graphviz.org/docs/attrs/pos/).
    pub fn get_pos(&self) -> Option<&str> {
        self.pos.as_ref().map(|x| x.0.as_str())
    }

    /// Set `pos` attribute. [Read more](https://graphviz.org/docs/attrs/pos/).
    pub fn set_pos(&mut self, s: &str) {
        self.pos = Some(Pos::new(s));
    }

    /// Unset `pos` attribute. [Read more](https://graphviz.org/docs/attrs/pos/).
    pub fn unset_pos(&mut self) {
        self.pos = None;
    }

    /// Edges with the same head and the same samehead value are aimed at the same point on the head. dot only. [Read more](https://graphviz.org/docs/attrs/samehead/).
    pub fn get_samehead(&self) -> Option<&str> {
        self.samehead.as_ref().map(|x| x.0.as_str())
    }

    /// Set `samehead` attribute. [Read more](https://graphviz.org/docs/attrs/samehead/).
    pub fn set_samehead(&mut self, s: &str) {
        self.samehead = Some(Samehead::new(s));
    }

    /// Unset `samehead` attribute. [Read more](https://graphviz.org/docs/attrs/samehead/).
    pub fn unset_samehead(&mut self) {
        self.samehead = None;
    }

    /// Edges with the same tail and the same sametail value are aimed at th. dot only. [Read more](https://graphviz.org/docs/attrs/sametail/).
    pub fn get_sametail(&self) -> Option<&str> {
        self.sametail.as_ref().map(|x| x.0.as_str())
    }

    /// Set `sametail` attribute. [Read more](https://graphviz.org/docs/attrs/sametail/).
    pub fn set_sametail(&mut self, s: &str) {
        self.sametail = Some(Sametail::new(s));
    }

    /// Unset `sametail` attribute. [Read more](https://graphviz.org/docs/attrs/sametail/).
    pub fn unset_sametail(&mut self) {
        self.sametail = None;
    }

    /// Print guide boxes for debugging. dot only. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn get_showboxes(&self) -> Option<&str> {
        self.showboxes.as_ref().map(|x| x.0.as_str())
    }

    /// Set `showboxes` attribute. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn set_showboxes(&mut self, s: &str) {
        self.showboxes = Some(Showboxes::new(s));
    }

    /// Unset `showboxes` attribute. [Read more](https://graphviz.org/docs/attrs/showboxes/).
    pub fn unset_showboxes(&mut self) {
        self.showboxes = None;
    }

    /// Set style information for components of the graph. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn get_style(&self) -> Option<&str> {
        self.style.as_ref().map(|x| x.0.as_str())
    }

    /// Set `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn set_style(&mut self, s: &str) {
        self.style = Some(Style::new(s));
    }

    /// Unset `style` attribute. [Read more](https://graphviz.org/docs/attrs/style/).
    pub fn unset_style(&mut self) {
        self.style = None;
    }

    /// Position of an edge's tail label, in points.. write only. [Read more](https://graphviz.org/docs/attrs/tail_lp/).
    pub fn get_tail_lp(&self) -> Option<&str> {
        self.tail_lp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tail_lp` attribute. [Read more](https://graphviz.org/docs/attrs/tail_lp/).
    pub fn set_tail_lp(&mut self, s: &str) {
        self.tail_lp = Some(TailLp::new(s));
    }

    /// Unset `tail_lp` attribute. [Read more](https://graphviz.org/docs/attrs/tail_lp/).
    pub fn unset_tail_lp(&mut self) {
        self.tail_lp = None;
    }

    /// If true, the tail of an edge is clipped to the boundary of the tail node. [Read more](https://graphviz.org/docs/attrs/tailclip/).
    pub fn get_tailclip(&self) -> Option<&str> {
        self.tailclip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tailclip` attribute. [Read more](https://graphviz.org/docs/attrs/tailclip/).
    pub fn set_tailclip(&mut self, s: &str) {
        self.tailclip = Some(Tailclip::new(s));
    }

    /// Unset `tailclip` attribute. [Read more](https://graphviz.org/docs/attrs/tailclip/).
    pub fn unset_tailclip(&mut self) {
        self.tailclip = None;
    }

    /// Synonym for tailURL.. map, svg only. [Read more](https://graphviz.org/docs/attrs/tailhref/).
    pub fn get_tailhref(&self) -> Option<&str> {
        self.tailhref.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tailhref` attribute. [Read more](https://graphviz.org/docs/attrs/tailhref/).
    pub fn set_tailhref(&mut self, s: &str) {
        self.tailhref = Some(Tailhref::new(s));
    }

    /// Unset `tailhref` attribute. [Read more](https://graphviz.org/docs/attrs/tailhref/).
    pub fn unset_tailhref(&mut self) {
        self.tailhref = None;
    }

    /// Text label to be placed near tail of edge. [Read more](https://graphviz.org/docs/attrs/taillabel/).
    pub fn get_taillabel(&self) -> Option<&str> {
        self.taillabel.as_ref().map(|x| x.0.as_str())
    }

    /// Set `taillabel` attribute. [Read more](https://graphviz.org/docs/attrs/taillabel/).
    pub fn set_taillabel(&mut self, s: &str) {
        self.taillabel = Some(Taillabel::new(s));
    }

    /// Unset `taillabel` attribute. [Read more](https://graphviz.org/docs/attrs/taillabel/).
    pub fn unset_taillabel(&mut self) {
        self.taillabel = None;
    }

    /// Indicates where on the tail node to attach the tail of the edge. [Read more](https://graphviz.org/docs/attrs/tailport/).
    pub fn get_tailport(&self) -> Option<&str> {
        self.tailport.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tailport` attribute. [Read more](https://graphviz.org/docs/attrs/tailport/).
    pub fn set_tailport(&mut self, s: &str) {
        self.tailport = Some(Tailport::new(s));
    }

    /// Unset `tailport` attribute. [Read more](https://graphviz.org/docs/attrs/tailport/).
    pub fn unset_tailport(&mut self) {
        self.tailport = None;
    }

    /// Browser window to use for the tailURL link. map, svg only. [Read more](https://graphviz.org/docs/attrs/tailtarget/).
    pub fn get_tailtarget(&self) -> Option<&str> {
        self.tailtarget.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tailtarget` attribute. [Read more](https://graphviz.org/docs/attrs/tailtarget/).
    pub fn set_tailtarget(&mut self, s: &str) {
        self.tailtarget = Some(Tailtarget::new(s));
    }

    /// Unset `tailtarget` attribute. [Read more](https://graphviz.org/docs/attrs/tailtarget/).
    pub fn unset_tailtarget(&mut self) {
        self.tailtarget = None;
    }

    /// Tooltip annotation attached to the tail of an edge. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/tailtooltip/).
    pub fn get_tailtooltip(&self) -> Option<&str> {
        self.tailtooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tailtooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tailtooltip/).
    pub fn set_tailtooltip(&mut self, s: &str) {
        self.tailtooltip = Some(Tailtooltip::new(s));
    }

    /// Unset `tailtooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tailtooltip/).
    pub fn unset_tailtooltip(&mut self) {
        self.tailtooltip = None;
    }

    /// If defined, tailURL is output as part of the tail label of th. map, svg only. [Read more](https://graphviz.org/docs/attrs/tailURL/).
    pub fn get_tailurl(&self) -> Option<&str> {
        self.tailurl.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tailurl` attribute. [Read more](https://graphviz.org/docs/attrs/tailURL/).
    pub fn set_tailurl(&mut self, s: &str) {
        self.tailurl = Some(Tailurl::new(s));
    }

    /// Unset `tailurl` attribute. [Read more](https://graphviz.org/docs/attrs/tailURL/).
    pub fn unset_tailurl(&mut self) {
        self.tailurl = None;
    }

    /// If the object has a URL, this attribute determines which window of the browser is used for the URL.. map, svg only. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn get_target(&self) -> Option<&str> {
        self.target.as_ref().map(|x| x.0.as_str())
    }

    /// Set `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn set_target(&mut self, s: &str) {
        self.target = Some(Target::new(s));
    }

    /// Unset `target` attribute. [Read more](https://graphviz.org/docs/attrs/target/).
    pub fn unset_target(&mut self) {
        self.target = None;
    }

    /// Tooltip (mouse hover text) attached to the node, edge, cluster, or graph. cmap, svg only. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn get_tooltip(&self) -> Option<&str> {
        self.tooltip.as_ref().map(|x| x.0.as_str())
    }

    /// Set `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn set_tooltip(&mut self, s: &str) {
        self.tooltip = Some(Tooltip::new(s));
    }

    /// Unset `tooltip` attribute. [Read more](https://graphviz.org/docs/attrs/tooltip/).
    pub fn unset_tooltip(&mut self) {
        self.tooltip = None;
    }

    /// Hyperlinks incorporated into device-dependent output. map, postscript, svg only. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_ref().map(|x| x.0.as_str())
    }

    /// Set `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn set_url(&mut self, s: &str) {
        self.url = Some(Url::new(s));
    }

    /// Unset `url` attribute. [Read more](https://graphviz.org/docs/attrs/URL/).
    pub fn unset_url(&mut self) {
        self.url = None;
    }

    /// Weight of edge. [Read more](https://graphviz.org/docs/attrs/weight/).
    pub fn get_weight(&self) -> Option<&str> {
        self.weight.as_ref().map(|x| x.0.as_str())
    }

    /// Set `weight` attribute. [Read more](https://graphviz.org/docs/attrs/weight/).
    pub fn set_weight(&mut self, s: &str) {
        self.weight = Some(Weight::new(s));
    }

    /// Unset `weight` attribute. [Read more](https://graphviz.org/docs/attrs/weight/).
    pub fn unset_weight(&mut self) {
        self.weight = None;
    }

    /// External label for a node or edge. [Read more](https://graphviz.org/docs/attrs/xlabel/).
    pub fn get_xlabel(&self) -> Option<&str> {
        self.xlabel.as_ref().map(|x| x.0.as_str())
    }

    /// Set `xlabel` attribute. [Read more](https://graphviz.org/docs/attrs/xlabel/).
    pub fn set_xlabel(&mut self, s: &str) {
        self.xlabel = Some(Xlabel::new(s));
    }

    /// Unset `xlabel` attribute. [Read more](https://graphviz.org/docs/attrs/xlabel/).
    pub fn unset_xlabel(&mut self) {
        self.xlabel = None;
    }

    /// Position of an exterior label, in points. write only. [Read more](https://graphviz.org/docs/attrs/xlp/).
    pub fn get_xlp(&self) -> Option<&str> {
        self.xlp.as_ref().map(|x| x.0.as_str())
    }

    /// Set `xlp` attribute. [Read more](https://graphviz.org/docs/attrs/xlp/).
    pub fn set_xlp(&mut self, s: &str) {
        self.xlp = Some(Xlp::new(s));
    }

    /// Unset `xlp` attribute. [Read more](https://graphviz.org/docs/attrs/xlp/).
    pub fn unset_xlp(&mut self) {
        self.xlp = None;
    }
}

impl IntoIterator for EdgeAttrs {
    type Item = String;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Self::Item>, 66>>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.arrowhead.map(|x| x.to_string()),
            self.arrowsize.map(|x| x.to_string()),
            self.arrowtail.map(|x| x.to_string()),
            self.class.map(|x| x.to_string()),
            self.color.map(|x| x.to_string()),
            self.colorscheme.map(|x| x.to_string()),
            self.comment.map(|x| x.to_string()),
            self.constraint.map(|x| x.to_string()),
            self.decorate.map(|x| x.to_string()),
            self.dir.map(|x| x.to_string()),
            self.edgehref.map(|x| x.to_string()),
            self.edgetarget.map(|x| x.to_string()),
            self.edgetooltip.map(|x| x.to_string()),
            self.edgeurl.map(|x| x.to_string()),
            self.fillcolor.map(|x| x.to_string()),
            self.fontcolor.map(|x| x.to_string()),
            self.fontname.map(|x| x.to_string()),
            self.fontsize.map(|x| x.to_string()),
            self.head_lp.map(|x| x.to_string()),
            self.headclip.map(|x| x.to_string()),
            self.headhref.map(|x| x.to_string()),
            self.headlabel.map(|x| x.to_string()),
            self.headport.map(|x| x.to_string()),
            self.headtarget.map(|x| x.to_string()),
            self.headtooltip.map(|x| x.to_string()),
            self.headurl.map(|x| x.to_string()),
            self.href.map(|x| x.to_string()),
            self.id.map(|x| x.to_string()),
            self.label.map(|x| x.to_string()),
            self.labelangle.map(|x| x.to_string()),
            self.labeldistance.map(|x| x.to_string()),
            self.labelfloat.map(|x| x.to_string()),
            self.labelfontcolor.map(|x| x.to_string()),
            self.labelfontname.map(|x| x.to_string()),
            self.labelfontsize.map(|x| x.to_string()),
            self.labelhref.map(|x| x.to_string()),
            self.labeltarget.map(|x| x.to_string()),
            self.labeltooltip.map(|x| x.to_string()),
            self.labelurl.map(|x| x.to_string()),
            self.layer.map(|x| x.to_string()),
            self.len.map(|x| x.to_string()),
            self.lhead.map(|x| x.to_string()),
            self.lp.map(|x| x.to_string()),
            self.ltail.map(|x| x.to_string()),
            self.minlen.map(|x| x.to_string()),
            self.nojustify.map(|x| x.to_string()),
            self.penwidth.map(|x| x.to_string()),
            self.pos.map(|x| x.to_string()),
            self.samehead.map(|x| x.to_string()),
            self.sametail.map(|x| x.to_string()),
            self.showboxes.map(|x| x.to_string()),
            self.style.map(|x| x.to_string()),
            self.tail_lp.map(|x| x.to_string()),
            self.tailclip.map(|x| x.to_string()),
            self.tailhref.map(|x| x.to_string()),
            self.taillabel.map(|x| x.to_string()),
            self.tailport.map(|x| x.to_string()),
            self.tailtarget.map(|x| x.to_string()),
            self.tailtooltip.map(|x| x.to_string()),
            self.tailurl.map(|x| x.to_string()),
            self.target.map(|x| x.to_string()),
            self.tooltip.map(|x| x.to_string()),
            self.url.map(|x| x.to_string()),
            self.weight.map(|x| x.to_string()),
            self.xlabel.map(|x| x.to_string()),
            self.xlp.map(|x| x.to_string()),
        ]
        .into_iter()
        .flatten()
    }
}

/// Subgraph attributes.
#[derive(Clone, Debug, Default)]
pub struct SubgraphAttrs {
    cluster: Option<Cluster>,
    rank: Option<Rank>,
}

impl SubgraphAttrs {
    /// Whether the subgraph is a cluster. [Read more](https://graphviz.org/docs/attrs/cluster/).
    pub fn get_cluster(&self) -> Option<&str> {
        self.cluster.as_ref().map(|x| x.0.as_str())
    }

    /// Set `cluster` attribute. [Read more](https://graphviz.org/docs/attrs/cluster/).
    pub fn set_cluster(&mut self, s: &str) {
        self.cluster = Some(Cluster::new(s));
    }

    /// Unset `cluster` attribute. [Read more](https://graphviz.org/docs/attrs/cluster/).
    pub fn unset_cluster(&mut self) {
        self.cluster = None;
    }

    /// Rank constraints on the nodes in a subgraph. dot only. [Read more](https://graphviz.org/docs/attrs/rank/).
    pub fn get_rank(&self) -> Option<&str> {
        self.rank.as_ref().map(|x| x.0.as_str())
    }

    /// Set `rank` attribute. [Read more](https://graphviz.org/docs/attrs/rank/).
    pub fn set_rank(&mut self, s: &str) {
        self.rank = Some(Rank::new(s));
    }

    /// Unset `rank` attribute. [Read more](https://graphviz.org/docs/attrs/rank/).
    pub fn unset_rank(&mut self) {
        self.rank = None;
    }
}

impl IntoIterator for SubgraphAttrs {
    type Item = String;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Self::Item>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        [
            self.cluster.map(|x| x.to_string()),
            self.rank.map(|x| x.to_string()),
        ]
        .into_iter()
        .flatten()
    }
}
