use pyo3::prelude::*;
 
use pyo3_stub_gen::derive::* ;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, PartialEq,Debug)]
pub struct Color {

    /// red channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub r: f32,

    /// green channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub g: f32,

    /// blue channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub b: f32,

    /// alpha channel. ranges from 0.0 to 1.0
    #[pyo3(get, set)]
    pub a: f32,

}

#[gen_stub_pymethods]
#[pymethods]
impl Color {

    /// create a new color.
    ///
    /// inputs range from:
    /// (0.0, 0.0, 0.0, 1.0) -> BLACK
    /// to
    /// (1.0, 1.0, 1.0, 1.0) -> WHITE
    #[new]
    pub fn new(r: f32, g: f32,b: f32,a: f32) -> Self {
       Self { r,g,b,a }
    }

    

    /// CLOUDY_BLUE
    #[classattr]
    fn CLOUDY_BLUE() -> Color {
        Color { r: 0.6745098039215687, g: 0.7607843137254902, b: 0.8509803921568627, a: 1.0 }
    }

    /// DARK_PASTEL_GREEN
    #[classattr]
    fn DARK_PASTEL_GREEN() -> Color {
        Color { r: 0.33725490196078434, g: 0.6823529411764706, b: 0.3411764705882353, a: 1.0 }
    }

    /// DUST
    #[classattr]
    fn DUST() -> Color {
        Color { r: 0.6980392156862745, g: 0.6, b: 0.43137254901960786, a: 1.0 }
    }

    /// ELECTRIC_LIME
    #[classattr]
    fn ELECTRIC_LIME() -> Color {
        Color { r: 0.6588235294117647, g: 1.0, b: 0.01568627450980392, a: 1.0 }
    }

    /// FRESH_GREEN
    #[classattr]
    fn FRESH_GREEN() -> Color {
        Color { r: 0.4117647058823529, g: 0.8470588235294118, b: 0.30980392156862746, a: 1.0 }
    }

    /// LIGHT_EGGPLANT
    #[classattr]
    fn LIGHT_EGGPLANT() -> Color {
        Color { r: 0.5372549019607843, g: 0.27058823529411763, b: 0.5215686274509804, a: 1.0 }
    }

    /// NASTY_GREEN
    #[classattr]
    fn NASTY_GREEN() -> Color {
        Color { r: 0.4392156862745098, g: 0.6980392156862745, b: 0.24705882352941178, a: 1.0 }
    }

    /// REALLY_LIGHT_BLUE
    #[classattr]
    fn REALLY_LIGHT_BLUE() -> Color {
        Color { r: 0.8313725490196079, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// TEA
    #[classattr]
    fn TEA() -> Color {
        Color { r: 0.396078431372549, g: 0.6705882352941176, b: 0.48627450980392156, a: 1.0 }
    }

    /// WARM_PURPLE
    #[classattr]
    fn WARM_PURPLE() -> Color {
        Color { r: 0.5843137254901961, g: 0.1803921568627451, b: 0.5607843137254902, a: 1.0 }
    }

    /// YELLOWISH_TAN
    #[classattr]
    fn YELLOWISH_TAN() -> Color {
        Color { r: 0.9882352941176471, g: 0.9882352941176471, b: 0.5058823529411764, a: 1.0 }
    }

    /// CEMENT
    #[classattr]
    fn CEMENT() -> Color {
        Color { r: 0.6470588235294118, g: 0.6392156862745098, b: 0.5686274509803921, a: 1.0 }
    }

    /// DARK_GRASS_GREEN
    #[classattr]
    fn DARK_GRASS_GREEN() -> Color {
        Color { r: 0.2196078431372549, g: 0.5019607843137255, b: 0.01568627450980392, a: 1.0 }
    }

    /// DUSTY_TEAL
    #[classattr]
    fn DUSTY_TEAL() -> Color {
        Color { r: 0.2980392156862745, g: 0.5647058823529412, b: 0.5215686274509804, a: 1.0 }
    }

    /// GREY_TEAL
    #[classattr]
    fn GREY_TEAL() -> Color {
        Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 }
    }

    /// MACARONI_AND_CHEESE
    #[classattr]
    fn MACARONI_AND_CHEESE() -> Color {
        Color { r: 0.9372549019607843, g: 0.7058823529411765, b: 0.20784313725490197, a: 1.0 }
    }

    /// PINKISH_TAN
    #[classattr]
    fn PINKISH_TAN() -> Color {
        Color { r: 0.8509803921568627, g: 0.6078431372549019, b: 0.5098039215686274, a: 1.0 }
    }

    /// SPRUCE
    #[classattr]
    fn SPRUCE() -> Color {
        Color { r: 0.0392156862745098, g: 0.37254901960784315, b: 0.2196078431372549, a: 1.0 }
    }

    /// STRONG_BLUE
    #[classattr]
    fn STRONG_BLUE() -> Color {
        Color { r: 0.047058823529411764, g: 0.023529411764705882, b: 0.9686274509803922, a: 1.0 }
    }

    /// TOXIC_GREEN
    #[classattr]
    fn TOXIC_GREEN() -> Color {
        Color { r: 0.3803921568627451, g: 0.8705882352941177, b: 0.16470588235294117, a: 1.0 }
    }

    /// WINDOWS_BLUE
    #[classattr]
    fn WINDOWS_BLUE() -> Color {
        Color { r: 0.21568627450980393, g: 0.47058823529411764, b: 0.7490196078431373, a: 1.0 }
    }

    /// BLUE_BLUE
    #[classattr]
    fn BLUE_BLUE() -> Color {
        Color { r: 0.13333333333333333, g: 0.25882352941176473, b: 0.7803921568627451, a: 1.0 }
    }

    /// BLUE_WITH_A_HINT_OF_PURPLE
    #[classattr]
    fn BLUE_WITH_A_HINT_OF_PURPLE() -> Color {
        Color { r: 0.3254901960784314, g: 0.23529411764705882, b: 0.7764705882352941, a: 1.0 }
    }

    /// BOOGER
    #[classattr]
    fn BOOGER() -> Color {
        Color { r: 0.6078431372549019, g: 0.7098039215686275, b: 0.23529411764705882, a: 1.0 }
    }

    /// BRIGHT_SEA_GREEN
    #[classattr]
    fn BRIGHT_SEA_GREEN() -> Color {
        Color { r: 0.0196078431372549, g: 1.0, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_GREEN_BLUE
    #[classattr]
    fn DARK_GREEN_BLUE() -> Color {
        Color { r: 0.12156862745098039, g: 0.38823529411764707, b: 0.3411764705882353, a: 1.0 }
    }

    /// DEEP_TURQUOISE
    #[classattr]
    fn DEEP_TURQUOISE() -> Color {
        Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.4549019607843137, a: 1.0 }
    }

    /// GREEN_TEAL
    #[classattr]
    fn GREEN_TEAL() -> Color {
        Color { r: 0.047058823529411764, g: 0.7098039215686275, b: 0.4666666666666667, a: 1.0 }
    }

    /// STRONG_PINK
    #[classattr]
    fn STRONG_PINK() -> Color {
        Color { r: 1.0, g: 0.027450980392156862, b: 0.5372549019607843, a: 1.0 }
    }

    /// BLAND
    #[classattr]
    fn BLAND() -> Color {
        Color { r: 0.6862745098039216, g: 0.6588235294117647, b: 0.5450980392156862, a: 1.0 }
    }

    /// DEEP_AQUA
    #[classattr]
    fn DEEP_AQUA() -> Color {
        Color { r: 0.03137254901960784, g: 0.47058823529411764, b: 0.4980392156862745, a: 1.0 }
    }

    /// LAVENDER_PINK
    #[classattr]
    fn LAVENDER_PINK() -> Color {
        Color { r: 0.8666666666666667, g: 0.5215686274509804, b: 0.8431372549019608, a: 1.0 }
    }

    /// LIGHT_MOSS_GREEN
    #[classattr]
    fn LIGHT_MOSS_GREEN() -> Color {
        Color { r: 0.6509803921568628, g: 0.7843137254901961, b: 0.4588235294117647, a: 1.0 }
    }

    /// LIGHT_SEAFOAM_GREEN
    #[classattr]
    fn LIGHT_SEAFOAM_GREEN() -> Color {
        Color { r: 0.6549019607843137, g: 1.0, b: 0.7098039215686275, a: 1.0 }
    }

    /// OLIVE_YELLOW
    #[classattr]
    fn OLIVE_YELLOW() -> Color {
        Color { r: 0.7607843137254902, g: 0.7176470588235294, b: 0.03529411764705882, a: 1.0 }
    }

    /// PIG_PINK
    #[classattr]
    fn PIG_PINK() -> Color {
        Color { r: 0.9058823529411765, g: 0.5568627450980392, b: 0.6470588235294118, a: 1.0 }
    }

    /// DEEP_LILAC
    #[classattr]
    fn DEEP_LILAC() -> Color {
        Color { r: 0.5882352941176471, g: 0.43137254901960786, b: 0.7411764705882353, a: 1.0 }
    }

    /// DESERT
    #[classattr]
    fn DESERT() -> Color {
        Color { r: 0.8, g: 0.6784313725490196, b: 0.3764705882352941, a: 1.0 }
    }

    /// DUSTY_LAVENDER
    #[classattr]
    fn DUSTY_LAVENDER() -> Color {
        Color { r: 0.6745098039215687, g: 0.5254901960784314, b: 0.6588235294117647, a: 1.0 }
    }

    /// PURPLEY_GREY
    #[classattr]
    fn PURPLEY_GREY() -> Color {
        Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 }
    }

    /// PURPLY
    #[classattr]
    fn PURPLY() -> Color {
        Color { r: 0.596078431372549, g: 0.24705882352941178, b: 0.6980392156862745, a: 1.0 }
    }

    /// CANDY_PINK
    #[classattr]
    fn CANDY_PINK() -> Color {
        Color { r: 1.0, g: 0.38823529411764707, b: 0.9137254901960784, a: 1.0 }
    }

    /// LIGHT_PASTEL_GREEN
    #[classattr]
    fn LIGHT_PASTEL_GREEN() -> Color {
        Color { r: 0.6980392156862745, g: 0.984313725490196, b: 0.6470588235294118, a: 1.0 }
    }

    /// BORING_GREEN
    #[classattr]
    fn BORING_GREEN() -> Color {
        Color { r: 0.38823529411764707, g: 0.7019607843137254, b: 0.396078431372549, a: 1.0 }
    }

    /// KIWI_GREEN
    #[classattr]
    fn KIWI_GREEN() -> Color {
        Color { r: 0.5568627450980392, g: 0.8980392156862745, b: 0.24705882352941178, a: 1.0 }
    }

    /// LIGHT_GREY_GREEN
    #[classattr]
    fn LIGHT_GREY_GREEN() -> Color {
        Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 }
    }

    /// ORANGE_PINK
    #[classattr]
    fn ORANGE_PINK() -> Color {
        Color { r: 1.0, g: 0.43529411764705883, b: 0.3215686274509804, a: 1.0 }
    }

    /// TEA_GREEN
    #[classattr]
    fn TEA_GREEN() -> Color {
        Color { r: 0.7411764705882353, g: 0.9725490196078431, b: 0.6392156862745098, a: 1.0 }
    }

    /// VERY_LIGHT_BROWN
    #[classattr]
    fn VERY_LIGHT_BROWN() -> Color {
        Color { r: 0.8274509803921568, g: 0.7137254901960784, b: 0.5137254901960784, a: 1.0 }
    }

    /// EGG_SHELL
    #[classattr]
    fn EGG_SHELL() -> Color {
        Color { r: 1.0, g: 0.9882352941176471, b: 0.7686274509803922, a: 1.0 }
    }

    /// EGGPLANT_PURPLE
    #[classattr]
    fn EGGPLANT_PURPLE() -> Color {
        Color { r: 0.2627450980392157, g: 0.0196078431372549, b: 0.2549019607843137, a: 1.0 }
    }

    /// POWDER_PINK
    #[classattr]
    fn POWDER_PINK() -> Color {
        Color { r: 1.0, g: 0.6980392156862745, b: 0.8156862745098039, a: 1.0 }
    }

    /// REDDISH_GREY
    #[classattr]
    fn REDDISH_GREY() -> Color {
        Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 }
    }

    /// BABY_SHIT_BROWN
    #[classattr]
    fn BABY_SHIT_BROWN() -> Color {
        Color { r: 0.6784313725490196, g: 0.5647058823529412, b: 0.050980392156862744, a: 1.0 }
    }

    /// LILIAC
    #[classattr]
    fn LILIAC() -> Color {
        Color { r: 0.7686274509803922, g: 0.5568627450980392, b: 0.9921568627450981, a: 1.0 }
    }

    /// STORMY_BLUE
    #[classattr]
    fn STORMY_BLUE() -> Color {
        Color { r: 0.3137254901960784, g: 0.4823529411764706, b: 0.611764705882353, a: 1.0 }
    }

    /// UGLY_BROWN
    #[classattr]
    fn UGLY_BROWN() -> Color {
        Color { r: 0.49019607843137253, g: 0.44313725490196076, b: 0.011764705882352941, a: 1.0 }
    }

    /// CUSTARD
    #[classattr]
    fn CUSTARD() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.47058823529411764, a: 1.0 }
    }

    /// DARKISH_PINK
    #[classattr]
    fn DARKISH_PINK() -> Color {
        Color { r: 0.8549019607843137, g: 0.27450980392156865, b: 0.49019607843137253, a: 1.0 }
    }

    /// DEEP_BROWN
    #[classattr]
    fn DEEP_BROWN() -> Color {
        Color { r: 0.2549019607843137, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// GREENISH_BEIGE
    #[classattr]
    fn GREENISH_BEIGE() -> Color {
        Color { r: 0.788235294117647, g: 0.8196078431372549, b: 0.4745098039215686, a: 1.0 }
    }

    /// MANILLA
    #[classattr]
    fn MANILLA() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.5254901960784314, a: 1.0 }
    }

    /// OFF_BLUE
    #[classattr]
    fn OFF_BLUE() -> Color {
        Color { r: 0.33725490196078434, g: 0.5176470588235295, b: 0.6823529411764706, a: 1.0 }
    }

    /// BATTLESHIP_GREY
    #[classattr]
    fn BATTLESHIP_GREY() -> Color {
        Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 }
    }

    /// BROWNY_GREEN
    #[classattr]
    fn BROWNY_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.4235294117647059, b: 0.0392156862745098, a: 1.0 }
    }

    /// BRUISE
    #[classattr]
    fn BRUISE() -> Color {
        Color { r: 0.49411764705882355, g: 0.25098039215686274, b: 0.44313725490196076, a: 1.0 }
    }

    /// KELLEY_GREEN
    #[classattr]
    fn KELLEY_GREEN() -> Color {
        Color { r: 0.0, g: 0.5764705882352941, b: 0.21568627450980393, a: 1.0 }
    }

    /// SICKLY_YELLOW
    #[classattr]
    fn SICKLY_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.8941176470588236, b: 0.1607843137254902, a: 1.0 }
    }

    /// SUNNY_YELLOW
    #[classattr]
    fn SUNNY_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9764705882352941, b: 0.09019607843137255, a: 1.0 }
    }

    /// AZUL
    #[classattr]
    fn AZUL() -> Color {
        Color { r: 0.11372549019607843, g: 0.36470588235294116, b: 0.9254901960784314, a: 1.0 }
    }

    /// DARKGREEN
    #[classattr]
    fn DARKGREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.28627450980392155, b: 0.027450980392156862, a: 1.0 }
    }

    /// GREEN_YELLOW
    #[classattr]
    fn GREEN_YELLOW() -> Color {
        Color { r: 0.7098039215686275, g: 0.807843137254902, b: 0.03137254901960784, a: 1.0 }
    }

    /// LICHEN
    #[classattr]
    fn LICHEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.7137254901960784, b: 0.4823529411764706, a: 1.0 }
    }

    /// LIGHT_LIGHT_GREEN
    #[classattr]
    fn LIGHT_LIGHT_GREEN() -> Color {
        Color { r: 0.7843137254901961, g: 1.0, b: 0.6901960784313725, a: 1.0 }
    }

    /// PALE_GOLD
    #[classattr]
    fn PALE_GOLD() -> Color {
        Color { r: 0.9921568627450981, g: 0.8705882352941177, b: 0.4235294117647059, a: 1.0 }
    }

    /// SUN_YELLOW
    #[classattr]
    fn SUN_YELLOW() -> Color {
        Color { r: 1.0, g: 0.8745098039215686, b: 0.13333333333333333, a: 1.0 }
    }

    /// TAN_GREEN
    #[classattr]
    fn TAN_GREEN() -> Color {
        Color { r: 0.6627450980392157, g: 0.7450980392156863, b: 0.4392156862745098, a: 1.0 }
    }

    /// BURPLE
    #[classattr]
    fn BURPLE() -> Color {
        Color { r: 0.40784313725490196, g: 0.19607843137254902, b: 0.8901960784313725, a: 1.0 }
    }

    /// BUTTERSCOTCH
    #[classattr]
    fn BUTTERSCOTCH() -> Color {
        Color { r: 0.9921568627450981, g: 0.6941176470588235, b: 0.2784313725490196, a: 1.0 }
    }

    /// TOUPE
    #[classattr]
    fn TOUPE() -> Color {
        Color { r: 0.7803921568627451, g: 0.6745098039215687, b: 0.49019607843137253, a: 1.0 }
    }

    /// DARK_CREAM
    #[classattr]
    fn DARK_CREAM() -> Color {
        Color { r: 1.0, g: 0.9529411764705882, b: 0.6039215686274509, a: 1.0 }
    }

    /// INDIAN_RED
    #[classattr]
    fn INDIAN_RED() -> Color {
        Color { r: 0.5215686274509804, g: 0.054901960784313725, b: 0.01568627450980392, a: 1.0 }
    }

    /// LIGHT_LAVENDAR
    #[classattr]
    fn LIGHT_LAVENDAR() -> Color {
        Color { r: 0.9372549019607843, g: 0.7529411764705882, b: 0.996078431372549, a: 1.0 }
    }

    /// POISON_GREEN
    #[classattr]
    fn POISON_GREEN() -> Color {
        Color { r: 0.25098039215686274, g: 0.9921568627450981, b: 0.0784313725490196, a: 1.0 }
    }

    /// BABY_PUKE_GREEN
    #[classattr]
    fn BABY_PUKE_GREEN() -> Color {
        Color { r: 0.7137254901960784, g: 0.7686274509803922, b: 0.023529411764705882, a: 1.0 }
    }

    /// BRIGHT_YELLOW_GREEN
    #[classattr]
    fn BRIGHT_YELLOW_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 1.0, b: 0.0, a: 1.0 }
    }

    /// CHARCOAL_GREY
    #[classattr]
    fn CHARCOAL_GREY() -> Color {
        Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 }
    }

    /// SQUASH
    #[classattr]
    fn SQUASH() -> Color {
        Color { r: 0.9490196078431372, g: 0.6705882352941176, b: 0.08235294117647059, a: 1.0 }
    }

    /// CINNAMON
    #[classattr]
    fn CINNAMON() -> Color {
        Color { r: 0.6745098039215687, g: 0.30980392156862746, b: 0.023529411764705882, a: 1.0 }
    }

    /// LIGHT_PEA_GREEN
    #[classattr]
    fn LIGHT_PEA_GREEN() -> Color {
        Color { r: 0.7686274509803922, g: 0.996078431372549, b: 0.5098039215686274, a: 1.0 }
    }

    /// RADIOACTIVE_GREEN
    #[classattr]
    fn RADIOACTIVE_GREEN() -> Color {
        Color { r: 0.17254901960784313, g: 0.9803921568627451, b: 0.12156862745098039, a: 1.0 }
    }

    /// RAW_SIENNA
    #[classattr]
    fn RAW_SIENNA() -> Color {
        Color { r: 0.6039215686274509, g: 0.3843137254901961, b: 0.0, a: 1.0 }
    }

    /// BABY_PURPLE
    #[classattr]
    fn BABY_PURPLE() -> Color {
        Color { r: 0.792156862745098, g: 0.6078431372549019, b: 0.9686274509803922, a: 1.0 }
    }

    /// COCOA
    #[classattr]
    fn COCOA() -> Color {
        Color { r: 0.5294117647058824, g: 0.37254901960784315, b: 0.25882352941176473, a: 1.0 }
    }

    /// LIGHT_ROYAL_BLUE
    #[classattr]
    fn LIGHT_ROYAL_BLUE() -> Color {
        Color { r: 0.22745098039215686, g: 0.1803921568627451, b: 0.996078431372549, a: 1.0 }
    }

    /// ORANGEISH
    #[classattr]
    fn ORANGEISH() -> Color {
        Color { r: 0.9921568627450981, g: 0.5529411764705883, b: 0.28627450980392155, a: 1.0 }
    }

    /// RUST_BROWN
    #[classattr]
    fn RUST_BROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.19215686274509805, b: 0.011764705882352941, a: 1.0 }
    }

    /// SAND_BROWN
    #[classattr]
    fn SAND_BROWN() -> Color {
        Color { r: 0.796078431372549, g: 0.6470588235294118, b: 0.3764705882352941, a: 1.0 }
    }

    /// SWAMP
    #[classattr]
    fn SWAMP() -> Color {
        Color { r: 0.4117647058823529, g: 0.5137254901960784, b: 0.2235294117647059, a: 1.0 }
    }

    /// TEALISH_GREEN
    #[classattr]
    fn TEALISH_GREEN() -> Color {
        Color { r: 0.047058823529411764, g: 0.8627450980392157, b: 0.45098039215686275, a: 1.0 }
    }

    /// BURNT_SIENA
    #[classattr]
    fn BURNT_SIENA() -> Color {
        Color { r: 0.7176470588235294, g: 0.3215686274509804, b: 0.011764705882352941, a: 1.0 }
    }

    /// CAMO
    #[classattr]
    fn CAMO() -> Color {
        Color { r: 0.4980392156862745, g: 0.5607843137254902, b: 0.3058823529411765, a: 1.0 }
    }

    /// DUSK_BLUE
    #[classattr]
    fn DUSK_BLUE() -> Color {
        Color { r: 0.14901960784313725, g: 0.3254901960784314, b: 0.5529411764705883, a: 1.0 }
    }

    /// FERN
    #[classattr]
    fn FERN() -> Color {
        Color { r: 0.38823529411764707, g: 0.6627450980392157, b: 0.3137254901960784, a: 1.0 }
    }

    /// OLD_ROSE
    #[classattr]
    fn OLD_ROSE() -> Color {
        Color { r: 0.7843137254901961, g: 0.4980392156862745, b: 0.5372549019607843, a: 1.0 }
    }

    /// PALE_LIGHT_GREEN
    #[classattr]
    fn PALE_LIGHT_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 0.9882352941176471, b: 0.6, a: 1.0 }
    }

    /// PEACHY_PINK
    #[classattr]
    fn PEACHY_PINK() -> Color {
        Color { r: 1.0, g: 0.6039215686274509, b: 0.5411764705882353, a: 1.0 }
    }

    /// ROSY_PINK
    #[classattr]
    fn ROSY_PINK() -> Color {
        Color { r: 0.9647058823529412, g: 0.40784313725490196, b: 0.5568627450980392, a: 1.0 }
    }

    /// LIGHT_BLUISH_GREEN
    #[classattr]
    fn LIGHT_BLUISH_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.9921568627450981, b: 0.6588235294117647, a: 1.0 }
    }

    /// LIGHT_BRIGHT_GREEN
    #[classattr]
    fn LIGHT_BRIGHT_GREEN() -> Color {
        Color { r: 0.3254901960784314, g: 0.996078431372549, b: 0.3607843137254902, a: 1.0 }
    }

    /// LIGHT_NEON_GREEN
    #[classattr]
    fn LIGHT_NEON_GREEN() -> Color {
        Color { r: 0.3058823529411765, g: 0.9921568627450981, b: 0.32941176470588235, a: 1.0 }
    }

    /// LIGHT_SEAFOAM
    #[classattr]
    fn LIGHT_SEAFOAM() -> Color {
        Color { r: 0.6274509803921569, g: 0.996078431372549, b: 0.7490196078431373, a: 1.0 }
    }

    /// TIFFANY_BLUE
    #[classattr]
    fn TIFFANY_BLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.9490196078431372, b: 0.8549019607843137, a: 1.0 }
    }

    /// WASHED_OUT_GREEN
    #[classattr]
    fn WASHED_OUT_GREEN() -> Color {
        Color { r: 0.7372549019607844, g: 0.9607843137254902, b: 0.6509803921568628, a: 1.0 }
    }

    /// BROWNY_ORANGE
    #[classattr]
    fn BROWNY_ORANGE() -> Color {
        Color { r: 0.792156862745098, g: 0.4196078431372549, b: 0.00784313725490196, a: 1.0 }
    }

    /// NICE_BLUE
    #[classattr]
    fn NICE_BLUE() -> Color {
        Color { r: 0.06274509803921569, g: 0.47843137254901963, b: 0.6901960784313725, a: 1.0 }
    }

    /// SAPPHIRE
    #[classattr]
    fn SAPPHIRE() -> Color {
        Color { r: 0.12941176470588237, g: 0.2196078431372549, b: 0.6705882352941176, a: 1.0 }
    }

    /// GREYISH_TEAL
    #[classattr]
    fn GREYISH_TEAL() -> Color {
        Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 }
    }

    /// ORANGEY_YELLOW
    #[classattr]
    fn ORANGEY_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 0.7254901960784313, b: 0.08235294117647059, a: 1.0 }
    }

    /// PARCHMENT
    #[classattr]
    fn PARCHMENT() -> Color {
        Color { r: 0.996078431372549, g: 0.9882352941176471, b: 0.6862745098039216, a: 1.0 }
    }

    /// STRAW
    #[classattr]
    fn STRAW() -> Color {
        Color { r: 0.9882352941176471, g: 0.9647058823529412, b: 0.4745098039215686, a: 1.0 }
    }

    /// VERY_DARK_BROWN
    #[classattr]
    fn VERY_DARK_BROWN() -> Color {
        Color { r: 0.11372549019607843, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// TERRACOTA
    #[classattr]
    fn TERRACOTA() -> Color {
        Color { r: 0.796078431372549, g: 0.40784313725490196, b: 0.2627450980392157, a: 1.0 }
    }

    /// UGLY_BLUE
    #[classattr]
    fn UGLY_BLUE() -> Color {
        Color { r: 0.19215686274509805, g: 0.4, b: 0.5411764705882353, a: 1.0 }
    }

    /// CLEAR_BLUE
    #[classattr]
    fn CLEAR_BLUE() -> Color {
        Color { r: 0.1411764705882353, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 }
    }

    /// CREME
    #[classattr]
    fn CREME() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.7137254901960784, a: 1.0 }
    }

    /// FOAM_GREEN
    #[classattr]
    fn FOAM_GREEN() -> Color {
        Color { r: 0.5647058823529412, g: 0.9921568627450981, b: 0.6627450980392157, a: 1.0 }
    }

    /// GREY_GREEN
    #[classattr]
    fn GREY_GREEN() -> Color {
        Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 }
    }

    /// LIGHT_GOLD
    #[classattr]
    fn LIGHT_GOLD() -> Color {
        Color { r: 0.9921568627450981, g: 0.8627450980392157, b: 0.3607843137254902, a: 1.0 }
    }

    /// SEAFOAM_BLUE
    #[classattr]
    fn SEAFOAM_BLUE() -> Color {
        Color { r: 0.47058823529411764, g: 0.8196078431372549, b: 0.7137254901960784, a: 1.0 }
    }

    /// TOPAZ
    #[classattr]
    fn TOPAZ() -> Color {
        Color { r: 0.07450980392156863, g: 0.7333333333333333, b: 0.6862745098039216, a: 1.0 }
    }

    /// VIOLET_PINK
    #[classattr]
    fn VIOLET_PINK() -> Color {
        Color { r: 0.984313725490196, g: 0.37254901960784315, b: 0.9882352941176471, a: 1.0 }
    }

    /// WINTERGREEN
    #[classattr]
    fn WINTERGREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.9764705882352941, b: 0.5254901960784314, a: 1.0 }
    }

    /// YELLOW_TAN
    #[classattr]
    fn YELLOW_TAN() -> Color {
        Color { r: 1.0, g: 0.8901960784313725, b: 0.43137254901960786, a: 1.0 }
    }

    /// DARK_FUCHSIA
    #[classattr]
    fn DARK_FUCHSIA() -> Color {
        Color { r: 0.615686274509804, g: 0.027450980392156862, b: 0.34901960784313724, a: 1.0 }
    }

    /// INDIGO_BLUE
    #[classattr]
    fn INDIGO_BLUE() -> Color {
        Color { r: 0.22745098039215686, g: 0.09411764705882353, b: 0.6941176470588235, a: 1.0 }
    }

    /// LIGHT_YELLOWISH_GREEN
    #[classattr]
    fn LIGHT_YELLOWISH_GREEN() -> Color {
        Color { r: 0.7607843137254902, g: 1.0, b: 0.5372549019607843, a: 1.0 }
    }

    /// PALE_MAGENTA
    #[classattr]
    fn PALE_MAGENTA() -> Color {
        Color { r: 0.8431372549019608, g: 0.403921568627451, b: 0.6784313725490196, a: 1.0 }
    }

    /// RICH_PURPLE
    #[classattr]
    fn RICH_PURPLE() -> Color {
        Color { r: 0.4470588235294118, g: 0.0, b: 0.34509803921568627, a: 1.0 }
    }

    /// SUNFLOWER_YELLOW
    #[classattr]
    fn SUNFLOWER_YELLOW() -> Color {
        Color { r: 1.0, g: 0.8549019607843137, b: 0.011764705882352941, a: 1.0 }
    }

    /// GREEN_BLUE
    #[classattr]
    fn GREEN_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.7529411764705882, b: 0.5529411764705883, a: 1.0 }
    }

    /// LEATHER
    #[classattr]
    fn LEATHER() -> Color {
        Color { r: 0.6745098039215687, g: 0.4549019607843137, b: 0.20392156862745098, a: 1.0 }
    }

    /// RACING_GREEN
    #[classattr]
    fn RACING_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.27450980392156865, b: 0.0, a: 1.0 }
    }

    /// VIVID_PURPLE
    #[classattr]
    fn VIVID_PURPLE() -> Color {
        Color { r: 0.6, g: 0.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// DARK_ROYAL_BLUE
    #[classattr]
    fn DARK_ROYAL_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.023529411764705882, b: 0.43529411764705883, a: 1.0 }
    }

    /// HAZEL
    #[classattr]
    fn HAZEL() -> Color {
        Color { r: 0.5568627450980392, g: 0.4627450980392157, b: 0.09411764705882353, a: 1.0 }
    }

    /// MUTED_PINK
    #[classattr]
    fn MUTED_PINK() -> Color {
        Color { r: 0.8196078431372549, g: 0.4627450980392157, b: 0.5607843137254902, a: 1.0 }
    }

    /// BOOGER_GREEN
    #[classattr]
    fn BOOGER_GREEN() -> Color {
        Color { r: 0.5882352941176471, g: 0.7058823529411765, b: 0.011764705882352941, a: 1.0 }
    }

    /// CANARY
    #[classattr]
    fn CANARY() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.38823529411764707, a: 1.0 }
    }

    /// COOL_GREY
    #[classattr]
    fn COOL_GREY() -> Color {
        Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_TAUPE
    #[classattr]
    fn DARK_TAUPE() -> Color {
        Color { r: 0.4980392156862745, g: 0.40784313725490196, b: 0.3058823529411765, a: 1.0 }
    }

    /// DARKISH_PURPLE
    #[classattr]
    fn DARKISH_PURPLE() -> Color {
        Color { r: 0.4588235294117647, g: 0.09803921568627451, b: 0.45098039215686275, a: 1.0 }
    }

    /// TRUE_GREEN
    #[classattr]
    fn TRUE_GREEN() -> Color {
        Color { r: 0.03137254901960784, g: 0.5803921568627451, b: 0.01568627450980392, a: 1.0 }
    }

    /// CORAL_PINK
    #[classattr]
    fn CORAL_PINK() -> Color {
        Color { r: 1.0, g: 0.3803921568627451, b: 0.38823529411764707, a: 1.0 }
    }

    /// DARK_SAGE
    #[classattr]
    fn DARK_SAGE() -> Color {
        Color { r: 0.34901960784313724, g: 0.5215686274509804, b: 0.33725490196078434, a: 1.0 }
    }

    /// DARK_SLATE_BLUE
    #[classattr]
    fn DARK_SLATE_BLUE() -> Color {
        Color { r: 0.12941176470588237, g: 0.2784313725490196, b: 0.3803921568627451, a: 1.0 }
    }

    /// FLAT_BLUE
    #[classattr]
    fn FLAT_BLUE() -> Color {
        Color { r: 0.23529411764705882, g: 0.45098039215686275, b: 0.6588235294117647, a: 1.0 }
    }

    /// MUSHROOM
    #[classattr]
    fn MUSHROOM() -> Color {
        Color { r: 0.7294117647058823, g: 0.6196078431372549, b: 0.5333333333333333, a: 1.0 }
    }

    /// RICH_BLUE
    #[classattr]
    fn RICH_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.10588235294117647, b: 0.9764705882352941, a: 1.0 }
    }

    /// DIRTY_PURPLE
    #[classattr]
    fn DIRTY_PURPLE() -> Color {
        Color { r: 0.45098039215686275, g: 0.2901960784313726, b: 0.396078431372549, a: 1.0 }
    }

    /// GREENBLUE
    #[classattr]
    fn GREENBLUE() -> Color {
        Color { r: 0.13725490196078433, g: 0.7686274509803922, b: 0.5450980392156862, a: 1.0 }
    }

    /// ICKY_GREEN
    #[classattr]
    fn ICKY_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.6823529411764706, b: 0.13333333333333333, a: 1.0 }
    }

    /// LIGHT_KHAKI
    #[classattr]
    fn LIGHT_KHAKI() -> Color {
        Color { r: 0.9019607843137255, g: 0.9490196078431372, b: 0.6352941176470588, a: 1.0 }
    }

    /// WARM_BLUE
    #[classattr]
    fn WARM_BLUE() -> Color {
        Color { r: 0.29411764705882354, g: 0.3411764705882353, b: 0.8588235294117647, a: 1.0 }
    }

    /// DARK_HOT_PINK
    #[classattr]
    fn DARK_HOT_PINK() -> Color {
        Color { r: 0.8509803921568627, g: 0.00392156862745098, b: 0.4, a: 1.0 }
    }

    /// DEEP_SEA_BLUE
    #[classattr]
    fn DEEP_SEA_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.32941176470588235, b: 0.5098039215686274, a: 1.0 }
    }

    /// CARMINE
    #[classattr]
    fn CARMINE() -> Color {
        Color { r: 0.615686274509804, g: 0.00784313725490196, b: 0.08627450980392157, a: 1.0 }
    }

    /// DARK_YELLOW_GREEN
    #[classattr]
    fn DARK_YELLOW_GREEN() -> Color {
        Color { r: 0.4470588235294118, g: 0.5607843137254902, b: 0.00784313725490196, a: 1.0 }
    }

    /// PALE_PEACH
    #[classattr]
    fn PALE_PEACH() -> Color {
        Color { r: 1.0, g: 0.8980392156862745, b: 0.6784313725490196, a: 1.0 }
    }

    /// PLUM_PURPLE
    #[classattr]
    fn PLUM_PURPLE() -> Color {
        Color { r: 0.3058823529411765, g: 0.0196078431372549, b: 0.3137254901960784, a: 1.0 }
    }

    /// GOLDEN_ROD
    #[classattr]
    fn GOLDEN_ROD() -> Color {
        Color { r: 0.9764705882352941, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 }
    }

    /// NEON_RED
    #[classattr]
    fn NEON_RED() -> Color {
        Color { r: 1.0, g: 0.027450980392156862, b: 0.22745098039215686, a: 1.0 }
    }

    /// OLD_PINK
    #[classattr]
    fn OLD_PINK() -> Color {
        Color { r: 0.7803921568627451, g: 0.4745098039215686, b: 0.5254901960784314, a: 1.0 }
    }

    /// VERY_PALE_BLUE
    #[classattr]
    fn VERY_PALE_BLUE() -> Color {
        Color { r: 0.8392156862745098, g: 1.0, b: 0.996078431372549, a: 1.0 }
    }

    /// BLOOD_ORANGE
    #[classattr]
    fn BLOOD_ORANGE() -> Color {
        Color { r: 0.996078431372549, g: 0.29411764705882354, b: 0.011764705882352941, a: 1.0 }
    }

    /// GRAPEFRUIT
    #[classattr]
    fn GRAPEFRUIT() -> Color {
        Color { r: 0.9921568627450981, g: 0.34901960784313724, b: 0.33725490196078434, a: 1.0 }
    }

    /// SAND_YELLOW
    #[classattr]
    fn SAND_YELLOW() -> Color {
        Color { r: 0.9882352941176471, g: 0.8823529411764706, b: 0.4, a: 1.0 }
    }

    /// CLAY_BROWN
    #[classattr]
    fn CLAY_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.44313725490196076, b: 0.23921568627450981, a: 1.0 }
    }

    /// DARK_BLUE_GREY
    #[classattr]
    fn DARK_BLUE_GREY() -> Color {
        Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 }
    }

    /// FLAT_GREEN
    #[classattr]
    fn FLAT_GREEN() -> Color {
        Color { r: 0.4117647058823529, g: 0.615686274509804, b: 0.2980392156862745, a: 1.0 }
    }

    /// LIGHT_GREEN_BLUE
    #[classattr]
    fn LIGHT_GREEN_BLUE() -> Color {
        Color { r: 0.33725490196078434, g: 0.9882352941176471, b: 0.6352941176470588, a: 1.0 }
    }

    /// WARM_PINK
    #[classattr]
    fn WARM_PINK() -> Color {
        Color { r: 0.984313725490196, g: 0.3333333333333333, b: 0.5058823529411764, a: 1.0 }
    }

    /// DODGER_BLUE
    #[classattr]
    fn DODGER_BLUE() -> Color {
        Color { r: 0.24313725490196078, g: 0.5098039215686274, b: 0.9882352941176471, a: 1.0 }
    }

    /// GROSS_GREEN
    #[classattr]
    fn GROSS_GREEN() -> Color {
        Color { r: 0.6274509803921569, g: 0.7490196078431373, b: 0.08627450980392157, a: 1.0 }
    }

    /// ICE
    #[classattr]
    fn ICE() -> Color {
        Color { r: 0.8392156862745098, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// METALLIC_BLUE
    #[classattr]
    fn METALLIC_BLUE() -> Color {
        Color { r: 0.30980392156862746, g: 0.45098039215686275, b: 0.5568627450980392, a: 1.0 }
    }

    /// PALE_SALMON
    #[classattr]
    fn PALE_SALMON() -> Color {
        Color { r: 1.0, g: 0.6941176470588235, b: 0.6039215686274509, a: 1.0 }
    }

    /// SAP_GREEN
    #[classattr]
    fn SAP_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.5450980392156862, b: 0.08235294117647059, a: 1.0 }
    }

    /// ALGAE
    #[classattr]
    fn ALGAE() -> Color {
        Color { r: 0.32941176470588235, g: 0.6745098039215687, b: 0.40784313725490196, a: 1.0 }
    }

    /// BLUEY_GREY
    #[classattr]
    fn BLUEY_GREY() -> Color {
        Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 }
    }

    /// GREENY_GREY
    #[classattr]
    fn GREENY_GREY() -> Color {
        Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// HIGHLIGHTER_GREEN
    #[classattr]
    fn HIGHLIGHTER_GREEN() -> Color {
        Color { r: 0.10588235294117647, g: 0.9882352941176471, b: 0.023529411764705882, a: 1.0 }
    }

    /// LIGHT_LIGHT_BLUE
    #[classattr]
    fn LIGHT_LIGHT_BLUE() -> Color {
        Color { r: 0.792156862745098, g: 1.0, b: 0.984313725490196, a: 1.0 }
    }

    /// LIGHT_MINT
    #[classattr]
    fn LIGHT_MINT() -> Color {
        Color { r: 0.7137254901960784, g: 1.0, b: 0.7333333333333333, a: 1.0 }
    }

    /// RAW_UMBER
    #[classattr]
    fn RAW_UMBER() -> Color {
        Color { r: 0.6549019607843137, g: 0.3686274509803922, b: 0.03529411764705882, a: 1.0 }
    }

    /// VIVID_BLUE
    #[classattr]
    fn VIVID_BLUE() -> Color {
        Color { r: 0.08235294117647059, g: 0.1803921568627451, b: 1.0, a: 1.0 }
    }

    /// DEEP_LAVENDER
    #[classattr]
    fn DEEP_LAVENDER() -> Color {
        Color { r: 0.5529411764705883, g: 0.3686274509803922, b: 0.7176470588235294, a: 1.0 }
    }

    /// DULL_TEAL
    #[classattr]
    fn DULL_TEAL() -> Color {
        Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.5607843137254902, a: 1.0 }
    }

    /// LIGHT_GREENISH_BLUE
    #[classattr]
    fn LIGHT_GREENISH_BLUE() -> Color {
        Color { r: 0.38823529411764707, g: 0.9686274509803922, b: 0.7058823529411765, a: 1.0 }
    }

    /// MUD_GREEN
    #[classattr]
    fn MUD_GREEN() -> Color {
        Color { r: 0.3764705882352941, g: 0.4, b: 0.00784313725490196, a: 1.0 }
    }

    /// PINKY
    #[classattr]
    fn PINKY() -> Color {
        Color { r: 0.9882352941176471, g: 0.5254901960784314, b: 0.6666666666666666, a: 1.0 }
    }

    /// RED_WINE
    #[classattr]
    fn RED_WINE() -> Color {
        Color { r: 0.5490196078431373, g: 0.0, b: 0.20392156862745098, a: 1.0 }
    }

    /// SHIT_GREEN
    #[classattr]
    fn SHIT_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.5019607843137255, b: 0.0, a: 1.0 }
    }

    /// TAN_BROWN
    #[classattr]
    fn TAN_BROWN() -> Color {
        Color { r: 0.6705882352941176, g: 0.49411764705882355, b: 0.2980392156862745, a: 1.0 }
    }

    /// DARKBLUE
    #[classattr]
    fn DARKBLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.027450980392156862, b: 0.39215686274509803, a: 1.0 }
    }

    /// ROSA
    #[classattr]
    fn ROSA() -> Color {
        Color { r: 0.996078431372549, g: 0.5254901960784314, b: 0.6431372549019608, a: 1.0 }
    }

    /// LIPSTICK
    #[classattr]
    fn LIPSTICK() -> Color {
        Color { r: 0.8352941176470589, g: 0.09019607843137255, b: 0.3058823529411765, a: 1.0 }
    }

    /// PALE_MAUVE
    #[classattr]
    fn PALE_MAUVE() -> Color {
        Color { r: 0.996078431372549, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 }
    }

    /// CLARET
    #[classattr]
    fn CLARET() -> Color {
        Color { r: 0.40784313725490196, g: 0.0, b: 0.09411764705882353, a: 1.0 }
    }

    /// DANDELION
    #[classattr]
    fn DANDELION() -> Color {
        Color { r: 0.996078431372549, g: 0.8745098039215686, b: 0.03137254901960784, a: 1.0 }
    }

    /// ORANGERED
    #[classattr]
    fn ORANGERED() -> Color {
        Color { r: 0.996078431372549, g: 0.25882352941176473, b: 0.058823529411764705, a: 1.0 }
    }

    /// POOP_GREEN
    #[classattr]
    fn POOP_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.48627450980392156, b: 0.0, a: 1.0 }
    }

    /// RUBY
    #[classattr]
    fn RUBY() -> Color {
        Color { r: 0.792156862745098, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 }
    }

    /// DARK
    #[classattr]
    fn DARK() -> Color {
        Color { r: 0.10588235294117647, g: 0.1411764705882353, b: 0.19215686274509805, a: 1.0 }
    }

    /// GREENISH_TURQUOISE
    #[classattr]
    fn GREENISH_TURQUOISE() -> Color {
        Color { r: 0.0, g: 0.984313725490196, b: 0.6901960784313725, a: 1.0 }
    }

    /// PASTEL_RED
    #[classattr]
    fn PASTEL_RED() -> Color {
        Color { r: 0.8588235294117647, g: 0.34509803921568627, b: 0.33725490196078434, a: 1.0 }
    }

    /// PISS_YELLOW
    #[classattr]
    fn PISS_YELLOW() -> Color {
        Color { r: 0.8666666666666667, g: 0.8392156862745098, b: 0.09411764705882353, a: 1.0 }
    }

    /// BRIGHT_CYAN
    #[classattr]
    fn BRIGHT_CYAN() -> Color {
        Color { r: 0.2549019607843137, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// DARK_CORAL
    #[classattr]
    fn DARK_CORAL() -> Color {
        Color { r: 0.8117647058823529, g: 0.3215686274509804, b: 0.3058823529411765, a: 1.0 }
    }

    /// ALGAE_GREEN
    #[classattr]
    fn ALGAE_GREEN() -> Color {
        Color { r: 0.12941176470588237, g: 0.7647058823529411, b: 0.43529411764705883, a: 1.0 }
    }

    /// DARKISH_RED
    #[classattr]
    fn DARKISH_RED() -> Color {
        Color { r: 0.6627450980392157, g: 0.011764705882352941, b: 0.03137254901960784, a: 1.0 }
    }

    /// REDDY_BROWN
    #[classattr]
    fn REDDY_BROWN() -> Color {
        Color { r: 0.43137254901960786, g: 0.06274509803921569, b: 0.0196078431372549, a: 1.0 }
    }

    /// BLUSH_PINK
    #[classattr]
    fn BLUSH_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.5098039215686274, b: 0.5490196078431373, a: 1.0 }
    }

    /// CAMOUFLAGE_GREEN
    #[classattr]
    fn CAMOUFLAGE_GREEN() -> Color {
        Color { r: 0.29411764705882354, g: 0.3803921568627451, b: 0.07450980392156863, a: 1.0 }
    }

    /// LAWN_GREEN
    #[classattr]
    fn LAWN_GREEN() -> Color {
        Color { r: 0.30196078431372547, g: 0.6431372549019608, b: 0.03529411764705882, a: 1.0 }
    }

    /// PUTTY
    #[classattr]
    fn PUTTY() -> Color {
        Color { r: 0.7450980392156863, g: 0.6823529411764706, b: 0.5411764705882353, a: 1.0 }
    }

    /// VIBRANT_BLUE
    #[classattr]
    fn VIBRANT_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.2235294117647059, b: 0.9725490196078431, a: 1.0 }
    }

    /// DARK_SAND
    #[classattr]
    fn DARK_SAND() -> Color {
        Color { r: 0.6588235294117647, g: 0.5607843137254902, b: 0.34901960784313724, a: 1.0 }
    }

    /// PURPLE_BLUE
    #[classattr]
    fn PURPLE_BLUE() -> Color {
        Color { r: 0.36470588235294116, g: 0.12941176470588237, b: 0.8156862745098039, a: 1.0 }
    }

    /// SAFFRON
    #[classattr]
    fn SAFFRON() -> Color {
        Color { r: 0.996078431372549, g: 0.6980392156862745, b: 0.03529411764705882, a: 1.0 }
    }

    /// TWILIGHT
    #[classattr]
    fn TWILIGHT() -> Color {
        Color { r: 0.3058823529411765, g: 0.3176470588235294, b: 0.5450980392156862, a: 1.0 }
    }

    /// WARM_BROWN
    #[classattr]
    fn WARM_BROWN() -> Color {
        Color { r: 0.5882352941176471, g: 0.3058823529411765, b: 0.00784313725490196, a: 1.0 }
    }

    /// BLUEGREY
    #[classattr]
    fn BLUEGREY() -> Color {
        Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 }
    }

    /// BUBBLE_GUM_PINK
    #[classattr]
    fn BUBBLE_GUM_PINK() -> Color {
        Color { r: 1.0, g: 0.4117647058823529, b: 0.6862745098039216, a: 1.0 }
    }

    /// DUCK_EGG_BLUE
    #[classattr]
    fn DUCK_EGG_BLUE() -> Color {
        Color { r: 0.7647058823529411, g: 0.984313725490196, b: 0.9568627450980393, a: 1.0 }
    }

    /// GREENISH_CYAN
    #[classattr]
    fn GREENISH_CYAN() -> Color {
        Color { r: 0.16470588235294117, g: 0.996078431372549, b: 0.7176470588235294, a: 1.0 }
    }

    /// PETROL
    #[classattr]
    fn PETROL() -> Color {
        Color { r: 0.0, g: 0.37254901960784315, b: 0.41568627450980394, a: 1.0 }
    }

    /// ROYAL
    #[classattr]
    fn ROYAL() -> Color {
        Color { r: 0.047058823529411764, g: 0.09019607843137255, b: 0.5764705882352941, a: 1.0 }
    }

    /// BUTTER
    #[classattr]
    fn BUTTER() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.5058823529411764, a: 1.0 }
    }

    /// DUSTY_ORANGE
    #[classattr]
    fn DUSTY_ORANGE() -> Color {
        Color { r: 0.9411764705882353, g: 0.5137254901960784, b: 0.22745098039215686, a: 1.0 }
    }

    /// OFF_YELLOW
    #[classattr]
    fn OFF_YELLOW() -> Color {
        Color { r: 0.9450980392156862, g: 0.9529411764705882, b: 0.24705882352941178, a: 1.0 }
    }

    /// PALE_OLIVE_GREEN
    #[classattr]
    fn PALE_OLIVE_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 0.8235294117647058, b: 0.4823529411764706, a: 1.0 }
    }

    /// ORANGISH
    #[classattr]
    fn ORANGISH() -> Color {
        Color { r: 0.9882352941176471, g: 0.5098039215686274, b: 0.2901960784313726, a: 1.0 }
    }

    /// LEAF
    #[classattr]
    fn LEAF() -> Color {
        Color { r: 0.44313725490196076, g: 0.6666666666666666, b: 0.20392156862745098, a: 1.0 }
    }

    /// LIGHT_BLUE_GREY
    #[classattr]
    fn LIGHT_BLUE_GREY() -> Color {
        Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 }
    }

    /// DRIED_BLOOD
    #[classattr]
    fn DRIED_BLOOD() -> Color {
        Color { r: 0.29411764705882354, g: 0.00392156862745098, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHTISH_PURPLE
    #[classattr]
    fn LIGHTISH_PURPLE() -> Color {
        Color { r: 0.6470588235294118, g: 0.3215686274509804, b: 0.9019607843137255, a: 1.0 }
    }

    /// RUSTY_RED
    #[classattr]
    fn RUSTY_RED() -> Color {
        Color { r: 0.6862745098039216, g: 0.1843137254901961, b: 0.050980392156862744, a: 1.0 }
    }

    /// LAVENDER_BLUE
    #[classattr]
    fn LAVENDER_BLUE() -> Color {
        Color { r: 0.5450980392156862, g: 0.5333333333333333, b: 0.9725490196078431, a: 1.0 }
    }

    /// LIGHT_GRASS_GREEN
    #[classattr]
    fn LIGHT_GRASS_GREEN() -> Color {
        Color { r: 0.6039215686274509, g: 0.9686274509803922, b: 0.39215686274509803, a: 1.0 }
    }

    /// LIGHT_MINT_GREEN
    #[classattr]
    fn LIGHT_MINT_GREEN() -> Color {
        Color { r: 0.6509803921568628, g: 0.984313725490196, b: 0.6980392156862745, a: 1.0 }
    }

    /// SUNFLOWER
    #[classattr]
    fn SUNFLOWER() -> Color {
        Color { r: 1.0, g: 0.7725490196078432, b: 0.07058823529411765, a: 1.0 }
    }

    /// VELVET
    #[classattr]
    fn VELVET() -> Color {
        Color { r: 0.4588235294117647, g: 0.03137254901960784, b: 0.3176470588235294, a: 1.0 }
    }

    /// BRICK_ORANGE
    #[classattr]
    fn BRICK_ORANGE() -> Color {
        Color { r: 0.7568627450980392, g: 0.2901960784313726, b: 0.03529411764705882, a: 1.0 }
    }

    /// LIGHTISH_RED
    #[classattr]
    fn LIGHTISH_RED() -> Color {
        Color { r: 0.996078431372549, g: 0.1843137254901961, b: 0.2901960784313726, a: 1.0 }
    }

    /// PURE_BLUE
    #[classattr]
    fn PURE_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.011764705882352941, b: 0.8862745098039215, a: 1.0 }
    }

    /// TWILIGHT_BLUE
    #[classattr]
    fn TWILIGHT_BLUE() -> Color {
        Color { r: 0.0392156862745098, g: 0.2627450980392157, b: 0.47843137254901963, a: 1.0 }
    }

    /// VIOLET_RED
    #[classattr]
    fn VIOLET_RED() -> Color {
        Color { r: 0.6470588235294118, g: 0.0, b: 0.3333333333333333, a: 1.0 }
    }

    /// YELLOWY_BROWN
    #[classattr]
    fn YELLOWY_BROWN() -> Color {
        Color { r: 0.6823529411764706, g: 0.5450980392156862, b: 0.047058823529411764, a: 1.0 }
    }

    /// CARNATION
    #[classattr]
    fn CARNATION() -> Color {
        Color { r: 0.9921568627450981, g: 0.4745098039215686, b: 0.5607843137254902, a: 1.0 }
    }

    /// MUDDY_YELLOW
    #[classattr]
    fn MUDDY_YELLOW() -> Color {
        Color { r: 0.7490196078431373, g: 0.6745098039215687, b: 0.0196078431372549, a: 1.0 }
    }

    /// DARK_SEAFOAM_GREEN
    #[classattr]
    fn DARK_SEAFOAM_GREEN() -> Color {
        Color { r: 0.24313725490196078, g: 0.6862745098039216, b: 0.4627450980392157, a: 1.0 }
    }

    /// DEEP_ROSE
    #[classattr]
    fn DEEP_ROSE() -> Color {
        Color { r: 0.7803921568627451, g: 0.2784313725490196, b: 0.403921568627451, a: 1.0 }
    }

    /// DUSTY_RED
    #[classattr]
    fn DUSTY_RED() -> Color {
        Color { r: 0.7254901960784313, g: 0.2823529411764706, b: 0.3058823529411765, a: 1.0 }
    }

    /// GREY_BLUE
    #[classattr]
    fn GREY_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 }
    }

    /// LEMON_LIME
    #[classattr]
    fn LEMON_LIME() -> Color {
        Color { r: 0.7490196078431373, g: 0.996078431372549, b: 0.1568627450980392, a: 1.0 }
    }

    /// PURPLE_PINK
    #[classattr]
    fn PURPLE_PINK() -> Color {
        Color { r: 0.8431372549019608, g: 0.1450980392156863, b: 0.8705882352941177, a: 1.0 }
    }

    /// BROWN_YELLOW
    #[classattr]
    fn BROWN_YELLOW() -> Color {
        Color { r: 0.6980392156862745, g: 0.592156862745098, b: 0.0196078431372549, a: 1.0 }
    }

    /// PURPLE_BROWN
    #[classattr]
    fn PURPLE_BROWN() -> Color {
        Color { r: 0.403921568627451, g: 0.22745098039215686, b: 0.24705882352941178, a: 1.0 }
    }

    /// WISTERIA
    #[classattr]
    fn WISTERIA() -> Color {
        Color { r: 0.6588235294117647, g: 0.49019607843137253, b: 0.7607843137254902, a: 1.0 }
    }

    /// BANANA_YELLOW
    #[classattr]
    fn BANANA_YELLOW() -> Color {
        Color { r: 0.9803921568627451, g: 0.996078431372549, b: 0.29411764705882354, a: 1.0 }
    }

    /// LIPSTICK_RED
    #[classattr]
    fn LIPSTICK_RED() -> Color {
        Color { r: 0.7529411764705882, g: 0.00784313725490196, b: 0.1843137254901961, a: 1.0 }
    }

    /// WATER_BLUE
    #[classattr]
    fn WATER_BLUE() -> Color {
        Color { r: 0.054901960784313725, g: 0.5294117647058824, b: 0.8, a: 1.0 }
    }

    /// BROWN_GREY
    #[classattr]
    fn BROWN_GREY() -> Color {
        Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 }
    }

    /// VIBRANT_PURPLE
    #[classattr]
    fn VIBRANT_PURPLE() -> Color {
        Color { r: 0.6784313725490196, g: 0.011764705882352941, b: 0.8705882352941177, a: 1.0 }
    }

    /// BABY_GREEN
    #[classattr]
    fn BABY_GREEN() -> Color {
        Color { r: 0.5490196078431373, g: 1.0, b: 0.6196078431372549, a: 1.0 }
    }

    /// BARF_GREEN
    #[classattr]
    fn BARF_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6745098039215687, b: 0.00784313725490196, a: 1.0 }
    }

    /// EGGSHELL_BLUE
    #[classattr]
    fn EGGSHELL_BLUE() -> Color {
        Color { r: 0.7686274509803922, g: 1.0, b: 0.9686274509803922, a: 1.0 }
    }

    /// SANDY_YELLOW
    #[classattr]
    fn SANDY_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 0.9333333333333333, b: 0.45098039215686275, a: 1.0 }
    }

    /// COOL_GREEN
    #[classattr]
    fn COOL_GREEN() -> Color {
        Color { r: 0.2, g: 0.7215686274509804, b: 0.39215686274509803, a: 1.0 }
    }

    /// PALE
    #[classattr]
    fn PALE() -> Color {
        Color { r: 1.0, g: 0.9764705882352941, b: 0.8156862745098039, a: 1.0 }
    }

    /// BLUE_GREY
    #[classattr]
    fn BLUE_GREY() -> Color {
        Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 }
    }

    /// HOT_MAGENTA
    #[classattr]
    fn HOT_MAGENTA() -> Color {
        Color { r: 0.9607843137254902, g: 0.01568627450980392, b: 0.788235294117647, a: 1.0 }
    }

    /// GREYBLUE
    #[classattr]
    fn GREYBLUE() -> Color {
        Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 }
    }

    /// PURPLEY
    #[classattr]
    fn PURPLEY() -> Color {
        Color { r: 0.5294117647058824, g: 0.33725490196078434, b: 0.8941176470588236, a: 1.0 }
    }

    /// BABY_SHIT_GREEN
    #[classattr]
    fn BABY_SHIT_GREEN() -> Color {
        Color { r: 0.5333333333333333, g: 0.592156862745098, b: 0.09019607843137255, a: 1.0 }
    }

    /// BROWNISH_PINK
    #[classattr]
    fn BROWNISH_PINK() -> Color {
        Color { r: 0.7607843137254902, g: 0.49411764705882355, b: 0.4745098039215686, a: 1.0 }
    }

    /// DARK_AQUAMARINE
    #[classattr]
    fn DARK_AQUAMARINE() -> Color {
        Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.44313725490196076, a: 1.0 }
    }

    /// DIARRHEA
    #[classattr]
    fn DIARRHEA() -> Color {
        Color { r: 0.6235294117647059, g: 0.5137254901960784, b: 0.011764705882352941, a: 1.0 }
    }

    /// LIGHT_MUSTARD
    #[classattr]
    fn LIGHT_MUSTARD() -> Color {
        Color { r: 0.9686274509803922, g: 0.8352941176470589, b: 0.3764705882352941, a: 1.0 }
    }

    /// PALE_SKY_BLUE
    #[classattr]
    fn PALE_SKY_BLUE() -> Color {
        Color { r: 0.7411764705882353, g: 0.9647058823529412, b: 0.996078431372549, a: 1.0 }
    }

    /// TURTLE_GREEN
    #[classattr]
    fn TURTLE_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.7215686274509804, b: 0.30980392156862746, a: 1.0 }
    }

    /// BRIGHT_OLIVE
    #[classattr]
    fn BRIGHT_OLIVE() -> Color {
        Color { r: 0.611764705882353, g: 0.7333333333333333, b: 0.01568627450980392, a: 1.0 }
    }

    /// DARK_GREY_BLUE
    #[classattr]
    fn DARK_GREY_BLUE() -> Color {
        Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 }
    }

    /// GREENY_BROWN
    #[classattr]
    fn GREENY_BROWN() -> Color {
        Color { r: 0.4117647058823529, g: 0.3764705882352941, b: 0.023529411764705882, a: 1.0 }
    }

    /// LEMON_GREEN
    #[classattr]
    fn LEMON_GREEN() -> Color {
        Color { r: 0.6784313725490196, g: 0.9725490196078431, b: 0.00784313725490196, a: 1.0 }
    }

    /// LIGHT_PERIWINKLE
    #[classattr]
    fn LIGHT_PERIWINKLE() -> Color {
        Color { r: 0.7568627450980392, g: 0.7764705882352941, b: 0.9882352941176471, a: 1.0 }
    }

    /// SEAWEED_GREEN
    #[classattr]
    fn SEAWEED_GREEN() -> Color {
        Color { r: 0.20784313725490197, g: 0.6784313725490196, b: 0.4196078431372549, a: 1.0 }
    }

    /// SUNSHINE_YELLOW
    #[classattr]
    fn SUNSHINE_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.21568627450980393, a: 1.0 }
    }

    /// UGLY_PURPLE
    #[classattr]
    fn UGLY_PURPLE() -> Color {
        Color { r: 0.6431372549019608, g: 0.25882352941176473, b: 0.6274509803921569, a: 1.0 }
    }

    /// MEDIUM_PINK
    #[classattr]
    fn MEDIUM_PINK() -> Color {
        Color { r: 0.9529411764705882, g: 0.3803921568627451, b: 0.5882352941176471, a: 1.0 }
    }

    /// PUKE_BROWN
    #[classattr]
    fn PUKE_BROWN() -> Color {
        Color { r: 0.5803921568627451, g: 0.4666666666666667, b: 0.023529411764705882, a: 1.0 }
    }

    /// VERY_LIGHT_PINK
    #[classattr]
    fn VERY_LIGHT_PINK() -> Color {
        Color { r: 1.0, g: 0.9568627450980393, b: 0.9490196078431372, a: 1.0 }
    }

    /// VIRIDIAN
    #[classattr]
    fn VIRIDIAN() -> Color {
        Color { r: 0.11764705882352941, g: 0.5686274509803921, b: 0.403921568627451, a: 1.0 }
    }

    /// BILE
    #[classattr]
    fn BILE() -> Color {
        Color { r: 0.7098039215686275, g: 0.7647058823529411, b: 0.023529411764705882, a: 1.0 }
    }

    /// FADED_YELLOW
    #[classattr]
    fn FADED_YELLOW() -> Color {
        Color { r: 0.996078431372549, g: 1.0, b: 0.4980392156862745, a: 1.0 }
    }

    /// VERY_PALE_GREEN
    #[classattr]
    fn VERY_PALE_GREEN() -> Color {
        Color { r: 0.8117647058823529, g: 0.9921568627450981, b: 0.7372549019607844, a: 1.0 }
    }

    /// VIBRANT_GREEN
    #[classattr]
    fn VIBRANT_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 0.8666666666666667, b: 0.03137254901960784, a: 1.0 }
    }

    /// BRIGHT_LIME
    #[classattr]
    fn BRIGHT_LIME() -> Color {
        Color { r: 0.5294117647058824, g: 0.9921568627450981, b: 0.0196078431372549, a: 1.0 }
    }

    /// SPEARMINT
    #[classattr]
    fn SPEARMINT() -> Color {
        Color { r: 0.11764705882352941, g: 0.9725490196078431, b: 0.4627450980392157, a: 1.0 }
    }

    /// LIGHT_AQUAMARINE
    #[classattr]
    fn LIGHT_AQUAMARINE() -> Color {
        Color { r: 0.4823529411764706, g: 0.9921568627450981, b: 0.7803921568627451, a: 1.0 }
    }

    /// LIGHT_SAGE
    #[classattr]
    fn LIGHT_SAGE() -> Color {
        Color { r: 0.7372549019607844, g: 0.9254901960784314, b: 0.6745098039215687, a: 1.0 }
    }

    /// YELLOWGREEN
    #[classattr]
    fn YELLOWGREEN() -> Color {
        Color { r: 0.7333333333333333, g: 0.9764705882352941, b: 0.058823529411764705, a: 1.0 }
    }

    /// BABY_POO
    #[classattr]
    fn BABY_POO() -> Color {
        Color { r: 0.6705882352941176, g: 0.5647058823529412, b: 0.01568627450980392, a: 1.0 }
    }

    /// DARK_SEAFOAM
    #[classattr]
    fn DARK_SEAFOAM() -> Color {
        Color { r: 0.12156862745098039, g: 0.7098039215686275, b: 0.47843137254901963, a: 1.0 }
    }

    /// DEEP_TEAL
    #[classattr]
    fn DEEP_TEAL() -> Color {
        Color { r: 0.0, g: 0.3333333333333333, b: 0.35294117647058826, a: 1.0 }
    }

    /// HEATHER
    #[classattr]
    fn HEATHER() -> Color {
        Color { r: 0.6431372549019608, g: 0.5176470588235295, b: 0.6745098039215687, a: 1.0 }
    }

    /// RUST_ORANGE
    #[classattr]
    fn RUST_ORANGE() -> Color {
        Color { r: 0.7686274509803922, g: 0.3333333333333333, b: 0.03137254901960784, a: 1.0 }
    }

    /// DIRTY_BLUE
    #[classattr]
    fn DIRTY_BLUE() -> Color {
        Color { r: 0.24705882352941178, g: 0.5098039215686274, b: 0.615686274509804, a: 1.0 }
    }

    /// FERN_GREEN
    #[classattr]
    fn FERN_GREEN() -> Color {
        Color { r: 0.32941176470588235, g: 0.5529411764705883, b: 0.26666666666666666, a: 1.0 }
    }

    /// BRIGHT_LILAC
    #[classattr]
    fn BRIGHT_LILAC() -> Color {
        Color { r: 0.788235294117647, g: 0.3686274509803922, b: 0.984313725490196, a: 1.0 }
    }

    /// WEIRD_GREEN
    #[classattr]
    fn WEIRD_GREEN() -> Color {
        Color { r: 0.22745098039215686, g: 0.8980392156862745, b: 0.4980392156862745, a: 1.0 }
    }

    /// PEACOCK_BLUE
    #[classattr]
    fn PEACOCK_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.403921568627451, b: 0.5843137254901961, a: 1.0 }
    }

    /// AVOCADO_GREEN
    #[classattr]
    fn AVOCADO_GREEN() -> Color {
        Color { r: 0.5294117647058824, g: 0.6627450980392157, b: 0.13333333333333333, a: 1.0 }
    }

    /// FADED_ORANGE
    #[classattr]
    fn FADED_ORANGE() -> Color {
        Color { r: 0.9411764705882353, g: 0.5803921568627451, b: 0.30196078431372547, a: 1.0 }
    }

    /// GRAPE_PURPLE
    #[classattr]
    fn GRAPE_PURPLE() -> Color {
        Color { r: 0.36470588235294116, g: 0.0784313725490196, b: 0.3176470588235294, a: 1.0 }
    }

    /// HOT_GREEN
    #[classattr]
    fn HOT_GREEN() -> Color {
        Color { r: 0.1450980392156863, g: 1.0, b: 0.1607843137254902, a: 1.0 }
    }

    /// LIME_YELLOW
    #[classattr]
    fn LIME_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.11372549019607843, a: 1.0 }
    }

    /// MANGO
    #[classattr]
    fn MANGO() -> Color {
        Color { r: 1.0, g: 0.6509803921568628, b: 0.16862745098039217, a: 1.0 }
    }

    /// SHAMROCK
    #[classattr]
    fn SHAMROCK() -> Color {
        Color { r: 0.00392156862745098, g: 0.7058823529411765, b: 0.2980392156862745, a: 1.0 }
    }

    /// BUBBLEGUM
    #[classattr]
    fn BUBBLEGUM() -> Color {
        Color { r: 1.0, g: 0.4235294117647059, b: 0.7098039215686275, a: 1.0 }
    }

    /// PURPLISH_BROWN
    #[classattr]
    fn PURPLISH_BROWN() -> Color {
        Color { r: 0.4196078431372549, g: 0.25882352941176473, b: 0.2784313725490196, a: 1.0 }
    }

    /// VOMIT_YELLOW
    #[classattr]
    fn VOMIT_YELLOW() -> Color {
        Color { r: 0.7803921568627451, g: 0.7568627450980392, b: 0.047058823529411764, a: 1.0 }
    }

    /// PALE_CYAN
    #[classattr]
    fn PALE_CYAN() -> Color {
        Color { r: 0.7176470588235294, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// KEY_LIME
    #[classattr]
    fn KEY_LIME() -> Color {
        Color { r: 0.6823529411764706, g: 1.0, b: 0.43137254901960786, a: 1.0 }
    }

    /// TOMATO_RED
    #[classattr]
    fn TOMATO_RED() -> Color {
        Color { r: 0.9254901960784314, g: 0.17647058823529413, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHTGREEN
    #[classattr]
    fn LIGHTGREEN() -> Color {
        Color { r: 0.4627450980392157, g: 1.0, b: 0.4823529411764706, a: 1.0 }
    }

    /// MERLOT
    #[classattr]
    fn MERLOT() -> Color {
        Color { r: 0.45098039215686275, g: 0.0, b: 0.2235294117647059, a: 1.0 }
    }

    /// NIGHT_BLUE
    #[classattr]
    fn NIGHT_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.011764705882352941, b: 0.2823529411764706, a: 1.0 }
    }

    /// PURPLEISH_PINK
    #[classattr]
    fn PURPLEISH_PINK() -> Color {
        Color { r: 0.8745098039215686, g: 0.3058823529411765, b: 0.7843137254901961, a: 1.0 }
    }

    /// APPLE
    #[classattr]
    fn APPLE() -> Color {
        Color { r: 0.43137254901960786, g: 0.796078431372549, b: 0.23529411764705882, a: 1.0 }
    }

    /// BABY_POOP_GREEN
    #[classattr]
    fn BABY_POOP_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.596078431372549, b: 0.0196078431372549, a: 1.0 }
    }

    /// GREEN_APPLE
    #[classattr]
    fn GREEN_APPLE() -> Color {
        Color { r: 0.3686274509803922, g: 0.8627450980392157, b: 0.12156862745098039, a: 1.0 }
    }

    /// HELIOTROPE
    #[classattr]
    fn HELIOTROPE() -> Color {
        Color { r: 0.8509803921568627, g: 0.30980392156862746, b: 0.9607843137254902, a: 1.0 }
    }

    /// YELLOW_GREEN
    #[classattr]
    fn YELLOW_GREEN() -> Color {
        Color { r: 0.7843137254901961, g: 0.9921568627450981, b: 0.23921568627450981, a: 1.0 }
    }

    /// ALMOST_BLACK
    #[classattr]
    fn ALMOST_BLACK() -> Color {
        Color { r: 0.027450980392156862, g: 0.050980392156862744, b: 0.050980392156862744, a: 1.0 }
    }

    /// COOL_BLUE
    #[classattr]
    fn COOL_BLUE() -> Color {
        Color { r: 0.28627450980392155, g: 0.5176470588235295, b: 0.7215686274509804, a: 1.0 }
    }

    /// LEAFY_GREEN
    #[classattr]
    fn LEAFY_GREEN() -> Color {
        Color { r: 0.3176470588235294, g: 0.7176470588235294, b: 0.23137254901960785, a: 1.0 }
    }

    /// MUSTARD_BROWN
    #[classattr]
    fn MUSTARD_BROWN() -> Color {
        Color { r: 0.6745098039215687, g: 0.49411764705882355, b: 0.01568627450980392, a: 1.0 }
    }

    /// DUSK
    #[classattr]
    fn DUSK() -> Color {
        Color { r: 0.3058823529411765, g: 0.32941176470588235, b: 0.5058823529411764, a: 1.0 }
    }

    /// DULL_BROWN
    #[classattr]
    fn DULL_BROWN() -> Color {
        Color { r: 0.5294117647058824, g: 0.43137254901960786, b: 0.29411764705882354, a: 1.0 }
    }

    /// FROG_GREEN
    #[classattr]
    fn FROG_GREEN() -> Color {
        Color { r: 0.34509803921568627, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 }
    }

    /// VIVID_GREEN
    #[classattr]
    fn VIVID_GREEN() -> Color {
        Color { r: 0.1843137254901961, g: 0.9372549019607843, b: 0.06274509803921569, a: 1.0 }
    }

    /// BRIGHT_LIGHT_GREEN
    #[classattr]
    fn BRIGHT_LIGHT_GREEN() -> Color {
        Color { r: 0.17647058823529413, g: 0.996078431372549, b: 0.32941176470588235, a: 1.0 }
    }

    /// FLURO_GREEN
    #[classattr]
    fn FLURO_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 1.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// KIWI
    #[classattr]
    fn KIWI() -> Color {
        Color { r: 0.611764705882353, g: 0.9372549019607843, b: 0.2627450980392157, a: 1.0 }
    }

    /// SEAWEED
    #[classattr]
    fn SEAWEED() -> Color {
        Color { r: 0.09411764705882353, g: 0.8196078431372549, b: 0.4823529411764706, a: 1.0 }
    }

    /// NAVY_GREEN
    #[classattr]
    fn NAVY_GREEN() -> Color {
        Color { r: 0.20784313725490197, g: 0.3254901960784314, b: 0.0392156862745098, a: 1.0 }
    }

    /// ULTRAMARINE_BLUE
    #[classattr]
    fn ULTRAMARINE_BLUE() -> Color {
        Color { r: 0.09411764705882353, g: 0.0196078431372549, b: 0.8588235294117647, a: 1.0 }
    }

    /// IRIS
    #[classattr]
    fn IRIS() -> Color {
        Color { r: 0.3843137254901961, g: 0.34509803921568627, b: 0.7686274509803922, a: 1.0 }
    }

    /// PASTEL_ORANGE
    #[classattr]
    fn PASTEL_ORANGE() -> Color {
        Color { r: 1.0, g: 0.5882352941176471, b: 0.30980392156862746, a: 1.0 }
    }

    /// YELLOWISH_ORANGE
    #[classattr]
    fn YELLOWISH_ORANGE() -> Color {
        Color { r: 1.0, g: 0.6705882352941176, b: 0.058823529411764705, a: 1.0 }
    }

    /// PERRYWINKLE
    #[classattr]
    fn PERRYWINKLE() -> Color {
        Color { r: 0.5607843137254902, g: 0.5490196078431373, b: 0.9058823529411765, a: 1.0 }
    }

    /// TEALISH
    #[classattr]
    fn TEALISH() -> Color {
        Color { r: 0.1411764705882353, g: 0.7372549019607844, b: 0.6588235294117647, a: 1.0 }
    }

    /// DARK_PLUM
    #[classattr]
    fn DARK_PLUM() -> Color {
        Color { r: 0.24705882352941178, g: 0.00392156862745098, b: 0.17254901960784313, a: 1.0 }
    }

    /// PEAR
    #[classattr]
    fn PEAR() -> Color {
        Color { r: 0.796078431372549, g: 0.9725490196078431, b: 0.37254901960784315, a: 1.0 }
    }

    /// PINKISH_ORANGE
    #[classattr]
    fn PINKISH_ORANGE() -> Color {
        Color { r: 1.0, g: 0.4470588235294118, b: 0.2980392156862745, a: 1.0 }
    }

    /// MIDNIGHT_PURPLE
    #[classattr]
    fn MIDNIGHT_PURPLE() -> Color {
        Color { r: 0.1568627450980392, g: 0.00392156862745098, b: 0.21568627450980393, a: 1.0 }
    }

    /// LIGHT_URPLE
    #[classattr]
    fn LIGHT_URPLE() -> Color {
        Color { r: 0.7019607843137254, g: 0.43529411764705883, b: 0.9647058823529412, a: 1.0 }
    }

    /// DARK_MINT
    #[classattr]
    fn DARK_MINT() -> Color {
        Color { r: 0.2823529411764706, g: 0.7529411764705882, b: 0.4470588235294118, a: 1.0 }
    }

    /// GREENISH_TAN
    #[classattr]
    fn GREENISH_TAN() -> Color {
        Color { r: 0.7372549019607844, g: 0.796078431372549, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHT_BURGUNDY
    #[classattr]
    fn LIGHT_BURGUNDY() -> Color {
        Color { r: 0.6588235294117647, g: 0.2549019607843137, b: 0.3568627450980392, a: 1.0 }
    }

    /// TURQUOISE_BLUE
    #[classattr]
    fn TURQUOISE_BLUE() -> Color {
        Color { r: 0.023529411764705882, g: 0.6941176470588235, b: 0.7686274509803922, a: 1.0 }
    }

    /// UGLY_PINK
    #[classattr]
    fn UGLY_PINK() -> Color {
        Color { r: 0.803921568627451, g: 0.4588235294117647, b: 0.5176470588235295, a: 1.0 }
    }

    /// SANDY
    #[classattr]
    fn SANDY() -> Color {
        Color { r: 0.9450980392156862, g: 0.8549019607843137, b: 0.47843137254901963, a: 1.0 }
    }

    /// ELECTRIC_PINK
    #[classattr]
    fn ELECTRIC_PINK() -> Color {
        Color { r: 1.0, g: 0.01568627450980392, b: 0.5647058823529412, a: 1.0 }
    }

    /// MUTED_PURPLE
    #[classattr]
    fn MUTED_PURPLE() -> Color {
        Color { r: 0.5019607843137255, g: 0.3568627450980392, b: 0.5294117647058824, a: 1.0 }
    }

    /// MID_GREEN
    #[classattr]
    fn MID_GREEN() -> Color {
        Color { r: 0.3137254901960784, g: 0.6549019607843137, b: 0.2784313725490196, a: 1.0 }
    }

    /// GREYISH
    #[classattr]
    fn GREYISH() -> Color {
        Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 }
    }

    /// NEON_YELLOW
    #[classattr]
    fn NEON_YELLOW() -> Color {
        Color { r: 0.8117647058823529, g: 1.0, b: 0.01568627450980392, a: 1.0 }
    }

    /// BANANA
    #[classattr]
    fn BANANA() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.49411764705882355, a: 1.0 }
    }

    /// CARNATION_PINK
    #[classattr]
    fn CARNATION_PINK() -> Color {
        Color { r: 1.0, g: 0.4980392156862745, b: 0.6549019607843137, a: 1.0 }
    }

    /// TOMATO
    #[classattr]
    fn TOMATO() -> Color {
        Color { r: 0.9372549019607843, g: 0.25098039215686274, b: 0.14901960784313725, a: 1.0 }
    }

    /// SEA
    #[classattr]
    fn SEA() -> Color {
        Color { r: 0.23529411764705882, g: 0.6, b: 0.5725490196078431, a: 1.0 }
    }

    /// MUDDY_BROWN
    #[classattr]
    fn MUDDY_BROWN() -> Color {
        Color { r: 0.5333333333333333, g: 0.40784313725490196, b: 0.023529411764705882, a: 1.0 }
    }

    /// TURQUOISE_GREEN
    #[classattr]
    fn TURQUOISE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.9568627450980393, b: 0.5372549019607843, a: 1.0 }
    }

    /// BUFF
    #[classattr]
    fn BUFF() -> Color {
        Color { r: 0.996078431372549, g: 0.9647058823529412, b: 0.6196078431372549, a: 1.0 }
    }

    /// FAWN
    #[classattr]
    fn FAWN() -> Color {
        Color { r: 0.8117647058823529, g: 0.6862745098039216, b: 0.4823529411764706, a: 1.0 }
    }

    /// MUTED_BLUE
    #[classattr]
    fn MUTED_BLUE() -> Color {
        Color { r: 0.23137254901960785, g: 0.44313725490196076, b: 0.6235294117647059, a: 1.0 }
    }

    /// PALE_ROSE
    #[classattr]
    fn PALE_ROSE() -> Color {
        Color { r: 0.9921568627450981, g: 0.7568627450980392, b: 0.7725490196078432, a: 1.0 }
    }

    /// DARK_MINT_GREEN
    #[classattr]
    fn DARK_MINT_GREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.7529411764705882, b: 0.45098039215686275, a: 1.0 }
    }

    /// AMETHYST
    #[classattr]
    fn AMETHYST() -> Color {
        Color { r: 0.6078431372549019, g: 0.37254901960784315, b: 0.7529411764705882, a: 1.0 }
    }

    /// BLUE_GREEN
    #[classattr]
    fn BLUE_GREEN() -> Color {
        Color { r: 0.058823529411764705, g: 0.6078431372549019, b: 0.5568627450980392, a: 1.0 }
    }

    /// CHESTNUT
    #[classattr]
    fn CHESTNUT() -> Color {
        Color { r: 0.4549019607843137, g: 0.1568627450980392, b: 0.00784313725490196, a: 1.0 }
    }

    /// SICK_GREEN
    #[classattr]
    fn SICK_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 0.7254901960784313, b: 0.17254901960784313, a: 1.0 }
    }

    /// PEA
    #[classattr]
    fn PEA() -> Color {
        Color { r: 0.6431372549019608, g: 0.7490196078431373, b: 0.12549019607843137, a: 1.0 }
    }

    /// RUSTY_ORANGE
    #[classattr]
    fn RUSTY_ORANGE() -> Color {
        Color { r: 0.803921568627451, g: 0.34901960784313724, b: 0.03529411764705882, a: 1.0 }
    }

    /// STONE
    #[classattr]
    fn STONE() -> Color {
        Color { r: 0.6784313725490196, g: 0.6470588235294118, b: 0.5294117647058824, a: 1.0 }
    }

    /// ROSE_RED
    #[classattr]
    fn ROSE_RED() -> Color {
        Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.23529411764705882, a: 1.0 }
    }

    /// PALE_AQUA
    #[classattr]
    fn PALE_AQUA() -> Color {
        Color { r: 0.7215686274509804, g: 1.0, b: 0.9215686274509803, a: 1.0 }
    }

    /// DEEP_ORANGE
    #[classattr]
    fn DEEP_ORANGE() -> Color {
        Color { r: 0.8627450980392157, g: 0.30196078431372547, b: 0.00392156862745098, a: 1.0 }
    }

    /// EARTH
    #[classattr]
    fn EARTH() -> Color {
        Color { r: 0.6352941176470588, g: 0.396078431372549, b: 0.24313725490196078, a: 1.0 }
    }

    /// MOSSY_GREEN
    #[classattr]
    fn MOSSY_GREEN() -> Color {
        Color { r: 0.38823529411764707, g: 0.5450980392156862, b: 0.15294117647058825, a: 1.0 }
    }

    /// GRASSY_GREEN
    #[classattr]
    fn GRASSY_GREEN() -> Color {
        Color { r: 0.2549019607843137, g: 0.611764705882353, b: 0.011764705882352941, a: 1.0 }
    }

    /// PALE_LIME_GREEN
    #[classattr]
    fn PALE_LIME_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 1.0, b: 0.396078431372549, a: 1.0 }
    }

    /// LIGHT_GREY_BLUE
    #[classattr]
    fn LIGHT_GREY_BLUE() -> Color {
        Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 }
    }

    /// PALE_GREY
    #[classattr]
    fn PALE_GREY() -> Color {
        Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// ASPARAGUS
    #[classattr]
    fn ASPARAGUS() -> Color {
        Color { r: 0.4666666666666667, g: 0.6705882352941176, b: 0.33725490196078434, a: 1.0 }
    }

    /// BLUEBERRY
    #[classattr]
    fn BLUEBERRY() -> Color {
        Color { r: 0.27450980392156865, g: 0.2549019607843137, b: 0.5882352941176471, a: 1.0 }
    }

    /// PURPLE_RED
    #[classattr]
    fn PURPLE_RED() -> Color {
        Color { r: 0.6, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 }
    }

    /// PALE_LIME
    #[classattr]
    fn PALE_LIME() -> Color {
        Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.45098039215686275, a: 1.0 }
    }

    /// GREENISH_TEAL
    #[classattr]
    fn GREENISH_TEAL() -> Color {
        Color { r: 0.19607843137254902, g: 0.7490196078431373, b: 0.5176470588235295, a: 1.0 }
    }

    /// CARAMEL
    #[classattr]
    fn CARAMEL() -> Color {
        Color { r: 0.6862745098039216, g: 0.43529411764705883, b: 0.03529411764705882, a: 1.0 }
    }

    /// DEEP_MAGENTA
    #[classattr]
    fn DEEP_MAGENTA() -> Color {
        Color { r: 0.6274509803921569, g: 0.00784313725490196, b: 0.3607843137254902, a: 1.0 }
    }

    /// LIGHT_PEACH
    #[classattr]
    fn LIGHT_PEACH() -> Color {
        Color { r: 1.0, g: 0.8470588235294118, b: 0.6941176470588235, a: 1.0 }
    }

    /// MILK_CHOCOLATE
    #[classattr]
    fn MILK_CHOCOLATE() -> Color {
        Color { r: 0.4980392156862745, g: 0.3058823529411765, b: 0.11764705882352941, a: 1.0 }
    }

    /// OCHER
    #[classattr]
    fn OCHER() -> Color {
        Color { r: 0.7490196078431373, g: 0.6078431372549019, b: 0.047058823529411764, a: 1.0 }
    }

    /// OFF_GREEN
    #[classattr]
    fn OFF_GREEN() -> Color {
        Color { r: 0.4196078431372549, g: 0.6392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// PURPLY_PINK
    #[classattr]
    fn PURPLY_PINK() -> Color {
        Color { r: 0.9411764705882353, g: 0.4588235294117647, b: 0.9019607843137255, a: 1.0 }
    }

    /// LIGHTBLUE
    #[classattr]
    fn LIGHTBLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.7843137254901961, b: 0.9647058823529412, a: 1.0 }
    }

    /// DUSKY_BLUE
    #[classattr]
    fn DUSKY_BLUE() -> Color {
        Color { r: 0.2784313725490196, g: 0.37254901960784315, b: 0.5803921568627451, a: 1.0 }
    }

    /// GOLDEN
    #[classattr]
    fn GOLDEN() -> Color {
        Color { r: 0.9607843137254902, g: 0.7490196078431373, b: 0.011764705882352941, a: 1.0 }
    }

    /// LIGHT_BEIGE
    #[classattr]
    fn LIGHT_BEIGE() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.7137254901960784, a: 1.0 }
    }

    /// BUTTER_YELLOW
    #[classattr]
    fn BUTTER_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.4549019607843137, a: 1.0 }
    }

    /// DUSKY_PURPLE
    #[classattr]
    fn DUSKY_PURPLE() -> Color {
        Color { r: 0.5372549019607843, g: 0.3568627450980392, b: 0.4823529411764706, a: 1.0 }
    }

    /// FRENCH_BLUE
    #[classattr]
    fn FRENCH_BLUE() -> Color {
        Color { r: 0.2627450980392157, g: 0.4196078431372549, b: 0.6784313725490196, a: 1.0 }
    }

    /// UGLY_YELLOW
    #[classattr]
    fn UGLY_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.7568627450980392, b: 0.00392156862745098, a: 1.0 }
    }

    /// GREENY_YELLOW
    #[classattr]
    fn GREENY_YELLOW() -> Color {
        Color { r: 0.7764705882352941, g: 0.9725490196078431, b: 0.03137254901960784, a: 1.0 }
    }

    /// ORANGISH_RED
    #[classattr]
    fn ORANGISH_RED() -> Color {
        Color { r: 0.9568627450980393, g: 0.21176470588235294, b: 0.0196078431372549, a: 1.0 }
    }

    /// SHAMROCK_GREEN
    #[classattr]
    fn SHAMROCK_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.7568627450980392, b: 0.30196078431372547, a: 1.0 }
    }

    /// ORANGISH_BROWN
    #[classattr]
    fn ORANGISH_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.37254901960784315, b: 0.011764705882352941, a: 1.0 }
    }

    /// TREE_GREEN
    #[classattr]
    fn TREE_GREEN() -> Color {
        Color { r: 0.16470588235294117, g: 0.49411764705882355, b: 0.09803921568627451, a: 1.0 }
    }

    /// DEEP_VIOLET
    #[classattr]
    fn DEEP_VIOLET() -> Color {
        Color { r: 0.28627450980392155, g: 0.023529411764705882, b: 0.2823529411764706, a: 1.0 }
    }

    /// GUNMETAL
    #[classattr]
    fn GUNMETAL() -> Color {
        Color { r: 0.3254901960784314, g: 0.3843137254901961, b: 0.403921568627451, a: 1.0 }
    }

    /// BLUE_PURPLE
    #[classattr]
    fn BLUE_PURPLE() -> Color {
        Color { r: 0.35294117647058826, g: 0.023529411764705882, b: 0.9372549019607843, a: 1.0 }
    }

    /// CHERRY
    #[classattr]
    fn CHERRY() -> Color {
        Color { r: 0.8117647058823529, g: 0.00784313725490196, b: 0.20392156862745098, a: 1.0 }
    }

    /// SANDY_BROWN
    #[classattr]
    fn SANDY_BROWN() -> Color {
        Color { r: 0.7686274509803922, g: 0.6509803921568628, b: 0.3803921568627451, a: 1.0 }
    }

    /// WARM_GREY
    #[classattr]
    fn WARM_GREY() -> Color {
        Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 }
    }

    /// DARK_INDIGO
    #[classattr]
    fn DARK_INDIGO() -> Color {
        Color { r: 0.12156862745098039, g: 0.03529411764705882, b: 0.32941176470588235, a: 1.0 }
    }

    /// MIDNIGHT
    #[classattr]
    fn MIDNIGHT() -> Color {
        Color { r: 0.011764705882352941, g: 0.00392156862745098, b: 0.17647058823529413, a: 1.0 }
    }

    /// BLUEY_GREEN
    #[classattr]
    fn BLUEY_GREEN() -> Color {
        Color { r: 0.16862745098039217, g: 0.6941176470588235, b: 0.4745098039215686, a: 1.0 }
    }

    /// GREY_PINK
    #[classattr]
    fn GREY_PINK() -> Color {
        Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 }
    }

    /// SOFT_PURPLE
    #[classattr]
    fn SOFT_PURPLE() -> Color {
        Color { r: 0.6509803921568628, g: 0.43529411764705883, b: 0.7098039215686275, a: 1.0 }
    }

    /// BLOOD
    #[classattr]
    fn BLOOD() -> Color {
        Color { r: 0.4666666666666667, g: 0.0, b: 0.00392156862745098, a: 1.0 }
    }

    /// BROWN_RED
    #[classattr]
    fn BROWN_RED() -> Color {
        Color { r: 0.5725490196078431, g: 0.16862745098039217, b: 0.0196078431372549, a: 1.0 }
    }

    /// MEDIUM_GREY
    #[classattr]
    fn MEDIUM_GREY() -> Color {
        Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 }
    }

    /// BERRY
    #[classattr]
    fn BERRY() -> Color {
        Color { r: 0.6, g: 0.058823529411764705, b: 0.29411764705882354, a: 1.0 }
    }

    /// POO
    #[classattr]
    fn POO() -> Color {
        Color { r: 0.5607843137254902, g: 0.45098039215686275, b: 0.011764705882352941, a: 1.0 }
    }

    /// PURPLEY_PINK
    #[classattr]
    fn PURPLEY_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.23529411764705882, b: 0.7254901960784313, a: 1.0 }
    }

    /// LIGHT_SALMON
    #[classattr]
    fn LIGHT_SALMON() -> Color {
        Color { r: 0.996078431372549, g: 0.6627450980392157, b: 0.5764705882352941, a: 1.0 }
    }

    /// SNOT
    #[classattr]
    fn SNOT() -> Color {
        Color { r: 0.6745098039215687, g: 0.7333333333333333, b: 0.050980392156862744, a: 1.0 }
    }

    /// EASTER_PURPLE
    #[classattr]
    fn EASTER_PURPLE() -> Color {
        Color { r: 0.7529411764705882, g: 0.44313725490196076, b: 0.996078431372549, a: 1.0 }
    }

    /// LIGHT_YELLOW_GREEN
    #[classattr]
    fn LIGHT_YELLOW_GREEN() -> Color {
        Color { r: 0.8, g: 0.9921568627450981, b: 0.4980392156862745, a: 1.0 }
    }

    /// DARK_NAVY_BLUE
    #[classattr]
    fn DARK_NAVY_BLUE() -> Color {
        Color { r: 0.0, g: 0.00784313725490196, b: 0.1803921568627451, a: 1.0 }
    }

    /// DRAB
    #[classattr]
    fn DRAB() -> Color {
        Color { r: 0.5098039215686274, g: 0.5137254901960784, b: 0.26666666666666666, a: 1.0 }
    }

    /// LIGHT_ROSE
    #[classattr]
    fn LIGHT_ROSE() -> Color {
        Color { r: 1.0, g: 0.7725490196078432, b: 0.796078431372549, a: 1.0 }
    }

    /// ROUGE
    #[classattr]
    fn ROUGE() -> Color {
        Color { r: 0.6705882352941176, g: 0.07058823529411765, b: 0.2235294117647059, a: 1.0 }
    }

    /// PURPLISH_RED
    #[classattr]
    fn PURPLISH_RED() -> Color {
        Color { r: 0.6901960784313725, g: 0.0196078431372549, b: 0.29411764705882354, a: 1.0 }
    }

    /// SLIME_GREEN
    #[classattr]
    fn SLIME_GREEN() -> Color {
        Color { r: 0.6, g: 0.8, b: 0.01568627450980392, a: 1.0 }
    }

    /// BABY_POOP
    #[classattr]
    fn BABY_POOP() -> Color {
        Color { r: 0.5764705882352941, g: 0.48627450980392156, b: 0.0, a: 1.0 }
    }

    /// IRISH_GREEN
    #[classattr]
    fn IRISH_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.5843137254901961, b: 0.1607843137254902, a: 1.0 }
    }

    /// PINK_PURPLE
    #[classattr]
    fn PINK_PURPLE() -> Color {
        Color { r: 0.9372549019607843, g: 0.11372549019607843, b: 0.9058823529411765, a: 1.0 }
    }

    /// DARK_NAVY
    #[classattr]
    fn DARK_NAVY() -> Color {
        Color { r: 0.0, g: 0.01568627450980392, b: 0.20784313725490197, a: 1.0 }
    }

    /// GREENY_BLUE
    #[classattr]
    fn GREENY_BLUE() -> Color {
        Color { r: 0.25882352941176473, g: 0.7019607843137254, b: 0.5843137254901961, a: 1.0 }
    }

    /// LIGHT_PLUM
    #[classattr]
    fn LIGHT_PLUM() -> Color {
        Color { r: 0.615686274509804, g: 0.3411764705882353, b: 0.5137254901960784, a: 1.0 }
    }

    /// PINKISH_GREY
    #[classattr]
    fn PINKISH_GREY() -> Color {
        Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 }
    }

    /// DIRTY_ORANGE
    #[classattr]
    fn DIRTY_ORANGE() -> Color {
        Color { r: 0.7843137254901961, g: 0.4627450980392157, b: 0.023529411764705882, a: 1.0 }
    }

    /// RUST_RED
    #[classattr]
    fn RUST_RED() -> Color {
        Color { r: 0.6666666666666666, g: 0.15294117647058825, b: 0.01568627450980392, a: 1.0 }
    }

    /// PALE_LILAC
    #[classattr]
    fn PALE_LILAC() -> Color {
        Color { r: 0.8941176470588236, g: 0.796078431372549, b: 1.0, a: 1.0 }
    }

    /// ORANGEY_RED
    #[classattr]
    fn ORANGEY_RED() -> Color {
        Color { r: 0.9803921568627451, g: 0.25882352941176473, b: 0.1411764705882353, a: 1.0 }
    }

    /// PRIMARY_BLUE
    #[classattr]
    fn PRIMARY_BLUE() -> Color {
        Color { r: 0.03137254901960784, g: 0.01568627450980392, b: 0.9764705882352941, a: 1.0 }
    }

    /// KERMIT_GREEN
    #[classattr]
    fn KERMIT_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.6980392156862745, b: 0.0, a: 1.0 }
    }

    /// BROWNISH_PURPLE
    #[classattr]
    fn BROWNISH_PURPLE() -> Color {
        Color { r: 0.4627450980392157, g: 0.25882352941176473, b: 0.3058823529411765, a: 1.0 }
    }

    /// MURKY_GREEN
    #[classattr]
    fn MURKY_GREEN() -> Color {
        Color { r: 0.4235294117647059, g: 0.47843137254901963, b: 0.054901960784313725, a: 1.0 }
    }

    /// WHEAT
    #[classattr]
    fn WHEAT() -> Color {
        Color { r: 0.984313725490196, g: 0.8666666666666667, b: 0.49411764705882355, a: 1.0 }
    }

    /// VERY_DARK_PURPLE
    #[classattr]
    fn VERY_DARK_PURPLE() -> Color {
        Color { r: 0.16470588235294117, g: 0.00392156862745098, b: 0.20392156862745098, a: 1.0 }
    }

    /// BOTTLE_GREEN
    #[classattr]
    fn BOTTLE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.2901960784313726, b: 0.0196078431372549, a: 1.0 }
    }

    /// WATERMELON
    #[classattr]
    fn WATERMELON() -> Color {
        Color { r: 0.9921568627450981, g: 0.27450980392156865, b: 0.34901960784313724, a: 1.0 }
    }

    /// DEEP_SKY_BLUE
    #[classattr]
    fn DEEP_SKY_BLUE() -> Color {
        Color { r: 0.050980392156862744, g: 0.4588235294117647, b: 0.9725490196078431, a: 1.0 }
    }

    /// FIRE_ENGINE_RED
    #[classattr]
    fn FIRE_ENGINE_RED() -> Color {
        Color { r: 0.996078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// YELLOW_OCHRE
    #[classattr]
    fn YELLOW_OCHRE() -> Color {
        Color { r: 0.796078431372549, g: 0.615686274509804, b: 0.023529411764705882, a: 1.0 }
    }

    /// PUMPKIN_ORANGE
    #[classattr]
    fn PUMPKIN_ORANGE() -> Color {
        Color { r: 0.984313725490196, g: 0.49019607843137253, b: 0.027450980392156862, a: 1.0 }
    }

    /// PALE_OLIVE
    #[classattr]
    fn PALE_OLIVE() -> Color {
        Color { r: 0.7254901960784313, g: 0.8, b: 0.5058823529411764, a: 1.0 }
    }

    /// LIGHT_LILAC
    #[classattr]
    fn LIGHT_LILAC() -> Color {
        Color { r: 0.9294117647058824, g: 0.7843137254901961, b: 1.0, a: 1.0 }
    }

    /// LIGHTISH_GREEN
    #[classattr]
    fn LIGHTISH_GREEN() -> Color {
        Color { r: 0.3803921568627451, g: 0.8823529411764706, b: 0.3764705882352941, a: 1.0 }
    }

    /// CAROLINA_BLUE
    #[classattr]
    fn CAROLINA_BLUE() -> Color {
        Color { r: 0.5411764705882353, g: 0.7215686274509804, b: 0.996078431372549, a: 1.0 }
    }

    /// MULBERRY
    #[classattr]
    fn MULBERRY() -> Color {
        Color { r: 0.5725490196078431, g: 0.0392156862745098, b: 0.3058823529411765, a: 1.0 }
    }

    /// SHOCKING_PINK
    #[classattr]
    fn SHOCKING_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00784313725490196, b: 0.6352941176470588, a: 1.0 }
    }

    /// AUBURN
    #[classattr]
    fn AUBURN() -> Color {
        Color { r: 0.6039215686274509, g: 0.18823529411764706, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_LIME_GREEN
    #[classattr]
    fn BRIGHT_LIME_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.996078431372549, b: 0.03137254901960784, a: 1.0 }
    }

    /// CELADON
    #[classattr]
    fn CELADON() -> Color {
        Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.7176470588235294, a: 1.0 }
    }

    /// PINKISH_BROWN
    #[classattr]
    fn PINKISH_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.4470588235294118, b: 0.3803921568627451, a: 1.0 }
    }

    /// POO_BROWN
    #[classattr]
    fn POO_BROWN() -> Color {
        Color { r: 0.5333333333333333, g: 0.37254901960784315, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_SKY_BLUE
    #[classattr]
    fn BRIGHT_SKY_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.8, b: 0.996078431372549, a: 1.0 }
    }

    /// CELERY
    #[classattr]
    fn CELERY() -> Color {
        Color { r: 0.7568627450980392, g: 0.9921568627450981, b: 0.5843137254901961, a: 1.0 }
    }

    /// DIRT_BROWN
    #[classattr]
    fn DIRT_BROWN() -> Color {
        Color { r: 0.5137254901960784, g: 0.396078431372549, b: 0.2235294117647059, a: 1.0 }
    }

    /// STRAWBERRY
    #[classattr]
    fn STRAWBERRY() -> Color {
        Color { r: 0.984313725490196, g: 0.1607843137254902, b: 0.2627450980392157, a: 1.0 }
    }

    /// DARK_LIME
    #[classattr]
    fn DARK_LIME() -> Color {
        Color { r: 0.5176470588235295, g: 0.7176470588235294, b: 0.00392156862745098, a: 1.0 }
    }

    /// COPPER
    #[classattr]
    fn COPPER() -> Color {
        Color { r: 0.7137254901960784, g: 0.38823529411764707, b: 0.1450980392156863, a: 1.0 }
    }

    /// MEDIUM_BROWN
    #[classattr]
    fn MEDIUM_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.3176470588235294, b: 0.07058823529411765, a: 1.0 }
    }

    /// MUTED_GREEN
    #[classattr]
    fn MUTED_GREEN() -> Color {
        Color { r: 0.37254901960784315, g: 0.6274509803921569, b: 0.3215686274509804, a: 1.0 }
    }

    /// ROBINS_EGG
    #[classattr]
    fn ROBINS_EGG() -> Color {
        Color { r: 0.42745098039215684, g: 0.9294117647058824, b: 0.9921568627450981, a: 1.0 }
    }

    /// BRIGHT_AQUA
    #[classattr]
    fn BRIGHT_AQUA() -> Color {
        Color { r: 0.043137254901960784, g: 0.9764705882352941, b: 0.9176470588235294, a: 1.0 }
    }

    /// BRIGHT_LAVENDER
    #[classattr]
    fn BRIGHT_LAVENDER() -> Color {
        Color { r: 0.7803921568627451, g: 0.3764705882352941, b: 1.0, a: 1.0 }
    }

    /// IVORY
    #[classattr]
    fn IVORY() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.796078431372549, a: 1.0 }
    }

    /// VERY_LIGHT_PURPLE
    #[classattr]
    fn VERY_LIGHT_PURPLE() -> Color {
        Color { r: 0.9647058823529412, g: 0.807843137254902, b: 0.9882352941176471, a: 1.0 }
    }

    /// LIGHT_NAVY
    #[classattr]
    fn LIGHT_NAVY() -> Color {
        Color { r: 0.08235294117647059, g: 0.3137254901960784, b: 0.5176470588235295, a: 1.0 }
    }

    /// PINK_RED
    #[classattr]
    fn PINK_RED() -> Color {
        Color { r: 0.9607843137254902, g: 0.0196078431372549, b: 0.30980392156862746, a: 1.0 }
    }

    /// OLIVE_BROWN
    #[classattr]
    fn OLIVE_BROWN() -> Color {
        Color { r: 0.39215686274509803, g: 0.32941176470588235, b: 0.011764705882352941, a: 1.0 }
    }

    /// POOP_BROWN
    #[classattr]
    fn POOP_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.34901960784313724, b: 0.00392156862745098, a: 1.0 }
    }

    /// MUSTARD_GREEN
    #[classattr]
    fn MUSTARD_GREEN() -> Color {
        Color { r: 0.6588235294117647, g: 0.7098039215686275, b: 0.01568627450980392, a: 1.0 }
    }

    /// OCEAN_GREEN
    #[classattr]
    fn OCEAN_GREEN() -> Color {
        Color { r: 0.23921568627450981, g: 0.6, b: 0.45098039215686275, a: 1.0 }
    }

    /// VERY_DARK_BLUE
    #[classattr]
    fn VERY_DARK_BLUE() -> Color {
        Color { r: 0.0, g: 0.00392156862745098, b: 0.2, a: 1.0 }
    }

    /// DUSTY_GREEN
    #[classattr]
    fn DUSTY_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.6627450980392157, b: 0.45098039215686275, a: 1.0 }
    }

    /// LIGHT_NAVY_BLUE
    #[classattr]
    fn LIGHT_NAVY_BLUE() -> Color {
        Color { r: 0.1803921568627451, g: 0.35294117647058826, b: 0.5333333333333333, a: 1.0 }
    }

    /// MINTY_GREEN
    #[classattr]
    fn MINTY_GREEN() -> Color {
        Color { r: 0.043137254901960784, g: 0.9686274509803922, b: 0.49019607843137253, a: 1.0 }
    }

    /// ADOBE
    #[classattr]
    fn ADOBE() -> Color {
        Color { r: 0.7411764705882353, g: 0.4235294117647059, b: 0.2823529411764706, a: 1.0 }
    }

    /// BARNEY
    #[classattr]
    fn BARNEY() -> Color {
        Color { r: 0.6745098039215687, g: 0.11372549019607843, b: 0.7215686274509804, a: 1.0 }
    }

    /// JADE_GREEN
    #[classattr]
    fn JADE_GREEN() -> Color {
        Color { r: 0.16862745098039217, g: 0.6862745098039216, b: 0.41568627450980394, a: 1.0 }
    }

    /// BRIGHT_LIGHT_BLUE
    #[classattr]
    fn BRIGHT_LIGHT_BLUE() -> Color {
        Color { r: 0.14901960784313725, g: 0.9686274509803922, b: 0.9921568627450981, a: 1.0 }
    }

    /// LIGHT_LIME
    #[classattr]
    fn LIGHT_LIME() -> Color {
        Color { r: 0.6823529411764706, g: 0.9921568627450981, b: 0.4235294117647059, a: 1.0 }
    }

    /// DARK_KHAKI
    #[classattr]
    fn DARK_KHAKI() -> Color {
        Color { r: 0.6078431372549019, g: 0.5607843137254902, b: 0.3333333333333333, a: 1.0 }
    }

    /// ORANGE_YELLOW
    #[classattr]
    fn ORANGE_YELLOW() -> Color {
        Color { r: 1.0, g: 0.6784313725490196, b: 0.00392156862745098, a: 1.0 }
    }

    /// OCRE
    #[classattr]
    fn OCRE() -> Color {
        Color { r: 0.7764705882352941, g: 0.611764705882353, b: 0.01568627450980392, a: 1.0 }
    }

    /// MAIZE
    #[classattr]
    fn MAIZE() -> Color {
        Color { r: 0.9568627450980393, g: 0.8156862745098039, b: 0.32941176470588235, a: 1.0 }
    }

    /// FADED_PINK
    #[classattr]
    fn FADED_PINK() -> Color {
        Color { r: 0.8705882352941177, g: 0.615686274509804, b: 0.6745098039215687, a: 1.0 }
    }

    /// BRITISH_RACING_GREEN
    #[classattr]
    fn BRITISH_RACING_GREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.2823529411764706, b: 0.050980392156862744, a: 1.0 }
    }

    /// SANDSTONE
    #[classattr]
    fn SANDSTONE() -> Color {
        Color { r: 0.788235294117647, g: 0.6823529411764706, b: 0.4549019607843137, a: 1.0 }
    }

    /// MUD_BROWN
    #[classattr]
    fn MUD_BROWN() -> Color {
        Color { r: 0.3764705882352941, g: 0.27450980392156865, b: 0.058823529411764705, a: 1.0 }
    }

    /// LIGHT_SEA_GREEN
    #[classattr]
    fn LIGHT_SEA_GREEN() -> Color {
        Color { r: 0.596078431372549, g: 0.9647058823529412, b: 0.6901960784313725, a: 1.0 }
    }

    /// ROBIN_EGG_BLUE
    #[classattr]
    fn ROBIN_EGG_BLUE() -> Color {
        Color { r: 0.5411764705882353, g: 0.9450980392156862, b: 0.996078431372549, a: 1.0 }
    }

    /// AQUA_MARINE
    #[classattr]
    fn AQUA_MARINE() -> Color {
        Color { r: 0.1803921568627451, g: 0.9098039215686274, b: 0.7333333333333333, a: 1.0 }
    }

    /// DARK_SEA_GREEN
    #[classattr]
    fn DARK_SEA_GREEN() -> Color {
        Color { r: 0.06666666666666667, g: 0.5294117647058824, b: 0.36470588235294116, a: 1.0 }
    }

    /// SOFT_PINK
    #[classattr]
    fn SOFT_PINK() -> Color {
        Color { r: 0.9921568627450981, g: 0.6901960784313725, b: 0.7529411764705882, a: 1.0 }
    }

    /// ORANGEY_BROWN
    #[classattr]
    fn ORANGEY_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.3764705882352941, b: 0.00784313725490196, a: 1.0 }
    }

    /// CHERRY_RED
    #[classattr]
    fn CHERRY_RED() -> Color {
        Color { r: 0.9686274509803922, g: 0.00784313725490196, b: 0.16470588235294117, a: 1.0 }
    }

    /// BURNT_YELLOW
    #[classattr]
    fn BURNT_YELLOW() -> Color {
        Color { r: 0.8352941176470589, g: 0.6705882352941176, b: 0.03529411764705882, a: 1.0 }
    }

    /// BROWNISH_GREY
    #[classattr]
    fn BROWNISH_GREY() -> Color {
        Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 }
    }

    /// CAMEL
    #[classattr]
    fn CAMEL() -> Color {
        Color { r: 0.7764705882352941, g: 0.6235294117647059, b: 0.34901960784313724, a: 1.0 }
    }

    /// PURPLISH_GREY
    #[classattr]
    fn PURPLISH_GREY() -> Color {
        Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 }
    }

    /// MARINE
    #[classattr]
    fn MARINE() -> Color {
        Color { r: 0.01568627450980392, g: 0.1803921568627451, b: 0.3764705882352941, a: 1.0 }
    }

    /// GREYISH_PINK
    #[classattr]
    fn GREYISH_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 }
    }

    /// PALE_TURQUOISE
    #[classattr]
    fn PALE_TURQUOISE() -> Color {
        Color { r: 0.6470588235294118, g: 0.984313725490196, b: 0.8352941176470589, a: 1.0 }
    }

    /// PASTEL_YELLOW
    #[classattr]
    fn PASTEL_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.44313725490196076, a: 1.0 }
    }

    /// BLUEY_PURPLE
    #[classattr]
    fn BLUEY_PURPLE() -> Color {
        Color { r: 0.3843137254901961, g: 0.2549019607843137, b: 0.7803921568627451, a: 1.0 }
    }

    /// CANARY_YELLOW
    #[classattr]
    fn CANARY_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.25098039215686274, a: 1.0 }
    }

    /// FADED_RED
    #[classattr]
    fn FADED_RED() -> Color {
        Color { r: 0.8274509803921568, g: 0.28627450980392155, b: 0.3058823529411765, a: 1.0 }
    }

    /// SEPIA
    #[classattr]
    fn SEPIA() -> Color {
        Color { r: 0.596078431372549, g: 0.3686274509803922, b: 0.16862745098039217, a: 1.0 }
    }

    /// COFFEE
    #[classattr]
    fn COFFEE() -> Color {
        Color { r: 0.6509803921568628, g: 0.5058823529411764, b: 0.2980392156862745, a: 1.0 }
    }

    /// BRIGHT_MAGENTA
    #[classattr]
    fn BRIGHT_MAGENTA() -> Color {
        Color { r: 1.0, g: 0.03137254901960784, b: 0.9098039215686274, a: 1.0 }
    }

    /// MOCHA
    #[classattr]
    fn MOCHA() -> Color {
        Color { r: 0.615686274509804, g: 0.4627450980392157, b: 0.3176470588235294, a: 1.0 }
    }

    /// ECRU
    #[classattr]
    fn ECRU() -> Color {
        Color { r: 0.996078431372549, g: 1.0, b: 0.792156862745098, a: 1.0 }
    }

    /// PURPLEISH
    #[classattr]
    fn PURPLEISH() -> Color {
        Color { r: 0.596078431372549, g: 0.33725490196078434, b: 0.5529411764705883, a: 1.0 }
    }

    /// CRANBERRY
    #[classattr]
    fn CRANBERRY() -> Color {
        Color { r: 0.6196078431372549, g: 0.0, b: 0.22745098039215686, a: 1.0 }
    }

    /// DARKISH_GREEN
    #[classattr]
    fn DARKISH_GREEN() -> Color {
        Color { r: 0.1568627450980392, g: 0.48627450980392156, b: 0.21568627450980393, a: 1.0 }
    }

    /// BROWN_ORANGE
    #[classattr]
    fn BROWN_ORANGE() -> Color {
        Color { r: 0.7254901960784313, g: 0.4117647058823529, b: 0.00784313725490196, a: 1.0 }
    }

    /// DUSKY_ROSE
    #[classattr]
    fn DUSKY_ROSE() -> Color {
        Color { r: 0.7294117647058823, g: 0.40784313725490196, b: 0.45098039215686275, a: 1.0 }
    }

    /// MELON
    #[classattr]
    fn MELON() -> Color {
        Color { r: 1.0, g: 0.47058823529411764, b: 0.3333333333333333, a: 1.0 }
    }

    /// SICKLY_GREEN
    #[classattr]
    fn SICKLY_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6980392156862745, b: 0.10980392156862745, a: 1.0 }
    }

    /// SILVER
    #[classattr]
    fn SILVER() -> Color {
        Color { r: 0.7725490196078432, g: 0.788235294117647, b: 0.7803921568627451, a: 1.0 }
    }

    /// PURPLY_BLUE
    #[classattr]
    fn PURPLY_BLUE() -> Color {
        Color { r: 0.4, g: 0.10196078431372549, b: 0.9333333333333333, a: 1.0 }
    }

    /// PURPLEISH_BLUE
    #[classattr]
    fn PURPLEISH_BLUE() -> Color {
        Color { r: 0.3803921568627451, g: 0.25098039215686274, b: 0.9372549019607843, a: 1.0 }
    }

    /// HOSPITAL_GREEN
    #[classattr]
    fn HOSPITAL_GREEN() -> Color {
        Color { r: 0.6078431372549019, g: 0.8980392156862745, b: 0.6666666666666666, a: 1.0 }
    }

    /// SHIT_BROWN
    #[classattr]
    fn SHIT_BROWN() -> Color {
        Color { r: 0.4823529411764706, g: 0.34509803921568627, b: 0.01568627450980392, a: 1.0 }
    }

    /// MID_BLUE
    #[classattr]
    fn MID_BLUE() -> Color {
        Color { r: 0.15294117647058825, g: 0.41568627450980394, b: 0.7019607843137254, a: 1.0 }
    }

    /// AMBER
    #[classattr]
    fn AMBER() -> Color {
        Color { r: 0.996078431372549, g: 0.7019607843137254, b: 0.03137254901960784, a: 1.0 }
    }

    /// EASTER_GREEN
    #[classattr]
    fn EASTER_GREEN() -> Color {
        Color { r: 0.5490196078431373, g: 0.9921568627450981, b: 0.49411764705882355, a: 1.0 }
    }

    /// SOFT_BLUE
    #[classattr]
    fn SOFT_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.5333333333333333, b: 0.9176470588235294, a: 1.0 }
    }

    /// CERULEAN_BLUE
    #[classattr]
    fn CERULEAN_BLUE() -> Color {
        Color { r: 0.0196078431372549, g: 0.43137254901960786, b: 0.9333333333333333, a: 1.0 }
    }

    /// GOLDEN_BROWN
    #[classattr]
    fn GOLDEN_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_TURQUOISE
    #[classattr]
    fn BRIGHT_TURQUOISE() -> Color {
        Color { r: 0.058823529411764705, g: 0.996078431372549, b: 0.9764705882352941, a: 1.0 }
    }

    /// RED_PINK
    #[classattr]
    fn RED_PINK() -> Color {
        Color { r: 0.9803921568627451, g: 0.16470588235294117, b: 0.3333333333333333, a: 1.0 }
    }

    /// RED_PURPLE
    #[classattr]
    fn RED_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.027450980392156862, b: 0.2784313725490196, a: 1.0 }
    }

    /// GREYISH_BROWN
    #[classattr]
    fn GREYISH_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 }
    }

    /// VERMILLION
    #[classattr]
    fn VERMILLION() -> Color {
        Color { r: 0.9568627450980393, g: 0.19607843137254902, b: 0.047058823529411764, a: 1.0 }
    }

    /// RUSSET
    #[classattr]
    fn RUSSET() -> Color {
        Color { r: 0.6313725490196078, g: 0.2235294117647059, b: 0.0196078431372549, a: 1.0 }
    }

    /// STEEL_GREY
    #[classattr]
    fn STEEL_GREY() -> Color {
        Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 }
    }

    /// LIGHTER_PURPLE
    #[classattr]
    fn LIGHTER_PURPLE() -> Color {
        Color { r: 0.6470588235294118, g: 0.35294117647058826, b: 0.9568627450980393, a: 1.0 }
    }

    /// BRIGHT_VIOLET
    #[classattr]
    fn BRIGHT_VIOLET() -> Color {
        Color { r: 0.6784313725490196, g: 0.0392156862745098, b: 0.9921568627450981, a: 1.0 }
    }

    /// PRUSSIAN_BLUE
    #[classattr]
    fn PRUSSIAN_BLUE() -> Color {
        Color { r: 0.0, g: 0.27058823529411763, b: 0.4666666666666667, a: 1.0 }
    }

    /// SLATE_GREEN
    #[classattr]
    fn SLATE_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.5529411764705883, b: 0.42745098039215684, a: 1.0 }
    }

    /// DIRTY_PINK
    #[classattr]
    fn DIRTY_PINK() -> Color {
        Color { r: 0.792156862745098, g: 0.4823529411764706, b: 0.5019607843137255, a: 1.0 }
    }

    /// DARK_BLUE_GREEN
    #[classattr]
    fn DARK_BLUE_GREEN() -> Color {
        Color { r: 0.0, g: 0.3215686274509804, b: 0.28627450980392155, a: 1.0 }
    }

    /// PINE
    #[classattr]
    fn PINE() -> Color {
        Color { r: 0.16862745098039217, g: 0.36470588235294116, b: 0.20392156862745098, a: 1.0 }
    }

    /// YELLOWY_GREEN
    #[classattr]
    fn YELLOWY_GREEN() -> Color {
        Color { r: 0.7490196078431373, g: 0.9450980392156862, b: 0.1568627450980392, a: 1.0 }
    }

    /// DARK_GOLD
    #[classattr]
    fn DARK_GOLD() -> Color {
        Color { r: 0.7098039215686275, g: 0.5803921568627451, b: 0.06274509803921569, a: 1.0 }
    }

    /// BLUISH
    #[classattr]
    fn BLUISH() -> Color {
        Color { r: 0.1607843137254902, g: 0.4627450980392157, b: 0.7333333333333333, a: 1.0 }
    }

    /// DARKISH_BLUE
    #[classattr]
    fn DARKISH_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.2549019607843137, b: 0.5098039215686274, a: 1.0 }
    }

    /// DULL_RED
    #[classattr]
    fn DULL_RED() -> Color {
        Color { r: 0.7333333333333333, g: 0.24705882352941178, b: 0.24705882352941178, a: 1.0 }
    }

    /// PINKY_RED
    #[classattr]
    fn PINKY_RED() -> Color {
        Color { r: 0.9882352941176471, g: 0.14901960784313725, b: 0.2784313725490196, a: 1.0 }
    }

    /// BRONZE
    #[classattr]
    fn BRONZE() -> Color {
        Color { r: 0.6588235294117647, g: 0.4745098039215686, b: 0.0, a: 1.0 }
    }

    /// PALE_TEAL
    #[classattr]
    fn PALE_TEAL() -> Color {
        Color { r: 0.5098039215686274, g: 0.796078431372549, b: 0.6980392156862745, a: 1.0 }
    }

    /// MILITARY_GREEN
    #[classattr]
    fn MILITARY_GREEN() -> Color {
        Color { r: 0.4, g: 0.48627450980392156, b: 0.24313725490196078, a: 1.0 }
    }

    /// BARBIE_PINK
    #[classattr]
    fn BARBIE_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.27450980392156865, b: 0.6470588235294118, a: 1.0 }
    }

    /// BUBBLEGUM_PINK
    #[classattr]
    fn BUBBLEGUM_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.5137254901960784, b: 0.8, a: 1.0 }
    }

    /// PEA_SOUP_GREEN
    #[classattr]
    fn PEA_SOUP_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6509803921568628, b: 0.09019607843137255, a: 1.0 }
    }

    /// DARK_MUSTARD
    #[classattr]
    fn DARK_MUSTARD() -> Color {
        Color { r: 0.6588235294117647, g: 0.5372549019607843, b: 0.0196078431372549, a: 1.0 }
    }

    /// SHIT
    #[classattr]
    fn SHIT() -> Color {
        Color { r: 0.4980392156862745, g: 0.37254901960784315, b: 0.0, a: 1.0 }
    }

    /// MEDIUM_PURPLE
    #[classattr]
    fn MEDIUM_PURPLE() -> Color {
        Color { r: 0.6196078431372549, g: 0.2627450980392157, b: 0.6352941176470588, a: 1.0 }
    }

    /// VERY_DARK_GREEN
    #[classattr]
    fn VERY_DARK_GREEN() -> Color {
        Color { r: 0.023529411764705882, g: 0.1803921568627451, b: 0.011764705882352941, a: 1.0 }
    }

    /// DIRT
    #[classattr]
    fn DIRT() -> Color {
        Color { r: 0.5411764705882353, g: 0.43137254901960786, b: 0.27058823529411763, a: 1.0 }
    }

    /// DUSKY_PINK
    #[classattr]
    fn DUSKY_PINK() -> Color {
        Color { r: 0.8, g: 0.47843137254901963, b: 0.5450980392156862, a: 1.0 }
    }

    /// RED_VIOLET
    #[classattr]
    fn RED_VIOLET() -> Color {
        Color { r: 0.6196078431372549, g: 0.00392156862745098, b: 0.40784313725490196, a: 1.0 }
    }

    /// LEMON_YELLOW
    #[classattr]
    fn LEMON_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.2196078431372549, a: 1.0 }
    }

    /// PISTACHIO
    #[classattr]
    fn PISTACHIO() -> Color {
        Color { r: 0.7529411764705882, g: 0.9803921568627451, b: 0.5450980392156862, a: 1.0 }
    }

    /// DULL_YELLOW
    #[classattr]
    fn DULL_YELLOW() -> Color {
        Color { r: 0.9333333333333333, g: 0.8627450980392157, b: 0.3568627450980392, a: 1.0 }
    }

    /// DARK_LIME_GREEN
    #[classattr]
    fn DARK_LIME_GREEN() -> Color {
        Color { r: 0.49411764705882355, g: 0.7411764705882353, b: 0.00392156862745098, a: 1.0 }
    }

    /// DENIM_BLUE
    #[classattr]
    fn DENIM_BLUE() -> Color {
        Color { r: 0.23137254901960785, g: 0.3568627450980392, b: 0.5725490196078431, a: 1.0 }
    }

    /// TEAL_BLUE
    #[classattr]
    fn TEAL_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.5333333333333333, b: 0.6235294117647059, a: 1.0 }
    }

    /// LIGHTISH_BLUE
    #[classattr]
    fn LIGHTISH_BLUE() -> Color {
        Color { r: 0.23921568627450981, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 }
    }

    /// PURPLEY_BLUE
    #[classattr]
    fn PURPLEY_BLUE() -> Color {
        Color { r: 0.37254901960784315, g: 0.20392156862745098, b: 0.9058823529411765, a: 1.0 }
    }

    /// LIGHT_INDIGO
    #[classattr]
    fn LIGHT_INDIGO() -> Color {
        Color { r: 0.42745098039215684, g: 0.35294117647058826, b: 0.8117647058823529, a: 1.0 }
    }

    /// SWAMP_GREEN
    #[classattr]
    fn SWAMP_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.5215686274509804, b: 0.0, a: 1.0 }
    }

    /// BROWN_GREEN
    #[classattr]
    fn BROWN_GREEN() -> Color {
        Color { r: 0.4392156862745098, g: 0.4235294117647059, b: 0.06666666666666667, a: 1.0 }
    }

    /// DARK_MAROON
    #[classattr]
    fn DARK_MAROON() -> Color {
        Color { r: 0.23529411764705882, g: 0.0, b: 0.03137254901960784, a: 1.0 }
    }

    /// HOT_PURPLE
    #[classattr]
    fn HOT_PURPLE() -> Color {
        Color { r: 0.796078431372549, g: 0.0, b: 0.9607843137254902, a: 1.0 }
    }

    /// DARK_FOREST_GREEN
    #[classattr]
    fn DARK_FOREST_GREEN() -> Color {
        Color { r: 0.0, g: 0.17647058823529413, b: 0.01568627450980392, a: 1.0 }
    }

    /// FADED_BLUE
    #[classattr]
    fn FADED_BLUE() -> Color {
        Color { r: 0.396078431372549, g: 0.5490196078431373, b: 0.7333333333333333, a: 1.0 }
    }

    /// DRAB_GREEN
    #[classattr]
    fn DRAB_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.5843137254901961, b: 0.3176470588235294, a: 1.0 }
    }

    /// LIGHT_LIME_GREEN
    #[classattr]
    fn LIGHT_LIME_GREEN() -> Color {
        Color { r: 0.7254901960784313, g: 1.0, b: 0.4, a: 1.0 }
    }

    /// SNOT_GREEN
    #[classattr]
    fn SNOT_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 0.7568627450980392, b: 0.0, a: 1.0 }
    }

    /// YELLOWISH
    #[classattr]
    fn YELLOWISH() -> Color {
        Color { r: 0.9803921568627451, g: 0.9333333333333333, b: 0.4, a: 1.0 }
    }

    /// LIGHT_BLUE_GREEN
    #[classattr]
    fn LIGHT_BLUE_GREEN() -> Color {
        Color { r: 0.49411764705882355, g: 0.984313725490196, b: 0.7019607843137254, a: 1.0 }
    }

    /// BORDEAUX
    #[classattr]
    fn BORDEAUX() -> Color {
        Color { r: 0.4823529411764706, g: 0.0, b: 0.17254901960784313, a: 1.0 }
    }

    /// LIGHT_MAUVE
    #[classattr]
    fn LIGHT_MAUVE() -> Color {
        Color { r: 0.7607843137254902, g: 0.5725490196078431, b: 0.6313725490196078, a: 1.0 }
    }

    /// OCEAN
    #[classattr]
    fn OCEAN() -> Color {
        Color { r: 0.00392156862745098, g: 0.4823529411764706, b: 0.5725490196078431, a: 1.0 }
    }

    /// MARIGOLD
    #[classattr]
    fn MARIGOLD() -> Color {
        Color { r: 0.9882352941176471, g: 0.7529411764705882, b: 0.023529411764705882, a: 1.0 }
    }

    /// MUDDY_GREEN
    #[classattr]
    fn MUDDY_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.4549019607843137, b: 0.19607843137254902, a: 1.0 }
    }

    /// DULL_ORANGE
    #[classattr]
    fn DULL_ORANGE() -> Color {
        Color { r: 0.8470588235294118, g: 0.5254901960784314, b: 0.23137254901960785, a: 1.0 }
    }

    /// STEEL
    #[classattr]
    fn STEEL() -> Color {
        Color { r: 0.45098039215686275, g: 0.5215686274509804, b: 0.5843137254901961, a: 1.0 }
    }

    /// ELECTRIC_PURPLE
    #[classattr]
    fn ELECTRIC_PURPLE() -> Color {
        Color { r: 0.6666666666666666, g: 0.13725490196078433, b: 1.0, a: 1.0 }
    }

    /// FLUORESCENT_GREEN
    #[classattr]
    fn FLUORESCENT_GREEN() -> Color {
        Color { r: 0.03137254901960784, g: 1.0, b: 0.03137254901960784, a: 1.0 }
    }

    /// YELLOWISH_BROWN
    #[classattr]
    fn YELLOWISH_BROWN() -> Color {
        Color { r: 0.6078431372549019, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 }
    }

    /// BLUSH
    #[classattr]
    fn BLUSH() -> Color {
        Color { r: 0.9490196078431372, g: 0.6196078431372549, b: 0.5568627450980392, a: 1.0 }
    }

    /// SOFT_GREEN
    #[classattr]
    fn SOFT_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.7607843137254902, b: 0.4627450980392157, a: 1.0 }
    }

    /// BRIGHT_ORANGE
    #[classattr]
    fn BRIGHT_ORANGE() -> Color {
        Color { r: 1.0, g: 0.3568627450980392, b: 0.0, a: 1.0 }
    }

    /// LEMON
    #[classattr]
    fn LEMON() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.3215686274509804, a: 1.0 }
    }

    /// PURPLE_GREY
    #[classattr]
    fn PURPLE_GREY() -> Color {
        Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 }
    }

    /// ACID_GREEN
    #[classattr]
    fn ACID_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.996078431372549, b: 0.03529411764705882, a: 1.0 }
    }

    /// PALE_LAVENDER
    #[classattr]
    fn PALE_LAVENDER() -> Color {
        Color { r: 0.9333333333333333, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 }
    }

    /// VIOLET_BLUE
    #[classattr]
    fn VIOLET_BLUE() -> Color {
        Color { r: 0.3176470588235294, g: 0.0392156862745098, b: 0.788235294117647, a: 1.0 }
    }

    /// LIGHT_FOREST_GREEN
    #[classattr]
    fn LIGHT_FOREST_GREEN() -> Color {
        Color { r: 0.30980392156862746, g: 0.5686274509803921, b: 0.3254901960784314, a: 1.0 }
    }

    /// BURNT_RED
    #[classattr]
    fn BURNT_RED() -> Color {
        Color { r: 0.6235294117647059, g: 0.13725490196078433, b: 0.0196078431372549, a: 1.0 }
    }

    /// KHAKI_GREEN
    #[classattr]
    fn KHAKI_GREEN() -> Color {
        Color { r: 0.4470588235294118, g: 0.5254901960784314, b: 0.2235294117647059, a: 1.0 }
    }

    /// CERISE
    #[classattr]
    fn CERISE() -> Color {
        Color { r: 0.8705882352941177, g: 0.047058823529411764, b: 0.3843137254901961, a: 1.0 }
    }

    /// FADED_PURPLE
    #[classattr]
    fn FADED_PURPLE() -> Color {
        Color { r: 0.5686274509803921, g: 0.43137254901960786, b: 0.6, a: 1.0 }
    }

    /// APRICOT
    #[classattr]
    fn APRICOT() -> Color {
        Color { r: 1.0, g: 0.6941176470588235, b: 0.42745098039215684, a: 1.0 }
    }

    /// DARK_OLIVE_GREEN
    #[classattr]
    fn DARK_OLIVE_GREEN() -> Color {
        Color { r: 0.23529411764705882, g: 0.30196078431372547, b: 0.011764705882352941, a: 1.0 }
    }

    /// GREY_BROWN
    #[classattr]
    fn GREY_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREEN_GREY
    #[classattr]
    fn GREEN_GREY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 }
    }

    /// TRUE_BLUE
    #[classattr]
    fn TRUE_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.058823529411764705, b: 0.8, a: 1.0 }
    }

    /// PALE_VIOLET
    #[classattr]
    fn PALE_VIOLET() -> Color {
        Color { r: 0.807843137254902, g: 0.6823529411764706, b: 0.9803921568627451, a: 1.0 }
    }

    /// PERIWINKLE_BLUE
    #[classattr]
    fn PERIWINKLE_BLUE() -> Color {
        Color { r: 0.5607843137254902, g: 0.6, b: 0.984313725490196, a: 1.0 }
    }

    /// LIGHT_SKY_BLUE
    #[classattr]
    fn LIGHT_SKY_BLUE() -> Color {
        Color { r: 0.7764705882352941, g: 0.9882352941176471, b: 1.0, a: 1.0 }
    }

    /// BLURPLE
    #[classattr]
    fn BLURPLE() -> Color {
        Color { r: 0.3333333333333333, g: 0.2235294117647059, b: 0.8, a: 1.0 }
    }

    /// GREEN_BROWN
    #[classattr]
    fn GREEN_BROWN() -> Color {
        Color { r: 0.32941176470588235, g: 0.3058823529411765, b: 0.011764705882352941, a: 1.0 }
    }

    /// BLUEGREEN
    #[classattr]
    fn BLUEGREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.47843137254901963, b: 0.4745098039215686, a: 1.0 }
    }

    /// BRIGHT_TEAL
    #[classattr]
    fn BRIGHT_TEAL() -> Color {
        Color { r: 0.00392156862745098, g: 0.9764705882352941, b: 0.7764705882352941, a: 1.0 }
    }

    /// BROWNISH_YELLOW
    #[classattr]
    fn BROWNISH_YELLOW() -> Color {
        Color { r: 0.788235294117647, g: 0.6901960784313725, b: 0.011764705882352941, a: 1.0 }
    }

    /// PEA_SOUP
    #[classattr]
    fn PEA_SOUP() -> Color {
        Color { r: 0.5725490196078431, g: 0.6, b: 0.00392156862745098, a: 1.0 }
    }

    /// FOREST
    #[classattr]
    fn FOREST() -> Color {
        Color { r: 0.043137254901960784, g: 0.3333333333333333, b: 0.03529411764705882, a: 1.0 }
    }

    /// BARNEY_PURPLE
    #[classattr]
    fn BARNEY_PURPLE() -> Color {
        Color { r: 0.6274509803921569, g: 0.01568627450980392, b: 0.596078431372549, a: 1.0 }
    }

    /// ULTRAMARINE
    #[classattr]
    fn ULTRAMARINE() -> Color {
        Color { r: 0.12549019607843137, g: 0.0, b: 0.6941176470588235, a: 1.0 }
    }

    /// PURPLISH
    #[classattr]
    fn PURPLISH() -> Color {
        Color { r: 0.5803921568627451, g: 0.33725490196078434, b: 0.5490196078431373, a: 1.0 }
    }

    /// PUKE_YELLOW
    #[classattr]
    fn PUKE_YELLOW() -> Color {
        Color { r: 0.7607843137254902, g: 0.7450980392156863, b: 0.054901960784313725, a: 1.0 }
    }

    /// BLUISH_GREY
    #[classattr]
    fn BLUISH_GREY() -> Color {
        Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 }
    }

    /// DARK_PERIWINKLE
    #[classattr]
    fn DARK_PERIWINKLE() -> Color {
        Color { r: 0.4, g: 0.37254901960784315, b: 0.8196078431372549, a: 1.0 }
    }

    /// DARK_LILAC
    #[classattr]
    fn DARK_LILAC() -> Color {
        Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.6470588235294118, a: 1.0 }
    }

    /// REDDISH
    #[classattr]
    fn REDDISH() -> Color {
        Color { r: 0.7686274509803922, g: 0.25882352941176473, b: 0.25098039215686274, a: 1.0 }
    }

    /// LIGHT_MAROON
    #[classattr]
    fn LIGHT_MAROON() -> Color {
        Color { r: 0.6352941176470588, g: 0.2823529411764706, b: 0.3411764705882353, a: 1.0 }
    }

    /// DUSTY_PURPLE
    #[classattr]
    fn DUSTY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.37254901960784315, b: 0.5294117647058824, a: 1.0 }
    }

    /// TERRA_COTTA
    #[classattr]
    fn TERRA_COTTA() -> Color {
        Color { r: 0.788235294117647, g: 0.39215686274509803, b: 0.23137254901960785, a: 1.0 }
    }

    /// AVOCADO
    #[classattr]
    fn AVOCADO() -> Color {
        Color { r: 0.5647058823529412, g: 0.6941176470588235, b: 0.20392156862745098, a: 1.0 }
    }

    /// MARINE_BLUE
    #[classattr]
    fn MARINE_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.2196078431372549, b: 0.41568627450980394, a: 1.0 }
    }

    /// TEAL_GREEN
    #[classattr]
    fn TEAL_GREEN() -> Color {
        Color { r: 0.1450980392156863, g: 0.6392156862745098, b: 0.43529411764705883, a: 1.0 }
    }

    /// SLATE_GREY
    #[classattr]
    fn SLATE_GREY() -> Color {
        Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 }
    }

    /// LIGHTER_GREEN
    #[classattr]
    fn LIGHTER_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.9921568627450981, b: 0.38823529411764707, a: 1.0 }
    }

    /// ELECTRIC_GREEN
    #[classattr]
    fn ELECTRIC_GREEN() -> Color {
        Color { r: 0.12941176470588237, g: 0.9882352941176471, b: 0.050980392156862744, a: 1.0 }
    }

    /// DUSTY_BLUE
    #[classattr]
    fn DUSTY_BLUE() -> Color {
        Color { r: 0.35294117647058826, g: 0.5254901960784314, b: 0.6784313725490196, a: 1.0 }
    }

    /// GOLDEN_YELLOW
    #[classattr]
    fn GOLDEN_YELLOW() -> Color {
        Color { r: 0.996078431372549, g: 0.7764705882352941, b: 0.08235294117647059, a: 1.0 }
    }

    /// BRIGHT_YELLOW
    #[classattr]
    fn BRIGHT_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHT_LAVENDER
    #[classattr]
    fn LIGHT_LAVENDER() -> Color {
        Color { r: 0.8745098039215686, g: 0.7725490196078432, b: 0.996078431372549, a: 1.0 }
    }

    /// UMBER
    #[classattr]
    fn UMBER() -> Color {
        Color { r: 0.6980392156862745, g: 0.39215686274509803, b: 0.0, a: 1.0 }
    }

    /// POOP
    #[classattr]
    fn POOP() -> Color {
        Color { r: 0.4980392156862745, g: 0.3686274509803922, b: 0.0, a: 1.0 }
    }

    /// DARK_PEACH
    #[classattr]
    fn DARK_PEACH() -> Color {
        Color { r: 0.8705882352941177, g: 0.49411764705882355, b: 0.36470588235294116, a: 1.0 }
    }

    /// JUNGLE_GREEN
    #[classattr]
    fn JUNGLE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.5098039215686274, b: 0.2627450980392157, a: 1.0 }
    }

    /// EGGSHELL
    #[classattr]
    fn EGGSHELL() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8313725490196079, a: 1.0 }
    }

    /// DENIM
    #[classattr]
    fn DENIM() -> Color {
        Color { r: 0.23137254901960785, g: 0.38823529411764707, b: 0.5490196078431373, a: 1.0 }
    }

    /// YELLOW_BROWN
    #[classattr]
    fn YELLOW_BROWN() -> Color {
        Color { r: 0.7176470588235294, g: 0.5803921568627451, b: 0.0, a: 1.0 }
    }

    /// DULL_PURPLE
    #[classattr]
    fn DULL_PURPLE() -> Color {
        Color { r: 0.5176470588235295, g: 0.34901960784313724, b: 0.49411764705882355, a: 1.0 }
    }

    /// CHOCOLATE_BROWN
    #[classattr]
    fn CHOCOLATE_BROWN() -> Color {
        Color { r: 0.2549019607843137, g: 0.09803921568627451, b: 0.0, a: 1.0 }
    }

    /// WINE_RED
    #[classattr]
    fn WINE_RED() -> Color {
        Color { r: 0.4823529411764706, g: 0.011764705882352941, b: 0.13725490196078433, a: 1.0 }
    }

    /// NEON_BLUE
    #[classattr]
    fn NEON_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.8509803921568627, b: 1.0, a: 1.0 }
    }

    /// DIRTY_GREEN
    #[classattr]
    fn DIRTY_GREEN() -> Color {
        Color { r: 0.4, g: 0.49411764705882355, b: 0.17254901960784313, a: 1.0 }
    }

    /// LIGHT_TAN
    #[classattr]
    fn LIGHT_TAN() -> Color {
        Color { r: 0.984313725490196, g: 0.9333333333333333, b: 0.6745098039215687, a: 1.0 }
    }

    /// ICE_BLUE
    #[classattr]
    fn ICE_BLUE() -> Color {
        Color { r: 0.8431372549019608, g: 1.0, b: 0.996078431372549, a: 1.0 }
    }

    /// CADET_BLUE
    #[classattr]
    fn CADET_BLUE() -> Color {
        Color { r: 0.3058823529411765, g: 0.4549019607843137, b: 0.5882352941176471, a: 1.0 }
    }

    /// DARK_MAUVE
    #[classattr]
    fn DARK_MAUVE() -> Color {
        Color { r: 0.5294117647058824, g: 0.2980392156862745, b: 0.3843137254901961, a: 1.0 }
    }

    /// VERY_LIGHT_BLUE
    #[classattr]
    fn VERY_LIGHT_BLUE() -> Color {
        Color { r: 0.8352941176470589, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// GREY_PURPLE
    #[classattr]
    fn GREY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 }
    }

    /// PASTEL_PINK
    #[classattr]
    fn PASTEL_PINK() -> Color {
        Color { r: 1.0, g: 0.7294117647058823, b: 0.803921568627451, a: 1.0 }
    }

    /// VERY_LIGHT_GREEN
    #[classattr]
    fn VERY_LIGHT_GREEN() -> Color {
        Color { r: 0.8196078431372549, g: 1.0, b: 0.7411764705882353, a: 1.0 }
    }

    /// DARK_SKY_BLUE
    #[classattr]
    fn DARK_SKY_BLUE() -> Color {
        Color { r: 0.26666666666666666, g: 0.5568627450980392, b: 0.8941176470588236, a: 1.0 }
    }

    /// EVERGREEN
    #[classattr]
    fn EVERGREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.2784313725490196, b: 0.16470588235294117, a: 1.0 }
    }

    /// DULL_PINK
    #[classattr]
    fn DULL_PINK() -> Color {
        Color { r: 0.8352941176470589, g: 0.5254901960784314, b: 0.615686274509804, a: 1.0 }
    }

    /// AUBERGINE
    #[classattr]
    fn AUBERGINE() -> Color {
        Color { r: 0.23921568627450981, g: 0.027450980392156862, b: 0.20392156862745098, a: 1.0 }
    }

    /// MAHOGANY
    #[classattr]
    fn MAHOGANY() -> Color {
        Color { r: 0.2901960784313726, g: 0.00392156862745098, b: 0.0, a: 1.0 }
    }

    /// REDDISH_ORANGE
    #[classattr]
    fn REDDISH_ORANGE() -> Color {
        Color { r: 0.9725490196078431, g: 0.2823529411764706, b: 0.10980392156862745, a: 1.0 }
    }

    /// DEEP_GREEN
    #[classattr]
    fn DEEP_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.34901960784313724, b: 0.058823529411764705, a: 1.0 }
    }

    /// VOMIT_GREEN
    #[classattr]
    fn VOMIT_GREEN() -> Color {
        Color { r: 0.5372549019607843, g: 0.6352941176470588, b: 0.011764705882352941, a: 1.0 }
    }

    /// DUSTY_PINK
    #[classattr]
    fn DUSTY_PINK() -> Color {
        Color { r: 0.8352941176470589, g: 0.5411764705882353, b: 0.5803921568627451, a: 1.0 }
    }

    /// FADED_GREEN
    #[classattr]
    fn FADED_GREEN() -> Color {
        Color { r: 0.4823529411764706, g: 0.6980392156862745, b: 0.4549019607843137, a: 1.0 }
    }

    /// CAMO_GREEN
    #[classattr]
    fn CAMO_GREEN() -> Color {
        Color { r: 0.3215686274509804, g: 0.396078431372549, b: 0.1450980392156863, a: 1.0 }
    }

    /// PINKY_PURPLE
    #[classattr]
    fn PINKY_PURPLE() -> Color {
        Color { r: 0.788235294117647, g: 0.2980392156862745, b: 0.7450980392156863, a: 1.0 }
    }

    /// BROWNISH_RED
    #[classattr]
    fn BROWNISH_RED() -> Color {
        Color { r: 0.6196078431372549, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_ROSE
    #[classattr]
    fn DARK_ROSE() -> Color {
        Color { r: 0.7098039215686275, g: 0.2823529411764706, b: 0.36470588235294116, a: 1.0 }
    }

    /// MUD
    #[classattr]
    fn MUD() -> Color {
        Color { r: 0.45098039215686275, g: 0.3607843137254902, b: 0.07058823529411765, a: 1.0 }
    }

    /// BROWNISH
    #[classattr]
    fn BROWNISH() -> Color {
        Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.3411764705882353, a: 1.0 }
    }

    /// EMERALD_GREEN
    #[classattr]
    fn EMERALD_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.5607843137254902, b: 0.11764705882352941, a: 1.0 }
    }

    /// PALE_BROWN
    #[classattr]
    fn PALE_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.5686274509803921, b: 0.43137254901960786, a: 1.0 }
    }

    /// DULL_BLUE
    #[classattr]
    fn DULL_BLUE() -> Color {
        Color { r: 0.28627450980392155, g: 0.4588235294117647, b: 0.611764705882353, a: 1.0 }
    }

    /// BURNT_UMBER
    #[classattr]
    fn BURNT_UMBER() -> Color {
        Color { r: 0.6274509803921569, g: 0.27058823529411763, b: 0.054901960784313725, a: 1.0 }
    }

    /// MEDIUM_GREEN
    #[classattr]
    fn MEDIUM_GREEN() -> Color {
        Color { r: 0.2235294117647059, g: 0.6784313725490196, b: 0.2823529411764706, a: 1.0 }
    }

    /// CLAY
    #[classattr]
    fn CLAY() -> Color {
        Color { r: 0.7137254901960784, g: 0.41568627450980394, b: 0.3137254901960784, a: 1.0 }
    }

    /// LIGHT_AQUA
    #[classattr]
    fn LIGHT_AQUA() -> Color {
        Color { r: 0.5490196078431373, g: 1.0, b: 0.8588235294117647, a: 1.0 }
    }

    /// LIGHT_OLIVE_GREEN
    #[classattr]
    fn LIGHT_OLIVE_GREEN() -> Color {
        Color { r: 0.6431372549019608, g: 0.7450980392156863, b: 0.3607843137254902, a: 1.0 }
    }

    /// BROWNISH_ORANGE
    #[classattr]
    fn BROWNISH_ORANGE() -> Color {
        Color { r: 0.796078431372549, g: 0.4666666666666667, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_AQUA
    #[classattr]
    fn DARK_AQUA() -> Color {
        Color { r: 0.0196078431372549, g: 0.4117647058823529, b: 0.4196078431372549, a: 1.0 }
    }

    /// PURPLISH_PINK
    #[classattr]
    fn PURPLISH_PINK() -> Color {
        Color { r: 0.807843137254902, g: 0.36470588235294116, b: 0.6823529411764706, a: 1.0 }
    }

    /// DARK_SALMON
    #[classattr]
    fn DARK_SALMON() -> Color {
        Color { r: 0.7843137254901961, g: 0.35294117647058826, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREENISH_GREY
    #[classattr]
    fn GREENISH_GREY() -> Color {
        Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 }
    }

    /// JADE
    #[classattr]
    fn JADE() -> Color {
        Color { r: 0.12156862745098039, g: 0.6549019607843137, b: 0.4549019607843137, a: 1.0 }
    }

    /// UGLY_GREEN
    #[classattr]
    fn UGLY_GREEN() -> Color {
        Color { r: 0.47843137254901963, g: 0.592156862745098, b: 0.011764705882352941, a: 1.0 }
    }

    /// DARK_BEIGE
    #[classattr]
    fn DARK_BEIGE() -> Color {
        Color { r: 0.6745098039215687, g: 0.5764705882352941, b: 0.3843137254901961, a: 1.0 }
    }

    /// EMERALD
    #[classattr]
    fn EMERALD() -> Color {
        Color { r: 0.00392156862745098, g: 0.6274509803921569, b: 0.28627450980392155, a: 1.0 }
    }

    /// PALE_RED
    #[classattr]
    fn PALE_RED() -> Color {
        Color { r: 0.8509803921568627, g: 0.32941176470588235, b: 0.30196078431372547, a: 1.0 }
    }

    /// LIGHT_MAGENTA
    #[classattr]
    fn LIGHT_MAGENTA() -> Color {
        Color { r: 0.9803921568627451, g: 0.37254901960784315, b: 0.9686274509803922, a: 1.0 }
    }

    /// SKY
    #[classattr]
    fn SKY() -> Color {
        Color { r: 0.5098039215686274, g: 0.792156862745098, b: 0.9882352941176471, a: 1.0 }
    }

    /// LIGHT_CYAN
    #[classattr]
    fn LIGHT_CYAN() -> Color {
        Color { r: 0.6745098039215687, g: 1.0, b: 0.9882352941176471, a: 1.0 }
    }

    /// YELLOW_ORANGE
    #[classattr]
    fn YELLOW_ORANGE() -> Color {
        Color { r: 0.9882352941176471, g: 0.6901960784313725, b: 0.00392156862745098, a: 1.0 }
    }

    /// REDDISH_PURPLE
    #[classattr]
    fn REDDISH_PURPLE() -> Color {
        Color { r: 0.5686274509803921, g: 0.03529411764705882, b: 0.3176470588235294, a: 1.0 }
    }

    /// REDDISH_PINK
    #[classattr]
    fn REDDISH_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.17254901960784313, b: 0.32941176470588235, a: 1.0 }
    }

    /// ORCHID
    #[classattr]
    fn ORCHID() -> Color {
        Color { r: 0.7843137254901961, g: 0.4588235294117647, b: 0.7686274509803922, a: 1.0 }
    }

    /// DIRTY_YELLOW
    #[classattr]
    fn DIRTY_YELLOW() -> Color {
        Color { r: 0.803921568627451, g: 0.7725490196078432, b: 0.0392156862745098, a: 1.0 }
    }

    /// ORANGE_RED
    #[classattr]
    fn ORANGE_RED() -> Color {
        Color { r: 0.9921568627450981, g: 0.2549019607843137, b: 0.11764705882352941, a: 1.0 }
    }

    /// DEEP_RED
    #[classattr]
    fn DEEP_RED() -> Color {
        Color { r: 0.6039215686274509, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// ORANGE_BROWN
    #[classattr]
    fn ORANGE_BROWN() -> Color {
        Color { r: 0.7450980392156863, g: 0.39215686274509803, b: 0.0, a: 1.0 }
    }

    /// COBALT_BLUE
    #[classattr]
    fn COBALT_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.0392156862745098, b: 0.6549019607843137, a: 1.0 }
    }

    /// NEON_PINK
    #[classattr]
    fn NEON_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6039215686274509, a: 1.0 }
    }

    /// ROSE_PINK
    #[classattr]
    fn ROSE_PINK() -> Color {
        Color { r: 0.9686274509803922, g: 0.5294117647058824, b: 0.6039215686274509, a: 1.0 }
    }

    /// GREYISH_PURPLE
    #[classattr]
    fn GREYISH_PURPLE() -> Color {
        Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 }
    }

    /// RASPBERRY
    #[classattr]
    fn RASPBERRY() -> Color {
        Color { r: 0.6901960784313725, g: 0.00392156862745098, b: 0.28627450980392155, a: 1.0 }
    }

    /// AQUA_GREEN
    #[classattr]
    fn AQUA_GREEN() -> Color {
        Color { r: 0.07058823529411765, g: 0.8823529411764706, b: 0.5764705882352941, a: 1.0 }
    }

    /// SALMON_PINK
    #[classattr]
    fn SALMON_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.4823529411764706, b: 0.48627450980392156, a: 1.0 }
    }

    /// TANGERINE
    #[classattr]
    fn TANGERINE() -> Color {
        Color { r: 1.0, g: 0.5803921568627451, b: 0.03137254901960784, a: 1.0 }
    }

    /// BROWNISH_GREEN
    #[classattr]
    fn BROWNISH_GREEN() -> Color {
        Color { r: 0.41568627450980394, g: 0.43137254901960786, b: 0.03529411764705882, a: 1.0 }
    }

    /// RED_BROWN
    #[classattr]
    fn RED_BROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.1803921568627451, b: 0.08627450980392157, a: 1.0 }
    }

    /// GREENISH_BROWN
    #[classattr]
    fn GREENISH_BROWN() -> Color {
        Color { r: 0.4117647058823529, g: 0.3803921568627451, b: 0.07058823529411765, a: 1.0 }
    }

    /// PUMPKIN
    #[classattr]
    fn PUMPKIN() -> Color {
        Color { r: 0.8823529411764706, g: 0.4666666666666667, b: 0.00392156862745098, a: 1.0 }
    }

    /// PINE_GREEN
    #[classattr]
    fn PINE_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 0.2823529411764706, b: 0.11764705882352941, a: 1.0 }
    }

    /// CHARCOAL
    #[classattr]
    fn CHARCOAL() -> Color {
        Color { r: 0.20392156862745098, g: 0.2196078431372549, b: 0.21568627450980393, a: 1.0 }
    }

    /// BABY_PINK
    #[classattr]
    fn BABY_PINK() -> Color {
        Color { r: 1.0, g: 0.7176470588235294, b: 0.807843137254902, a: 1.0 }
    }

    /// CORNFLOWER
    #[classattr]
    fn CORNFLOWER() -> Color {
        Color { r: 0.41568627450980394, g: 0.4745098039215686, b: 0.9686274509803922, a: 1.0 }
    }

    /// BLUE_VIOLET
    #[classattr]
    fn BLUE_VIOLET() -> Color {
        Color { r: 0.36470588235294116, g: 0.023529411764705882, b: 0.9137254901960784, a: 1.0 }
    }

    /// CHOCOLATE
    #[classattr]
    fn CHOCOLATE() -> Color {
        Color { r: 0.23921568627450981, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 }
    }

    /// GREYISH_GREEN
    #[classattr]
    fn GREYISH_GREEN() -> Color {
        Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 }
    }

    /// SCARLET
    #[classattr]
    fn SCARLET() -> Color {
        Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.09803921568627451, a: 1.0 }
    }

    /// DARK_OLIVE
    #[classattr]
    fn DARK_OLIVE() -> Color {
        Color { r: 0.21568627450980393, g: 0.24313725490196078, b: 0.00784313725490196, a: 1.0 }
    }

    /// SIENNA
    #[classattr]
    fn SIENNA() -> Color {
        Color { r: 0.6627450980392157, g: 0.33725490196078434, b: 0.11764705882352941, a: 1.0 }
    }

    /// PASTEL_PURPLE
    #[classattr]
    fn PASTEL_PURPLE() -> Color {
        Color { r: 0.792156862745098, g: 0.6274509803921569, b: 1.0, a: 1.0 }
    }

    /// TERRACOTTA
    #[classattr]
    fn TERRACOTTA() -> Color {
        Color { r: 0.792156862745098, g: 0.4, b: 0.2549019607843137, a: 1.0 }
    }

    /// AQUA_BLUE
    #[classattr]
    fn AQUA_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.8470588235294118, b: 0.9137254901960784, a: 1.0 }
    }

    /// SAGE_GREEN
    #[classattr]
    fn SAGE_GREEN() -> Color {
        Color { r: 0.5333333333333333, g: 0.7019607843137254, b: 0.47058823529411764, a: 1.0 }
    }

    /// BLOOD_RED
    #[classattr]
    fn BLOOD_RED() -> Color {
        Color { r: 0.596078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// DEEP_PINK
    #[classattr]
    fn DEEP_PINK() -> Color {
        Color { r: 0.796078431372549, g: 0.00392156862745098, b: 0.3843137254901961, a: 1.0 }
    }

    /// GRASS
    #[classattr]
    fn GRASS() -> Color {
        Color { r: 0.3607843137254902, g: 0.6745098039215687, b: 0.17647058823529413, a: 1.0 }
    }

    /// MOSS
    #[classattr]
    fn MOSS() -> Color {
        Color { r: 0.4627450980392157, g: 0.6, b: 0.34509803921568627, a: 1.0 }
    }

    /// PASTEL_BLUE
    #[classattr]
    fn PASTEL_BLUE() -> Color {
        Color { r: 0.6352941176470588, g: 0.7490196078431373, b: 0.996078431372549, a: 1.0 }
    }

    /// BLUISH_GREEN
    #[classattr]
    fn BLUISH_GREEN() -> Color {
        Color { r: 0.06274509803921569, g: 0.6509803921568628, b: 0.4549019607843137, a: 1.0 }
    }

    /// DARK_TAN
    #[classattr]
    fn DARK_TAN() -> Color {
        Color { r: 0.6862745098039216, g: 0.5333333333333333, b: 0.2901960784313726, a: 1.0 }
    }

    /// GREENISH_BLUE
    #[classattr]
    fn GREENISH_BLUE() -> Color {
        Color { r: 0.043137254901960784, g: 0.5450980392156862, b: 0.5294117647058824, a: 1.0 }
    }

    /// PALE_ORANGE
    #[classattr]
    fn PALE_ORANGE() -> Color {
        Color { r: 1.0, g: 0.6549019607843137, b: 0.33725490196078434, a: 1.0 }
    }

    /// VOMIT
    #[classattr]
    fn VOMIT() -> Color {
        Color { r: 0.6352941176470588, g: 0.6431372549019608, b: 0.08235294117647059, a: 1.0 }
    }

    /// FORREST_GREEN
    #[classattr]
    fn FORREST_GREEN() -> Color {
        Color { r: 0.08235294117647059, g: 0.26666666666666666, b: 0.023529411764705882, a: 1.0 }
    }

    /// DARK_LAVENDER
    #[classattr]
    fn DARK_LAVENDER() -> Color {
        Color { r: 0.5215686274509804, g: 0.403921568627451, b: 0.596078431372549, a: 1.0 }
    }

    /// DARK_VIOLET
    #[classattr]
    fn DARK_VIOLET() -> Color {
        Color { r: 0.20392156862745098, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// DARK_CYAN
    #[classattr]
    fn DARK_CYAN() -> Color {
        Color { r: 0.0392156862745098, g: 0.5333333333333333, b: 0.5411764705882353, a: 1.0 }
    }

    /// OLIVE_DRAB
    #[classattr]
    fn OLIVE_DRAB() -> Color {
        Color { r: 0.43529411764705883, g: 0.4627450980392157, b: 0.19607843137254902, a: 1.0 }
    }

    /// PINKISH
    #[classattr]
    fn PINKISH() -> Color {
        Color { r: 0.8313725490196079, g: 0.41568627450980394, b: 0.49411764705882355, a: 1.0 }
    }

    /// COBALT
    #[classattr]
    fn COBALT() -> Color {
        Color { r: 0.11764705882352941, g: 0.2823529411764706, b: 0.5607843137254902, a: 1.0 }
    }

    /// NEON_PURPLE
    #[classattr]
    fn NEON_PURPLE() -> Color {
        Color { r: 0.7372549019607844, g: 0.07450980392156863, b: 0.996078431372549, a: 1.0 }
    }

    /// LIGHT_TURQUOISE
    #[classattr]
    fn LIGHT_TURQUOISE() -> Color {
        Color { r: 0.49411764705882355, g: 0.9568627450980393, b: 0.8, a: 1.0 }
    }

    /// APPLE_GREEN
    #[classattr]
    fn APPLE_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.803921568627451, b: 0.14901960784313725, a: 1.0 }
    }

    /// DULL_GREEN
    #[classattr]
    fn DULL_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 }
    }

    /// WINE
    #[classattr]
    fn WINE() -> Color {
        Color { r: 0.5019607843137255, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// POWDER_BLUE
    #[classattr]
    fn POWDER_BLUE() -> Color {
        Color { r: 0.6941176470588235, g: 0.8196078431372549, b: 0.9882352941176471, a: 1.0 }
    }

    /// OFF_WHITE
    #[classattr]
    fn OFF_WHITE() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8941176470588236, a: 1.0 }
    }

    /// ELECTRIC_BLUE
    #[classattr]
    fn ELECTRIC_BLUE() -> Color {
        Color { r: 0.023529411764705882, g: 0.3215686274509804, b: 1.0, a: 1.0 }
    }

    /// DARK_TURQUOISE
    #[classattr]
    fn DARK_TURQUOISE() -> Color {
        Color { r: 0.01568627450980392, g: 0.3607843137254902, b: 0.35294117647058826, a: 1.0 }
    }

    /// AZURE
    #[classattr]
    fn AZURE() -> Color {
        Color { r: 0.023529411764705882, g: 0.6039215686274509, b: 0.9529411764705882, a: 1.0 }
    }

    /// BRIGHT_RED
    #[classattr]
    fn BRIGHT_RED() -> Color {
        Color { r: 1.0, g: 0.0, b: 0.050980392156862744, a: 1.0 }
    }

    /// PINKISH_RED
    #[classattr]
    fn PINKISH_RED() -> Color {
        Color { r: 0.9450980392156862, g: 0.047058823529411764, b: 0.27058823529411763, a: 1.0 }
    }

    /// CORNFLOWER_BLUE
    #[classattr]
    fn CORNFLOWER_BLUE() -> Color {
        Color { r: 0.3176470588235294, g: 0.4392156862745098, b: 0.8431372549019608, a: 1.0 }
    }

    /// LIGHT_OLIVE
    #[classattr]
    fn LIGHT_OLIVE() -> Color {
        Color { r: 0.6745098039215687, g: 0.7490196078431373, b: 0.4117647058823529, a: 1.0 }
    }

    /// GRAPE
    #[classattr]
    fn GRAPE() -> Color {
        Color { r: 0.4235294117647059, g: 0.20392156862745098, b: 0.3803921568627451, a: 1.0 }
    }

    /// GREYISH_BLUE
    #[classattr]
    fn GREYISH_BLUE() -> Color {
        Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 }
    }

    /// PURPLISH_BLUE
    #[classattr]
    fn PURPLISH_BLUE() -> Color {
        Color { r: 0.3764705882352941, g: 0.11764705882352941, b: 0.9764705882352941, a: 1.0 }
    }

    /// YELLOWISH_GREEN
    #[classattr]
    fn YELLOWISH_GREEN() -> Color {
        Color { r: 0.6901960784313725, g: 0.8666666666666667, b: 0.08627450980392157, a: 1.0 }
    }

    /// GREENISH_YELLOW
    #[classattr]
    fn GREENISH_YELLOW() -> Color {
        Color { r: 0.803921568627451, g: 0.9921568627450981, b: 0.00784313725490196, a: 1.0 }
    }

    /// MEDIUM_BLUE
    #[classattr]
    fn MEDIUM_BLUE() -> Color {
        Color { r: 0.17254901960784313, g: 0.43529411764705883, b: 0.7333333333333333, a: 1.0 }
    }

    /// DUSTY_ROSE
    #[classattr]
    fn DUSTY_ROSE() -> Color {
        Color { r: 0.7529411764705882, g: 0.45098039215686275, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHT_VIOLET
    #[classattr]
    fn LIGHT_VIOLET() -> Color {
        Color { r: 0.8392156862745098, g: 0.7058823529411765, b: 0.9882352941176471, a: 1.0 }
    }

    /// MIDNIGHT_BLUE
    #[classattr]
    fn MIDNIGHT_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.0, b: 0.20784313725490197, a: 1.0 }
    }

    /// BLUISH_PURPLE
    #[classattr]
    fn BLUISH_PURPLE() -> Color {
        Color { r: 0.4392156862745098, g: 0.23137254901960785, b: 0.9058823529411765, a: 1.0 }
    }

    /// RED_ORANGE
    #[classattr]
    fn RED_ORANGE() -> Color {
        Color { r: 0.9921568627450981, g: 0.23529411764705882, b: 0.023529411764705882, a: 1.0 }
    }

    /// DARK_MAGENTA
    #[classattr]
    fn DARK_MAGENTA() -> Color {
        Color { r: 0.5882352941176471, g: 0.0, b: 0.33725490196078434, a: 1.0 }
    }

    /// GREENISH
    #[classattr]
    fn GREENISH() -> Color {
        Color { r: 0.25098039215686274, g: 0.6392156862745098, b: 0.40784313725490196, a: 1.0 }
    }

    /// OCEAN_BLUE
    #[classattr]
    fn OCEAN_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.44313725490196076, b: 0.611764705882353, a: 1.0 }
    }

    /// CORAL
    #[classattr]
    fn CORAL() -> Color {
        Color { r: 0.9882352941176471, g: 0.35294117647058826, b: 0.3137254901960784, a: 1.0 }
    }

    /// CREAM
    #[classattr]
    fn CREAM() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.7607843137254902, a: 1.0 }
    }

    /// REDDISH_BROWN
    #[classattr]
    fn REDDISH_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.16862745098039217, b: 0.0392156862745098, a: 1.0 }
    }

    /// BURNT_SIENNA
    #[classattr]
    fn BURNT_SIENNA() -> Color {
        Color { r: 0.6901960784313725, g: 0.3058823529411765, b: 0.058823529411764705, a: 1.0 }
    }

    /// BRICK
    #[classattr]
    fn BRICK() -> Color {
        Color { r: 0.6274509803921569, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 }
    }

    /// SAGE
    #[classattr]
    fn SAGE() -> Color {
        Color { r: 0.5294117647058824, g: 0.6823529411764706, b: 0.45098039215686275, a: 1.0 }
    }

    /// WHITE
    #[classattr]
    fn WHITE() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// ROBINS_EGG_BLUE
    #[classattr]
    fn ROBINS_EGG_BLUE() -> Color {
        Color { r: 0.596078431372549, g: 0.9372549019607843, b: 0.9764705882352941, a: 1.0 }
    }

    /// MOSS_GREEN
    #[classattr]
    fn MOSS_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.5450980392156862, b: 0.2196078431372549, a: 1.0 }
    }

    /// STEEL_BLUE
    #[classattr]
    fn STEEL_BLUE() -> Color {
        Color { r: 0.35294117647058826, g: 0.49019607843137253, b: 0.6039215686274509, a: 1.0 }
    }

    /// EGGPLANT
    #[classattr]
    fn EGGPLANT() -> Color {
        Color { r: 0.2196078431372549, g: 0.03137254901960784, b: 0.20784313725490197, a: 1.0 }
    }

    /// LIGHT_YELLOW
    #[classattr]
    fn LIGHT_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.47843137254901963, a: 1.0 }
    }

    /// LEAF_GREEN
    #[classattr]
    fn LEAF_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.6627450980392157, b: 0.01568627450980392, a: 1.0 }
    }

    /// LIGHT_GREY
    #[classattr]
    fn LIGHT_GREY() -> Color {
        Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 }
    }

    /// PUKE
    #[classattr]
    fn PUKE() -> Color {
        Color { r: 0.6470588235294118, g: 0.6470588235294118, b: 0.00784313725490196, a: 1.0 }
    }

    /// PINKISH_PURPLE
    #[classattr]
    fn PINKISH_PURPLE() -> Color {
        Color { r: 0.8392156862745098, g: 0.2823529411764706, b: 0.8431372549019608, a: 1.0 }
    }

    /// SEA_BLUE
    #[classattr]
    fn SEA_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.4549019607843137, b: 0.5843137254901961, a: 1.0 }
    }

    /// PALE_PURPLE
    #[classattr]
    fn PALE_PURPLE() -> Color {
        Color { r: 0.7176470588235294, g: 0.5647058823529412, b: 0.8313725490196079, a: 1.0 }
    }

    /// SLATE_BLUE
    #[classattr]
    fn SLATE_BLUE() -> Color {
        Color { r: 0.3568627450980392, g: 0.48627450980392156, b: 0.6, a: 1.0 }
    }

    /// HUNTER_GREEN
    #[classattr]
    fn HUNTER_GREEN() -> Color {
        Color { r: 0.043137254901960784, g: 0.25098039215686274, b: 0.03137254901960784, a: 1.0 }
    }

    /// FUCHSIA
    #[classattr]
    fn FUCHSIA() -> Color {
        Color { r: 0.9294117647058824, g: 0.050980392156862744, b: 0.8509803921568627, a: 1.0 }
    }

    /// CRIMSON
    #[classattr]
    fn CRIMSON() -> Color {
        Color { r: 0.5490196078431373, g: 0.0, b: 0.058823529411764705, a: 1.0 }
    }

    /// PALE_YELLOW
    #[classattr]
    fn PALE_YELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.5176470588235295, a: 1.0 }
    }

    /// OCHRE
    #[classattr]
    fn OCHRE() -> Color {
        Color { r: 0.7490196078431373, g: 0.5647058823529412, b: 0.0196078431372549, a: 1.0 }
    }

    /// MUSTARD_YELLOW
    #[classattr]
    fn MUSTARD_YELLOW() -> Color {
        Color { r: 0.8235294117647058, g: 0.7411764705882353, b: 0.0392156862745098, a: 1.0 }
    }

    /// LIGHT_RED
    #[classattr]
    fn LIGHT_RED() -> Color {
        Color { r: 1.0, g: 0.2784313725490196, b: 0.2980392156862745, a: 1.0 }
    }

    /// CERULEAN
    #[classattr]
    fn CERULEAN() -> Color {
        Color { r: 0.01568627450980392, g: 0.5215686274509804, b: 0.8196078431372549, a: 1.0 }
    }

    /// PALE_PINK
    #[classattr]
    fn PALE_PINK() -> Color {
        Color { r: 1.0, g: 0.8117647058823529, b: 0.8627450980392157, a: 1.0 }
    }

    /// DEEP_BLUE
    #[classattr]
    fn DEEP_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.00784313725490196, b: 0.45098039215686275, a: 1.0 }
    }

    /// RUST
    #[classattr]
    fn RUST() -> Color {
        Color { r: 0.6588235294117647, g: 0.23529411764705882, b: 0.03529411764705882, a: 1.0 }
    }

    /// LIGHT_TEAL
    #[classattr]
    fn LIGHT_TEAL() -> Color {
        Color { r: 0.5647058823529412, g: 0.8941176470588236, b: 0.7568627450980392, a: 1.0 }
    }

    /// SLATE
    #[classattr]
    fn SLATE() -> Color {
        Color { r: 0.3176470588235294, g: 0.396078431372549, b: 0.4470588235294118, a: 1.0 }
    }

    /// GOLDENROD
    #[classattr]
    fn GOLDENROD() -> Color {
        Color { r: 0.9803921568627451, g: 0.7607843137254902, b: 0.0196078431372549, a: 1.0 }
    }

    /// DARK_YELLOW
    #[classattr]
    fn DARK_YELLOW() -> Color {
        Color { r: 0.8352941176470589, g: 0.7137254901960784, b: 0.0392156862745098, a: 1.0 }
    }

    /// DARK_GREY
    #[classattr]
    fn DARK_GREY() -> Color {
        Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 }
    }

    /// ARMY_GREEN
    #[classattr]
    fn ARMY_GREEN() -> Color {
        Color { r: 0.29411764705882354, g: 0.36470588235294116, b: 0.08627450980392157, a: 1.0 }
    }

    /// SEAFOAM
    #[classattr]
    fn SEAFOAM() -> Color {
        Color { r: 0.5019607843137255, g: 0.9764705882352941, b: 0.6784313725490196, a: 1.0 }
    }

    /// PUCE
    #[classattr]
    fn PUCE() -> Color {
        Color { r: 0.6470588235294118, g: 0.49411764705882355, b: 0.3215686274509804, a: 1.0 }
    }

    /// SPRING_GREEN
    #[classattr]
    fn SPRING_GREEN() -> Color {
        Color { r: 0.6627450980392157, g: 0.9764705882352941, b: 0.44313725490196076, a: 1.0 }
    }

    /// DARK_ORANGE
    #[classattr]
    fn DARK_ORANGE() -> Color {
        Color { r: 0.7764705882352941, g: 0.3176470588235294, b: 0.00784313725490196, a: 1.0 }
    }

    /// SAND
    #[classattr]
    fn SAND() -> Color {
        Color { r: 0.8862745098039215, g: 0.792156862745098, b: 0.4627450980392157, a: 1.0 }
    }

    /// PASTEL_GREEN
    #[classattr]
    fn PASTEL_GREEN() -> Color {
        Color { r: 0.6901960784313725, g: 1.0, b: 0.615686274509804, a: 1.0 }
    }

    /// MINT
    #[classattr]
    fn MINT() -> Color {
        Color { r: 0.6235294117647059, g: 0.996078431372549, b: 0.6901960784313725, a: 1.0 }
    }

    /// LIGHT_ORANGE
    #[classattr]
    fn LIGHT_ORANGE() -> Color {
        Color { r: 0.9921568627450981, g: 0.6666666666666666, b: 0.2823529411764706, a: 1.0 }
    }

    /// BRIGHT_PINK
    #[classattr]
    fn BRIGHT_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6941176470588235, a: 1.0 }
    }

    /// CHARTREUSE
    #[classattr]
    fn CHARTREUSE() -> Color {
        Color { r: 0.7568627450980392, g: 0.9725490196078431, b: 0.0392156862745098, a: 1.0 }
    }

    /// DEEP_PURPLE
    #[classattr]
    fn DEEP_PURPLE() -> Color {
        Color { r: 0.21176470588235294, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// DARK_BROWN
    #[classattr]
    fn DARK_BROWN() -> Color {
        Color { r: 0.20392156862745098, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 }
    }

    /// TAUPE
    #[classattr]
    fn TAUPE() -> Color {
        Color { r: 0.7254901960784313, g: 0.6352941176470588, b: 0.5058823529411764, a: 1.0 }
    }

    /// PEA_GREEN
    #[classattr]
    fn PEA_GREEN() -> Color {
        Color { r: 0.5568627450980392, g: 0.6705882352941176, b: 0.07058823529411765, a: 1.0 }
    }

    /// PUKE_GREEN
    #[classattr]
    fn PUKE_GREEN() -> Color {
        Color { r: 0.6039215686274509, g: 0.6823529411764706, b: 0.027450980392156862, a: 1.0 }
    }

    /// KELLY_GREEN
    #[classattr]
    fn KELLY_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.6705882352941176, b: 0.1803921568627451, a: 1.0 }
    }

    /// SEAFOAM_GREEN
    #[classattr]
    fn SEAFOAM_GREEN() -> Color {
        Color { r: 0.47843137254901963, g: 0.9764705882352941, b: 0.6705882352941176, a: 1.0 }
    }

    /// KHAKI
    #[classattr]
    fn KHAKI() -> Color {
        Color { r: 0.6666666666666666, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 }
    }

    /// BURGUNDY
    #[classattr]
    fn BURGUNDY() -> Color {
        Color { r: 0.3803921568627451, g: 0.0, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_TEAL
    #[classattr]
    fn DARK_TEAL() -> Color {
        Color { r: 0.00392156862745098, g: 0.30196078431372547, b: 0.3058823529411765, a: 1.0 }
    }

    /// BRICK_RED
    #[classattr]
    fn BRICK_RED() -> Color {
        Color { r: 0.5607843137254902, g: 0.0784313725490196, b: 0.00784313725490196, a: 1.0 }
    }

    /// ROYAL_PURPLE
    #[classattr]
    fn ROYAL_PURPLE() -> Color {
        Color { r: 0.29411764705882354, g: 0.0, b: 0.43137254901960786, a: 1.0 }
    }

    /// PLUM
    #[classattr]
    fn PLUM() -> Color {
        Color { r: 0.34509803921568627, g: 0.058823529411764705, b: 0.2549019607843137, a: 1.0 }
    }

    /// MINT_GREEN
    #[classattr]
    fn MINT_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 1.0, b: 0.6235294117647059, a: 1.0 }
    }

    /// GOLD
    #[classattr]
    fn GOLD() -> Color {
        Color { r: 0.8588235294117647, g: 0.7058823529411765, b: 0.047058823529411764, a: 1.0 }
    }

    /// BABY_BLUE
    #[classattr]
    fn BABY_BLUE() -> Color {
        Color { r: 0.6352941176470588, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 }
    }

    /// BRIGHT_PURPLE
    #[classattr]
    fn BRIGHT_PURPLE() -> Color {
        Color { r: 0.7450980392156863, g: 0.011764705882352941, b: 0.9921568627450981, a: 1.0 }
    }

    /// DARK_RED
    #[classattr]
    fn DARK_RED() -> Color {
        Color { r: 0.5176470588235295, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// PALE_BLUE
    #[classattr]
    fn PALE_BLUE() -> Color {
        Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.996078431372549, a: 1.0 }
    }

    /// GRASS_GREEN
    #[classattr]
    fn GRASS_GREEN() -> Color {
        Color { r: 0.24705882352941178, g: 0.6078431372549019, b: 0.043137254901960784, a: 1.0 }
    }

    /// NAVY
    #[classattr]
    fn NAVY() -> Color {
        Color { r: 0.00392156862745098, g: 0.08235294117647059, b: 0.24313725490196078, a: 1.0 }
    }

    /// AQUAMARINE
    #[classattr]
    fn AQUAMARINE() -> Color {
        Color { r: 0.01568627450980392, g: 0.8470588235294118, b: 0.6980392156862745, a: 1.0 }
    }

    /// BURNT_ORANGE
    #[classattr]
    fn BURNT_ORANGE() -> Color {
        Color { r: 0.7529411764705882, g: 0.3058823529411765, b: 0.00392156862745098, a: 1.0 }
    }

    /// NEON_GREEN
    #[classattr]
    fn NEON_GREEN() -> Color {
        Color { r: 0.047058823529411764, g: 1.0, b: 0.047058823529411764, a: 1.0 }
    }

    /// BRIGHT_BLUE
    #[classattr]
    fn BRIGHT_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.396078431372549, b: 0.9882352941176471, a: 1.0 }
    }

    /// ROSE
    #[classattr]
    fn ROSE() -> Color {
        Color { r: 0.8117647058823529, g: 0.3843137254901961, b: 0.4588235294117647, a: 1.0 }
    }

    /// LIGHT_PINK
    #[classattr]
    fn LIGHT_PINK() -> Color {
        Color { r: 1.0, g: 0.8196078431372549, b: 0.8745098039215686, a: 1.0 }
    }

    /// MUSTARD
    #[classattr]
    fn MUSTARD() -> Color {
        Color { r: 0.807843137254902, g: 0.7019607843137254, b: 0.00392156862745098, a: 1.0 }
    }

    /// INDIGO
    #[classattr]
    fn INDIGO() -> Color {
        Color { r: 0.2196078431372549, g: 0.00784313725490196, b: 0.5098039215686274, a: 1.0 }
    }

    /// LIME
    #[classattr]
    fn LIME() -> Color {
        Color { r: 0.6666666666666666, g: 1.0, b: 0.19607843137254902, a: 1.0 }
    }

    /// SEA_GREEN
    #[classattr]
    fn SEA_GREEN() -> Color {
        Color { r: 0.3254901960784314, g: 0.9882352941176471, b: 0.6313725490196078, a: 1.0 }
    }

    /// PERIWINKLE
    #[classattr]
    fn PERIWINKLE() -> Color {
        Color { r: 0.5568627450980392, g: 0.5098039215686274, b: 0.996078431372549, a: 1.0 }
    }

    /// DARK_PINK
    #[classattr]
    fn DARK_PINK() -> Color {
        Color { r: 0.796078431372549, g: 0.2549019607843137, b: 0.4196078431372549, a: 1.0 }
    }

    /// OLIVE_GREEN
    #[classattr]
    fn OLIVE_GREEN() -> Color {
        Color { r: 0.403921568627451, g: 0.47843137254901963, b: 0.01568627450980392, a: 1.0 }
    }

    /// PEACH
    #[classattr]
    fn PEACH() -> Color {
        Color { r: 1.0, g: 0.6901960784313725, b: 0.48627450980392156, a: 1.0 }
    }

    /// PALE_GREEN
    #[classattr]
    fn PALE_GREEN() -> Color {
        Color { r: 0.7803921568627451, g: 0.9921568627450981, b: 0.7098039215686275, a: 1.0 }
    }

    /// LIGHT_BROWN
    #[classattr]
    fn LIGHT_BROWN() -> Color {
        Color { r: 0.6784313725490196, g: 0.5058823529411764, b: 0.3137254901960784, a: 1.0 }
    }

    /// HOT_PINK
    #[classattr]
    fn HOT_PINK() -> Color {
        Color { r: 1.0, g: 0.00784313725490196, b: 0.5529411764705883, a: 1.0 }
    }

    /// BLACK
    #[classattr]
    fn BLACK() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// LILAC
    #[classattr]
    fn LILAC() -> Color {
        Color { r: 0.807843137254902, g: 0.6352941176470588, b: 0.9921568627450981, a: 1.0 }
    }

    /// NAVY_BLUE
    #[classattr]
    fn NAVY_BLUE() -> Color {
        Color { r: 0.0, g: 0.06666666666666667, b: 0.27450980392156865, a: 1.0 }
    }

    /// ROYAL_BLUE
    #[classattr]
    fn ROYAL_BLUE() -> Color {
        Color { r: 0.0196078431372549, g: 0.01568627450980392, b: 0.6666666666666666, a: 1.0 }
    }

    /// BEIGE
    #[classattr]
    fn BEIGE() -> Color {
        Color { r: 0.9019607843137255, g: 0.8549019607843137, b: 0.6509803921568628, a: 1.0 }
    }

    /// SALMON
    #[classattr]
    fn SALMON() -> Color {
        Color { r: 1.0, g: 0.4745098039215686, b: 0.4235294117647059, a: 1.0 }
    }

    /// OLIVE
    #[classattr]
    fn OLIVE() -> Color {
        Color { r: 0.43137254901960786, g: 0.4588235294117647, b: 0.054901960784313725, a: 1.0 }
    }

    /// MAROON
    #[classattr]
    fn MAROON() -> Color {
        Color { r: 0.396078431372549, g: 0.0, b: 0.12941176470588237, a: 1.0 }
    }

    /// BRIGHT_GREEN
    #[classattr]
    fn BRIGHT_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 1.0, b: 0.027450980392156862, a: 1.0 }
    }

    /// DARK_PURPLE
    #[classattr]
    fn DARK_PURPLE() -> Color {
        Color { r: 0.20784313725490197, g: 0.023529411764705882, b: 0.24313725490196078, a: 1.0 }
    }

    /// MAUVE
    #[classattr]
    fn MAUVE() -> Color {
        Color { r: 0.6823529411764706, g: 0.44313725490196076, b: 0.5058823529411764, a: 1.0 }
    }

    /// FOREST_GREEN
    #[classattr]
    fn FOREST_GREEN() -> Color {
        Color { r: 0.023529411764705882, g: 0.2784313725490196, b: 0.047058823529411764, a: 1.0 }
    }

    /// AQUA
    #[classattr]
    fn AQUA() -> Color {
        Color { r: 0.07450980392156863, g: 0.9176470588235294, b: 0.788235294117647, a: 1.0 }
    }

    /// CYAN
    #[classattr]
    fn CYAN() -> Color {
        Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// TAN
    #[classattr]
    fn TAN() -> Color {
        Color { r: 0.8196078431372549, g: 0.6980392156862745, b: 0.43529411764705883, a: 1.0 }
    }

    /// DARK_BLUE
    #[classattr]
    fn DARK_BLUE() -> Color {
        Color { r: 0.0, g: 0.011764705882352941, b: 0.3568627450980392, a: 1.0 }
    }

    /// LAVENDER
    #[classattr]
    fn LAVENDER() -> Color {
        Color { r: 0.7803921568627451, g: 0.6235294117647059, b: 0.9372549019607843, a: 1.0 }
    }

    /// TURQUOISE
    #[classattr]
    fn TURQUOISE() -> Color {
        Color { r: 0.023529411764705882, g: 0.7607843137254902, b: 0.6745098039215687, a: 1.0 }
    }

    /// DARK_GREEN
    #[classattr]
    fn DARK_GREEN() -> Color {
        Color { r: 0.011764705882352941, g: 0.20784313725490197, b: 0.0, a: 1.0 }
    }

    /// VIOLET
    #[classattr]
    fn VIOLET() -> Color {
        Color { r: 0.6039215686274509, g: 0.054901960784313725, b: 0.9176470588235294, a: 1.0 }
    }

    /// LIGHT_PURPLE
    #[classattr]
    fn LIGHT_PURPLE() -> Color {
        Color { r: 0.7490196078431373, g: 0.4666666666666667, b: 0.9647058823529412, a: 1.0 }
    }

    /// LIME_GREEN
    #[classattr]
    fn LIME_GREEN() -> Color {
        Color { r: 0.5372549019607843, g: 0.996078431372549, b: 0.0196078431372549, a: 1.0 }
    }

    /// GREY
    #[classattr]
    fn GREY() -> Color {
        Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 }
    }

    /// SKY_BLUE
    #[classattr]
    fn SKY_BLUE() -> Color {
        Color { r: 0.4588235294117647, g: 0.7333333333333333, b: 0.9921568627450981, a: 1.0 }
    }

    /// YELLOW
    #[classattr]
    fn YELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.0784313725490196, a: 1.0 }
    }

    /// MAGENTA
    #[classattr]
    fn MAGENTA() -> Color {
        Color { r: 0.7607843137254902, g: 0.0, b: 0.47058823529411764, a: 1.0 }
    }

    /// LIGHT_GREEN
    #[classattr]
    fn LIGHT_GREEN() -> Color {
        Color { r: 0.5882352941176471, g: 0.9764705882352941, b: 0.4823529411764706, a: 1.0 }
    }

    /// ORANGE
    #[classattr]
    fn ORANGE() -> Color {
        Color { r: 0.9764705882352941, g: 0.45098039215686275, b: 0.023529411764705882, a: 1.0 }
    }

    /// TEAL
    #[classattr]
    fn TEAL() -> Color {
        Color { r: 0.00784313725490196, g: 0.5764705882352941, b: 0.5254901960784314, a: 1.0 }
    }

    /// LIGHT_BLUE
    #[classattr]
    fn LIGHT_BLUE() -> Color {
        Color { r: 0.5843137254901961, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 }
    }

    /// RED
    #[classattr]
    fn RED() -> Color {
        Color { r: 0.8980392156862745, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// BROWN
    #[classattr]
    fn BROWN() -> Color {
        Color { r: 0.396078431372549, g: 0.21568627450980393, b: 0.0, a: 1.0 }
    }

    /// PINK
    #[classattr]
    fn PINK() -> Color {
        Color { r: 1.0, g: 0.5058823529411764, b: 0.7529411764705882, a: 1.0 }
    }

    /// BLUE
    #[classattr]
    fn BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.2627450980392157, b: 0.8745098039215686, a: 1.0 }
    }

    /// GREEN
    #[classattr]
    fn GREEN() -> Color {
        Color { r: 0.08235294117647059, g: 0.6901960784313725, b: 0.10196078431372549, a: 1.0 }
    }

    /// PURPLE
    #[classattr]
    fn PURPLE() -> Color {
        Color { r: 0.49411764705882355, g: 0.11764705882352941, b: 0.611764705882353, a: 1.0 }
    }

    /// GRAY_TEAL
    #[classattr]
    fn GRAY_TEAL() -> Color {
        Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 }
    }

    /// PURPLEY_GRAY
    #[classattr]
    fn PURPLEY_GRAY() -> Color {
        Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 }
    }

    /// LIGHT_GRAY_GREEN
    #[classattr]
    fn LIGHT_GRAY_GREEN() -> Color {
        Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 }
    }

    /// REDDISH_GRAY
    #[classattr]
    fn REDDISH_GRAY() -> Color {
        Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 }
    }

    /// BATTLESHIP_GRAY
    #[classattr]
    fn BATTLESHIP_GRAY() -> Color {
        Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 }
    }

    /// CHARCOAL_GRAY
    #[classattr]
    fn CHARCOAL_GRAY() -> Color {
        Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 }
    }

    /// GRAYISH_TEAL
    #[classattr]
    fn GRAYISH_TEAL() -> Color {
        Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 }
    }

    /// GRAY_GREEN
    #[classattr]
    fn GRAY_GREEN() -> Color {
        Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 }
    }

    /// COOL_GRAY
    #[classattr]
    fn COOL_GRAY() -> Color {
        Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_BLUE_GRAY
    #[classattr]
    fn DARK_BLUE_GRAY() -> Color {
        Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 }
    }

    /// BLUEY_GRAY
    #[classattr]
    fn BLUEY_GRAY() -> Color {
        Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 }
    }

    /// GREENY_GRAY
    #[classattr]
    fn GREENY_GRAY() -> Color {
        Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// BLUEGRAY
    #[classattr]
    fn BLUEGRAY() -> Color {
        Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 }
    }

    /// LIGHT_BLUE_GRAY
    #[classattr]
    fn LIGHT_BLUE_GRAY() -> Color {
        Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 }
    }

    /// GRAY_BLUE
    #[classattr]
    fn GRAY_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 }
    }

    /// BROWN_GRAY
    #[classattr]
    fn BROWN_GRAY() -> Color {
        Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 }
    }

    /// BLUE_GRAY
    #[classattr]
    fn BLUE_GRAY() -> Color {
        Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 }
    }

    /// GRAYBLUE
    #[classattr]
    fn GRAYBLUE() -> Color {
        Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 }
    }

    /// DARK_GRAY_BLUE
    #[classattr]
    fn DARK_GRAY_BLUE() -> Color {
        Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 }
    }

    /// GRAYISH
    #[classattr]
    fn GRAYISH() -> Color {
        Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 }
    }

    /// LIGHT_GRAY_BLUE
    #[classattr]
    fn LIGHT_GRAY_BLUE() -> Color {
        Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 }
    }

    /// PALE_GRAY
    #[classattr]
    fn PALE_GRAY() -> Color {
        Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// WARM_GRAY
    #[classattr]
    fn WARM_GRAY() -> Color {
        Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 }
    }

    /// GRAY_PINK
    #[classattr]
    fn GRAY_PINK() -> Color {
        Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 }
    }

    /// MEDIUM_GRAY
    #[classattr]
    fn MEDIUM_GRAY() -> Color {
        Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 }
    }

    /// PINKISH_GRAY
    #[classattr]
    fn PINKISH_GRAY() -> Color {
        Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 }
    }

    /// BROWNISH_GRAY
    #[classattr]
    fn BROWNISH_GRAY() -> Color {
        Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 }
    }

    /// PURPLISH_GRAY
    #[classattr]
    fn PURPLISH_GRAY() -> Color {
        Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 }
    }

    /// GRAYISH_PINK
    #[classattr]
    fn GRAYISH_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 }
    }

    /// GRAYISH_BROWN
    #[classattr]
    fn GRAYISH_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 }
    }

    /// STEEL_GRAY
    #[classattr]
    fn STEEL_GRAY() -> Color {
        Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 }
    }

    /// PURPLE_GRAY
    #[classattr]
    fn PURPLE_GRAY() -> Color {
        Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 }
    }

    /// GRAY_BROWN
    #[classattr]
    fn GRAY_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREEN_GRAY
    #[classattr]
    fn GREEN_GRAY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 }
    }

    /// BLUISH_GRAY
    #[classattr]
    fn BLUISH_GRAY() -> Color {
        Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 }
    }

    /// SLATE_GRAY
    #[classattr]
    fn SLATE_GRAY() -> Color {
        Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 }
    }

    /// GRAY_PURPLE
    #[classattr]
    fn GRAY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 }
    }

    /// GREENISH_GRAY
    #[classattr]
    fn GREENISH_GRAY() -> Color {
        Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 }
    }

    /// GRAYISH_PURPLE
    #[classattr]
    fn GRAYISH_PURPLE() -> Color {
        Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 }
    }

    /// GRAYISH_GREEN
    #[classattr]
    fn GRAYISH_GREEN() -> Color {
        Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 }
    }

    /// GRAYISH_BLUE
    #[classattr]
    fn GRAYISH_BLUE() -> Color {
        Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 }
    }

    /// LIGHT_GRAY
    #[classattr]
    fn LIGHT_GRAY() -> Color {
        Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 }
    }

    /// DARK_GRAY
    #[classattr]
    fn DARK_GRAY() -> Color {
        Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 }
    }

    /// GRAY
    #[classattr]
    fn GRAY() -> Color {
        Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 }
    }

    /// ALICEBLUE
    #[classattr]
    fn ALICEBLUE() -> Color {
        Color { r: 0.9411764705882353, g: 0.9725490196078431, b: 1.0, a: 1.0 }
    }

    /// ANTIQUEWHITE
    #[classattr]
    fn ANTIQUEWHITE() -> Color {
        Color { r: 0.9803921568627451, g: 0.9215686274509803, b: 0.8431372549019608, a: 1.0 }
    }

    /// BISQUE
    #[classattr]
    fn BISQUE() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.7686274509803922, a: 1.0 }
    }

    /// BLANCHEDALMOND
    #[classattr]
    fn BLANCHEDALMOND() -> Color {
        Color { r: 1.0, g: 0.9215686274509803, b: 0.803921568627451, a: 1.0 }
    }

    /// BLUEVIOLET
    #[classattr]
    fn BLUEVIOLET() -> Color {
        Color { r: 0.5411764705882353, g: 0.16862745098039217, b: 0.8862745098039215, a: 1.0 }
    }

    /// BURLYWOOD
    #[classattr]
    fn BURLYWOOD() -> Color {
        Color { r: 0.8705882352941177, g: 0.7215686274509804, b: 0.5294117647058824, a: 1.0 }
    }

    /// CADETBLUE
    #[classattr]
    fn CADETBLUE() -> Color {
        Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.6274509803921569, a: 1.0 }
    }

    /// CORNFLOWERBLUE
    #[classattr]
    fn CORNFLOWERBLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.5843137254901961, b: 0.9294117647058824, a: 1.0 }
    }

    /// CORNSILK
    #[classattr]
    fn CORNSILK() -> Color {
        Color { r: 1.0, g: 0.9725490196078431, b: 0.8627450980392157, a: 1.0 }
    }

    /// DARKCYAN
    #[classattr]
    fn DARKCYAN() -> Color {
        Color { r: 0.0, g: 0.5450980392156862, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKGOLDENROD
    #[classattr]
    fn DARKGOLDENROD() -> Color {
        Color { r: 0.7215686274509804, g: 0.5254901960784314, b: 0.043137254901960784, a: 1.0 }
    }

    /// DARKGRAY
    #[classattr]
    fn DARKGRAY() -> Color {
        Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 }
    }

    /// DARKGREY
    #[classattr]
    fn DARKGREY() -> Color {
        Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 }
    }

    /// DARKKHAKI
    #[classattr]
    fn DARKKHAKI() -> Color {
        Color { r: 0.7411764705882353, g: 0.7176470588235294, b: 0.4196078431372549, a: 1.0 }
    }

    /// DARKMAGENTA
    #[classattr]
    fn DARKMAGENTA() -> Color {
        Color { r: 0.5450980392156862, g: 0.0, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKOLIVEGREEN
    #[classattr]
    fn DARKOLIVEGREEN() -> Color {
        Color { r: 0.3333333333333333, g: 0.4196078431372549, b: 0.1843137254901961, a: 1.0 }
    }

    /// DARKORANGE
    #[classattr]
    fn DARKORANGE() -> Color {
        Color { r: 1.0, g: 0.5490196078431373, b: 0.0, a: 1.0 }
    }

    /// DARKORCHID
    #[classattr]
    fn DARKORCHID() -> Color {
        Color { r: 0.6, g: 0.19607843137254902, b: 0.8, a: 1.0 }
    }

    /// DARKRED
    #[classattr]
    fn DARKRED() -> Color {
        Color { r: 0.5450980392156862, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// DARKSALMON
    #[classattr]
    fn DARKSALMON() -> Color {
        Color { r: 0.9137254901960784, g: 0.5882352941176471, b: 0.47843137254901963, a: 1.0 }
    }

    /// DARKSEAGREEN
    #[classattr]
    fn DARKSEAGREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.7372549019607844, b: 0.5607843137254902, a: 1.0 }
    }

    /// DARKSLATEBLUE
    #[classattr]
    fn DARKSLATEBLUE() -> Color {
        Color { r: 0.2823529411764706, g: 0.23921568627450981, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKSLATEGRAY
    #[classattr]
    fn DARKSLATEGRAY() -> Color {
        Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 }
    }

    /// DARKSLATEGREY
    #[classattr]
    fn DARKSLATEGREY() -> Color {
        Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 }
    }

    /// DARKTURQUOISE
    #[classattr]
    fn DARKTURQUOISE() -> Color {
        Color { r: 0.0, g: 0.807843137254902, b: 0.8196078431372549, a: 1.0 }
    }

    /// DARKVIOLET
    #[classattr]
    fn DARKVIOLET() -> Color {
        Color { r: 0.5803921568627451, g: 0.0, b: 0.8274509803921568, a: 1.0 }
    }

    /// DEEPPINK
    #[classattr]
    fn DEEPPINK() -> Color {
        Color { r: 1.0, g: 0.0784313725490196, b: 0.5764705882352941, a: 1.0 }
    }

    /// DEEPSKYBLUE
    #[classattr]
    fn DEEPSKYBLUE() -> Color {
        Color { r: 0.0, g: 0.7490196078431373, b: 1.0, a: 1.0 }
    }

    /// DIMGRAY
    #[classattr]
    fn DIMGRAY() -> Color {
        Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 }
    }

    /// DIMGREY
    #[classattr]
    fn DIMGREY() -> Color {
        Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 }
    }

    /// DODGERBLUE
    #[classattr]
    fn DODGERBLUE() -> Color {
        Color { r: 0.11764705882352941, g: 0.5647058823529412, b: 1.0, a: 1.0 }
    }

    /// FIREBRICK
    #[classattr]
    fn FIREBRICK() -> Color {
        Color { r: 0.6980392156862745, g: 0.13333333333333333, b: 0.13333333333333333, a: 1.0 }
    }

    /// FLORALWHITE
    #[classattr]
    fn FLORALWHITE() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.9411764705882353, a: 1.0 }
    }

    /// FORESTGREEN
    #[classattr]
    fn FORESTGREEN() -> Color {
        Color { r: 0.13333333333333333, g: 0.5450980392156862, b: 0.13333333333333333, a: 1.0 }
    }

    /// GAINSBORO
    #[classattr]
    fn GAINSBORO() -> Color {
        Color { r: 0.8627450980392157, g: 0.8627450980392157, b: 0.8627450980392157, a: 1.0 }
    }

    /// GHOSTWHITE
    #[classattr]
    fn GHOSTWHITE() -> Color {
        Color { r: 0.9725490196078431, g: 0.9725490196078431, b: 1.0, a: 1.0 }
    }

    /// GREENYELLOW
    #[classattr]
    fn GREENYELLOW() -> Color {
        Color { r: 0.6784313725490196, g: 1.0, b: 0.1843137254901961, a: 1.0 }
    }

    /// HONEYDEW
    #[classattr]
    fn HONEYDEW() -> Color {
        Color { r: 0.9411764705882353, g: 1.0, b: 0.9411764705882353, a: 1.0 }
    }

    /// HOTPINK
    #[classattr]
    fn HOTPINK() -> Color {
        Color { r: 1.0, g: 0.4117647058823529, b: 0.7058823529411765, a: 1.0 }
    }

    /// INDIANRED
    #[classattr]
    fn INDIANRED() -> Color {
        Color { r: 0.803921568627451, g: 0.3607843137254902, b: 0.3607843137254902, a: 1.0 }
    }

    /// LAVENDERBLUSH
    #[classattr]
    fn LAVENDERBLUSH() -> Color {
        Color { r: 1.0, g: 0.9411764705882353, b: 0.9607843137254902, a: 1.0 }
    }

    /// LAWNGREEN
    #[classattr]
    fn LAWNGREEN() -> Color {
        Color { r: 0.48627450980392156, g: 0.9882352941176471, b: 0.0, a: 1.0 }
    }

    /// LEMONCHIFFON
    #[classattr]
    fn LEMONCHIFFON() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.803921568627451, a: 1.0 }
    }

    /// LIGHTCORAL
    #[classattr]
    fn LIGHTCORAL() -> Color {
        Color { r: 0.9411764705882353, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 }
    }

    /// LIGHTCYAN
    #[classattr]
    fn LIGHTCYAN() -> Color {
        Color { r: 0.8784313725490196, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// LIGHTGOLDENRODYELLOW
    #[classattr]
    fn LIGHTGOLDENRODYELLOW() -> Color {
        Color { r: 0.9803921568627451, g: 0.9803921568627451, b: 0.8235294117647058, a: 1.0 }
    }

    /// LIGHTGRAY
    #[classattr]
    fn LIGHTGRAY() -> Color {
        Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 }
    }

    /// LIGHTGREY
    #[classattr]
    fn LIGHTGREY() -> Color {
        Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 }
    }

    /// LIGHTPINK
    #[classattr]
    fn LIGHTPINK() -> Color {
        Color { r: 1.0, g: 0.7137254901960784, b: 0.7568627450980392, a: 1.0 }
    }

    /// LIGHTSALMON
    #[classattr]
    fn LIGHTSALMON() -> Color {
        Color { r: 1.0, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHTSEAGREEN
    #[classattr]
    fn LIGHTSEAGREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.6980392156862745, b: 0.6666666666666666, a: 1.0 }
    }

    /// LIGHTSKYBLUE
    #[classattr]
    fn LIGHTSKYBLUE() -> Color {
        Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9803921568627451, a: 1.0 }
    }

    /// LIGHTSLATEGRAY
    #[classattr]
    fn LIGHTSLATEGRAY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 }
    }

    /// LIGHTSLATEGREY
    #[classattr]
    fn LIGHTSLATEGREY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 }
    }

    /// LIGHTSTEELBLUE
    #[classattr]
    fn LIGHTSTEELBLUE() -> Color {
        Color { r: 0.6901960784313725, g: 0.7686274509803922, b: 0.8705882352941177, a: 1.0 }
    }

    /// LIGHTYELLOW
    #[classattr]
    fn LIGHTYELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8784313725490196, a: 1.0 }
    }

    /// LIMEGREEN
    #[classattr]
    fn LIMEGREEN() -> Color {
        Color { r: 0.19607843137254902, g: 0.803921568627451, b: 0.19607843137254902, a: 1.0 }
    }

    /// LINEN
    #[classattr]
    fn LINEN() -> Color {
        Color { r: 0.9803921568627451, g: 0.9411764705882353, b: 0.9019607843137255, a: 1.0 }
    }

    /// MEDIUMAQUAMARINE
    #[classattr]
    fn MEDIUMAQUAMARINE() -> Color {
        Color { r: 0.4, g: 0.803921568627451, b: 0.6666666666666666, a: 1.0 }
    }

    /// MEDIUMBLUE
    #[classattr]
    fn MEDIUMBLUE() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.803921568627451, a: 1.0 }
    }

    /// MEDIUMORCHID
    #[classattr]
    fn MEDIUMORCHID() -> Color {
        Color { r: 0.7294117647058823, g: 0.3333333333333333, b: 0.8274509803921568, a: 1.0 }
    }

    /// MEDIUMPURPLE
    #[classattr]
    fn MEDIUMPURPLE() -> Color {
        Color { r: 0.5764705882352941, g: 0.4392156862745098, b: 0.8588235294117647, a: 1.0 }
    }

    /// MEDIUMSEAGREEN
    #[classattr]
    fn MEDIUMSEAGREEN() -> Color {
        Color { r: 0.23529411764705882, g: 0.7019607843137254, b: 0.44313725490196076, a: 1.0 }
    }

    /// MEDIUMSLATEBLUE
    #[classattr]
    fn MEDIUMSLATEBLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.40784313725490196, b: 0.9333333333333333, a: 1.0 }
    }

    /// MEDIUMSPRINGGREEN
    #[classattr]
    fn MEDIUMSPRINGGREEN() -> Color {
        Color { r: 0.0, g: 0.9803921568627451, b: 0.6039215686274509, a: 1.0 }
    }

    /// MEDIUMTURQUOISE
    #[classattr]
    fn MEDIUMTURQUOISE() -> Color {
        Color { r: 0.2823529411764706, g: 0.8196078431372549, b: 0.8, a: 1.0 }
    }

    /// MEDIUMVIOLETRED
    #[classattr]
    fn MEDIUMVIOLETRED() -> Color {
        Color { r: 0.7803921568627451, g: 0.08235294117647059, b: 0.5215686274509804, a: 1.0 }
    }

    /// MIDNIGHTBLUE
    #[classattr]
    fn MIDNIGHTBLUE() -> Color {
        Color { r: 0.09803921568627451, g: 0.09803921568627451, b: 0.4392156862745098, a: 1.0 }
    }

    /// MINTCREAM
    #[classattr]
    fn MINTCREAM() -> Color {
        Color { r: 0.9607843137254902, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// MISTYROSE
    #[classattr]
    fn MISTYROSE() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.8823529411764706, a: 1.0 }
    }

    /// MOCCASIN
    #[classattr]
    fn MOCCASIN() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.7098039215686275, a: 1.0 }
    }

    /// NAVAJOWHITE
    #[classattr]
    fn NAVAJOWHITE() -> Color {
        Color { r: 1.0, g: 0.8705882352941177, b: 0.6784313725490196, a: 1.0 }
    }

    /// OLDLACE
    #[classattr]
    fn OLDLACE() -> Color {
        Color { r: 0.9921568627450981, g: 0.9607843137254902, b: 0.9019607843137255, a: 1.0 }
    }

    /// OLIVEDRAB
    #[classattr]
    fn OLIVEDRAB() -> Color {
        Color { r: 0.4196078431372549, g: 0.5568627450980392, b: 0.13725490196078433, a: 1.0 }
    }

    /// PALEGOLDENROD
    #[classattr]
    fn PALEGOLDENROD() -> Color {
        Color { r: 0.9333333333333333, g: 0.9098039215686274, b: 0.6666666666666666, a: 1.0 }
    }

    /// PALEGREEN
    #[classattr]
    fn PALEGREEN() -> Color {
        Color { r: 0.596078431372549, g: 0.984313725490196, b: 0.596078431372549, a: 1.0 }
    }

    /// PALETURQUOISE
    #[classattr]
    fn PALETURQUOISE() -> Color {
        Color { r: 0.6862745098039216, g: 0.9333333333333333, b: 0.9333333333333333, a: 1.0 }
    }

    /// PALEVIOLETRED
    #[classattr]
    fn PALEVIOLETRED() -> Color {
        Color { r: 0.8588235294117647, g: 0.4392156862745098, b: 0.5764705882352941, a: 1.0 }
    }

    /// PAPAYAWHIP
    #[classattr]
    fn PAPAYAWHIP() -> Color {
        Color { r: 1.0, g: 0.9372549019607843, b: 0.8352941176470589, a: 1.0 }
    }

    /// PEACHPUFF
    #[classattr]
    fn PEACHPUFF() -> Color {
        Color { r: 1.0, g: 0.8549019607843137, b: 0.7254901960784313, a: 1.0 }
    }

    /// PERU
    #[classattr]
    fn PERU() -> Color {
        Color { r: 0.803921568627451, g: 0.5215686274509804, b: 0.24705882352941178, a: 1.0 }
    }

    /// POWDERBLUE
    #[classattr]
    fn POWDERBLUE() -> Color {
        Color { r: 0.6901960784313725, g: 0.8784313725490196, b: 0.9019607843137255, a: 1.0 }
    }

    /// REBECCAPURPLE
    #[classattr]
    fn REBECCAPURPLE() -> Color {
        Color { r: 0.4, g: 0.2, b: 0.6, a: 1.0 }
    }

    /// ROSYBROWN
    #[classattr]
    fn ROSYBROWN() -> Color {
        Color { r: 0.7372549019607844, g: 0.5607843137254902, b: 0.5607843137254902, a: 1.0 }
    }

    /// ROYALBLUE
    #[classattr]
    fn ROYALBLUE() -> Color {
        Color { r: 0.2549019607843137, g: 0.4117647058823529, b: 0.8823529411764706, a: 1.0 }
    }

    /// SADDLEBROWN
    #[classattr]
    fn SADDLEBROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.27058823529411763, b: 0.07450980392156863, a: 1.0 }
    }

    /// SANDYBROWN
    #[classattr]
    fn SANDYBROWN() -> Color {
        Color { r: 0.9568627450980393, g: 0.6431372549019608, b: 0.3764705882352941, a: 1.0 }
    }

    /// SEAGREEN
    #[classattr]
    fn SEAGREEN() -> Color {
        Color { r: 0.1803921568627451, g: 0.5450980392156862, b: 0.3411764705882353, a: 1.0 }
    }

    /// SEASHELL
    #[classattr]
    fn SEASHELL() -> Color {
        Color { r: 1.0, g: 0.9607843137254902, b: 0.9333333333333333, a: 1.0 }
    }

    /// SKYBLUE
    #[classattr]
    fn SKYBLUE() -> Color {
        Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9215686274509803, a: 1.0 }
    }

    /// SLATEBLUE
    #[classattr]
    fn SLATEBLUE() -> Color {
        Color { r: 0.41568627450980394, g: 0.35294117647058826, b: 0.803921568627451, a: 1.0 }
    }

    /// SLATEGRAY
    #[classattr]
    fn SLATEGRAY() -> Color {
        Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 }
    }

    /// SLATEGREY
    #[classattr]
    fn SLATEGREY() -> Color {
        Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 }
    }

    /// SNOW
    #[classattr]
    fn SNOW() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.9803921568627451, a: 1.0 }
    }

    /// SPRINGGREEN
    #[classattr]
    fn SPRINGGREEN() -> Color {
        Color { r: 0.0, g: 1.0, b: 0.4980392156862745, a: 1.0 }
    }

    /// STEELBLUE
    #[classattr]
    fn STEELBLUE() -> Color {
        Color { r: 0.27450980392156865, g: 0.5098039215686274, b: 0.7058823529411765, a: 1.0 }
    }

    /// THISTLE
    #[classattr]
    fn THISTLE() -> Color {
        Color { r: 0.8470588235294118, g: 0.7490196078431373, b: 0.8470588235294118, a: 1.0 }
    }

    /// WHITESMOKE
    #[classattr]
    fn WHITESMOKE() -> Color {
        Color { r: 0.9607843137254902, g: 0.9607843137254902, b: 0.9607843137254902, a: 1.0 }
    }



}


impl From<macroquad::prelude::Color> for Color {
    fn from(t: macroquad::prelude::Color) -> Self {
        Color { r: t.r, g: t.g, b: t.b, a: t.a }
    }
}

impl From<Color> for macroquad::prelude::Color {
    fn from(t: Color) -> Self {
        macroquad::prelude::Color {  r: t.r, g: t.g, b: t.b, a: t.a  }
    }
}
