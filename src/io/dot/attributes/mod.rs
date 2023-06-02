// Automatically generated on: 2023-02-01 07:56:16.127273 .

use std::hash::{Hash, Hasher};

use crate::types::FxIndexSet;

/// Quote string if necessary.
fn quote(s: &str) -> String {
    // Check if quoted and needs quoting.
    if !(s.starts_with('"') && s.ends_with('"')) && s.contains(' ') {
        // Add quoting to given string.
        return format!("\"{s}\"");
    }

    s.into()
}

/// Attribute enumerator.
#[derive(Clone, Debug)]
pub enum Attribute {
    /// Indicates the preferred area for a node or empty cluster. patchwork only. <a href="https://graphviz.org/docs/attrs/area/" target="_blank">Read more</a>.
    Area(String),
    /// Style of arrowhead on the head node of an edge. <a href="https://graphviz.org/docs/attrs/arrowhead/" target="_blank">Read more</a>.
    Arrowhead(String),
    /// Multiplicative scale factor for arrowheads. <a href="https://graphviz.org/docs/attrs/arrowsize/" target="_blank">Read more</a>.
    Arrowsize(String),
    /// Style of arrowhead on the tail node of an edge. <a href="https://graphviz.org/docs/attrs/arrowtail/" target="_blank">Read more</a>.
    Arrowtail(String),
    /// A string in the xdot format specifying an arbitrary background. <a href="https://graphviz.org/docs/attrs/background/" target="_blank">Read more</a>.
    Background(String),
    /// Bounding box of drawing in points. write only. <a href="https://graphviz.org/docs/attrs/bb/" target="_blank">Read more</a>.
    Bb(String),
    /// Whether to draw leaf nodes uniformly in a circle around the root node in sfdp.. sfdp only. <a href="https://graphviz.org/docs/attrs/beautify/" target="_blank">Read more</a>.
    Beautify(String),
    /// Canvas background color. <a href="https://graphviz.org/docs/attrs/bgcolor/" target="_blank">Read more</a>.
    Bgcolor(String),
    /// Whether to center the drawing in the output canvas. <a href="https://graphviz.org/docs/attrs/center/" target="_blank">Read more</a>.
    Center(String),
    /// Character encoding used when interpreting string input as a text label.. <a href="https://graphviz.org/docs/attrs/charset/" target="_blank">Read more</a>.
    Charset(String),
    /// Classnames to attach to the node, edge, graph, or cluster's SVG element. svg only. <a href="https://graphviz.org/docs/attrs/class/" target="_blank">Read more</a>.
    Class(String),
    /// Whether the subgraph is a cluster. <a href="https://graphviz.org/docs/attrs/cluster/" target="_blank">Read more</a>.
    Cluster(String),
    /// Mode used for handling clusters. dot only. <a href="https://graphviz.org/docs/attrs/clusterrank/" target="_blank">Read more</a>.
    Clusterrank(String),
    /// Basic drawing color for graphics, not text. <a href="https://graphviz.org/docs/attrs/color/" target="_blank">Read more</a>.
    Color(String),
    /// A color scheme namespace: the context for interpreting color names. <a href="https://graphviz.org/docs/attrs/colorscheme/" target="_blank">Read more</a>.
    Colorscheme(String),
    /// Comments are inserted into output. <a href="https://graphviz.org/docs/attrs/comment/" target="_blank">Read more</a>.
    Comment(String),
    /// If true, allow edges between clusters. dot only. <a href="https://graphviz.org/docs/attrs/compound/" target="_blank">Read more</a>.
    Compound(String),
    /// If true, use edge concentrators. <a href="https://graphviz.org/docs/attrs/concentrate/" target="_blank">Read more</a>.
    Concentrate(String),
    /// If false, the edge is not used in ranking the nodes. dot only. <a href="https://graphviz.org/docs/attrs/constraint/" target="_blank">Read more</a>.
    Constraint(String),
    /// Factor damping force motions.. neato only. <a href="https://graphviz.org/docs/attrs/Damping/" target="_blank">Read more</a>.
    Damping(String),
    /// Whether to connect the edge label to the edge with a line. <a href="https://graphviz.org/docs/attrs/decorate/" target="_blank">Read more</a>.
    Decorate(String),
    /// The distance between nodes in separate connected components. neato only. <a href="https://graphviz.org/docs/attrs/defaultdist/" target="_blank">Read more</a>.
    Defaultdist(String),
    /// Set the number of dimensions used for the layout. neato, fdp, sfdp only. <a href="https://graphviz.org/docs/attrs/dim/" target="_blank">Read more</a>.
    Dim(String),
    /// Set the number of dimensions used for rendering. neato, fdp, sfdp only. <a href="https://graphviz.org/docs/attrs/dimen/" target="_blank">Read more</a>.
    Dimen(String),
    /// Edge type for drawing arrowheads. <a href="https://graphviz.org/docs/attrs/dir/" target="_blank">Read more</a>.
    Dir(String),
    /// Whether to constrain most edges to point downwards. neato only. <a href="https://graphviz.org/docs/attrs/diredgeconstraints/" target="_blank">Read more</a>.
    Diredgeconstraints(String),
    /// Distortion factor for shape=polygon. <a href="https://graphviz.org/docs/attrs/distortion/" target="_blank">Read more</a>.
    Distortion(String),
    /// Specifies the expected number of pixels per inch on a display device. bitmap output, svg only. <a href="https://graphviz.org/docs/attrs/dpi/" target="_blank">Read more</a>.
    Dpi(String),
    /// Synonym for edgeURL. map, svg only. <a href="https://graphviz.org/docs/attrs/edgehref/" target="_blank">Read more</a>.
    Edgehref(String),
    /// Browser window to use for the edgeURL link. map, svg only. <a href="https://graphviz.org/docs/attrs/edgetarget/" target="_blank">Read more</a>.
    Edgetarget(String),
    /// Tooltip annotation attached to the non-label part of an edge. cmap, svg only. <a href="https://graphviz.org/docs/attrs/edgetooltip/" target="_blank">Read more</a>.
    Edgetooltip(String),
    /// The link for the non-label parts of an edge. map, svg only. <a href="https://graphviz.org/docs/attrs/edgeURL/" target="_blank">Read more</a>.
    Edgeurl(String),
    /// Terminating condition. neato only. <a href="https://graphviz.org/docs/attrs/epsilon/" target="_blank">Read more</a>.
    Epsilon(String),
    /// Margin used around polygons for purposes of spline edge routing. neato only. <a href="https://graphviz.org/docs/attrs/esep/" target="_blank">Read more</a>.
    Esep(String),
    /// Color used to fill the background of a node or cluster. <a href="https://graphviz.org/docs/attrs/fillcolor/" target="_blank">Read more</a>.
    Fillcolor(String),
    /// Whether to use the specified width and height attributes to choose node size (rather than sizing to fit the node contents). <a href="https://graphviz.org/docs/attrs/fixedsize/" target="_blank">Read more</a>.
    Fixedsize(String),
    /// Color used for text. <a href="https://graphviz.org/docs/attrs/fontcolor/" target="_blank">Read more</a>.
    Fontcolor(String),
    /// Font used for text. <a href="https://graphviz.org/docs/attrs/fontname/" target="_blank">Read more</a>.
    Fontname(String),
    /// Allows user control of how basic fontnames are represented in SVG output. svg only. <a href="https://graphviz.org/docs/attrs/fontnames/" target="_blank">Read more</a>.
    Fontnames(String),
    /// Directory list used by libgd to search for bitmap fonts. <a href="https://graphviz.org/docs/attrs/fontpath/" target="_blank">Read more</a>.
    Fontpath(String),
    /// Font size, in points, used for text. <a href="https://graphviz.org/docs/attrs/fontsize/" target="_blank">Read more</a>.
    Fontsize(String),
    /// Whether to force placement of all xlabels, even if overlapping. <a href="https://graphviz.org/docs/attrs/forcelabels/" target="_blank">Read more</a>.
    Forcelabels(String),
    /// If a gradient fill is being used, this determines the angle of the fill. <a href="https://graphviz.org/docs/attrs/gradientangle/" target="_blank">Read more</a>.
    Gradientangle(String),
    /// Name for a group of nodes, for bundling edges avoiding crossings.. dot only. <a href="https://graphviz.org/docs/attrs/group/" target="_blank">Read more</a>.
    Group(String),
    /// Center position of an edge's head label. write only. <a href="https://graphviz.org/docs/attrs/head_lp/" target="_blank">Read more</a>.
    HeadLp(String),
    /// If true, the head of an edge is clipped to the boundary of the head node. <a href="https://graphviz.org/docs/attrs/headclip/" target="_blank">Read more</a>.
    Headclip(String),
    /// Synonym for headURL. map, svg only. <a href="https://graphviz.org/docs/attrs/headhref/" target="_blank">Read more</a>.
    Headhref(String),
    /// Text label to be placed near head of edge. <a href="https://graphviz.org/docs/attrs/headlabel/" target="_blank">Read more</a>.
    Headlabel(String),
    /// Indicates where on the head node to attach the head of the edge. <a href="https://graphviz.org/docs/attrs/headport/" target="_blank">Read more</a>.
    Headport(String),
    /// Browser window to use for the headURL link. map, svg only. <a href="https://graphviz.org/docs/attrs/headtarget/" target="_blank">Read more</a>.
    Headtarget(String),
    /// Tooltip annotation attached to the head of an edge. cmap, svg only. <a href="https://graphviz.org/docs/attrs/headtooltip/" target="_blank">Read more</a>.
    Headtooltip(String),
    /// If defined, headURL is output as part of the head label of the edge. map, svg only. <a href="https://graphviz.org/docs/attrs/headURL/" target="_blank">Read more</a>.
    Headurl(String),
    /// Height of node, in inches. <a href="https://graphviz.org/docs/attrs/height/" target="_blank">Read more</a>.
    Height(String),
    /// Synonym for URL. map, postscript, svg only. <a href="https://graphviz.org/docs/attrs/href/" target="_blank">Read more</a>.
    Href(String),
    /// Identifier for graph objects. map, postscript, svg only. <a href="https://graphviz.org/docs/attrs/id/" target="_blank">Read more</a>.
    Id(String),
    /// Gives the name of a file containing an image to be displayed inside a node. <a href="https://graphviz.org/docs/attrs/image/" target="_blank">Read more</a>.
    Image(String),
    /// A list of directories in which to look for image files. <a href="https://graphviz.org/docs/attrs/imagepath/" target="_blank">Read more</a>.
    Imagepath(String),
    /// Controls how an image is positioned within its containing node. <a href="https://graphviz.org/docs/attrs/imagepos/" target="_blank">Read more</a>.
    Imagepos(String),
    /// Controls how an image fills its containing node. <a href="https://graphviz.org/docs/attrs/imagescale/" target="_blank">Read more</a>.
    Imagescale(String),
    /// Scales the input positions to convert between length units. neato, fdp only. <a href="https://graphviz.org/docs/attrs/inputscale/" target="_blank">Read more</a>.
    Inputscale(String),
    /// Spring constant used in virtual physical model. fdp, sfdp only. <a href="https://graphviz.org/docs/attrs/K/" target="_blank">Read more</a>.
    K(String),
    /// Text label attached to objects. <a href="https://graphviz.org/docs/attrs/label/" target="_blank">Read more</a>.
    Label(String),
    /// Whether to treat a node whose name has the form |edgelabel|* as a special node representing an edge label.. sfdp only. <a href="https://graphviz.org/docs/attrs/label_scheme/" target="_blank">Read more</a>.
    LabelScheme(String),
    /// The angle (in degrees) in polar coordinates of the head & tail edge labels.. <a href="https://graphviz.org/docs/attrs/labelangle/" target="_blank">Read more</a>.
    Labelangle(String),
    /// Scaling factor for the distance of headlabel / taillabel from the head / tail nodes.. <a href="https://graphviz.org/docs/attrs/labeldistance/" target="_blank">Read more</a>.
    Labeldistance(String),
    /// If true, allows edge labels to be less constrained in position. <a href="https://graphviz.org/docs/attrs/labelfloat/" target="_blank">Read more</a>.
    Labelfloat(String),
    /// Color used for headlabel and taillabel.. <a href="https://graphviz.org/docs/attrs/labelfontcolor/" target="_blank">Read more</a>.
    Labelfontcolor(String),
    /// Font for headlabel and taillabel. <a href="https://graphviz.org/docs/attrs/labelfontname/" target="_blank">Read more</a>.
    Labelfontname(String),
    /// Font size of headlabel and taillabel. <a href="https://graphviz.org/docs/attrs/labelfontsize/" target="_blank">Read more</a>.
    Labelfontsize(String),
    /// Synonym for labelURL. map, svg only. <a href="https://graphviz.org/docs/attrs/labelhref/" target="_blank">Read more</a>.
    Labelhref(String),
    /// Justification for graph & cluster labels. <a href="https://graphviz.org/docs/attrs/labeljust/" target="_blank">Read more</a>.
    Labeljust(String),
    /// Vertical placement of labels for nodes, root graphs and clusters. <a href="https://graphviz.org/docs/attrs/labelloc/" target="_blank">Read more</a>.
    Labelloc(String),
    /// Browser window to open labelURL links in. map, svg only. <a href="https://graphviz.org/docs/attrs/labeltarget/" target="_blank">Read more</a>.
    Labeltarget(String),
    /// Tooltip annotation attached to label of an edge. cmap, svg only. <a href="https://graphviz.org/docs/attrs/labeltooltip/" target="_blank">Read more</a>.
    Labeltooltip(String),
    /// If defined, labelURL is the link used for the label of an edge. map, svg only. <a href="https://graphviz.org/docs/attrs/labelURL/" target="_blank">Read more</a>.
    Labelurl(String),
    /// If true, the graph is rendered in landscape mode. <a href="https://graphviz.org/docs/attrs/landscape/" target="_blank">Read more</a>.
    Landscape(String),
    /// Specifies layers in which the node, edge or cluster is present. <a href="https://graphviz.org/docs/attrs/layer/" target="_blank">Read more</a>.
    Layer(String),
    /// The separator characters used to split attributes of type layerRange into a list of ranges.. <a href="https://graphviz.org/docs/attrs/layerlistsep/" target="_blank">Read more</a>.
    Layerlistsep(String),
    /// A linearly ordered list of layer names attached to the graph. <a href="https://graphviz.org/docs/attrs/layers/" target="_blank">Read more</a>.
    Layers(String),
    /// Selects a list of layers to be emitted. <a href="https://graphviz.org/docs/attrs/layerselect/" target="_blank">Read more</a>.
    Layerselect(String),
    /// The separator characters for splitting the layers attribute into a list of layer names.. <a href="https://graphviz.org/docs/attrs/layersep/" target="_blank">Read more</a>.
    Layersep(String),
    /// Which layout engine to use. <a href="https://graphviz.org/docs/attrs/layout/" target="_blank">Read more</a>.
    Layout(String),
    /// Preferred edge length, in inches. neato, fdp only. <a href="https://graphviz.org/docs/attrs/len/" target="_blank">Read more</a>.
    Len(String),
    /// Number of levels allowed in the multilevel scheme. sfdp only. <a href="https://graphviz.org/docs/attrs/levels/" target="_blank">Read more</a>.
    Levels(String),
    /// strictness of neato level constraints. neato only. <a href="https://graphviz.org/docs/attrs/levelsgap/" target="_blank">Read more</a>.
    Levelsgap(String),
    /// Logical head of an edge. dot only. <a href="https://graphviz.org/docs/attrs/lhead/" target="_blank">Read more</a>.
    Lhead(String),
    /// Height of graph or cluster label, in inches. write only. <a href="https://graphviz.org/docs/attrs/lheight/" target="_blank">Read more</a>.
    Lheight(String),
    /// How long strings should get before overflowing to next line, for text output.. <a href="https://graphviz.org/docs/attrs/linelength/" target="_blank">Read more</a>.
    Linelength(String),
    /// Label center position. write only. <a href="https://graphviz.org/docs/attrs/lp/" target="_blank">Read more</a>.
    Lp(String),
    /// Logical tail of an edge. dot only. <a href="https://graphviz.org/docs/attrs/ltail/" target="_blank">Read more</a>.
    Ltail(String),
    /// Width of graph or cluster label, in inches. write only. <a href="https://graphviz.org/docs/attrs/lwidth/" target="_blank">Read more</a>.
    Lwidth(String),
    /// For graphs, this sets x and y margins of canvas, in inches. <a href="https://graphviz.org/docs/attrs/margin/" target="_blank">Read more</a>.
    Margin(String),
    /// Sets the number of iterations used. neato, fdp only. <a href="https://graphviz.org/docs/attrs/maxiter/" target="_blank">Read more</a>.
    Maxiter(String),
    /// Scale factor for mincross (mc) edge crossing minimiser parameters. dot only. <a href="https://graphviz.org/docs/attrs/mclimit/" target="_blank">Read more</a>.
    Mclimit(String),
    /// Specifies the minimum separation between all nodes. circo only. <a href="https://graphviz.org/docs/attrs/mindist/" target="_blank">Read more</a>.
    Mindist(String),
    /// Minimum edge length (rank difference between head and tail). dot only. <a href="https://graphviz.org/docs/attrs/minlen/" target="_blank">Read more</a>.
    Minlen(String),
    /// Technique for optimizing the layout. neato only. <a href="https://graphviz.org/docs/attrs/mode/" target="_blank">Read more</a>.
    Mode(String),
    /// Specifies how the distance matrix is computed for the input graph. neato only. <a href="https://graphviz.org/docs/attrs/model/" target="_blank">Read more</a>.
    Model(String),
    /// Whether to use a single global ranking, ignoring clusters. dot only. <a href="https://graphviz.org/docs/attrs/newrank/" target="_blank">Read more</a>.
    Newrank(String),
    /// In dot, nodesep specifies the minimum space between two adjacent nodes in the same rank, in inches. <a href="https://graphviz.org/docs/attrs/nodesep/" target="_blank">Read more</a>.
    Nodesep(String),
    /// Whether to justify multiline text vs the previous text line (rather than the side of the container).. <a href="https://graphviz.org/docs/attrs/nojustify/" target="_blank">Read more</a>.
    Nojustify(String),
    /// normalizes coordinates of final layout. neato, fdp, sfdp, twopi, circo only. <a href="https://graphviz.org/docs/attrs/normalize/" target="_blank">Read more</a>.
    Normalize(String),
    /// Whether to avoid translating layout to the origin point. neato only. <a href="https://graphviz.org/docs/attrs/notranslate/" target="_blank">Read more</a>.
    Notranslate(String),
    /// Sets number of iterations in network simplex applications. dot only. <a href="https://graphviz.org/docs/attrs/nslimit/" target="_blank">Read more</a>.
    Nslimit(String),
    /// Sets number of iterations in network simplex applications. dot only. <a href="https://graphviz.org/docs/attrs/nslimit1/" target="_blank">Read more</a>.
    Nslimit1(String),
    /// Whether to draw circo graphs around one circle.. circo only. <a href="https://graphviz.org/docs/attrs/oneblock/" target="_blank">Read more</a>.
    Oneblock(String),
    /// Constrains the left-to-right ordering of node edges.. dot only. <a href="https://graphviz.org/docs/attrs/ordering/" target="_blank">Read more</a>.
    Ordering(String),
    /// node shape rotation angle, or graph orientation. <a href="https://graphviz.org/docs/attrs/orientation/" target="_blank">Read more</a>.
    Orientation(String),
    /// Specify order in which nodes and edges are drawn. <a href="https://graphviz.org/docs/attrs/outputorder/" target="_blank">Read more</a>.
    Outputorder(String),
    /// Determines if and how node overlaps should be removed. fdp, neato only. <a href="https://graphviz.org/docs/attrs/overlap/" target="_blank">Read more</a>.
    Overlap(String),
    /// Scale layout by factor, to reduce node overlap.. prism, neato, sfdp, fdp, circo, twopi only. <a href="https://graphviz.org/docs/attrs/overlap_scaling/" target="_blank">Read more</a>.
    OverlapScaling(String),
    /// Whether the overlap removal algorithm should perform a compression pass to reduce the size of the layout. prism only. <a href="https://graphviz.org/docs/attrs/overlap_shrink/" target="_blank">Read more</a>.
    OverlapShrink(String),
    /// Whether each connected component of the graph should be laid out separately, and then the graphs packed together.. <a href="https://graphviz.org/docs/attrs/pack/" target="_blank">Read more</a>.
    Pack(String),
    /// How connected components should be packed. <a href="https://graphviz.org/docs/attrs/packmode/" target="_blank">Read more</a>.
    Packmode(String),
    /// Inches to extend the drawing area around the minimal area needed to draw the graph. <a href="https://graphviz.org/docs/attrs/pad/" target="_blank">Read more</a>.
    Pad(String),
    /// Width and height of output pages, in inches. <a href="https://graphviz.org/docs/attrs/page/" target="_blank">Read more</a>.
    Page(String),
    /// The order in which pages are emitted. <a href="https://graphviz.org/docs/attrs/pagedir/" target="_blank">Read more</a>.
    Pagedir(String),
    /// Color used to draw the bounding box around a cluster. <a href="https://graphviz.org/docs/attrs/pencolor/" target="_blank">Read more</a>.
    Pencolor(String),
    /// Specifies the width of the pen, in points, used to draw lines and curves. <a href="https://graphviz.org/docs/attrs/penwidth/" target="_blank">Read more</a>.
    Penwidth(String),
    /// Set number of peripheries used in polygonal shapes and cluster boundaries. <a href="https://graphviz.org/docs/attrs/peripheries/" target="_blank">Read more</a>.
    Peripheries(String),
    /// Keeps the node at the node's given input position. neato, fdp only. <a href="https://graphviz.org/docs/attrs/pin/" target="_blank">Read more</a>.
    Pin(String),
    /// Position of node, or spline control points. neato, fdp only. <a href="https://graphviz.org/docs/attrs/pos/" target="_blank">Read more</a>.
    Pos(String),
    /// Quadtree scheme to use. sfdp only. <a href="https://graphviz.org/docs/attrs/quadtree/" target="_blank">Read more</a>.
    Quadtree(String),
    /// If quantum > 0.0, node label dimensions will be rounded to integral multiples of the quantum. <a href="https://graphviz.org/docs/attrs/quantum/" target="_blank">Read more</a>.
    Quantum(String),
    /// Rank constraints on the nodes in a subgraph. dot only. <a href="https://graphviz.org/docs/attrs/rank/" target="_blank">Read more</a>.
    Rank(String),
    /// Sets direction of graph layout. dot only. <a href="https://graphviz.org/docs/attrs/rankdir/" target="_blank">Read more</a>.
    Rankdir(String),
    /// Specifies separation between ranks. dot, twopi only. <a href="https://graphviz.org/docs/attrs/ranksep/" target="_blank">Read more</a>.
    Ranksep(String),
    /// Sets the aspect ratio (drawing height/drawing width) for the drawing. <a href="https://graphviz.org/docs/attrs/ratio/" target="_blank">Read more</a>.
    Ratio(String),
    /// Rectangles for fields of records, in points. write only. <a href="https://graphviz.org/docs/attrs/rects/" target="_blank">Read more</a>.
    Rects(String),
    /// If true, force polygon to be regular, i.e., the vertices of th. <a href="https://graphviz.org/docs/attrs/regular/" target="_blank">Read more</a>.
    Regular(String),
    /// If there are multiple clusters, whether to run edge crossing minimization a second time.. dot only. <a href="https://graphviz.org/docs/attrs/remincross/" target="_blank">Read more</a>.
    Remincross(String),
    /// The power of the repulsive force used in an extended Fruchterman-Reingold. sfdp only. <a href="https://graphviz.org/docs/attrs/repulsiveforce/" target="_blank">Read more</a>.
    Repulsiveforce(String),
    /// Synonym for dpi.. bitmap output, svg only. <a href="https://graphviz.org/docs/attrs/resolution/" target="_blank">Read more</a>.
    Resolution(String),
    /// Specifies nodes to be used as the center of the layout. twopi, circo only. <a href="https://graphviz.org/docs/attrs/root/" target="_blank">Read more</a>.
    Root(String),
    /// If rotate=90, sets drawing orientation to landscape. <a href="https://graphviz.org/docs/attrs/rotate/" target="_blank">Read more</a>.
    Rotate(String),
    /// Rotates the final layout counter-clockwise by the specified number of degrees. sfdp only. <a href="https://graphviz.org/docs/attrs/rotation/" target="_blank">Read more</a>.
    Rotation(String),
    /// Edges with the same head and the same samehead value are aimed at the same point on the head. dot only. <a href="https://graphviz.org/docs/attrs/samehead/" target="_blank">Read more</a>.
    Samehead(String),
    /// Edges with the same tail and the same sametail value are aimed at th. dot only. <a href="https://graphviz.org/docs/attrs/sametail/" target="_blank">Read more</a>.
    Sametail(String),
    /// Gives the number of points used for a circle/ellipse node. <a href="https://graphviz.org/docs/attrs/samplepoints/" target="_blank">Read more</a>.
    Samplepoints(String),
    /// Scales layout by the given factor after the initial layout. neato, twopi only. <a href="https://graphviz.org/docs/attrs/scale/" target="_blank">Read more</a>.
    Scale(String),
    /// During network simplex, the maximum number of edges with negative cut values to search when looking for an edge with minimum cut value.. dot only. <a href="https://graphviz.org/docs/attrs/searchsize/" target="_blank">Read more</a>.
    Searchsize(String),
    /// Margin to leave around nodes when removing node overlap. fdp, neato only. <a href="https://graphviz.org/docs/attrs/sep/" target="_blank">Read more</a>.
    Sep(String),
    /// Sets the shape of a node. <a href="https://graphviz.org/docs/attrs/shape/" target="_blank">Read more</a>.
    Shape(String),
    /// A file containing user-supplied node content. <a href="https://graphviz.org/docs/attrs/shapefile/" target="_blank">Read more</a>.
    Shapefile(String),
    /// Print guide boxes for debugging. dot only. <a href="https://graphviz.org/docs/attrs/showboxes/" target="_blank">Read more</a>.
    Showboxes(String),
    /// Number of sides when shape=polygon. <a href="https://graphviz.org/docs/attrs/sides/" target="_blank">Read more</a>.
    Sides(String),
    /// Maximum width and height of drawing, in inches. <a href="https://graphviz.org/docs/attrs/size/" target="_blank">Read more</a>.
    Size(String),
    /// Skew factor for shape=polygon. <a href="https://graphviz.org/docs/attrs/skew/" target="_blank">Read more</a>.
    Skew(String),
    /// Specifies a post-processing step used to smooth out an uneven distribution of nodes.. sfdp only. <a href="https://graphviz.org/docs/attrs/smoothing/" target="_blank">Read more</a>.
    Smoothing(String),
    /// Sort order of graph components for ordering packmode packing.. <a href="https://graphviz.org/docs/attrs/sortv/" target="_blank">Read more</a>.
    Sortv(String),
    /// Controls how, and if, edges are represented. <a href="https://graphviz.org/docs/attrs/splines/" target="_blank">Read more</a>.
    Splines(String),
    /// Parameter used to determine the initial layout of nodes. neato, fdp, sfdp only. <a href="https://graphviz.org/docs/attrs/start/" target="_blank">Read more</a>.
    Start(String),
    /// Set style information for components of the graph. <a href="https://graphviz.org/docs/attrs/style/" target="_blank">Read more</a>.
    Style(String),
    /// A URL or pathname specifying an XML style sheet, used in SVG output. svg only. <a href="https://graphviz.org/docs/attrs/stylesheet/" target="_blank">Read more</a>.
    Stylesheet(String),
    /// Position of an edge's tail label, in points.. write only. <a href="https://graphviz.org/docs/attrs/tail_lp/" target="_blank">Read more</a>.
    TailLp(String),
    /// If true, the tail of an edge is clipped to the boundary of the tail node. <a href="https://graphviz.org/docs/attrs/tailclip/" target="_blank">Read more</a>.
    Tailclip(String),
    /// Synonym for tailURL.. map, svg only. <a href="https://graphviz.org/docs/attrs/tailhref/" target="_blank">Read more</a>.
    Tailhref(String),
    /// Text label to be placed near tail of edge. <a href="https://graphviz.org/docs/attrs/taillabel/" target="_blank">Read more</a>.
    Taillabel(String),
    /// Indicates where on the tail node to attach the tail of the edge. <a href="https://graphviz.org/docs/attrs/tailport/" target="_blank">Read more</a>.
    Tailport(String),
    /// Browser window to use for the tailURL link. map, svg only. <a href="https://graphviz.org/docs/attrs/tailtarget/" target="_blank">Read more</a>.
    Tailtarget(String),
    /// Tooltip annotation attached to the tail of an edge. cmap, svg only. <a href="https://graphviz.org/docs/attrs/tailtooltip/" target="_blank">Read more</a>.
    Tailtooltip(String),
    /// If defined, tailURL is output as part of the tail label of th. map, svg only. <a href="https://graphviz.org/docs/attrs/tailURL/" target="_blank">Read more</a>.
    Tailurl(String),
    /// If the object has a URL, this attribute determines which window of the browser is used for the URL.. map, svg only. <a href="https://graphviz.org/docs/attrs/target/" target="_blank">Read more</a>.
    Target(String),
    /// Which rank to move floating (loose) nodes to. dot only. <a href="https://graphviz.org/docs/attrs/TBbalance/" target="_blank">Read more</a>.
    Tbbalance(String),
    /// Tooltip (mouse hover text) attached to the node, edge, cluster, or graph. cmap, svg only. <a href="https://graphviz.org/docs/attrs/tooltip/" target="_blank">Read more</a>.
    Tooltip(String),
    /// Whether internal bitmap rendering relies on a truecolor color model or uses. bitmap output only. <a href="https://graphviz.org/docs/attrs/truecolor/" target="_blank">Read more</a>.
    Truecolor(String),
    /// Hyperlinks incorporated into device-dependent output. map, postscript, svg only. <a href="https://graphviz.org/docs/attrs/URL/" target="_blank">Read more</a>.
    Url(String),
    /// Sets the coordinates of the vertices of the node's polygon, in inches. write only. <a href="https://graphviz.org/docs/attrs/vertices/" target="_blank">Read more</a>.
    Vertices(String),
    /// Clipping window on final drawing. <a href="https://graphviz.org/docs/attrs/viewport/" target="_blank">Read more</a>.
    Viewport(String),
    /// Tuning margin of Voronoi technique. neato, fdp, sfdp, twopi, circo only. <a href="https://graphviz.org/docs/attrs/voro_margin/" target="_blank">Read more</a>.
    VoroMargin(String),
    /// Weight of edge. <a href="https://graphviz.org/docs/attrs/weight/" target="_blank">Read more</a>.
    Weight(String),
    /// Width of node, in inches. <a href="https://graphviz.org/docs/attrs/width/" target="_blank">Read more</a>.
    Width(String),
    /// Determines the version of xdot used in output. xdot only. <a href="https://graphviz.org/docs/attrs/xdotversion/" target="_blank">Read more</a>.
    Xdotversion(String),
    /// External label for a node or edge. <a href="https://graphviz.org/docs/attrs/xlabel/" target="_blank">Read more</a>.
    Xlabel(String),
    /// Position of an exterior label, in points. write only. <a href="https://graphviz.org/docs/attrs/xlp/" target="_blank">Read more</a>.
    Xlp(String),
    /// Z-coordinate value for 3D layouts and displays. <a href="https://graphviz.org/docs/attrs/z/" target="_blank">Read more</a>.
    Z(String),
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        // Compare attributes based on their discriminant.
        std::mem::discriminant(self).eq(&std::mem::discriminant(other))
    }
}

impl Eq for Attribute {}

impl Hash for Attribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash attributes based on their discriminant.
        std::mem::discriminant(self).hash(state);
    }
}

impl From<Attribute> for (String, String) {
    fn from(attribute: Attribute) -> Self {
        let (key, value) = match attribute {
            Attribute::Area(x) => ("area", x),
            Attribute::Arrowhead(x) => ("arrowhead", x),
            Attribute::Arrowsize(x) => ("arrowsize", x),
            Attribute::Arrowtail(x) => ("arrowtail", x),
            Attribute::Background(x) => ("_background", x),
            Attribute::Bb(x) => ("bb", x),
            Attribute::Beautify(x) => ("beautify", x),
            Attribute::Bgcolor(x) => ("bgcolor", x),
            Attribute::Center(x) => ("center", x),
            Attribute::Charset(x) => ("charset", x),
            Attribute::Class(x) => ("class", x),
            Attribute::Cluster(x) => ("cluster", x),
            Attribute::Clusterrank(x) => ("clusterrank", x),
            Attribute::Color(x) => ("color", x),
            Attribute::Colorscheme(x) => ("colorscheme", x),
            Attribute::Comment(x) => ("comment", x),
            Attribute::Compound(x) => ("compound", x),
            Attribute::Concentrate(x) => ("concentrate", x),
            Attribute::Constraint(x) => ("constraint", x),
            Attribute::Damping(x) => ("Damping", x),
            Attribute::Decorate(x) => ("decorate", x),
            Attribute::Defaultdist(x) => ("defaultdist", x),
            Attribute::Dim(x) => ("dim", x),
            Attribute::Dimen(x) => ("dimen", x),
            Attribute::Dir(x) => ("dir", x),
            Attribute::Diredgeconstraints(x) => ("diredgeconstraints", x),
            Attribute::Distortion(x) => ("distortion", x),
            Attribute::Dpi(x) => ("dpi", x),
            Attribute::Edgehref(x) => ("edgehref", x),
            Attribute::Edgetarget(x) => ("edgetarget", x),
            Attribute::Edgetooltip(x) => ("edgetooltip", x),
            Attribute::Edgeurl(x) => ("edgeURL", x),
            Attribute::Epsilon(x) => ("epsilon", x),
            Attribute::Esep(x) => ("esep", x),
            Attribute::Fillcolor(x) => ("fillcolor", x),
            Attribute::Fixedsize(x) => ("fixedsize", x),
            Attribute::Fontcolor(x) => ("fontcolor", x),
            Attribute::Fontname(x) => ("fontname", x),
            Attribute::Fontnames(x) => ("fontnames", x),
            Attribute::Fontpath(x) => ("fontpath", x),
            Attribute::Fontsize(x) => ("fontsize", x),
            Attribute::Forcelabels(x) => ("forcelabels", x),
            Attribute::Gradientangle(x) => ("gradientangle", x),
            Attribute::Group(x) => ("group", x),
            Attribute::HeadLp(x) => ("head_lp", x),
            Attribute::Headclip(x) => ("headclip", x),
            Attribute::Headhref(x) => ("headhref", x),
            Attribute::Headlabel(x) => ("headlabel", x),
            Attribute::Headport(x) => ("headport", x),
            Attribute::Headtarget(x) => ("headtarget", x),
            Attribute::Headtooltip(x) => ("headtooltip", x),
            Attribute::Headurl(x) => ("headURL", x),
            Attribute::Height(x) => ("height", x),
            Attribute::Href(x) => ("href", x),
            Attribute::Id(x) => ("id", x),
            Attribute::Image(x) => ("image", x),
            Attribute::Imagepath(x) => ("imagepath", x),
            Attribute::Imagepos(x) => ("imagepos", x),
            Attribute::Imagescale(x) => ("imagescale", x),
            Attribute::Inputscale(x) => ("inputscale", x),
            Attribute::K(x) => ("K", x),
            Attribute::Label(x) => ("label", x),
            Attribute::LabelScheme(x) => ("label_scheme", x),
            Attribute::Labelangle(x) => ("labelangle", x),
            Attribute::Labeldistance(x) => ("labeldistance", x),
            Attribute::Labelfloat(x) => ("labelfloat", x),
            Attribute::Labelfontcolor(x) => ("labelfontcolor", x),
            Attribute::Labelfontname(x) => ("labelfontname", x),
            Attribute::Labelfontsize(x) => ("labelfontsize", x),
            Attribute::Labelhref(x) => ("labelhref", x),
            Attribute::Labeljust(x) => ("labeljust", x),
            Attribute::Labelloc(x) => ("labelloc", x),
            Attribute::Labeltarget(x) => ("labeltarget", x),
            Attribute::Labeltooltip(x) => ("labeltooltip", x),
            Attribute::Labelurl(x) => ("labelURL", x),
            Attribute::Landscape(x) => ("landscape", x),
            Attribute::Layer(x) => ("layer", x),
            Attribute::Layerlistsep(x) => ("layerlistsep", x),
            Attribute::Layers(x) => ("layers", x),
            Attribute::Layerselect(x) => ("layerselect", x),
            Attribute::Layersep(x) => ("layersep", x),
            Attribute::Layout(x) => ("layout", x),
            Attribute::Len(x) => ("len", x),
            Attribute::Levels(x) => ("levels", x),
            Attribute::Levelsgap(x) => ("levelsgap", x),
            Attribute::Lhead(x) => ("lhead", x),
            Attribute::Lheight(x) => ("lheight", x),
            Attribute::Linelength(x) => ("linelength", x),
            Attribute::Lp(x) => ("lp", x),
            Attribute::Ltail(x) => ("ltail", x),
            Attribute::Lwidth(x) => ("lwidth", x),
            Attribute::Margin(x) => ("margin", x),
            Attribute::Maxiter(x) => ("maxiter", x),
            Attribute::Mclimit(x) => ("mclimit", x),
            Attribute::Mindist(x) => ("mindist", x),
            Attribute::Minlen(x) => ("minlen", x),
            Attribute::Mode(x) => ("mode", x),
            Attribute::Model(x) => ("model", x),
            Attribute::Newrank(x) => ("newrank", x),
            Attribute::Nodesep(x) => ("nodesep", x),
            Attribute::Nojustify(x) => ("nojustify", x),
            Attribute::Normalize(x) => ("normalize", x),
            Attribute::Notranslate(x) => ("notranslate", x),
            Attribute::Nslimit(x) => ("nslimit", x),
            Attribute::Nslimit1(x) => ("nslimit1", x),
            Attribute::Oneblock(x) => ("oneblock", x),
            Attribute::Ordering(x) => ("ordering", x),
            Attribute::Orientation(x) => ("orientation", x),
            Attribute::Outputorder(x) => ("outputorder", x),
            Attribute::Overlap(x) => ("overlap", x),
            Attribute::OverlapScaling(x) => ("overlap_scaling", x),
            Attribute::OverlapShrink(x) => ("overlap_shrink", x),
            Attribute::Pack(x) => ("pack", x),
            Attribute::Packmode(x) => ("packmode", x),
            Attribute::Pad(x) => ("pad", x),
            Attribute::Page(x) => ("page", x),
            Attribute::Pagedir(x) => ("pagedir", x),
            Attribute::Pencolor(x) => ("pencolor", x),
            Attribute::Penwidth(x) => ("penwidth", x),
            Attribute::Peripheries(x) => ("peripheries", x),
            Attribute::Pin(x) => ("pin", x),
            Attribute::Pos(x) => ("pos", x),
            Attribute::Quadtree(x) => ("quadtree", x),
            Attribute::Quantum(x) => ("quantum", x),
            Attribute::Rank(x) => ("rank", x),
            Attribute::Rankdir(x) => ("rankdir", x),
            Attribute::Ranksep(x) => ("ranksep", x),
            Attribute::Ratio(x) => ("ratio", x),
            Attribute::Rects(x) => ("rects", x),
            Attribute::Regular(x) => ("regular", x),
            Attribute::Remincross(x) => ("remincross", x),
            Attribute::Repulsiveforce(x) => ("repulsiveforce", x),
            Attribute::Resolution(x) => ("resolution", x),
            Attribute::Root(x) => ("root", x),
            Attribute::Rotate(x) => ("rotate", x),
            Attribute::Rotation(x) => ("rotation", x),
            Attribute::Samehead(x) => ("samehead", x),
            Attribute::Sametail(x) => ("sametail", x),
            Attribute::Samplepoints(x) => ("samplepoints", x),
            Attribute::Scale(x) => ("scale", x),
            Attribute::Searchsize(x) => ("searchsize", x),
            Attribute::Sep(x) => ("sep", x),
            Attribute::Shape(x) => ("shape", x),
            Attribute::Shapefile(x) => ("shapefile", x),
            Attribute::Showboxes(x) => ("showboxes", x),
            Attribute::Sides(x) => ("sides", x),
            Attribute::Size(x) => ("size", x),
            Attribute::Skew(x) => ("skew", x),
            Attribute::Smoothing(x) => ("smoothing", x),
            Attribute::Sortv(x) => ("sortv", x),
            Attribute::Splines(x) => ("splines", x),
            Attribute::Start(x) => ("start", x),
            Attribute::Style(x) => ("style", x),
            Attribute::Stylesheet(x) => ("stylesheet", x),
            Attribute::TailLp(x) => ("tail_lp", x),
            Attribute::Tailclip(x) => ("tailclip", x),
            Attribute::Tailhref(x) => ("tailhref", x),
            Attribute::Taillabel(x) => ("taillabel", x),
            Attribute::Tailport(x) => ("tailport", x),
            Attribute::Tailtarget(x) => ("tailtarget", x),
            Attribute::Tailtooltip(x) => ("tailtooltip", x),
            Attribute::Tailurl(x) => ("tailURL", x),
            Attribute::Target(x) => ("target", x),
            Attribute::Tbbalance(x) => ("TBbalance", x),
            Attribute::Tooltip(x) => ("tooltip", x),
            Attribute::Truecolor(x) => ("truecolor", x),
            Attribute::Url(x) => ("URL", x),
            Attribute::Vertices(x) => ("vertices", x),
            Attribute::Viewport(x) => ("viewport", x),
            Attribute::VoroMargin(x) => ("voro_margin", x),
            Attribute::Weight(x) => ("weight", x),
            Attribute::Width(x) => ("width", x),
            Attribute::Xdotversion(x) => ("xdotversion", x),
            Attribute::Xlabel(x) => ("xlabel", x),
            Attribute::Xlp(x) => ("xlp", x),
            Attribute::Z(x) => ("z", x),
        };

        (key.into(), value)
    }
}

/// Graph attributes.
#[derive(Clone, Debug, Default)]
pub struct GraphAttributes {
    attributes: FxIndexSet<Attribute>,
}

impl GraphAttributes {
    /// Set attribute from `key` and `value` raw parts. Returns whether the attribute was newly set.
    ///
    /// # Panics
    ///
    /// Key is not valid for this attributes set. <a href="https://graphviz.org/doc/info/attrs.html#h:uses" target="_blank">Read more</a>.
    ///
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) -> bool {
        let value = quote(value);
        let item = match key {
            "_background" => Attribute::Background(value),
            "bb" => Attribute::Bb(value),
            "beautify" => Attribute::Beautify(value),
            "bgcolor" => Attribute::Bgcolor(value),
            "center" => Attribute::Center(value),
            "charset" => Attribute::Charset(value),
            "class" => Attribute::Class(value),
            "clusterrank" => Attribute::Clusterrank(value),
            "colorscheme" => Attribute::Colorscheme(value),
            "comment" => Attribute::Comment(value),
            "compound" => Attribute::Compound(value),
            "concentrate" => Attribute::Concentrate(value),
            "Damping" => Attribute::Damping(value),
            "defaultdist" => Attribute::Defaultdist(value),
            "dim" => Attribute::Dim(value),
            "dimen" => Attribute::Dimen(value),
            "diredgeconstraints" => Attribute::Diredgeconstraints(value),
            "dpi" => Attribute::Dpi(value),
            "epsilon" => Attribute::Epsilon(value),
            "esep" => Attribute::Esep(value),
            "fontcolor" => Attribute::Fontcolor(value),
            "fontname" => Attribute::Fontname(value),
            "fontnames" => Attribute::Fontnames(value),
            "fontpath" => Attribute::Fontpath(value),
            "fontsize" => Attribute::Fontsize(value),
            "forcelabels" => Attribute::Forcelabels(value),
            "gradientangle" => Attribute::Gradientangle(value),
            "href" => Attribute::Href(value),
            "id" => Attribute::Id(value),
            "imagepath" => Attribute::Imagepath(value),
            "inputscale" => Attribute::Inputscale(value),
            "K" => Attribute::K(value),
            "label" => Attribute::Label(value),
            "label_scheme" => Attribute::LabelScheme(value),
            "labeljust" => Attribute::Labeljust(value),
            "labelloc" => Attribute::Labelloc(value),
            "landscape" => Attribute::Landscape(value),
            "layerlistsep" => Attribute::Layerlistsep(value),
            "layers" => Attribute::Layers(value),
            "layerselect" => Attribute::Layerselect(value),
            "layersep" => Attribute::Layersep(value),
            "layout" => Attribute::Layout(value),
            "levels" => Attribute::Levels(value),
            "levelsgap" => Attribute::Levelsgap(value),
            "lheight" => Attribute::Lheight(value),
            "linelength" => Attribute::Linelength(value),
            "lp" => Attribute::Lp(value),
            "lwidth" => Attribute::Lwidth(value),
            "margin" => Attribute::Margin(value),
            "maxiter" => Attribute::Maxiter(value),
            "mclimit" => Attribute::Mclimit(value),
            "mindist" => Attribute::Mindist(value),
            "mode" => Attribute::Mode(value),
            "model" => Attribute::Model(value),
            "newrank" => Attribute::Newrank(value),
            "nodesep" => Attribute::Nodesep(value),
            "nojustify" => Attribute::Nojustify(value),
            "normalize" => Attribute::Normalize(value),
            "notranslate" => Attribute::Notranslate(value),
            "nslimit" => Attribute::Nslimit(value),
            "nslimit1" => Attribute::Nslimit1(value),
            "oneblock" => Attribute::Oneblock(value),
            "ordering" => Attribute::Ordering(value),
            "orientation" => Attribute::Orientation(value),
            "outputorder" => Attribute::Outputorder(value),
            "overlap" => Attribute::Overlap(value),
            "overlap_scaling" => Attribute::OverlapScaling(value),
            "overlap_shrink" => Attribute::OverlapShrink(value),
            "pack" => Attribute::Pack(value),
            "packmode" => Attribute::Packmode(value),
            "pad" => Attribute::Pad(value),
            "page" => Attribute::Page(value),
            "pagedir" => Attribute::Pagedir(value),
            "quadtree" => Attribute::Quadtree(value),
            "quantum" => Attribute::Quantum(value),
            "rankdir" => Attribute::Rankdir(value),
            "ranksep" => Attribute::Ranksep(value),
            "ratio" => Attribute::Ratio(value),
            "remincross" => Attribute::Remincross(value),
            "repulsiveforce" => Attribute::Repulsiveforce(value),
            "resolution" => Attribute::Resolution(value),
            "root" => Attribute::Root(value),
            "rotate" => Attribute::Rotate(value),
            "rotation" => Attribute::Rotation(value),
            "scale" => Attribute::Scale(value),
            "searchsize" => Attribute::Searchsize(value),
            "sep" => Attribute::Sep(value),
            "showboxes" => Attribute::Showboxes(value),
            "size" => Attribute::Size(value),
            "smoothing" => Attribute::Smoothing(value),
            "sortv" => Attribute::Sortv(value),
            "splines" => Attribute::Splines(value),
            "start" => Attribute::Start(value),
            "style" => Attribute::Style(value),
            "stylesheet" => Attribute::Stylesheet(value),
            "target" => Attribute::Target(value),
            "TBbalance" => Attribute::Tbbalance(value),
            "tooltip" => Attribute::Tooltip(value),
            "truecolor" => Attribute::Truecolor(value),
            "URL" => Attribute::Url(value),
            "viewport" => Attribute::Viewport(value),
            "voro_margin" => Attribute::VoroMargin(value),
            "xdotversion" => Attribute::Xdotversion(value),
            _ => panic!("Invalid attribute key `{key}` for GraphAttributes"),
        };

        self.attributes.replace(item).is_none()
    }

    /// Get attributes length.
    #[inline]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Check if attributes is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Set [`Attribute::Background`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_background(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Background(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Background`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_background(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Background(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Bb`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_bb(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Bb(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Bb`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_bb(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Bb(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Beautify`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_beautify(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Beautify(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Beautify`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_beautify(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Beautify(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Bgcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_bgcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Bgcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Bgcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_bgcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Bgcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Center`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_center(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Center(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Center`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_center(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Center(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Charset`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_charset(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Charset(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Charset`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_charset(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Charset(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Class`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_class(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Class(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Class`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_class(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Class(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Clusterrank`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_clusterrank(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Clusterrank(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Clusterrank`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_clusterrank(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Clusterrank(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Colorscheme`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_colorscheme(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Colorscheme(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Colorscheme`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_colorscheme(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Colorscheme(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Comment`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_comment(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Comment(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Comment`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_comment(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Comment(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Compound`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_compound(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Compound(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Compound`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_compound(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Compound(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Concentrate`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_concentrate(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Concentrate(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Concentrate`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_concentrate(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Concentrate(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Damping`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_damping(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Damping(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Damping`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_damping(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Damping(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Defaultdist`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_defaultdist(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Defaultdist(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Defaultdist`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_defaultdist(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Defaultdist(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Dim`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_dim(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Dim(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Dim`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_dim(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Dim(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Dimen`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_dimen(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Dimen(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Dimen`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_dimen(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Dimen(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Diredgeconstraints`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_diredgeconstraints(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Diredgeconstraints(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Diredgeconstraints`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_diredgeconstraints(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Diredgeconstraints(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Dpi`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_dpi(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Dpi(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Dpi`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_dpi(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Dpi(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Epsilon`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_epsilon(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Epsilon(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Epsilon`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_epsilon(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Epsilon(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Esep`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_esep(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Esep(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Esep`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_esep(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Esep(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontname`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontname(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontname(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontname`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontname(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontname(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontnames`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontnames(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontnames(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontnames`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontnames(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontnames(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontpath`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontpath(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontpath(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontpath`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontpath(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontpath(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Forcelabels`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_forcelabels(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Forcelabels(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Forcelabels`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_forcelabels(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Forcelabels(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Gradientangle`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_gradientangle(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Gradientangle(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Gradientangle`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_gradientangle(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Gradientangle(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Href`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_href(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Href(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Href`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_href(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Href(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Id`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_id(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Id(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Id`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_id(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Id(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Imagepath`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_imagepath(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Imagepath(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Imagepath`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_imagepath(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Imagepath(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Inputscale`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_inputscale(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Inputscale(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Inputscale`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_inputscale(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Inputscale(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::K`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_k(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::K(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::K`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_k(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::K(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Label`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_label(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Label(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Label`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_label(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Label(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::LabelScheme`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_label_scheme(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::LabelScheme(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::LabelScheme`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_label_scheme(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::LabelScheme(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labeljust`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labeljust(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labeljust(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labeljust`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labeljust(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labeljust(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelloc`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelloc(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelloc(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelloc`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelloc(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelloc(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Landscape`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_landscape(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Landscape(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Landscape`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_landscape(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Landscape(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layerlistsep`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layerlistsep(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layerlistsep(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layerlistsep`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layerlistsep(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layerlistsep(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layers`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layers(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layers(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layers`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layers(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layers(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layerselect`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layerselect(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layerselect(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layerselect`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layerselect(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layerselect(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layersep`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layersep(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layersep(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layersep`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layersep(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layersep(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layout`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layout(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layout(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layout`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layout(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layout(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Levels`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_levels(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Levels(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Levels`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_levels(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Levels(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Levelsgap`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_levelsgap(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Levelsgap(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Levelsgap`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_levelsgap(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Levelsgap(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lheight`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lheight(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lheight(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lheight`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lheight(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lheight(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Linelength`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_linelength(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Linelength(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Linelength`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_linelength(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Linelength(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lp(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lwidth`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lwidth(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lwidth(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lwidth`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lwidth(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lwidth(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Margin`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_margin(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Margin(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Margin`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_margin(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Margin(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Maxiter`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_maxiter(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Maxiter(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Maxiter`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_maxiter(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Maxiter(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Mclimit`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_mclimit(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Mclimit(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Mclimit`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_mclimit(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Mclimit(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Mindist`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_mindist(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Mindist(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Mindist`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_mindist(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Mindist(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Mode`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_mode(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Mode(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Mode`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_mode(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Mode(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Model`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_model(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Model(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Model`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_model(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Model(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Newrank`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_newrank(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Newrank(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Newrank`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_newrank(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Newrank(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nodesep`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nodesep(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nodesep(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nodesep`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nodesep(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nodesep(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nojustify`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nojustify(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nojustify(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nojustify`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nojustify(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nojustify(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Normalize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_normalize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Normalize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Normalize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_normalize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Normalize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Notranslate`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_notranslate(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Notranslate(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Notranslate`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_notranslate(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Notranslate(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nslimit`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nslimit(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nslimit(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nslimit`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nslimit(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nslimit(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nslimit1`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nslimit1(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nslimit1(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nslimit1`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nslimit1(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nslimit1(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Oneblock`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_oneblock(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Oneblock(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Oneblock`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_oneblock(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Oneblock(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Ordering`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_ordering(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Ordering(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Ordering`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_ordering(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Ordering(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Orientation`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_orientation(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Orientation(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Orientation`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_orientation(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Orientation(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Outputorder`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_outputorder(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Outputorder(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Outputorder`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_outputorder(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Outputorder(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Overlap`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_overlap(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Overlap(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Overlap`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_overlap(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Overlap(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::OverlapScaling`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_overlap_scaling(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::OverlapScaling(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::OverlapScaling`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_overlap_scaling(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::OverlapScaling(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::OverlapShrink`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_overlap_shrink(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::OverlapShrink(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::OverlapShrink`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_overlap_shrink(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::OverlapShrink(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pack`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pack(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pack(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pack`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pack(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pack(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Packmode`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_packmode(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Packmode(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Packmode`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_packmode(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Packmode(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pad`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pad(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pad(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pad`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pad(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pad(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Page`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_page(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Page(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Page`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_page(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Page(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pagedir`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pagedir(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pagedir(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pagedir`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pagedir(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pagedir(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Quadtree`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_quadtree(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Quadtree(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Quadtree`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_quadtree(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Quadtree(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Quantum`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_quantum(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Quantum(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Quantum`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_quantum(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Quantum(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Rankdir`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_rankdir(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Rankdir(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Rankdir`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_rankdir(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Rankdir(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Ranksep`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_ranksep(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Ranksep(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Ranksep`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_ranksep(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Ranksep(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Ratio`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_ratio(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Ratio(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Ratio`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_ratio(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Ratio(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Remincross`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_remincross(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Remincross(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Remincross`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_remincross(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Remincross(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Repulsiveforce`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_repulsiveforce(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Repulsiveforce(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Repulsiveforce`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_repulsiveforce(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Repulsiveforce(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Resolution`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_resolution(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Resolution(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Resolution`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_resolution(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Resolution(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Root`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_root(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Root(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Root`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_root(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Root(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Rotate`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_rotate(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Rotate(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Rotate`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_rotate(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Rotate(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Rotation`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_rotation(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Rotation(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Rotation`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_rotation(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Rotation(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Scale`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_scale(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Scale(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Scale`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_scale(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Scale(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Searchsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_searchsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Searchsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Searchsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_searchsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Searchsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Sep`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_sep(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Sep(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Sep`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_sep(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Sep(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Showboxes`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_showboxes(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Showboxes(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Showboxes`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_showboxes(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Showboxes(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Size`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_size(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Size(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Size`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_size(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Size(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Smoothing`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_smoothing(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Smoothing(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Smoothing`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_smoothing(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Smoothing(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Sortv`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_sortv(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Sortv(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Sortv`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_sortv(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Sortv(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Splines`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_splines(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Splines(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Splines`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_splines(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Splines(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Start`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_start(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Start(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Start`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_start(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Start(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Style`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_style(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Style(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Style`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_style(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Style(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Stylesheet`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_stylesheet(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Stylesheet(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Stylesheet`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_stylesheet(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Stylesheet(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Target`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_target(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Target(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Target`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_target(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Target(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tbbalance`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tbbalance(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tbbalance(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tbbalance`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tbbalance(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tbbalance(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Truecolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_truecolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Truecolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Truecolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_truecolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Truecolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Url`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_url(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Url(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Url`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_url(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Url(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Viewport`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_viewport(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Viewport(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Viewport`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_viewport(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Viewport(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::VoroMargin`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_voro_margin(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::VoroMargin(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::VoroMargin`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_voro_margin(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::VoroMargin(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Xdotversion`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_xdotversion(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Xdotversion(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Xdotversion`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_xdotversion(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Xdotversion(String::new());

        self.attributes.remove(&item)
    }
}

impl IntoIterator for GraphAttributes {
    type Item = Attribute;

    type IntoIter = indexmap::set::IntoIter<Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

/// Vertex attributes.
#[derive(Clone, Debug, Default)]
pub struct VertexAttributes {
    attributes: FxIndexSet<Attribute>,
}

impl VertexAttributes {
    /// Set attribute from `key` and `value` raw parts. Returns whether the attribute was newly set.
    ///
    /// # Panics
    ///
    /// Key is not valid for this attributes set. <a href="https://graphviz.org/doc/info/attrs.html#h:uses" target="_blank">Read more</a>.
    ///
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) -> bool {
        let value = quote(value);
        let item = match key {
            "area" => Attribute::Area(value),
            "class" => Attribute::Class(value),
            "color" => Attribute::Color(value),
            "colorscheme" => Attribute::Colorscheme(value),
            "comment" => Attribute::Comment(value),
            "distortion" => Attribute::Distortion(value),
            "fillcolor" => Attribute::Fillcolor(value),
            "fixedsize" => Attribute::Fixedsize(value),
            "fontcolor" => Attribute::Fontcolor(value),
            "fontname" => Attribute::Fontname(value),
            "fontsize" => Attribute::Fontsize(value),
            "gradientangle" => Attribute::Gradientangle(value),
            "group" => Attribute::Group(value),
            "height" => Attribute::Height(value),
            "href" => Attribute::Href(value),
            "id" => Attribute::Id(value),
            "image" => Attribute::Image(value),
            "imagepos" => Attribute::Imagepos(value),
            "imagescale" => Attribute::Imagescale(value),
            "label" => Attribute::Label(value),
            "labelloc" => Attribute::Labelloc(value),
            "layer" => Attribute::Layer(value),
            "margin" => Attribute::Margin(value),
            "nojustify" => Attribute::Nojustify(value),
            "ordering" => Attribute::Ordering(value),
            "orientation" => Attribute::Orientation(value),
            "penwidth" => Attribute::Penwidth(value),
            "peripheries" => Attribute::Peripheries(value),
            "pin" => Attribute::Pin(value),
            "pos" => Attribute::Pos(value),
            "rects" => Attribute::Rects(value),
            "regular" => Attribute::Regular(value),
            "root" => Attribute::Root(value),
            "samplepoints" => Attribute::Samplepoints(value),
            "shape" => Attribute::Shape(value),
            "shapefile" => Attribute::Shapefile(value),
            "showboxes" => Attribute::Showboxes(value),
            "sides" => Attribute::Sides(value),
            "skew" => Attribute::Skew(value),
            "sortv" => Attribute::Sortv(value),
            "style" => Attribute::Style(value),
            "target" => Attribute::Target(value),
            "tooltip" => Attribute::Tooltip(value),
            "URL" => Attribute::Url(value),
            "vertices" => Attribute::Vertices(value),
            "width" => Attribute::Width(value),
            "xlabel" => Attribute::Xlabel(value),
            "xlp" => Attribute::Xlp(value),
            "z" => Attribute::Z(value),
            _ => panic!("Invalid attribute key `{key}` for VertexAttributes"),
        };

        self.attributes.replace(item).is_none()
    }

    /// Get attributes length.
    #[inline]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Check if attributes is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Set [`Attribute::Area`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_area(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Area(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Area`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_area(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Area(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Class`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_class(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Class(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Class`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_class(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Class(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Color`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_color(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Color(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Color`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_color(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Color(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Colorscheme`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_colorscheme(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Colorscheme(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Colorscheme`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_colorscheme(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Colorscheme(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Comment`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_comment(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Comment(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Comment`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_comment(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Comment(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Distortion`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_distortion(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Distortion(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Distortion`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_distortion(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Distortion(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fillcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fillcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fillcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fillcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fillcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fillcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fixedsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fixedsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fixedsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fixedsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fixedsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fixedsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontname`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontname(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontname(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontname`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontname(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontname(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Gradientangle`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_gradientangle(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Gradientangle(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Gradientangle`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_gradientangle(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Gradientangle(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Group`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_group(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Group(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Group`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_group(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Group(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Height`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_height(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Height(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Height`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_height(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Height(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Href`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_href(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Href(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Href`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_href(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Href(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Id`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_id(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Id(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Id`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_id(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Id(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Image`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_image(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Image(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Image`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_image(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Image(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Imagepos`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_imagepos(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Imagepos(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Imagepos`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_imagepos(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Imagepos(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Imagescale`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_imagescale(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Imagescale(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Imagescale`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_imagescale(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Imagescale(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Label`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_label(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Label(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Label`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_label(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Label(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelloc`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelloc(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelloc(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelloc`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelloc(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelloc(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layer`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layer(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layer(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layer`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layer(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layer(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Margin`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_margin(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Margin(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Margin`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_margin(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Margin(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nojustify`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nojustify(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nojustify(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nojustify`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nojustify(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nojustify(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Ordering`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_ordering(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Ordering(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Ordering`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_ordering(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Ordering(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Orientation`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_orientation(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Orientation(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Orientation`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_orientation(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Orientation(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Penwidth`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_penwidth(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Penwidth(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Penwidth`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_penwidth(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Penwidth(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Peripheries`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_peripheries(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Peripheries(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Peripheries`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_peripheries(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Peripheries(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pin`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pin(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pin(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pin`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pin(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pin(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pos`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pos(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pos(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pos`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pos(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pos(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Rects`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_rects(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Rects(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Rects`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_rects(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Rects(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Regular`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_regular(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Regular(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Regular`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_regular(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Regular(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Root`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_root(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Root(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Root`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_root(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Root(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Samplepoints`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_samplepoints(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Samplepoints(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Samplepoints`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_samplepoints(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Samplepoints(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Shape`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_shape(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Shape(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Shape`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_shape(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Shape(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Shapefile`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_shapefile(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Shapefile(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Shapefile`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_shapefile(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Shapefile(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Showboxes`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_showboxes(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Showboxes(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Showboxes`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_showboxes(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Showboxes(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Sides`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_sides(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Sides(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Sides`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_sides(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Sides(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Skew`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_skew(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Skew(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Skew`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_skew(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Skew(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Sortv`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_sortv(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Sortv(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Sortv`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_sortv(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Sortv(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Style`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_style(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Style(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Style`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_style(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Style(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Target`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_target(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Target(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Target`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_target(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Target(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Url`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_url(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Url(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Url`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_url(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Url(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Vertices`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_vertices(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Vertices(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Vertices`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_vertices(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Vertices(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Width`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_width(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Width(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Width`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_width(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Width(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Xlabel`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_xlabel(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Xlabel(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Xlabel`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_xlabel(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Xlabel(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Xlp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_xlp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Xlp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Xlp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_xlp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Xlp(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Z`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_z(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Z(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Z`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_z(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Z(String::new());

        self.attributes.remove(&item)
    }
}

impl IntoIterator for VertexAttributes {
    type Item = Attribute;

    type IntoIter = indexmap::set::IntoIter<Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

/// Cluster attributes.
#[derive(Clone, Debug, Default)]
pub struct ClusterAttributes {
    attributes: FxIndexSet<Attribute>,
}

impl ClusterAttributes {
    /// Set attribute from `key` and `value` raw parts. Returns whether the attribute was newly set.
    ///
    /// # Panics
    ///
    /// Key is not valid for this attributes set. <a href="https://graphviz.org/doc/info/attrs.html#h:uses" target="_blank">Read more</a>.
    ///
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) -> bool {
        let value = quote(value);
        let item = match key {
            "area" => Attribute::Area(value),
            "bgcolor" => Attribute::Bgcolor(value),
            "class" => Attribute::Class(value),
            "cluster" => Attribute::Cluster(value),
            "color" => Attribute::Color(value),
            "colorscheme" => Attribute::Colorscheme(value),
            "fillcolor" => Attribute::Fillcolor(value),
            "fontcolor" => Attribute::Fontcolor(value),
            "fontname" => Attribute::Fontname(value),
            "fontsize" => Attribute::Fontsize(value),
            "gradientangle" => Attribute::Gradientangle(value),
            "href" => Attribute::Href(value),
            "id" => Attribute::Id(value),
            "K" => Attribute::K(value),
            "label" => Attribute::Label(value),
            "labeljust" => Attribute::Labeljust(value),
            "labelloc" => Attribute::Labelloc(value),
            "layer" => Attribute::Layer(value),
            "lheight" => Attribute::Lheight(value),
            "lp" => Attribute::Lp(value),
            "lwidth" => Attribute::Lwidth(value),
            "margin" => Attribute::Margin(value),
            "nojustify" => Attribute::Nojustify(value),
            "pencolor" => Attribute::Pencolor(value),
            "penwidth" => Attribute::Penwidth(value),
            "peripheries" => Attribute::Peripheries(value),
            "sortv" => Attribute::Sortv(value),
            "style" => Attribute::Style(value),
            "target" => Attribute::Target(value),
            "tooltip" => Attribute::Tooltip(value),
            "URL" => Attribute::Url(value),
            _ => panic!("Invalid attribute key `{key}` for ClusterAttributes"),
        };

        self.attributes.replace(item).is_none()
    }

    /// Get attributes length.
    #[inline]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Check if attributes is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Set [`Attribute::Area`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_area(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Area(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Area`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_area(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Area(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Bgcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_bgcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Bgcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Bgcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_bgcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Bgcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Class`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_class(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Class(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Class`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_class(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Class(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Cluster`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_cluster(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Cluster(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Cluster`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_cluster(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Cluster(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Color`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_color(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Color(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Color`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_color(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Color(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Colorscheme`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_colorscheme(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Colorscheme(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Colorscheme`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_colorscheme(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Colorscheme(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fillcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fillcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fillcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fillcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fillcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fillcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontname`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontname(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontname(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontname`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontname(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontname(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Gradientangle`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_gradientangle(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Gradientangle(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Gradientangle`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_gradientangle(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Gradientangle(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Href`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_href(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Href(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Href`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_href(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Href(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Id`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_id(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Id(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Id`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_id(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Id(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::K`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_k(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::K(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::K`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_k(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::K(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Label`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_label(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Label(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Label`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_label(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Label(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labeljust`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labeljust(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labeljust(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labeljust`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labeljust(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labeljust(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelloc`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelloc(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelloc(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelloc`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelloc(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelloc(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layer`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layer(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layer(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layer`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layer(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layer(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lheight`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lheight(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lheight(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lheight`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lheight(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lheight(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lp(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lwidth`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lwidth(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lwidth(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lwidth`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lwidth(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lwidth(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Margin`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_margin(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Margin(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Margin`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_margin(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Margin(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nojustify`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nojustify(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nojustify(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nojustify`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nojustify(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nojustify(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pencolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pencolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pencolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pencolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pencolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pencolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Penwidth`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_penwidth(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Penwidth(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Penwidth`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_penwidth(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Penwidth(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Peripheries`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_peripheries(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Peripheries(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Peripheries`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_peripheries(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Peripheries(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Sortv`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_sortv(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Sortv(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Sortv`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_sortv(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Sortv(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Style`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_style(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Style(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Style`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_style(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Style(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Target`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_target(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Target(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Target`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_target(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Target(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Url`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_url(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Url(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Url`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_url(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Url(String::new());

        self.attributes.remove(&item)
    }
}

impl IntoIterator for ClusterAttributes {
    type Item = Attribute;

    type IntoIter = indexmap::set::IntoIter<Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

/// Edge attributes.
#[derive(Clone, Debug, Default)]
pub struct EdgeAttributes {
    attributes: FxIndexSet<Attribute>,
}

impl EdgeAttributes {
    /// Set attribute from `key` and `value` raw parts. Returns whether the attribute was newly set.
    ///
    /// # Panics
    ///
    /// Key is not valid for this attributes set. <a href="https://graphviz.org/doc/info/attrs.html#h:uses" target="_blank">Read more</a>.
    ///
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) -> bool {
        let value = quote(value);
        let item = match key {
            "arrowhead" => Attribute::Arrowhead(value),
            "arrowsize" => Attribute::Arrowsize(value),
            "arrowtail" => Attribute::Arrowtail(value),
            "class" => Attribute::Class(value),
            "color" => Attribute::Color(value),
            "colorscheme" => Attribute::Colorscheme(value),
            "comment" => Attribute::Comment(value),
            "constraint" => Attribute::Constraint(value),
            "decorate" => Attribute::Decorate(value),
            "dir" => Attribute::Dir(value),
            "edgehref" => Attribute::Edgehref(value),
            "edgetarget" => Attribute::Edgetarget(value),
            "edgetooltip" => Attribute::Edgetooltip(value),
            "edgeURL" => Attribute::Edgeurl(value),
            "fillcolor" => Attribute::Fillcolor(value),
            "fontcolor" => Attribute::Fontcolor(value),
            "fontname" => Attribute::Fontname(value),
            "fontsize" => Attribute::Fontsize(value),
            "head_lp" => Attribute::HeadLp(value),
            "headclip" => Attribute::Headclip(value),
            "headhref" => Attribute::Headhref(value),
            "headlabel" => Attribute::Headlabel(value),
            "headport" => Attribute::Headport(value),
            "headtarget" => Attribute::Headtarget(value),
            "headtooltip" => Attribute::Headtooltip(value),
            "headURL" => Attribute::Headurl(value),
            "href" => Attribute::Href(value),
            "id" => Attribute::Id(value),
            "label" => Attribute::Label(value),
            "labelangle" => Attribute::Labelangle(value),
            "labeldistance" => Attribute::Labeldistance(value),
            "labelfloat" => Attribute::Labelfloat(value),
            "labelfontcolor" => Attribute::Labelfontcolor(value),
            "labelfontname" => Attribute::Labelfontname(value),
            "labelfontsize" => Attribute::Labelfontsize(value),
            "labelhref" => Attribute::Labelhref(value),
            "labeltarget" => Attribute::Labeltarget(value),
            "labeltooltip" => Attribute::Labeltooltip(value),
            "labelURL" => Attribute::Labelurl(value),
            "layer" => Attribute::Layer(value),
            "len" => Attribute::Len(value),
            "lhead" => Attribute::Lhead(value),
            "lp" => Attribute::Lp(value),
            "ltail" => Attribute::Ltail(value),
            "minlen" => Attribute::Minlen(value),
            "nojustify" => Attribute::Nojustify(value),
            "penwidth" => Attribute::Penwidth(value),
            "pos" => Attribute::Pos(value),
            "samehead" => Attribute::Samehead(value),
            "sametail" => Attribute::Sametail(value),
            "showboxes" => Attribute::Showboxes(value),
            "style" => Attribute::Style(value),
            "tail_lp" => Attribute::TailLp(value),
            "tailclip" => Attribute::Tailclip(value),
            "tailhref" => Attribute::Tailhref(value),
            "taillabel" => Attribute::Taillabel(value),
            "tailport" => Attribute::Tailport(value),
            "tailtarget" => Attribute::Tailtarget(value),
            "tailtooltip" => Attribute::Tailtooltip(value),
            "tailURL" => Attribute::Tailurl(value),
            "target" => Attribute::Target(value),
            "tooltip" => Attribute::Tooltip(value),
            "URL" => Attribute::Url(value),
            "weight" => Attribute::Weight(value),
            "xlabel" => Attribute::Xlabel(value),
            "xlp" => Attribute::Xlp(value),
            _ => panic!("Invalid attribute key `{key}` for EdgeAttributes"),
        };

        self.attributes.replace(item).is_none()
    }

    /// Get attributes length.
    #[inline]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Check if attributes is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Set [`Attribute::Arrowhead`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_arrowhead(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Arrowhead(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Arrowhead`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_arrowhead(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Arrowhead(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Arrowsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_arrowsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Arrowsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Arrowsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_arrowsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Arrowsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Arrowtail`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_arrowtail(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Arrowtail(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Arrowtail`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_arrowtail(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Arrowtail(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Class`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_class(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Class(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Class`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_class(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Class(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Color`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_color(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Color(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Color`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_color(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Color(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Colorscheme`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_colorscheme(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Colorscheme(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Colorscheme`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_colorscheme(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Colorscheme(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Comment`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_comment(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Comment(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Comment`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_comment(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Comment(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Constraint`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_constraint(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Constraint(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Constraint`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_constraint(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Constraint(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Decorate`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_decorate(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Decorate(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Decorate`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_decorate(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Decorate(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Dir`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_dir(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Dir(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Dir`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_dir(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Dir(String::new());

        self.attributes.remove(&item)
    }

    /// Get [`Attribute::Dir`] attribute.
    #[inline]
    pub fn get_edge_dir(&self) -> Option<String> {
        for att in &self.attributes {
            if let Attribute::Dir(x) = att {
                return Some(x.into());
            }
        }
        None
    }

    /// Set [`Attribute::Edgehref`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_edgehref(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Edgehref(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Edgehref`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_edgehref(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Edgehref(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Edgetarget`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_edgetarget(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Edgetarget(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Edgetarget`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_edgetarget(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Edgetarget(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Edgetooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_edgetooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Edgetooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Edgetooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_edgetooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Edgetooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Edgeurl`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_edgeurl(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Edgeurl(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Edgeurl`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_edgeurl(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Edgeurl(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fillcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fillcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fillcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fillcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fillcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fillcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontname`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontname(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontname(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontname`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontname(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontname(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Fontsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_fontsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Fontsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Fontsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_fontsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Fontsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::HeadLp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_head_lp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::HeadLp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::HeadLp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_head_lp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::HeadLp(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headclip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headclip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headclip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headclip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headclip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headclip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headhref`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headhref(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headhref(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headhref`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headhref(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headhref(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headlabel`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headlabel(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headlabel(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headlabel`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headlabel(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headlabel(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headport`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headport(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headport(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headport`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headport(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headport(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headtarget`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headtarget(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headtarget(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headtarget`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headtarget(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headtarget(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headtooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headtooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headtooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headtooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headtooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headtooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Headurl`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_headurl(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Headurl(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Headurl`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_headurl(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Headurl(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Href`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_href(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Href(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Href`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_href(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Href(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Id`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_id(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Id(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Id`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_id(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Id(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Label`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_label(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Label(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Label`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_label(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Label(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelangle`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelangle(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelangle(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelangle`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelangle(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelangle(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labeldistance`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labeldistance(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labeldistance(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labeldistance`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labeldistance(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labeldistance(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelfloat`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelfloat(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelfloat(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelfloat`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelfloat(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelfloat(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelfontcolor`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelfontcolor(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelfontcolor(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelfontcolor`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelfontcolor(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelfontcolor(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelfontname`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelfontname(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelfontname(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelfontname`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelfontname(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelfontname(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelfontsize`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelfontsize(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelfontsize(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelfontsize`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelfontsize(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelfontsize(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelhref`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelhref(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelhref(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelhref`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelhref(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelhref(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labeltarget`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labeltarget(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labeltarget(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labeltarget`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labeltarget(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labeltarget(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labeltooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labeltooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labeltooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labeltooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labeltooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labeltooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Labelurl`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_labelurl(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Labelurl(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Labelurl`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_labelurl(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Labelurl(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Layer`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_layer(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Layer(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Layer`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_layer(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Layer(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Len`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_len(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Len(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Len`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_len(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Len(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lhead`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lhead(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lhead(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lhead`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lhead(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lhead(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Lp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_lp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Lp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Lp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_lp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Lp(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Ltail`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_ltail(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Ltail(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Ltail`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_ltail(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Ltail(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Minlen`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_minlen(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Minlen(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Minlen`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_minlen(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Minlen(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Nojustify`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_nojustify(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Nojustify(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Nojustify`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_nojustify(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Nojustify(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Penwidth`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_penwidth(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Penwidth(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Penwidth`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_penwidth(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Penwidth(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Pos`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_pos(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Pos(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Pos`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_pos(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Pos(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Samehead`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_samehead(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Samehead(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Samehead`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_samehead(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Samehead(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Sametail`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_sametail(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Sametail(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Sametail`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_sametail(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Sametail(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Showboxes`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_showboxes(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Showboxes(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Showboxes`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_showboxes(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Showboxes(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Style`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_style(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Style(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Style`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_style(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Style(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::TailLp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tail_lp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::TailLp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::TailLp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tail_lp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::TailLp(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tailclip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tailclip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tailclip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tailclip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tailclip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tailclip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tailhref`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tailhref(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tailhref(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tailhref`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tailhref(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tailhref(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Taillabel`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_taillabel(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Taillabel(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Taillabel`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_taillabel(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Taillabel(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tailport`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tailport(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tailport(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tailport`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tailport(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tailport(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tailtarget`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tailtarget(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tailtarget(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tailtarget`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tailtarget(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tailtarget(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tailtooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tailtooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tailtooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tailtooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tailtooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tailtooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tailurl`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tailurl(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tailurl(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tailurl`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tailurl(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tailurl(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Target`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_target(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Target(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Target`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_target(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Target(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Tooltip`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_tooltip(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Tooltip(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Tooltip`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_tooltip(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Tooltip(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Url`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_url(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Url(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Url`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_url(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Url(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Weight`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_weight(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Weight(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Weight`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_weight(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Weight(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Xlabel`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_xlabel(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Xlabel(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Xlabel`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_xlabel(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Xlabel(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Xlp`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_xlp(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Xlp(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Xlp`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_xlp(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Xlp(String::new());

        self.attributes.remove(&item)
    }
}

impl IntoIterator for EdgeAttributes {
    type Item = Attribute;

    type IntoIter = indexmap::set::IntoIter<Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

/// Subgraph attributes.
#[derive(Clone, Debug, Default)]
pub struct SubgraphAttributes {
    attributes: FxIndexSet<Attribute>,
}

impl SubgraphAttributes {
    /// Set attribute from `key` and `value` raw parts. Returns whether the attribute was newly set.
    ///
    /// # Panics
    ///
    /// Key is not valid for this attributes set. <a href="https://graphviz.org/doc/info/attrs.html#h:uses" target="_blank">Read more</a>.
    ///
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) -> bool {
        let value = quote(value);
        let item = match key {
            "cluster" => Attribute::Cluster(value),
            "rank" => Attribute::Rank(value),
            _ => panic!("Invalid attribute key `{key}` for SubgraphAttributes"),
        };

        self.attributes.replace(item).is_none()
    }

    /// Get attributes length.
    #[inline]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Check if attributes is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Set [`Attribute::Cluster`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_cluster(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Cluster(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Cluster`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_cluster(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Cluster(String::new());

        self.attributes.remove(&item)
    }

    /// Set [`Attribute::Rank`] attribute. Returns whether the attribute was newly set.
    #[inline]
    pub fn set_rank(&mut self, s: &str) -> bool {
        // Initialize new item for insertion or replacement.
        let item = Attribute::Rank(quote(s));

        self.attributes.replace(item).is_none()
    }

    /// Unset [`Attribute::Rank`] attribute. Returns whether the attribute was set.
    #[inline]
    pub fn unset_rank(&mut self) -> bool {
        // Allocate item placeholder for removal.
        let item = Attribute::Rank(String::new());

        self.attributes.remove(&item)
    }
}

impl IntoIterator for SubgraphAttributes {
    type Item = Attribute;

    type IntoIter = indexmap::set::IntoIter<Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}
