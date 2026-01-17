use pyo3::prelude::*;
 
use pyo3_stub_gen::derive::* ;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, PartialEq, Debug)]
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

    /// creates a new color.
    ///
    /// inputs range from:
    /// ```
    /// >>>Color(r=0.0, g=0.0, b=0.0, a=1.0) -> Color.BLACK()
    /// ...#to
    /// >>Color(r=1.0, g=1.0, b=1.0, a=1.0) -> Color.WHITE()
    /// ```
    /// r represents the red channel.
    /// 
    /// g represents the green channel.
    /// 
    /// b represents the blue channel.
    /// 
    /// a represents the alpha channel aka. transparency.
    #[pyo3(signature = (r= 1.0, g= 1.0, b= 1.0, a= 1.0))]
    #[new]
    pub fn new(r: f32, g: f32,b: f32,a: f32) -> Self {
       Self { r,g,b,a }
    }

    

    /// CLOUDY_BLUE
    #[classattr]
    pub fn CLOUDY_BLUE() -> Color {
        Color { r: 0.6745098039215687, g: 0.7607843137254902, b: 0.8509803921568627, a: 1.0 }
    }

    /// DARK_PASTEL_GREEN
    #[classattr]
    pub fn DARK_PASTEL_GREEN() -> Color {
        Color { r: 0.33725490196078434, g: 0.6823529411764706, b: 0.3411764705882353, a: 1.0 }
    }

    /// DUST
    #[classattr]
    pub fn DUST() -> Color {
        Color { r: 0.6980392156862745, g: 0.6, b: 0.43137254901960786, a: 1.0 }
    }

    /// ELECTRIC_LIME
    #[classattr]
    pub fn ELECTRIC_LIME() -> Color {
        Color { r: 0.6588235294117647, g: 1.0, b: 0.01568627450980392, a: 1.0 }
    }

    /// FRESH_GREEN
    #[classattr]
    pub fn FRESH_GREEN() -> Color {
        Color { r: 0.4117647058823529, g: 0.8470588235294118, b: 0.30980392156862746, a: 1.0 }
    }

    /// LIGHT_EGGPLANT
    #[classattr]
    pub fn LIGHT_EGGPLANT() -> Color {
        Color { r: 0.5372549019607843, g: 0.27058823529411763, b: 0.5215686274509804, a: 1.0 }
    }

    /// NASTY_GREEN
    #[classattr]
    pub fn NASTY_GREEN() -> Color {
        Color { r: 0.4392156862745098, g: 0.6980392156862745, b: 0.24705882352941178, a: 1.0 }
    }

    /// REALLY_LIGHT_BLUE
    #[classattr]
    pub fn REALLY_LIGHT_BLUE() -> Color {
        Color { r: 0.8313725490196079, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// TEA
    #[classattr]
    pub fn TEA() -> Color {
        Color { r: 0.396078431372549, g: 0.6705882352941176, b: 0.48627450980392156, a: 1.0 }
    }

    /// WARM_PURPLE
    #[classattr]
    pub fn WARM_PURPLE() -> Color {
        Color { r: 0.5843137254901961, g: 0.1803921568627451, b: 0.5607843137254902, a: 1.0 }
    }

    /// YELLOWISH_TAN
    #[classattr]
    pub fn YELLOWISH_TAN() -> Color {
        Color { r: 0.9882352941176471, g: 0.9882352941176471, b: 0.5058823529411764, a: 1.0 }
    }

    /// CEMENT
    #[classattr]
    pub fn CEMENT() -> Color {
        Color { r: 0.6470588235294118, g: 0.6392156862745098, b: 0.5686274509803921, a: 1.0 }
    }

    /// DARK_GRASS_GREEN
    #[classattr]
    pub fn DARK_GRASS_GREEN() -> Color {
        Color { r: 0.2196078431372549, g: 0.5019607843137255, b: 0.01568627450980392, a: 1.0 }
    }

    /// DUSTY_TEAL
    #[classattr]
    pub fn DUSTY_TEAL() -> Color {
        Color { r: 0.2980392156862745, g: 0.5647058823529412, b: 0.5215686274509804, a: 1.0 }
    }

    /// GREY_TEAL
    #[classattr]
    pub fn GREY_TEAL() -> Color {
        Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 }
    }

    /// MACARONI_AND_CHEESE
    #[classattr]
    pub fn MACARONI_AND_CHEESE() -> Color {
        Color { r: 0.9372549019607843, g: 0.7058823529411765, b: 0.20784313725490197, a: 1.0 }
    }

    /// PINKISH_TAN
    #[classattr]
    pub fn PINKISH_TAN() -> Color {
        Color { r: 0.8509803921568627, g: 0.6078431372549019, b: 0.5098039215686274, a: 1.0 }
    }

    /// SPRUCE
    #[classattr]
    pub fn SPRUCE() -> Color {
        Color { r: 0.0392156862745098, g: 0.37254901960784315, b: 0.2196078431372549, a: 1.0 }
    }

    /// STRONG_BLUE
    #[classattr]
    pub fn STRONG_BLUE() -> Color {
        Color { r: 0.047058823529411764, g: 0.023529411764705882, b: 0.9686274509803922, a: 1.0 }
    }

    /// TOXIC_GREEN
    #[classattr]
    pub fn TOXIC_GREEN() -> Color {
        Color { r: 0.3803921568627451, g: 0.8705882352941177, b: 0.16470588235294117, a: 1.0 }
    }

    /// WINDOWS_BLUE
    #[classattr]
    pub fn WINDOWS_BLUE() -> Color {
        Color { r: 0.21568627450980393, g: 0.47058823529411764, b: 0.7490196078431373, a: 1.0 }
    }

    /// BLUE_BLUE
    #[classattr]
    pub fn BLUE_BLUE() -> Color {
        Color { r: 0.13333333333333333, g: 0.25882352941176473, b: 0.7803921568627451, a: 1.0 }
    }

    /// BLUE_WITH_A_HINT_OF_PURPLE
    #[classattr]
    pub fn BLUE_WITH_A_HINT_OF_PURPLE() -> Color {
        Color { r: 0.3254901960784314, g: 0.23529411764705882, b: 0.7764705882352941, a: 1.0 }
    }

    /// BOOGER
    #[classattr]
    pub fn BOOGER() -> Color {
        Color { r: 0.6078431372549019, g: 0.7098039215686275, b: 0.23529411764705882, a: 1.0 }
    }

    /// BRIGHT_SEA_GREEN
    #[classattr]
    pub fn BRIGHT_SEA_GREEN() -> Color {
        Color { r: 0.0196078431372549, g: 1.0, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_GREEN_BLUE
    #[classattr]
    pub fn DARK_GREEN_BLUE() -> Color {
        Color { r: 0.12156862745098039, g: 0.38823529411764707, b: 0.3411764705882353, a: 1.0 }
    }

    /// DEEP_TURQUOISE
    #[classattr]
    pub fn DEEP_TURQUOISE() -> Color {
        Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.4549019607843137, a: 1.0 }
    }

    /// GREEN_TEAL
    #[classattr]
    pub fn GREEN_TEAL() -> Color {
        Color { r: 0.047058823529411764, g: 0.7098039215686275, b: 0.4666666666666667, a: 1.0 }
    }

    /// STRONG_PINK
    #[classattr]
    pub fn STRONG_PINK() -> Color {
        Color { r: 1.0, g: 0.027450980392156862, b: 0.5372549019607843, a: 1.0 }
    }

    /// BLAND
    #[classattr]
    pub fn BLAND() -> Color {
        Color { r: 0.6862745098039216, g: 0.6588235294117647, b: 0.5450980392156862, a: 1.0 }
    }

    /// DEEP_AQUA
    #[classattr]
    pub fn DEEP_AQUA() -> Color {
        Color { r: 0.03137254901960784, g: 0.47058823529411764, b: 0.4980392156862745, a: 1.0 }
    }

    /// LAVENDER_PINK
    #[classattr]
    pub fn LAVENDER_PINK() -> Color {
        Color { r: 0.8666666666666667, g: 0.5215686274509804, b: 0.8431372549019608, a: 1.0 }
    }

    /// LIGHT_MOSS_GREEN
    #[classattr]
    pub fn LIGHT_MOSS_GREEN() -> Color {
        Color { r: 0.6509803921568628, g: 0.7843137254901961, b: 0.4588235294117647, a: 1.0 }
    }

    /// LIGHT_SEAFOAM_GREEN
    #[classattr]
    pub fn LIGHT_SEAFOAM_GREEN() -> Color {
        Color { r: 0.6549019607843137, g: 1.0, b: 0.7098039215686275, a: 1.0 }
    }

    /// OLIVE_YELLOW
    #[classattr]
    pub fn OLIVE_YELLOW() -> Color {
        Color { r: 0.7607843137254902, g: 0.7176470588235294, b: 0.03529411764705882, a: 1.0 }
    }

    /// PIG_PINK
    #[classattr]
    pub fn PIG_PINK() -> Color {
        Color { r: 0.9058823529411765, g: 0.5568627450980392, b: 0.6470588235294118, a: 1.0 }
    }

    /// DEEP_LILAC
    #[classattr]
    pub fn DEEP_LILAC() -> Color {
        Color { r: 0.5882352941176471, g: 0.43137254901960786, b: 0.7411764705882353, a: 1.0 }
    }

    /// DESERT
    #[classattr]
    pub fn DESERT() -> Color {
        Color { r: 0.8, g: 0.6784313725490196, b: 0.3764705882352941, a: 1.0 }
    }

    /// DUSTY_LAVENDER
    #[classattr]
    pub fn DUSTY_LAVENDER() -> Color {
        Color { r: 0.6745098039215687, g: 0.5254901960784314, b: 0.6588235294117647, a: 1.0 }
    }

    /// PURPLEY_GREY
    #[classattr]
    pub fn PURPLEY_GREY() -> Color {
        Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 }
    }

    /// PURPLY
    #[classattr]
    pub fn PURPLY() -> Color {
        Color { r: 0.596078431372549, g: 0.24705882352941178, b: 0.6980392156862745, a: 1.0 }
    }

    /// CANDY_PINK
    #[classattr]
    pub fn CANDY_PINK() -> Color {
        Color { r: 1.0, g: 0.38823529411764707, b: 0.9137254901960784, a: 1.0 }
    }

    /// LIGHT_PASTEL_GREEN
    #[classattr]
    pub fn LIGHT_PASTEL_GREEN() -> Color {
        Color { r: 0.6980392156862745, g: 0.984313725490196, b: 0.6470588235294118, a: 1.0 }
    }

    /// BORING_GREEN
    #[classattr]
    pub fn BORING_GREEN() -> Color {
        Color { r: 0.38823529411764707, g: 0.7019607843137254, b: 0.396078431372549, a: 1.0 }
    }

    /// KIWI_GREEN
    #[classattr]
    pub fn KIWI_GREEN() -> Color {
        Color { r: 0.5568627450980392, g: 0.8980392156862745, b: 0.24705882352941178, a: 1.0 }
    }

    /// LIGHT_GREY_GREEN
    #[classattr]
    pub fn LIGHT_GREY_GREEN() -> Color {
        Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 }
    }

    /// ORANGE_PINK
    #[classattr]
    pub fn ORANGE_PINK() -> Color {
        Color { r: 1.0, g: 0.43529411764705883, b: 0.3215686274509804, a: 1.0 }
    }

    /// TEA_GREEN
    #[classattr]
    pub fn TEA_GREEN() -> Color {
        Color { r: 0.7411764705882353, g: 0.9725490196078431, b: 0.6392156862745098, a: 1.0 }
    }

    /// VERY_LIGHT_BROWN
    #[classattr]
    pub fn VERY_LIGHT_BROWN() -> Color {
        Color { r: 0.8274509803921568, g: 0.7137254901960784, b: 0.5137254901960784, a: 1.0 }
    }

    /// EGG_SHELL
    #[classattr]
    pub fn EGG_SHELL() -> Color {
        Color { r: 1.0, g: 0.9882352941176471, b: 0.7686274509803922, a: 1.0 }
    }

    /// EGGPLANT_PURPLE
    #[classattr]
    pub fn EGGPLANT_PURPLE() -> Color {
        Color { r: 0.2627450980392157, g: 0.0196078431372549, b: 0.2549019607843137, a: 1.0 }
    }

    /// POWDER_PINK
    #[classattr]
    pub fn POWDER_PINK() -> Color {
        Color { r: 1.0, g: 0.6980392156862745, b: 0.8156862745098039, a: 1.0 }
    }

    /// REDDISH_GREY
    #[classattr]
    pub fn REDDISH_GREY() -> Color {
        Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 }
    }

    /// BABY_SHIT_BROWN
    #[classattr]
    pub fn BABY_SHIT_BROWN() -> Color {
        Color { r: 0.6784313725490196, g: 0.5647058823529412, b: 0.050980392156862744, a: 1.0 }
    }

    /// LILIAC
    #[classattr]
    pub fn LILIAC() -> Color {
        Color { r: 0.7686274509803922, g: 0.5568627450980392, b: 0.9921568627450981, a: 1.0 }
    }

    /// STORMY_BLUE
    #[classattr]
    pub fn STORMY_BLUE() -> Color {
        Color { r: 0.3137254901960784, g: 0.4823529411764706, b: 0.611764705882353, a: 1.0 }
    }

    /// UGLY_BROWN
    #[classattr]
    pub fn UGLY_BROWN() -> Color {
        Color { r: 0.49019607843137253, g: 0.44313725490196076, b: 0.011764705882352941, a: 1.0 }
    }

    /// CUSTARD
    #[classattr]
    pub fn CUSTARD() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.47058823529411764, a: 1.0 }
    }

    /// DARKISH_PINK
    #[classattr]
    pub fn DARKISH_PINK() -> Color {
        Color { r: 0.8549019607843137, g: 0.27450980392156865, b: 0.49019607843137253, a: 1.0 }
    }

    /// DEEP_BROWN
    #[classattr]
    pub fn DEEP_BROWN() -> Color {
        Color { r: 0.2549019607843137, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// GREENISH_BEIGE
    #[classattr]
    pub fn GREENISH_BEIGE() -> Color {
        Color { r: 0.788235294117647, g: 0.8196078431372549, b: 0.4745098039215686, a: 1.0 }
    }

    /// MANILLA
    #[classattr]
    pub fn MANILLA() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.5254901960784314, a: 1.0 }
    }

    /// OFF_BLUE
    #[classattr]
    pub fn OFF_BLUE() -> Color {
        Color { r: 0.33725490196078434, g: 0.5176470588235295, b: 0.6823529411764706, a: 1.0 }
    }

    /// BATTLESHIP_GREY
    #[classattr]
    pub fn BATTLESHIP_GREY() -> Color {
        Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 }
    }

    /// BROWNY_GREEN
    #[classattr]
    pub fn BROWNY_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.4235294117647059, b: 0.0392156862745098, a: 1.0 }
    }

    /// BRUISE
    #[classattr]
    pub fn BRUISE() -> Color {
        Color { r: 0.49411764705882355, g: 0.25098039215686274, b: 0.44313725490196076, a: 1.0 }
    }

    /// KELLEY_GREEN
    #[classattr]
    pub fn KELLEY_GREEN() -> Color {
        Color { r: 0.0, g: 0.5764705882352941, b: 0.21568627450980393, a: 1.0 }
    }

    /// SICKLY_YELLOW
    #[classattr]
    pub fn SICKLY_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.8941176470588236, b: 0.1607843137254902, a: 1.0 }
    }

    /// SUNNY_YELLOW
    #[classattr]
    pub fn SUNNY_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9764705882352941, b: 0.09019607843137255, a: 1.0 }
    }

    /// AZUL
    #[classattr]
    pub fn AZUL() -> Color {
        Color { r: 0.11372549019607843, g: 0.36470588235294116, b: 0.9254901960784314, a: 1.0 }
    }

    /// DARKGREEN
    #[classattr]
    pub fn DARKGREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.28627450980392155, b: 0.027450980392156862, a: 1.0 }
    }

    /// GREEN_YELLOW
    #[classattr]
    pub fn GREEN_YELLOW() -> Color {
        Color { r: 0.7098039215686275, g: 0.807843137254902, b: 0.03137254901960784, a: 1.0 }
    }

    /// LICHEN
    #[classattr]
    pub fn LICHEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.7137254901960784, b: 0.4823529411764706, a: 1.0 }
    }

    /// LIGHT_LIGHT_GREEN
    #[classattr]
    pub fn LIGHT_LIGHT_GREEN() -> Color {
        Color { r: 0.7843137254901961, g: 1.0, b: 0.6901960784313725, a: 1.0 }
    }

    /// PALE_GOLD
    #[classattr]
    pub fn PALE_GOLD() -> Color {
        Color { r: 0.9921568627450981, g: 0.8705882352941177, b: 0.4235294117647059, a: 1.0 }
    }

    /// SUN_YELLOW
    #[classattr]
    pub fn SUN_YELLOW() -> Color {
        Color { r: 1.0, g: 0.8745098039215686, b: 0.13333333333333333, a: 1.0 }
    }

    /// TAN_GREEN
    #[classattr]
    pub fn TAN_GREEN() -> Color {
        Color { r: 0.6627450980392157, g: 0.7450980392156863, b: 0.4392156862745098, a: 1.0 }
    }

    /// BURPLE
    #[classattr]
    pub fn BURPLE() -> Color {
        Color { r: 0.40784313725490196, g: 0.19607843137254902, b: 0.8901960784313725, a: 1.0 }
    }

    /// BUTTERSCOTCH
    #[classattr]
    pub fn BUTTERSCOTCH() -> Color {
        Color { r: 0.9921568627450981, g: 0.6941176470588235, b: 0.2784313725490196, a: 1.0 }
    }

    /// TOUPE
    #[classattr]
    pub fn TOUPE() -> Color {
        Color { r: 0.7803921568627451, g: 0.6745098039215687, b: 0.49019607843137253, a: 1.0 }
    }

    /// DARK_CREAM
    #[classattr]
    pub fn DARK_CREAM() -> Color {
        Color { r: 1.0, g: 0.9529411764705882, b: 0.6039215686274509, a: 1.0 }
    }

    /// INDIAN_RED
    #[classattr]
    pub fn INDIAN_RED() -> Color {
        Color { r: 0.5215686274509804, g: 0.054901960784313725, b: 0.01568627450980392, a: 1.0 }
    }

    /// LIGHT_LAVENDAR
    #[classattr]
    pub fn LIGHT_LAVENDAR() -> Color {
        Color { r: 0.9372549019607843, g: 0.7529411764705882, b: 0.996078431372549, a: 1.0 }
    }

    /// POISON_GREEN
    #[classattr]
    pub fn POISON_GREEN() -> Color {
        Color { r: 0.25098039215686274, g: 0.9921568627450981, b: 0.0784313725490196, a: 1.0 }
    }

    /// BABY_PUKE_GREEN
    #[classattr]
    pub fn BABY_PUKE_GREEN() -> Color {
        Color { r: 0.7137254901960784, g: 0.7686274509803922, b: 0.023529411764705882, a: 1.0 }
    }

    /// BRIGHT_YELLOW_GREEN
    #[classattr]
    pub fn BRIGHT_YELLOW_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 1.0, b: 0.0, a: 1.0 }
    }

    /// CHARCOAL_GREY
    #[classattr]
    pub fn CHARCOAL_GREY() -> Color {
        Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 }
    }

    /// SQUASH
    #[classattr]
    pub fn SQUASH() -> Color {
        Color { r: 0.9490196078431372, g: 0.6705882352941176, b: 0.08235294117647059, a: 1.0 }
    }

    /// CINNAMON
    #[classattr]
    pub fn CINNAMON() -> Color {
        Color { r: 0.6745098039215687, g: 0.30980392156862746, b: 0.023529411764705882, a: 1.0 }
    }

    /// LIGHT_PEA_GREEN
    #[classattr]
    pub fn LIGHT_PEA_GREEN() -> Color {
        Color { r: 0.7686274509803922, g: 0.996078431372549, b: 0.5098039215686274, a: 1.0 }
    }

    /// RADIOACTIVE_GREEN
    #[classattr]
    pub fn RADIOACTIVE_GREEN() -> Color {
        Color { r: 0.17254901960784313, g: 0.9803921568627451, b: 0.12156862745098039, a: 1.0 }
    }

    /// RAW_SIENNA
    #[classattr]
    pub fn RAW_SIENNA() -> Color {
        Color { r: 0.6039215686274509, g: 0.3843137254901961, b: 0.0, a: 1.0 }
    }

    /// BABY_PURPLE
    #[classattr]
    pub fn BABY_PURPLE() -> Color {
        Color { r: 0.792156862745098, g: 0.6078431372549019, b: 0.9686274509803922, a: 1.0 }
    }

    /// COCOA
    #[classattr]
    pub fn COCOA() -> Color {
        Color { r: 0.5294117647058824, g: 0.37254901960784315, b: 0.25882352941176473, a: 1.0 }
    }

    /// LIGHT_ROYAL_BLUE
    #[classattr]
    pub fn LIGHT_ROYAL_BLUE() -> Color {
        Color { r: 0.22745098039215686, g: 0.1803921568627451, b: 0.996078431372549, a: 1.0 }
    }

    /// ORANGEISH
    #[classattr]
    pub fn ORANGEISH() -> Color {
        Color { r: 0.9921568627450981, g: 0.5529411764705883, b: 0.28627450980392155, a: 1.0 }
    }

    /// RUST_BROWN
    #[classattr]
    pub fn RUST_BROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.19215686274509805, b: 0.011764705882352941, a: 1.0 }
    }

    /// SAND_BROWN
    #[classattr]
    pub fn SAND_BROWN() -> Color {
        Color { r: 0.796078431372549, g: 0.6470588235294118, b: 0.3764705882352941, a: 1.0 }
    }

    /// SWAMP
    #[classattr]
    pub fn SWAMP() -> Color {
        Color { r: 0.4117647058823529, g: 0.5137254901960784, b: 0.2235294117647059, a: 1.0 }
    }

    /// TEALISH_GREEN
    #[classattr]
    pub fn TEALISH_GREEN() -> Color {
        Color { r: 0.047058823529411764, g: 0.8627450980392157, b: 0.45098039215686275, a: 1.0 }
    }

    /// BURNT_SIENA
    #[classattr]
    pub fn BURNT_SIENA() -> Color {
        Color { r: 0.7176470588235294, g: 0.3215686274509804, b: 0.011764705882352941, a: 1.0 }
    }

    /// CAMO
    #[classattr]
    pub fn CAMO() -> Color {
        Color { r: 0.4980392156862745, g: 0.5607843137254902, b: 0.3058823529411765, a: 1.0 }
    }

    /// DUSK_BLUE
    #[classattr]
    pub fn DUSK_BLUE() -> Color {
        Color { r: 0.14901960784313725, g: 0.3254901960784314, b: 0.5529411764705883, a: 1.0 }
    }

    /// FERN
    #[classattr]
    pub fn FERN() -> Color {
        Color { r: 0.38823529411764707, g: 0.6627450980392157, b: 0.3137254901960784, a: 1.0 }
    }

    /// OLD_ROSE
    #[classattr]
    pub fn OLD_ROSE() -> Color {
        Color { r: 0.7843137254901961, g: 0.4980392156862745, b: 0.5372549019607843, a: 1.0 }
    }

    /// PALE_LIGHT_GREEN
    #[classattr]
    pub fn PALE_LIGHT_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 0.9882352941176471, b: 0.6, a: 1.0 }
    }

    /// PEACHY_PINK
    #[classattr]
    pub fn PEACHY_PINK() -> Color {
        Color { r: 1.0, g: 0.6039215686274509, b: 0.5411764705882353, a: 1.0 }
    }

    /// ROSY_PINK
    #[classattr]
    pub fn ROSY_PINK() -> Color {
        Color { r: 0.9647058823529412, g: 0.40784313725490196, b: 0.5568627450980392, a: 1.0 }
    }

    /// LIGHT_BLUISH_GREEN
    #[classattr]
    pub fn LIGHT_BLUISH_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.9921568627450981, b: 0.6588235294117647, a: 1.0 }
    }

    /// LIGHT_BRIGHT_GREEN
    #[classattr]
    pub fn LIGHT_BRIGHT_GREEN() -> Color {
        Color { r: 0.3254901960784314, g: 0.996078431372549, b: 0.3607843137254902, a: 1.0 }
    }

    /// LIGHT_NEON_GREEN
    #[classattr]
    pub fn LIGHT_NEON_GREEN() -> Color {
        Color { r: 0.3058823529411765, g: 0.9921568627450981, b: 0.32941176470588235, a: 1.0 }
    }

    /// LIGHT_SEAFOAM
    #[classattr]
    pub fn LIGHT_SEAFOAM() -> Color {
        Color { r: 0.6274509803921569, g: 0.996078431372549, b: 0.7490196078431373, a: 1.0 }
    }

    /// TIFFANY_BLUE
    #[classattr]
    pub fn TIFFANY_BLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.9490196078431372, b: 0.8549019607843137, a: 1.0 }
    }

    /// WASHED_OUT_GREEN
    #[classattr]
    pub fn WASHED_OUT_GREEN() -> Color {
        Color { r: 0.7372549019607844, g: 0.9607843137254902, b: 0.6509803921568628, a: 1.0 }
    }

    /// BROWNY_ORANGE
    #[classattr]
    pub fn BROWNY_ORANGE() -> Color {
        Color { r: 0.792156862745098, g: 0.4196078431372549, b: 0.00784313725490196, a: 1.0 }
    }

    /// NICE_BLUE
    #[classattr]
    pub fn NICE_BLUE() -> Color {
        Color { r: 0.06274509803921569, g: 0.47843137254901963, b: 0.6901960784313725, a: 1.0 }
    }

    /// SAPPHIRE
    #[classattr]
    pub fn SAPPHIRE() -> Color {
        Color { r: 0.12941176470588237, g: 0.2196078431372549, b: 0.6705882352941176, a: 1.0 }
    }

    /// GREYISH_TEAL
    #[classattr]
    pub fn GREYISH_TEAL() -> Color {
        Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 }
    }

    /// ORANGEY_YELLOW
    #[classattr]
    pub fn ORANGEY_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 0.7254901960784313, b: 0.08235294117647059, a: 1.0 }
    }

    /// PARCHMENT
    #[classattr]
    pub fn PARCHMENT() -> Color {
        Color { r: 0.996078431372549, g: 0.9882352941176471, b: 0.6862745098039216, a: 1.0 }
    }

    /// STRAW
    #[classattr]
    pub fn STRAW() -> Color {
        Color { r: 0.9882352941176471, g: 0.9647058823529412, b: 0.4745098039215686, a: 1.0 }
    }

    /// VERY_DARK_BROWN
    #[classattr]
    pub fn VERY_DARK_BROWN() -> Color {
        Color { r: 0.11372549019607843, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// TERRACOTA
    #[classattr]
    pub fn TERRACOTA() -> Color {
        Color { r: 0.796078431372549, g: 0.40784313725490196, b: 0.2627450980392157, a: 1.0 }
    }

    /// UGLY_BLUE
    #[classattr]
    pub fn UGLY_BLUE() -> Color {
        Color { r: 0.19215686274509805, g: 0.4, b: 0.5411764705882353, a: 1.0 }
    }

    /// CLEAR_BLUE
    #[classattr]
    pub fn CLEAR_BLUE() -> Color {
        Color { r: 0.1411764705882353, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 }
    }

    /// CREME
    #[classattr]
    pub fn CREME() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.7137254901960784, a: 1.0 }
    }

    /// FOAM_GREEN
    #[classattr]
    pub fn FOAM_GREEN() -> Color {
        Color { r: 0.5647058823529412, g: 0.9921568627450981, b: 0.6627450980392157, a: 1.0 }
    }

    /// GREY_GREEN
    #[classattr]
    pub fn GREY_GREEN() -> Color {
        Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 }
    }

    /// LIGHT_GOLD
    #[classattr]
    pub fn LIGHT_GOLD() -> Color {
        Color { r: 0.9921568627450981, g: 0.8627450980392157, b: 0.3607843137254902, a: 1.0 }
    }

    /// SEAFOAM_BLUE
    #[classattr]
    pub fn SEAFOAM_BLUE() -> Color {
        Color { r: 0.47058823529411764, g: 0.8196078431372549, b: 0.7137254901960784, a: 1.0 }
    }

    /// TOPAZ
    #[classattr]
    pub fn TOPAZ() -> Color {
        Color { r: 0.07450980392156863, g: 0.7333333333333333, b: 0.6862745098039216, a: 1.0 }
    }

    /// VIOLET_PINK
    #[classattr]
    pub fn VIOLET_PINK() -> Color {
        Color { r: 0.984313725490196, g: 0.37254901960784315, b: 0.9882352941176471, a: 1.0 }
    }

    /// WINTERGREEN
    #[classattr]
    pub fn WINTERGREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.9764705882352941, b: 0.5254901960784314, a: 1.0 }
    }

    /// YELLOW_TAN
    #[classattr]
    pub fn YELLOW_TAN() -> Color {
        Color { r: 1.0, g: 0.8901960784313725, b: 0.43137254901960786, a: 1.0 }
    }

    /// DARK_FUCHSIA
    #[classattr]
    pub fn DARK_FUCHSIA() -> Color {
        Color { r: 0.615686274509804, g: 0.027450980392156862, b: 0.34901960784313724, a: 1.0 }
    }

    /// INDIGO_BLUE
    #[classattr]
    pub fn INDIGO_BLUE() -> Color {
        Color { r: 0.22745098039215686, g: 0.09411764705882353, b: 0.6941176470588235, a: 1.0 }
    }

    /// LIGHT_YELLOWISH_GREEN
    #[classattr]
    pub fn LIGHT_YELLOWISH_GREEN() -> Color {
        Color { r: 0.7607843137254902, g: 1.0, b: 0.5372549019607843, a: 1.0 }
    }

    /// PALE_MAGENTA
    #[classattr]
    pub fn PALE_MAGENTA() -> Color {
        Color { r: 0.8431372549019608, g: 0.403921568627451, b: 0.6784313725490196, a: 1.0 }
    }

    /// RICH_PURPLE
    #[classattr]
    pub fn RICH_PURPLE() -> Color {
        Color { r: 0.4470588235294118, g: 0.0, b: 0.34509803921568627, a: 1.0 }
    }

    /// SUNFLOWER_YELLOW
    #[classattr]
    pub fn SUNFLOWER_YELLOW() -> Color {
        Color { r: 1.0, g: 0.8549019607843137, b: 0.011764705882352941, a: 1.0 }
    }

    /// GREEN_BLUE
    #[classattr]
    pub fn GREEN_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.7529411764705882, b: 0.5529411764705883, a: 1.0 }
    }

    /// LEATHER
    #[classattr]
    pub fn LEATHER() -> Color {
        Color { r: 0.6745098039215687, g: 0.4549019607843137, b: 0.20392156862745098, a: 1.0 }
    }

    /// RACING_GREEN
    #[classattr]
    pub fn RACING_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.27450980392156865, b: 0.0, a: 1.0 }
    }

    /// VIVID_PURPLE
    #[classattr]
    pub fn VIVID_PURPLE() -> Color {
        Color { r: 0.6, g: 0.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// DARK_ROYAL_BLUE
    #[classattr]
    pub fn DARK_ROYAL_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.023529411764705882, b: 0.43529411764705883, a: 1.0 }
    }

    /// HAZEL
    #[classattr]
    pub fn HAZEL() -> Color {
        Color { r: 0.5568627450980392, g: 0.4627450980392157, b: 0.09411764705882353, a: 1.0 }
    }

    /// MUTED_PINK
    #[classattr]
    pub fn MUTED_PINK() -> Color {
        Color { r: 0.8196078431372549, g: 0.4627450980392157, b: 0.5607843137254902, a: 1.0 }
    }

    /// BOOGER_GREEN
    #[classattr]
    pub fn BOOGER_GREEN() -> Color {
        Color { r: 0.5882352941176471, g: 0.7058823529411765, b: 0.011764705882352941, a: 1.0 }
    }

    /// CANARY
    #[classattr]
    pub fn CANARY() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.38823529411764707, a: 1.0 }
    }

    /// COOL_GREY
    #[classattr]
    pub fn COOL_GREY() -> Color {
        Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_TAUPE
    #[classattr]
    pub fn DARK_TAUPE() -> Color {
        Color { r: 0.4980392156862745, g: 0.40784313725490196, b: 0.3058823529411765, a: 1.0 }
    }

    /// DARKISH_PURPLE
    #[classattr]
    pub fn DARKISH_PURPLE() -> Color {
        Color { r: 0.4588235294117647, g: 0.09803921568627451, b: 0.45098039215686275, a: 1.0 }
    }

    /// TRUE_GREEN
    #[classattr]
    pub fn TRUE_GREEN() -> Color {
        Color { r: 0.03137254901960784, g: 0.5803921568627451, b: 0.01568627450980392, a: 1.0 }
    }

    /// CORAL_PINK
    #[classattr]
    pub fn CORAL_PINK() -> Color {
        Color { r: 1.0, g: 0.3803921568627451, b: 0.38823529411764707, a: 1.0 }
    }

    /// DARK_SAGE
    #[classattr]
    pub fn DARK_SAGE() -> Color {
        Color { r: 0.34901960784313724, g: 0.5215686274509804, b: 0.33725490196078434, a: 1.0 }
    }

    /// DARK_SLATE_BLUE
    #[classattr]
    pub fn DARK_SLATE_BLUE() -> Color {
        Color { r: 0.12941176470588237, g: 0.2784313725490196, b: 0.3803921568627451, a: 1.0 }
    }

    /// FLAT_BLUE
    #[classattr]
    pub fn FLAT_BLUE() -> Color {
        Color { r: 0.23529411764705882, g: 0.45098039215686275, b: 0.6588235294117647, a: 1.0 }
    }

    /// MUSHROOM
    #[classattr]
    pub fn MUSHROOM() -> Color {
        Color { r: 0.7294117647058823, g: 0.6196078431372549, b: 0.5333333333333333, a: 1.0 }
    }

    /// RICH_BLUE
    #[classattr]
    pub fn RICH_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.10588235294117647, b: 0.9764705882352941, a: 1.0 }
    }

    /// DIRTY_PURPLE
    #[classattr]
    pub fn DIRTY_PURPLE() -> Color {
        Color { r: 0.45098039215686275, g: 0.2901960784313726, b: 0.396078431372549, a: 1.0 }
    }

    /// GREENBLUE
    #[classattr]
    pub fn GREENBLUE() -> Color {
        Color { r: 0.13725490196078433, g: 0.7686274509803922, b: 0.5450980392156862, a: 1.0 }
    }

    /// ICKY_GREEN
    #[classattr]
    pub fn ICKY_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.6823529411764706, b: 0.13333333333333333, a: 1.0 }
    }

    /// LIGHT_KHAKI
    #[classattr]
    pub fn LIGHT_KHAKI() -> Color {
        Color { r: 0.9019607843137255, g: 0.9490196078431372, b: 0.6352941176470588, a: 1.0 }
    }

    /// WARM_BLUE
    #[classattr]
    pub fn WARM_BLUE() -> Color {
        Color { r: 0.29411764705882354, g: 0.3411764705882353, b: 0.8588235294117647, a: 1.0 }
    }

    /// DARK_HOT_PINK
    #[classattr]
    pub fn DARK_HOT_PINK() -> Color {
        Color { r: 0.8509803921568627, g: 0.00392156862745098, b: 0.4, a: 1.0 }
    }

    /// DEEP_SEA_BLUE
    #[classattr]
    pub fn DEEP_SEA_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.32941176470588235, b: 0.5098039215686274, a: 1.0 }
    }

    /// CARMINE
    #[classattr]
    pub fn CARMINE() -> Color {
        Color { r: 0.615686274509804, g: 0.00784313725490196, b: 0.08627450980392157, a: 1.0 }
    }

    /// DARK_YELLOW_GREEN
    #[classattr]
    pub fn DARK_YELLOW_GREEN() -> Color {
        Color { r: 0.4470588235294118, g: 0.5607843137254902, b: 0.00784313725490196, a: 1.0 }
    }

    /// PALE_PEACH
    #[classattr]
    pub fn PALE_PEACH() -> Color {
        Color { r: 1.0, g: 0.8980392156862745, b: 0.6784313725490196, a: 1.0 }
    }

    /// PLUM_PURPLE
    #[classattr]
    pub fn PLUM_PURPLE() -> Color {
        Color { r: 0.3058823529411765, g: 0.0196078431372549, b: 0.3137254901960784, a: 1.0 }
    }

    /// GOLDEN_ROD
    #[classattr]
    pub fn GOLDEN_ROD() -> Color {
        Color { r: 0.9764705882352941, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 }
    }

    /// NEON_RED
    #[classattr]
    pub fn NEON_RED() -> Color {
        Color { r: 1.0, g: 0.027450980392156862, b: 0.22745098039215686, a: 1.0 }
    }

    /// OLD_PINK
    #[classattr]
    pub fn OLD_PINK() -> Color {
        Color { r: 0.7803921568627451, g: 0.4745098039215686, b: 0.5254901960784314, a: 1.0 }
    }

    /// VERY_PALE_BLUE
    #[classattr]
    pub fn VERY_PALE_BLUE() -> Color {
        Color { r: 0.8392156862745098, g: 1.0, b: 0.996078431372549, a: 1.0 }
    }

    /// BLOOD_ORANGE
    #[classattr]
    pub fn BLOOD_ORANGE() -> Color {
        Color { r: 0.996078431372549, g: 0.29411764705882354, b: 0.011764705882352941, a: 1.0 }
    }

    /// GRAPEFRUIT
    #[classattr]
    pub fn GRAPEFRUIT() -> Color {
        Color { r: 0.9921568627450981, g: 0.34901960784313724, b: 0.33725490196078434, a: 1.0 }
    }

    /// SAND_YELLOW
    #[classattr]
    pub fn SAND_YELLOW() -> Color {
        Color { r: 0.9882352941176471, g: 0.8823529411764706, b: 0.4, a: 1.0 }
    }

    /// CLAY_BROWN
    #[classattr]
    pub fn CLAY_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.44313725490196076, b: 0.23921568627450981, a: 1.0 }
    }

    /// DARK_BLUE_GREY
    #[classattr]
    pub fn DARK_BLUE_GREY() -> Color {
        Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 }
    }

    /// FLAT_GREEN
    #[classattr]
    pub fn FLAT_GREEN() -> Color {
        Color { r: 0.4117647058823529, g: 0.615686274509804, b: 0.2980392156862745, a: 1.0 }
    }

    /// LIGHT_GREEN_BLUE
    #[classattr]
    pub fn LIGHT_GREEN_BLUE() -> Color {
        Color { r: 0.33725490196078434, g: 0.9882352941176471, b: 0.6352941176470588, a: 1.0 }
    }

    /// WARM_PINK
    #[classattr]
    pub fn WARM_PINK() -> Color {
        Color { r: 0.984313725490196, g: 0.3333333333333333, b: 0.5058823529411764, a: 1.0 }
    }

    /// DODGER_BLUE
    #[classattr]
    pub fn DODGER_BLUE() -> Color {
        Color { r: 0.24313725490196078, g: 0.5098039215686274, b: 0.9882352941176471, a: 1.0 }
    }

    /// GROSS_GREEN
    #[classattr]
    pub fn GROSS_GREEN() -> Color {
        Color { r: 0.6274509803921569, g: 0.7490196078431373, b: 0.08627450980392157, a: 1.0 }
    }

    /// ICE
    #[classattr]
    pub fn ICE() -> Color {
        Color { r: 0.8392156862745098, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// METALLIC_BLUE
    #[classattr]
    pub fn METALLIC_BLUE() -> Color {
        Color { r: 0.30980392156862746, g: 0.45098039215686275, b: 0.5568627450980392, a: 1.0 }
    }

    /// PALE_SALMON
    #[classattr]
    pub fn PALE_SALMON() -> Color {
        Color { r: 1.0, g: 0.6941176470588235, b: 0.6039215686274509, a: 1.0 }
    }

    /// SAP_GREEN
    #[classattr]
    pub fn SAP_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.5450980392156862, b: 0.08235294117647059, a: 1.0 }
    }

    /// ALGAE
    #[classattr]
    pub fn ALGAE() -> Color {
        Color { r: 0.32941176470588235, g: 0.6745098039215687, b: 0.40784313725490196, a: 1.0 }
    }

    /// BLUEY_GREY
    #[classattr]
    pub fn BLUEY_GREY() -> Color {
        Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 }
    }

    /// GREENY_GREY
    #[classattr]
    pub fn GREENY_GREY() -> Color {
        Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// HIGHLIGHTER_GREEN
    #[classattr]
    pub fn HIGHLIGHTER_GREEN() -> Color {
        Color { r: 0.10588235294117647, g: 0.9882352941176471, b: 0.023529411764705882, a: 1.0 }
    }

    /// LIGHT_LIGHT_BLUE
    #[classattr]
    pub fn LIGHT_LIGHT_BLUE() -> Color {
        Color { r: 0.792156862745098, g: 1.0, b: 0.984313725490196, a: 1.0 }
    }

    /// LIGHT_MINT
    #[classattr]
    pub fn LIGHT_MINT() -> Color {
        Color { r: 0.7137254901960784, g: 1.0, b: 0.7333333333333333, a: 1.0 }
    }

    /// RAW_UMBER
    #[classattr]
    pub fn RAW_UMBER() -> Color {
        Color { r: 0.6549019607843137, g: 0.3686274509803922, b: 0.03529411764705882, a: 1.0 }
    }

    /// VIVID_BLUE
    #[classattr]
    pub fn VIVID_BLUE() -> Color {
        Color { r: 0.08235294117647059, g: 0.1803921568627451, b: 1.0, a: 1.0 }
    }

    /// DEEP_LAVENDER
    #[classattr]
    pub fn DEEP_LAVENDER() -> Color {
        Color { r: 0.5529411764705883, g: 0.3686274509803922, b: 0.7176470588235294, a: 1.0 }
    }

    /// DULL_TEAL
    #[classattr]
    pub fn DULL_TEAL() -> Color {
        Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.5607843137254902, a: 1.0 }
    }

    /// LIGHT_GREENISH_BLUE
    #[classattr]
    pub fn LIGHT_GREENISH_BLUE() -> Color {
        Color { r: 0.38823529411764707, g: 0.9686274509803922, b: 0.7058823529411765, a: 1.0 }
    }

    /// MUD_GREEN
    #[classattr]
    pub fn MUD_GREEN() -> Color {
        Color { r: 0.3764705882352941, g: 0.4, b: 0.00784313725490196, a: 1.0 }
    }

    /// PINKY
    #[classattr]
    pub fn PINKY() -> Color {
        Color { r: 0.9882352941176471, g: 0.5254901960784314, b: 0.6666666666666666, a: 1.0 }
    }

    /// RED_WINE
    #[classattr]
    pub fn RED_WINE() -> Color {
        Color { r: 0.5490196078431373, g: 0.0, b: 0.20392156862745098, a: 1.0 }
    }

    /// SHIT_GREEN
    #[classattr]
    pub fn SHIT_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.5019607843137255, b: 0.0, a: 1.0 }
    }

    /// TAN_BROWN
    #[classattr]
    pub fn TAN_BROWN() -> Color {
        Color { r: 0.6705882352941176, g: 0.49411764705882355, b: 0.2980392156862745, a: 1.0 }
    }

    /// DARKBLUE
    #[classattr]
    pub fn DARKBLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.027450980392156862, b: 0.39215686274509803, a: 1.0 }
    }

    /// ROSA
    #[classattr]
    pub fn ROSA() -> Color {
        Color { r: 0.996078431372549, g: 0.5254901960784314, b: 0.6431372549019608, a: 1.0 }
    }

    /// LIPSTICK
    #[classattr]
    pub fn LIPSTICK() -> Color {
        Color { r: 0.8352941176470589, g: 0.09019607843137255, b: 0.3058823529411765, a: 1.0 }
    }

    /// PALE_MAUVE
    #[classattr]
    pub fn PALE_MAUVE() -> Color {
        Color { r: 0.996078431372549, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 }
    }

    /// CLARET
    #[classattr]
    pub fn CLARET() -> Color {
        Color { r: 0.40784313725490196, g: 0.0, b: 0.09411764705882353, a: 1.0 }
    }

    /// DANDELION
    #[classattr]
    pub fn DANDELION() -> Color {
        Color { r: 0.996078431372549, g: 0.8745098039215686, b: 0.03137254901960784, a: 1.0 }
    }

    /// ORANGERED
    #[classattr]
    pub fn ORANGERED() -> Color {
        Color { r: 0.996078431372549, g: 0.25882352941176473, b: 0.058823529411764705, a: 1.0 }
    }

    /// POOP_GREEN
    #[classattr]
    pub fn POOP_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.48627450980392156, b: 0.0, a: 1.0 }
    }

    /// RUBY
    #[classattr]
    pub fn RUBY() -> Color {
        Color { r: 0.792156862745098, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 }
    }

    /// DARK
    #[classattr]
    pub fn DARK() -> Color {
        Color { r: 0.10588235294117647, g: 0.1411764705882353, b: 0.19215686274509805, a: 1.0 }
    }

    /// GREENISH_TURQUOISE
    #[classattr]
    pub fn GREENISH_TURQUOISE() -> Color {
        Color { r: 0.0, g: 0.984313725490196, b: 0.6901960784313725, a: 1.0 }
    }

    /// PASTEL_RED
    #[classattr]
    pub fn PASTEL_RED() -> Color {
        Color { r: 0.8588235294117647, g: 0.34509803921568627, b: 0.33725490196078434, a: 1.0 }
    }

    /// PISS_YELLOW
    #[classattr]
    pub fn PISS_YELLOW() -> Color {
        Color { r: 0.8666666666666667, g: 0.8392156862745098, b: 0.09411764705882353, a: 1.0 }
    }

    /// BRIGHT_CYAN
    #[classattr]
    pub fn BRIGHT_CYAN() -> Color {
        Color { r: 0.2549019607843137, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// DARK_CORAL
    #[classattr]
    pub fn DARK_CORAL() -> Color {
        Color { r: 0.8117647058823529, g: 0.3215686274509804, b: 0.3058823529411765, a: 1.0 }
    }

    /// ALGAE_GREEN
    #[classattr]
    pub fn ALGAE_GREEN() -> Color {
        Color { r: 0.12941176470588237, g: 0.7647058823529411, b: 0.43529411764705883, a: 1.0 }
    }

    /// DARKISH_RED
    #[classattr]
    pub fn DARKISH_RED() -> Color {
        Color { r: 0.6627450980392157, g: 0.011764705882352941, b: 0.03137254901960784, a: 1.0 }
    }

    /// REDDY_BROWN
    #[classattr]
    pub fn REDDY_BROWN() -> Color {
        Color { r: 0.43137254901960786, g: 0.06274509803921569, b: 0.0196078431372549, a: 1.0 }
    }

    /// BLUSH_PINK
    #[classattr]
    pub fn BLUSH_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.5098039215686274, b: 0.5490196078431373, a: 1.0 }
    }

    /// CAMOUFLAGE_GREEN
    #[classattr]
    pub fn CAMOUFLAGE_GREEN() -> Color {
        Color { r: 0.29411764705882354, g: 0.3803921568627451, b: 0.07450980392156863, a: 1.0 }
    }

    /// LAWN_GREEN
    #[classattr]
    pub fn LAWN_GREEN() -> Color {
        Color { r: 0.30196078431372547, g: 0.6431372549019608, b: 0.03529411764705882, a: 1.0 }
    }

    /// PUTTY
    #[classattr]
    pub fn PUTTY() -> Color {
        Color { r: 0.7450980392156863, g: 0.6823529411764706, b: 0.5411764705882353, a: 1.0 }
    }

    /// VIBRANT_BLUE
    #[classattr]
    pub fn VIBRANT_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.2235294117647059, b: 0.9725490196078431, a: 1.0 }
    }

    /// DARK_SAND
    #[classattr]
    pub fn DARK_SAND() -> Color {
        Color { r: 0.6588235294117647, g: 0.5607843137254902, b: 0.34901960784313724, a: 1.0 }
    }

    /// PURPLE_BLUE
    #[classattr]
    pub fn PURPLE_BLUE() -> Color {
        Color { r: 0.36470588235294116, g: 0.12941176470588237, b: 0.8156862745098039, a: 1.0 }
    }

    /// SAFFRON
    #[classattr]
    pub fn SAFFRON() -> Color {
        Color { r: 0.996078431372549, g: 0.6980392156862745, b: 0.03529411764705882, a: 1.0 }
    }

    /// TWILIGHT
    #[classattr]
    pub fn TWILIGHT() -> Color {
        Color { r: 0.3058823529411765, g: 0.3176470588235294, b: 0.5450980392156862, a: 1.0 }
    }

    /// WARM_BROWN
    #[classattr]
    pub fn WARM_BROWN() -> Color {
        Color { r: 0.5882352941176471, g: 0.3058823529411765, b: 0.00784313725490196, a: 1.0 }
    }

    /// BLUEGREY
    #[classattr]
    pub fn BLUEGREY() -> Color {
        Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 }
    }

    /// BUBBLE_GUM_PINK
    #[classattr]
    pub fn BUBBLE_GUM_PINK() -> Color {
        Color { r: 1.0, g: 0.4117647058823529, b: 0.6862745098039216, a: 1.0 }
    }

    /// DUCK_EGG_BLUE
    #[classattr]
    pub fn DUCK_EGG_BLUE() -> Color {
        Color { r: 0.7647058823529411, g: 0.984313725490196, b: 0.9568627450980393, a: 1.0 }
    }

    /// GREENISH_CYAN
    #[classattr]
    pub fn GREENISH_CYAN() -> Color {
        Color { r: 0.16470588235294117, g: 0.996078431372549, b: 0.7176470588235294, a: 1.0 }
    }

    /// PETROL
    #[classattr]
    pub fn PETROL() -> Color {
        Color { r: 0.0, g: 0.37254901960784315, b: 0.41568627450980394, a: 1.0 }
    }

    /// ROYAL
    #[classattr]
    pub fn ROYAL() -> Color {
        Color { r: 0.047058823529411764, g: 0.09019607843137255, b: 0.5764705882352941, a: 1.0 }
    }

    /// BUTTER
    #[classattr]
    pub fn BUTTER() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.5058823529411764, a: 1.0 }
    }

    /// DUSTY_ORANGE
    #[classattr]
    pub fn DUSTY_ORANGE() -> Color {
        Color { r: 0.9411764705882353, g: 0.5137254901960784, b: 0.22745098039215686, a: 1.0 }
    }

    /// OFF_YELLOW
    #[classattr]
    pub fn OFF_YELLOW() -> Color {
        Color { r: 0.9450980392156862, g: 0.9529411764705882, b: 0.24705882352941178, a: 1.0 }
    }

    /// PALE_OLIVE_GREEN
    #[classattr]
    pub fn PALE_OLIVE_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 0.8235294117647058, b: 0.4823529411764706, a: 1.0 }
    }

    /// ORANGISH
    #[classattr]
    pub fn ORANGISH() -> Color {
        Color { r: 0.9882352941176471, g: 0.5098039215686274, b: 0.2901960784313726, a: 1.0 }
    }

    /// LEAF
    #[classattr]
    pub fn LEAF() -> Color {
        Color { r: 0.44313725490196076, g: 0.6666666666666666, b: 0.20392156862745098, a: 1.0 }
    }

    /// LIGHT_BLUE_GREY
    #[classattr]
    pub fn LIGHT_BLUE_GREY() -> Color {
        Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 }
    }

    /// DRIED_BLOOD
    #[classattr]
    pub fn DRIED_BLOOD() -> Color {
        Color { r: 0.29411764705882354, g: 0.00392156862745098, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHTISH_PURPLE
    #[classattr]
    pub fn LIGHTISH_PURPLE() -> Color {
        Color { r: 0.6470588235294118, g: 0.3215686274509804, b: 0.9019607843137255, a: 1.0 }
    }

    /// RUSTY_RED
    #[classattr]
    pub fn RUSTY_RED() -> Color {
        Color { r: 0.6862745098039216, g: 0.1843137254901961, b: 0.050980392156862744, a: 1.0 }
    }

    /// LAVENDER_BLUE
    #[classattr]
    pub fn LAVENDER_BLUE() -> Color {
        Color { r: 0.5450980392156862, g: 0.5333333333333333, b: 0.9725490196078431, a: 1.0 }
    }

    /// LIGHT_GRASS_GREEN
    #[classattr]
    pub fn LIGHT_GRASS_GREEN() -> Color {
        Color { r: 0.6039215686274509, g: 0.9686274509803922, b: 0.39215686274509803, a: 1.0 }
    }

    /// LIGHT_MINT_GREEN
    #[classattr]
    pub fn LIGHT_MINT_GREEN() -> Color {
        Color { r: 0.6509803921568628, g: 0.984313725490196, b: 0.6980392156862745, a: 1.0 }
    }

    /// SUNFLOWER
    #[classattr]
    pub fn SUNFLOWER() -> Color {
        Color { r: 1.0, g: 0.7725490196078432, b: 0.07058823529411765, a: 1.0 }
    }

    /// VELVET
    #[classattr]
    pub fn VELVET() -> Color {
        Color { r: 0.4588235294117647, g: 0.03137254901960784, b: 0.3176470588235294, a: 1.0 }
    }

    /// BRICK_ORANGE
    #[classattr]
    pub fn BRICK_ORANGE() -> Color {
        Color { r: 0.7568627450980392, g: 0.2901960784313726, b: 0.03529411764705882, a: 1.0 }
    }

    /// LIGHTISH_RED
    #[classattr]
    pub fn LIGHTISH_RED() -> Color {
        Color { r: 0.996078431372549, g: 0.1843137254901961, b: 0.2901960784313726, a: 1.0 }
    }

    /// PURE_BLUE
    #[classattr]
    pub fn PURE_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.011764705882352941, b: 0.8862745098039215, a: 1.0 }
    }

    /// TWILIGHT_BLUE
    #[classattr]
    pub fn TWILIGHT_BLUE() -> Color {
        Color { r: 0.0392156862745098, g: 0.2627450980392157, b: 0.47843137254901963, a: 1.0 }
    }

    /// VIOLET_RED
    #[classattr]
    pub fn VIOLET_RED() -> Color {
        Color { r: 0.6470588235294118, g: 0.0, b: 0.3333333333333333, a: 1.0 }
    }

    /// YELLOWY_BROWN
    #[classattr]
    pub fn YELLOWY_BROWN() -> Color {
        Color { r: 0.6823529411764706, g: 0.5450980392156862, b: 0.047058823529411764, a: 1.0 }
    }

    /// CARNATION
    #[classattr]
    pub fn CARNATION() -> Color {
        Color { r: 0.9921568627450981, g: 0.4745098039215686, b: 0.5607843137254902, a: 1.0 }
    }

    /// MUDDY_YELLOW
    #[classattr]
    pub fn MUDDY_YELLOW() -> Color {
        Color { r: 0.7490196078431373, g: 0.6745098039215687, b: 0.0196078431372549, a: 1.0 }
    }

    /// DARK_SEAFOAM_GREEN
    #[classattr]
    pub fn DARK_SEAFOAM_GREEN() -> Color {
        Color { r: 0.24313725490196078, g: 0.6862745098039216, b: 0.4627450980392157, a: 1.0 }
    }

    /// DEEP_ROSE
    #[classattr]
    pub fn DEEP_ROSE() -> Color {
        Color { r: 0.7803921568627451, g: 0.2784313725490196, b: 0.403921568627451, a: 1.0 }
    }

    /// DUSTY_RED
    #[classattr]
    pub fn DUSTY_RED() -> Color {
        Color { r: 0.7254901960784313, g: 0.2823529411764706, b: 0.3058823529411765, a: 1.0 }
    }

    /// GREY_BLUE
    #[classattr]
    pub fn GREY_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 }
    }

    /// LEMON_LIME
    #[classattr]
    pub fn LEMON_LIME() -> Color {
        Color { r: 0.7490196078431373, g: 0.996078431372549, b: 0.1568627450980392, a: 1.0 }
    }

    /// PURPLE_PINK
    #[classattr]
    pub fn PURPLE_PINK() -> Color {
        Color { r: 0.8431372549019608, g: 0.1450980392156863, b: 0.8705882352941177, a: 1.0 }
    }

    /// BROWN_YELLOW
    #[classattr]
    pub fn BROWN_YELLOW() -> Color {
        Color { r: 0.6980392156862745, g: 0.592156862745098, b: 0.0196078431372549, a: 1.0 }
    }

    /// PURPLE_BROWN
    #[classattr]
    pub fn PURPLE_BROWN() -> Color {
        Color { r: 0.403921568627451, g: 0.22745098039215686, b: 0.24705882352941178, a: 1.0 }
    }

    /// WISTERIA
    #[classattr]
    pub fn WISTERIA() -> Color {
        Color { r: 0.6588235294117647, g: 0.49019607843137253, b: 0.7607843137254902, a: 1.0 }
    }

    /// BANANA_YELLOW
    #[classattr]
    pub fn BANANA_YELLOW() -> Color {
        Color { r: 0.9803921568627451, g: 0.996078431372549, b: 0.29411764705882354, a: 1.0 }
    }

    /// LIPSTICK_RED
    #[classattr]
    pub fn LIPSTICK_RED() -> Color {
        Color { r: 0.7529411764705882, g: 0.00784313725490196, b: 0.1843137254901961, a: 1.0 }
    }

    /// WATER_BLUE
    #[classattr]
    pub fn WATER_BLUE() -> Color {
        Color { r: 0.054901960784313725, g: 0.5294117647058824, b: 0.8, a: 1.0 }
    }

    /// BROWN_GREY
    #[classattr]
    pub fn BROWN_GREY() -> Color {
        Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 }
    }

    /// VIBRANT_PURPLE
    #[classattr]
    pub fn VIBRANT_PURPLE() -> Color {
        Color { r: 0.6784313725490196, g: 0.011764705882352941, b: 0.8705882352941177, a: 1.0 }
    }

    /// BABY_GREEN
    #[classattr]
    pub fn BABY_GREEN() -> Color {
        Color { r: 0.5490196078431373, g: 1.0, b: 0.6196078431372549, a: 1.0 }
    }

    /// BARF_GREEN
    #[classattr]
    pub fn BARF_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6745098039215687, b: 0.00784313725490196, a: 1.0 }
    }

    /// EGGSHELL_BLUE
    #[classattr]
    pub fn EGGSHELL_BLUE() -> Color {
        Color { r: 0.7686274509803922, g: 1.0, b: 0.9686274509803922, a: 1.0 }
    }

    /// SANDY_YELLOW
    #[classattr]
    pub fn SANDY_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 0.9333333333333333, b: 0.45098039215686275, a: 1.0 }
    }

    /// COOL_GREEN
    #[classattr]
    pub fn COOL_GREEN() -> Color {
        Color { r: 0.2, g: 0.7215686274509804, b: 0.39215686274509803, a: 1.0 }
    }

    /// PALE
    #[classattr]
    pub fn PALE() -> Color {
        Color { r: 1.0, g: 0.9764705882352941, b: 0.8156862745098039, a: 1.0 }
    }

    /// BLUE_GREY
    #[classattr]
    pub fn BLUE_GREY() -> Color {
        Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 }
    }

    /// HOT_MAGENTA
    #[classattr]
    pub fn HOT_MAGENTA() -> Color {
        Color { r: 0.9607843137254902, g: 0.01568627450980392, b: 0.788235294117647, a: 1.0 }
    }

    /// GREYBLUE
    #[classattr]
    pub fn GREYBLUE() -> Color {
        Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 }
    }

    /// PURPLEY
    #[classattr]
    pub fn PURPLEY() -> Color {
        Color { r: 0.5294117647058824, g: 0.33725490196078434, b: 0.8941176470588236, a: 1.0 }
    }

    /// BABY_SHIT_GREEN
    #[classattr]
    pub fn BABY_SHIT_GREEN() -> Color {
        Color { r: 0.5333333333333333, g: 0.592156862745098, b: 0.09019607843137255, a: 1.0 }
    }

    /// BROWNISH_PINK
    #[classattr]
    pub fn BROWNISH_PINK() -> Color {
        Color { r: 0.7607843137254902, g: 0.49411764705882355, b: 0.4745098039215686, a: 1.0 }
    }

    /// DARK_AQUAMARINE
    #[classattr]
    pub fn DARK_AQUAMARINE() -> Color {
        Color { r: 0.00392156862745098, g: 0.45098039215686275, b: 0.44313725490196076, a: 1.0 }
    }

    /// DIARRHEA
    #[classattr]
    pub fn DIARRHEA() -> Color {
        Color { r: 0.6235294117647059, g: 0.5137254901960784, b: 0.011764705882352941, a: 1.0 }
    }

    /// LIGHT_MUSTARD
    #[classattr]
    pub fn LIGHT_MUSTARD() -> Color {
        Color { r: 0.9686274509803922, g: 0.8352941176470589, b: 0.3764705882352941, a: 1.0 }
    }

    /// PALE_SKY_BLUE
    #[classattr]
    pub fn PALE_SKY_BLUE() -> Color {
        Color { r: 0.7411764705882353, g: 0.9647058823529412, b: 0.996078431372549, a: 1.0 }
    }

    /// TURTLE_GREEN
    #[classattr]
    pub fn TURTLE_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.7215686274509804, b: 0.30980392156862746, a: 1.0 }
    }

    /// BRIGHT_OLIVE
    #[classattr]
    pub fn BRIGHT_OLIVE() -> Color {
        Color { r: 0.611764705882353, g: 0.7333333333333333, b: 0.01568627450980392, a: 1.0 }
    }

    /// DARK_GREY_BLUE
    #[classattr]
    pub fn DARK_GREY_BLUE() -> Color {
        Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 }
    }

    /// GREENY_BROWN
    #[classattr]
    pub fn GREENY_BROWN() -> Color {
        Color { r: 0.4117647058823529, g: 0.3764705882352941, b: 0.023529411764705882, a: 1.0 }
    }

    /// LEMON_GREEN
    #[classattr]
    pub fn LEMON_GREEN() -> Color {
        Color { r: 0.6784313725490196, g: 0.9725490196078431, b: 0.00784313725490196, a: 1.0 }
    }

    /// LIGHT_PERIWINKLE
    #[classattr]
    pub fn LIGHT_PERIWINKLE() -> Color {
        Color { r: 0.7568627450980392, g: 0.7764705882352941, b: 0.9882352941176471, a: 1.0 }
    }

    /// SEAWEED_GREEN
    #[classattr]
    pub fn SEAWEED_GREEN() -> Color {
        Color { r: 0.20784313725490197, g: 0.6784313725490196, b: 0.4196078431372549, a: 1.0 }
    }

    /// SUNSHINE_YELLOW
    #[classattr]
    pub fn SUNSHINE_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.21568627450980393, a: 1.0 }
    }

    /// UGLY_PURPLE
    #[classattr]
    pub fn UGLY_PURPLE() -> Color {
        Color { r: 0.6431372549019608, g: 0.25882352941176473, b: 0.6274509803921569, a: 1.0 }
    }

    /// MEDIUM_PINK
    #[classattr]
    pub fn MEDIUM_PINK() -> Color {
        Color { r: 0.9529411764705882, g: 0.3803921568627451, b: 0.5882352941176471, a: 1.0 }
    }

    /// PUKE_BROWN
    #[classattr]
    pub fn PUKE_BROWN() -> Color {
        Color { r: 0.5803921568627451, g: 0.4666666666666667, b: 0.023529411764705882, a: 1.0 }
    }

    /// VERY_LIGHT_PINK
    #[classattr]
    pub fn VERY_LIGHT_PINK() -> Color {
        Color { r: 1.0, g: 0.9568627450980393, b: 0.9490196078431372, a: 1.0 }
    }

    /// VIRIDIAN
    #[classattr]
    pub fn VIRIDIAN() -> Color {
        Color { r: 0.11764705882352941, g: 0.5686274509803921, b: 0.403921568627451, a: 1.0 }
    }

    /// BILE
    #[classattr]
    pub fn BILE() -> Color {
        Color { r: 0.7098039215686275, g: 0.7647058823529411, b: 0.023529411764705882, a: 1.0 }
    }

    /// FADED_YELLOW
    #[classattr]
    pub fn FADED_YELLOW() -> Color {
        Color { r: 0.996078431372549, g: 1.0, b: 0.4980392156862745, a: 1.0 }
    }

    /// VERY_PALE_GREEN
    #[classattr]
    pub fn VERY_PALE_GREEN() -> Color {
        Color { r: 0.8117647058823529, g: 0.9921568627450981, b: 0.7372549019607844, a: 1.0 }
    }

    /// VIBRANT_GREEN
    #[classattr]
    pub fn VIBRANT_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 0.8666666666666667, b: 0.03137254901960784, a: 1.0 }
    }

    /// BRIGHT_LIME
    #[classattr]
    pub fn BRIGHT_LIME() -> Color {
        Color { r: 0.5294117647058824, g: 0.9921568627450981, b: 0.0196078431372549, a: 1.0 }
    }

    /// SPEARMINT
    #[classattr]
    pub fn SPEARMINT() -> Color {
        Color { r: 0.11764705882352941, g: 0.9725490196078431, b: 0.4627450980392157, a: 1.0 }
    }

    /// LIGHT_AQUAMARINE
    #[classattr]
    pub fn LIGHT_AQUAMARINE() -> Color {
        Color { r: 0.4823529411764706, g: 0.9921568627450981, b: 0.7803921568627451, a: 1.0 }
    }

    /// LIGHT_SAGE
    #[classattr]
    pub fn LIGHT_SAGE() -> Color {
        Color { r: 0.7372549019607844, g: 0.9254901960784314, b: 0.6745098039215687, a: 1.0 }
    }

    /// YELLOWGREEN
    #[classattr]
    pub fn YELLOWGREEN() -> Color {
        Color { r: 0.7333333333333333, g: 0.9764705882352941, b: 0.058823529411764705, a: 1.0 }
    }

    /// BABY_POO
    #[classattr]
    pub fn BABY_POO() -> Color {
        Color { r: 0.6705882352941176, g: 0.5647058823529412, b: 0.01568627450980392, a: 1.0 }
    }

    /// DARK_SEAFOAM
    #[classattr]
    pub fn DARK_SEAFOAM() -> Color {
        Color { r: 0.12156862745098039, g: 0.7098039215686275, b: 0.47843137254901963, a: 1.0 }
    }

    /// DEEP_TEAL
    #[classattr]
    pub fn DEEP_TEAL() -> Color {
        Color { r: 0.0, g: 0.3333333333333333, b: 0.35294117647058826, a: 1.0 }
    }

    /// HEATHER
    #[classattr]
    pub fn HEATHER() -> Color {
        Color { r: 0.6431372549019608, g: 0.5176470588235295, b: 0.6745098039215687, a: 1.0 }
    }

    /// RUST_ORANGE
    #[classattr]
    pub fn RUST_ORANGE() -> Color {
        Color { r: 0.7686274509803922, g: 0.3333333333333333, b: 0.03137254901960784, a: 1.0 }
    }

    /// DIRTY_BLUE
    #[classattr]
    pub fn DIRTY_BLUE() -> Color {
        Color { r: 0.24705882352941178, g: 0.5098039215686274, b: 0.615686274509804, a: 1.0 }
    }

    /// FERN_GREEN
    #[classattr]
    pub fn FERN_GREEN() -> Color {
        Color { r: 0.32941176470588235, g: 0.5529411764705883, b: 0.26666666666666666, a: 1.0 }
    }

    /// BRIGHT_LILAC
    #[classattr]
    pub fn BRIGHT_LILAC() -> Color {
        Color { r: 0.788235294117647, g: 0.3686274509803922, b: 0.984313725490196, a: 1.0 }
    }

    /// WEIRD_GREEN
    #[classattr]
    pub fn WEIRD_GREEN() -> Color {
        Color { r: 0.22745098039215686, g: 0.8980392156862745, b: 0.4980392156862745, a: 1.0 }
    }

    /// PEACOCK_BLUE
    #[classattr]
    pub fn PEACOCK_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.403921568627451, b: 0.5843137254901961, a: 1.0 }
    }

    /// AVOCADO_GREEN
    #[classattr]
    pub fn AVOCADO_GREEN() -> Color {
        Color { r: 0.5294117647058824, g: 0.6627450980392157, b: 0.13333333333333333, a: 1.0 }
    }

    /// FADED_ORANGE
    #[classattr]
    pub fn FADED_ORANGE() -> Color {
        Color { r: 0.9411764705882353, g: 0.5803921568627451, b: 0.30196078431372547, a: 1.0 }
    }

    /// GRAPE_PURPLE
    #[classattr]
    pub fn GRAPE_PURPLE() -> Color {
        Color { r: 0.36470588235294116, g: 0.0784313725490196, b: 0.3176470588235294, a: 1.0 }
    }

    /// HOT_GREEN
    #[classattr]
    pub fn HOT_GREEN() -> Color {
        Color { r: 0.1450980392156863, g: 1.0, b: 0.1607843137254902, a: 1.0 }
    }

    /// LIME_YELLOW
    #[classattr]
    pub fn LIME_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.11372549019607843, a: 1.0 }
    }

    /// MANGO
    #[classattr]
    pub fn MANGO() -> Color {
        Color { r: 1.0, g: 0.6509803921568628, b: 0.16862745098039217, a: 1.0 }
    }

    /// SHAMROCK
    #[classattr]
    pub fn SHAMROCK() -> Color {
        Color { r: 0.00392156862745098, g: 0.7058823529411765, b: 0.2980392156862745, a: 1.0 }
    }

    /// BUBBLEGUM
    #[classattr]
    pub fn BUBBLEGUM() -> Color {
        Color { r: 1.0, g: 0.4235294117647059, b: 0.7098039215686275, a: 1.0 }
    }

    /// PURPLISH_BROWN
    #[classattr]
    pub fn PURPLISH_BROWN() -> Color {
        Color { r: 0.4196078431372549, g: 0.25882352941176473, b: 0.2784313725490196, a: 1.0 }
    }

    /// VOMIT_YELLOW
    #[classattr]
    pub fn VOMIT_YELLOW() -> Color {
        Color { r: 0.7803921568627451, g: 0.7568627450980392, b: 0.047058823529411764, a: 1.0 }
    }

    /// PALE_CYAN
    #[classattr]
    pub fn PALE_CYAN() -> Color {
        Color { r: 0.7176470588235294, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// KEY_LIME
    #[classattr]
    pub fn KEY_LIME() -> Color {
        Color { r: 0.6823529411764706, g: 1.0, b: 0.43137254901960786, a: 1.0 }
    }

    /// TOMATO_RED
    #[classattr]
    pub fn TOMATO_RED() -> Color {
        Color { r: 0.9254901960784314, g: 0.17647058823529413, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHTGREEN
    #[classattr]
    pub fn LIGHTGREEN() -> Color {
        Color { r: 0.4627450980392157, g: 1.0, b: 0.4823529411764706, a: 1.0 }
    }

    /// MERLOT
    #[classattr]
    pub fn MERLOT() -> Color {
        Color { r: 0.45098039215686275, g: 0.0, b: 0.2235294117647059, a: 1.0 }
    }

    /// NIGHT_BLUE
    #[classattr]
    pub fn NIGHT_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.011764705882352941, b: 0.2823529411764706, a: 1.0 }
    }

    /// PURPLEISH_PINK
    #[classattr]
    pub fn PURPLEISH_PINK() -> Color {
        Color { r: 0.8745098039215686, g: 0.3058823529411765, b: 0.7843137254901961, a: 1.0 }
    }

    /// APPLE
    #[classattr]
    pub fn APPLE() -> Color {
        Color { r: 0.43137254901960786, g: 0.796078431372549, b: 0.23529411764705882, a: 1.0 }
    }

    /// BABY_POOP_GREEN
    #[classattr]
    pub fn BABY_POOP_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.596078431372549, b: 0.0196078431372549, a: 1.0 }
    }

    /// GREEN_APPLE
    #[classattr]
    pub fn GREEN_APPLE() -> Color {
        Color { r: 0.3686274509803922, g: 0.8627450980392157, b: 0.12156862745098039, a: 1.0 }
    }

    /// HELIOTROPE
    #[classattr]
    pub fn HELIOTROPE() -> Color {
        Color { r: 0.8509803921568627, g: 0.30980392156862746, b: 0.9607843137254902, a: 1.0 }
    }

    /// YELLOW_GREEN
    #[classattr]
    pub fn YELLOW_GREEN() -> Color {
        Color { r: 0.7843137254901961, g: 0.9921568627450981, b: 0.23921568627450981, a: 1.0 }
    }

    /// ALMOST_BLACK
    #[classattr]
    pub fn ALMOST_BLACK() -> Color {
        Color { r: 0.027450980392156862, g: 0.050980392156862744, b: 0.050980392156862744, a: 1.0 }
    }

    /// COOL_BLUE
    #[classattr]
    pub fn COOL_BLUE() -> Color {
        Color { r: 0.28627450980392155, g: 0.5176470588235295, b: 0.7215686274509804, a: 1.0 }
    }

    /// LEAFY_GREEN
    #[classattr]
    pub fn LEAFY_GREEN() -> Color {
        Color { r: 0.3176470588235294, g: 0.7176470588235294, b: 0.23137254901960785, a: 1.0 }
    }

    /// MUSTARD_BROWN
    #[classattr]
    pub fn MUSTARD_BROWN() -> Color {
        Color { r: 0.6745098039215687, g: 0.49411764705882355, b: 0.01568627450980392, a: 1.0 }
    }

    /// DUSK
    #[classattr]
    pub fn DUSK() -> Color {
        Color { r: 0.3058823529411765, g: 0.32941176470588235, b: 0.5058823529411764, a: 1.0 }
    }

    /// DULL_BROWN
    #[classattr]
    pub fn DULL_BROWN() -> Color {
        Color { r: 0.5294117647058824, g: 0.43137254901960786, b: 0.29411764705882354, a: 1.0 }
    }

    /// FROG_GREEN
    #[classattr]
    pub fn FROG_GREEN() -> Color {
        Color { r: 0.34509803921568627, g: 0.7372549019607844, b: 0.03137254901960784, a: 1.0 }
    }

    /// VIVID_GREEN
    #[classattr]
    pub fn VIVID_GREEN() -> Color {
        Color { r: 0.1843137254901961, g: 0.9372549019607843, b: 0.06274509803921569, a: 1.0 }
    }

    /// BRIGHT_LIGHT_GREEN
    #[classattr]
    pub fn BRIGHT_LIGHT_GREEN() -> Color {
        Color { r: 0.17647058823529413, g: 0.996078431372549, b: 0.32941176470588235, a: 1.0 }
    }

    /// FLURO_GREEN
    #[classattr]
    pub fn FLURO_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 1.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// KIWI
    #[classattr]
    pub fn KIWI() -> Color {
        Color { r: 0.611764705882353, g: 0.9372549019607843, b: 0.2627450980392157, a: 1.0 }
    }

    /// SEAWEED
    #[classattr]
    pub fn SEAWEED() -> Color {
        Color { r: 0.09411764705882353, g: 0.8196078431372549, b: 0.4823529411764706, a: 1.0 }
    }

    /// NAVY_GREEN
    #[classattr]
    pub fn NAVY_GREEN() -> Color {
        Color { r: 0.20784313725490197, g: 0.3254901960784314, b: 0.0392156862745098, a: 1.0 }
    }

    /// ULTRAMARINE_BLUE
    #[classattr]
    pub fn ULTRAMARINE_BLUE() -> Color {
        Color { r: 0.09411764705882353, g: 0.0196078431372549, b: 0.8588235294117647, a: 1.0 }
    }

    /// IRIS
    #[classattr]
    pub fn IRIS() -> Color {
        Color { r: 0.3843137254901961, g: 0.34509803921568627, b: 0.7686274509803922, a: 1.0 }
    }

    /// PASTEL_ORANGE
    #[classattr]
    pub fn PASTEL_ORANGE() -> Color {
        Color { r: 1.0, g: 0.5882352941176471, b: 0.30980392156862746, a: 1.0 }
    }

    /// YELLOWISH_ORANGE
    #[classattr]
    pub fn YELLOWISH_ORANGE() -> Color {
        Color { r: 1.0, g: 0.6705882352941176, b: 0.058823529411764705, a: 1.0 }
    }

    /// PERRYWINKLE
    #[classattr]
    pub fn PERRYWINKLE() -> Color {
        Color { r: 0.5607843137254902, g: 0.5490196078431373, b: 0.9058823529411765, a: 1.0 }
    }

    /// TEALISH
    #[classattr]
    pub fn TEALISH() -> Color {
        Color { r: 0.1411764705882353, g: 0.7372549019607844, b: 0.6588235294117647, a: 1.0 }
    }

    /// DARK_PLUM
    #[classattr]
    pub fn DARK_PLUM() -> Color {
        Color { r: 0.24705882352941178, g: 0.00392156862745098, b: 0.17254901960784313, a: 1.0 }
    }

    /// PEAR
    #[classattr]
    pub fn PEAR() -> Color {
        Color { r: 0.796078431372549, g: 0.9725490196078431, b: 0.37254901960784315, a: 1.0 }
    }

    /// PINKISH_ORANGE
    #[classattr]
    pub fn PINKISH_ORANGE() -> Color {
        Color { r: 1.0, g: 0.4470588235294118, b: 0.2980392156862745, a: 1.0 }
    }

    /// MIDNIGHT_PURPLE
    #[classattr]
    pub fn MIDNIGHT_PURPLE() -> Color {
        Color { r: 0.1568627450980392, g: 0.00392156862745098, b: 0.21568627450980393, a: 1.0 }
    }

    /// LIGHT_URPLE
    #[classattr]
    pub fn LIGHT_URPLE() -> Color {
        Color { r: 0.7019607843137254, g: 0.43529411764705883, b: 0.9647058823529412, a: 1.0 }
    }

    /// DARK_MINT
    #[classattr]
    pub fn DARK_MINT() -> Color {
        Color { r: 0.2823529411764706, g: 0.7529411764705882, b: 0.4470588235294118, a: 1.0 }
    }

    /// GREENISH_TAN
    #[classattr]
    pub fn GREENISH_TAN() -> Color {
        Color { r: 0.7372549019607844, g: 0.796078431372549, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHT_BURGUNDY
    #[classattr]
    pub fn LIGHT_BURGUNDY() -> Color {
        Color { r: 0.6588235294117647, g: 0.2549019607843137, b: 0.3568627450980392, a: 1.0 }
    }

    /// TURQUOISE_BLUE
    #[classattr]
    pub fn TURQUOISE_BLUE() -> Color {
        Color { r: 0.023529411764705882, g: 0.6941176470588235, b: 0.7686274509803922, a: 1.0 }
    }

    /// UGLY_PINK
    #[classattr]
    pub fn UGLY_PINK() -> Color {
        Color { r: 0.803921568627451, g: 0.4588235294117647, b: 0.5176470588235295, a: 1.0 }
    }

    /// SANDY
    #[classattr]
    pub fn SANDY() -> Color {
        Color { r: 0.9450980392156862, g: 0.8549019607843137, b: 0.47843137254901963, a: 1.0 }
    }

    /// ELECTRIC_PINK
    #[classattr]
    pub fn ELECTRIC_PINK() -> Color {
        Color { r: 1.0, g: 0.01568627450980392, b: 0.5647058823529412, a: 1.0 }
    }

    /// MUTED_PURPLE
    #[classattr]
    pub fn MUTED_PURPLE() -> Color {
        Color { r: 0.5019607843137255, g: 0.3568627450980392, b: 0.5294117647058824, a: 1.0 }
    }

    /// MID_GREEN
    #[classattr]
    pub fn MID_GREEN() -> Color {
        Color { r: 0.3137254901960784, g: 0.6549019607843137, b: 0.2784313725490196, a: 1.0 }
    }

    /// GREYISH
    #[classattr]
    pub fn GREYISH() -> Color {
        Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 }
    }

    /// NEON_YELLOW
    #[classattr]
    pub fn NEON_YELLOW() -> Color {
        Color { r: 0.8117647058823529, g: 1.0, b: 0.01568627450980392, a: 1.0 }
    }

    /// BANANA
    #[classattr]
    pub fn BANANA() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.49411764705882355, a: 1.0 }
    }

    /// CARNATION_PINK
    #[classattr]
    pub fn CARNATION_PINK() -> Color {
        Color { r: 1.0, g: 0.4980392156862745, b: 0.6549019607843137, a: 1.0 }
    }

    /// TOMATO
    #[classattr]
    pub fn TOMATO() -> Color {
        Color { r: 0.9372549019607843, g: 0.25098039215686274, b: 0.14901960784313725, a: 1.0 }
    }

    /// SEA
    #[classattr]
    pub fn SEA() -> Color {
        Color { r: 0.23529411764705882, g: 0.6, b: 0.5725490196078431, a: 1.0 }
    }

    /// MUDDY_BROWN
    #[classattr]
    pub fn MUDDY_BROWN() -> Color {
        Color { r: 0.5333333333333333, g: 0.40784313725490196, b: 0.023529411764705882, a: 1.0 }
    }

    /// TURQUOISE_GREEN
    #[classattr]
    pub fn TURQUOISE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.9568627450980393, b: 0.5372549019607843, a: 1.0 }
    }

    /// BUFF
    #[classattr]
    pub fn BUFF() -> Color {
        Color { r: 0.996078431372549, g: 0.9647058823529412, b: 0.6196078431372549, a: 1.0 }
    }

    /// FAWN
    #[classattr]
    pub fn FAWN() -> Color {
        Color { r: 0.8117647058823529, g: 0.6862745098039216, b: 0.4823529411764706, a: 1.0 }
    }

    /// MUTED_BLUE
    #[classattr]
    pub fn MUTED_BLUE() -> Color {
        Color { r: 0.23137254901960785, g: 0.44313725490196076, b: 0.6235294117647059, a: 1.0 }
    }

    /// PALE_ROSE
    #[classattr]
    pub fn PALE_ROSE() -> Color {
        Color { r: 0.9921568627450981, g: 0.7568627450980392, b: 0.7725490196078432, a: 1.0 }
    }

    /// DARK_MINT_GREEN
    #[classattr]
    pub fn DARK_MINT_GREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.7529411764705882, b: 0.45098039215686275, a: 1.0 }
    }

    /// AMETHYST
    #[classattr]
    pub fn AMETHYST() -> Color {
        Color { r: 0.6078431372549019, g: 0.37254901960784315, b: 0.7529411764705882, a: 1.0 }
    }

    /// BLUE_GREEN
    #[classattr]
    pub fn BLUE_GREEN() -> Color {
        Color { r: 0.058823529411764705, g: 0.6078431372549019, b: 0.5568627450980392, a: 1.0 }
    }

    /// CHESTNUT
    #[classattr]
    pub fn CHESTNUT() -> Color {
        Color { r: 0.4549019607843137, g: 0.1568627450980392, b: 0.00784313725490196, a: 1.0 }
    }

    /// SICK_GREEN
    #[classattr]
    pub fn SICK_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 0.7254901960784313, b: 0.17254901960784313, a: 1.0 }
    }

    /// PEA
    #[classattr]
    pub fn PEA() -> Color {
        Color { r: 0.6431372549019608, g: 0.7490196078431373, b: 0.12549019607843137, a: 1.0 }
    }

    /// RUSTY_ORANGE
    #[classattr]
    pub fn RUSTY_ORANGE() -> Color {
        Color { r: 0.803921568627451, g: 0.34901960784313724, b: 0.03529411764705882, a: 1.0 }
    }

    /// STONE
    #[classattr]
    pub fn STONE() -> Color {
        Color { r: 0.6784313725490196, g: 0.6470588235294118, b: 0.5294117647058824, a: 1.0 }
    }

    /// ROSE_RED
    #[classattr]
    pub fn ROSE_RED() -> Color {
        Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.23529411764705882, a: 1.0 }
    }

    /// PALE_AQUA
    #[classattr]
    pub fn PALE_AQUA() -> Color {
        Color { r: 0.7215686274509804, g: 1.0, b: 0.9215686274509803, a: 1.0 }
    }

    /// DEEP_ORANGE
    #[classattr]
    pub fn DEEP_ORANGE() -> Color {
        Color { r: 0.8627450980392157, g: 0.30196078431372547, b: 0.00392156862745098, a: 1.0 }
    }

    /// EARTH
    #[classattr]
    pub fn EARTH() -> Color {
        Color { r: 0.6352941176470588, g: 0.396078431372549, b: 0.24313725490196078, a: 1.0 }
    }

    /// MOSSY_GREEN
    #[classattr]
    pub fn MOSSY_GREEN() -> Color {
        Color { r: 0.38823529411764707, g: 0.5450980392156862, b: 0.15294117647058825, a: 1.0 }
    }

    /// GRASSY_GREEN
    #[classattr]
    pub fn GRASSY_GREEN() -> Color {
        Color { r: 0.2549019607843137, g: 0.611764705882353, b: 0.011764705882352941, a: 1.0 }
    }

    /// PALE_LIME_GREEN
    #[classattr]
    pub fn PALE_LIME_GREEN() -> Color {
        Color { r: 0.6941176470588235, g: 1.0, b: 0.396078431372549, a: 1.0 }
    }

    /// LIGHT_GREY_BLUE
    #[classattr]
    pub fn LIGHT_GREY_BLUE() -> Color {
        Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 }
    }

    /// PALE_GREY
    #[classattr]
    pub fn PALE_GREY() -> Color {
        Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// ASPARAGUS
    #[classattr]
    pub fn ASPARAGUS() -> Color {
        Color { r: 0.4666666666666667, g: 0.6705882352941176, b: 0.33725490196078434, a: 1.0 }
    }

    /// BLUEBERRY
    #[classattr]
    pub fn BLUEBERRY() -> Color {
        Color { r: 0.27450980392156865, g: 0.2549019607843137, b: 0.5882352941176471, a: 1.0 }
    }

    /// PURPLE_RED
    #[classattr]
    pub fn PURPLE_RED() -> Color {
        Color { r: 0.6, g: 0.00392156862745098, b: 0.2784313725490196, a: 1.0 }
    }

    /// PALE_LIME
    #[classattr]
    pub fn PALE_LIME() -> Color {
        Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.45098039215686275, a: 1.0 }
    }

    /// GREENISH_TEAL
    #[classattr]
    pub fn GREENISH_TEAL() -> Color {
        Color { r: 0.19607843137254902, g: 0.7490196078431373, b: 0.5176470588235295, a: 1.0 }
    }

    /// CARAMEL
    #[classattr]
    pub fn CARAMEL() -> Color {
        Color { r: 0.6862745098039216, g: 0.43529411764705883, b: 0.03529411764705882, a: 1.0 }
    }

    /// DEEP_MAGENTA
    #[classattr]
    pub fn DEEP_MAGENTA() -> Color {
        Color { r: 0.6274509803921569, g: 0.00784313725490196, b: 0.3607843137254902, a: 1.0 }
    }

    /// LIGHT_PEACH
    #[classattr]
    pub fn LIGHT_PEACH() -> Color {
        Color { r: 1.0, g: 0.8470588235294118, b: 0.6941176470588235, a: 1.0 }
    }

    /// MILK_CHOCOLATE
    #[classattr]
    pub fn MILK_CHOCOLATE() -> Color {
        Color { r: 0.4980392156862745, g: 0.3058823529411765, b: 0.11764705882352941, a: 1.0 }
    }

    /// OCHER
    #[classattr]
    pub fn OCHER() -> Color {
        Color { r: 0.7490196078431373, g: 0.6078431372549019, b: 0.047058823529411764, a: 1.0 }
    }

    /// OFF_GREEN
    #[classattr]
    pub fn OFF_GREEN() -> Color {
        Color { r: 0.4196078431372549, g: 0.6392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// PURPLY_PINK
    #[classattr]
    pub fn PURPLY_PINK() -> Color {
        Color { r: 0.9411764705882353, g: 0.4588235294117647, b: 0.9019607843137255, a: 1.0 }
    }

    /// LIGHTBLUE
    #[classattr]
    pub fn LIGHTBLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.7843137254901961, b: 0.9647058823529412, a: 1.0 }
    }

    /// DUSKY_BLUE
    #[classattr]
    pub fn DUSKY_BLUE() -> Color {
        Color { r: 0.2784313725490196, g: 0.37254901960784315, b: 0.5803921568627451, a: 1.0 }
    }

    /// GOLDEN
    #[classattr]
    pub fn GOLDEN() -> Color {
        Color { r: 0.9607843137254902, g: 0.7490196078431373, b: 0.011764705882352941, a: 1.0 }
    }

    /// LIGHT_BEIGE
    #[classattr]
    pub fn LIGHT_BEIGE() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.7137254901960784, a: 1.0 }
    }

    /// BUTTER_YELLOW
    #[classattr]
    pub fn BUTTER_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.4549019607843137, a: 1.0 }
    }

    /// DUSKY_PURPLE
    #[classattr]
    pub fn DUSKY_PURPLE() -> Color {
        Color { r: 0.5372549019607843, g: 0.3568627450980392, b: 0.4823529411764706, a: 1.0 }
    }

    /// FRENCH_BLUE
    #[classattr]
    pub fn FRENCH_BLUE() -> Color {
        Color { r: 0.2627450980392157, g: 0.4196078431372549, b: 0.6784313725490196, a: 1.0 }
    }

    /// UGLY_YELLOW
    #[classattr]
    pub fn UGLY_YELLOW() -> Color {
        Color { r: 0.8156862745098039, g: 0.7568627450980392, b: 0.00392156862745098, a: 1.0 }
    }

    /// GREENY_YELLOW
    #[classattr]
    pub fn GREENY_YELLOW() -> Color {
        Color { r: 0.7764705882352941, g: 0.9725490196078431, b: 0.03137254901960784, a: 1.0 }
    }

    /// ORANGISH_RED
    #[classattr]
    pub fn ORANGISH_RED() -> Color {
        Color { r: 0.9568627450980393, g: 0.21176470588235294, b: 0.0196078431372549, a: 1.0 }
    }

    /// SHAMROCK_GREEN
    #[classattr]
    pub fn SHAMROCK_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.7568627450980392, b: 0.30196078431372547, a: 1.0 }
    }

    /// ORANGISH_BROWN
    #[classattr]
    pub fn ORANGISH_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.37254901960784315, b: 0.011764705882352941, a: 1.0 }
    }

    /// TREE_GREEN
    #[classattr]
    pub fn TREE_GREEN() -> Color {
        Color { r: 0.16470588235294117, g: 0.49411764705882355, b: 0.09803921568627451, a: 1.0 }
    }

    /// DEEP_VIOLET
    #[classattr]
    pub fn DEEP_VIOLET() -> Color {
        Color { r: 0.28627450980392155, g: 0.023529411764705882, b: 0.2823529411764706, a: 1.0 }
    }

    /// GUNMETAL
    #[classattr]
    pub fn GUNMETAL() -> Color {
        Color { r: 0.3254901960784314, g: 0.3843137254901961, b: 0.403921568627451, a: 1.0 }
    }

    /// BLUE_PURPLE
    #[classattr]
    pub fn BLUE_PURPLE() -> Color {
        Color { r: 0.35294117647058826, g: 0.023529411764705882, b: 0.9372549019607843, a: 1.0 }
    }

    /// CHERRY
    #[classattr]
    pub fn CHERRY() -> Color {
        Color { r: 0.8117647058823529, g: 0.00784313725490196, b: 0.20392156862745098, a: 1.0 }
    }

    /// SANDY_BROWN
    #[classattr]
    pub fn SANDY_BROWN() -> Color {
        Color { r: 0.7686274509803922, g: 0.6509803921568628, b: 0.3803921568627451, a: 1.0 }
    }

    /// WARM_GREY
    #[classattr]
    pub fn WARM_GREY() -> Color {
        Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 }
    }

    /// DARK_INDIGO
    #[classattr]
    pub fn DARK_INDIGO() -> Color {
        Color { r: 0.12156862745098039, g: 0.03529411764705882, b: 0.32941176470588235, a: 1.0 }
    }

    /// MIDNIGHT
    #[classattr]
    pub fn MIDNIGHT() -> Color {
        Color { r: 0.011764705882352941, g: 0.00392156862745098, b: 0.17647058823529413, a: 1.0 }
    }

    /// BLUEY_GREEN
    #[classattr]
    pub fn BLUEY_GREEN() -> Color {
        Color { r: 0.16862745098039217, g: 0.6941176470588235, b: 0.4745098039215686, a: 1.0 }
    }

    /// GREY_PINK
    #[classattr]
    pub fn GREY_PINK() -> Color {
        Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 }
    }

    /// SOFT_PURPLE
    #[classattr]
    pub fn SOFT_PURPLE() -> Color {
        Color { r: 0.6509803921568628, g: 0.43529411764705883, b: 0.7098039215686275, a: 1.0 }
    }

    /// BLOOD
    #[classattr]
    pub fn BLOOD() -> Color {
        Color { r: 0.4666666666666667, g: 0.0, b: 0.00392156862745098, a: 1.0 }
    }

    /// BROWN_RED
    #[classattr]
    pub fn BROWN_RED() -> Color {
        Color { r: 0.5725490196078431, g: 0.16862745098039217, b: 0.0196078431372549, a: 1.0 }
    }

    /// MEDIUM_GREY
    #[classattr]
    pub fn MEDIUM_GREY() -> Color {
        Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 }
    }

    /// BERRY
    #[classattr]
    pub fn BERRY() -> Color {
        Color { r: 0.6, g: 0.058823529411764705, b: 0.29411764705882354, a: 1.0 }
    }

    /// POO
    #[classattr]
    pub fn POO() -> Color {
        Color { r: 0.5607843137254902, g: 0.45098039215686275, b: 0.011764705882352941, a: 1.0 }
    }

    /// PURPLEY_PINK
    #[classattr]
    pub fn PURPLEY_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.23529411764705882, b: 0.7254901960784313, a: 1.0 }
    }

    /// LIGHT_SALMON
    #[classattr]
    pub fn LIGHT_SALMON() -> Color {
        Color { r: 0.996078431372549, g: 0.6627450980392157, b: 0.5764705882352941, a: 1.0 }
    }

    /// SNOT
    #[classattr]
    pub fn SNOT() -> Color {
        Color { r: 0.6745098039215687, g: 0.7333333333333333, b: 0.050980392156862744, a: 1.0 }
    }

    /// EASTER_PURPLE
    #[classattr]
    pub fn EASTER_PURPLE() -> Color {
        Color { r: 0.7529411764705882, g: 0.44313725490196076, b: 0.996078431372549, a: 1.0 }
    }

    /// LIGHT_YELLOW_GREEN
    #[classattr]
    pub fn LIGHT_YELLOW_GREEN() -> Color {
        Color { r: 0.8, g: 0.9921568627450981, b: 0.4980392156862745, a: 1.0 }
    }

    /// DARK_NAVY_BLUE
    #[classattr]
    pub fn DARK_NAVY_BLUE() -> Color {
        Color { r: 0.0, g: 0.00784313725490196, b: 0.1803921568627451, a: 1.0 }
    }

    /// DRAB
    #[classattr]
    pub fn DRAB() -> Color {
        Color { r: 0.5098039215686274, g: 0.5137254901960784, b: 0.26666666666666666, a: 1.0 }
    }

    /// LIGHT_ROSE
    #[classattr]
    pub fn LIGHT_ROSE() -> Color {
        Color { r: 1.0, g: 0.7725490196078432, b: 0.796078431372549, a: 1.0 }
    }

    /// ROUGE
    #[classattr]
    pub fn ROUGE() -> Color {
        Color { r: 0.6705882352941176, g: 0.07058823529411765, b: 0.2235294117647059, a: 1.0 }
    }

    /// PURPLISH_RED
    #[classattr]
    pub fn PURPLISH_RED() -> Color {
        Color { r: 0.6901960784313725, g: 0.0196078431372549, b: 0.29411764705882354, a: 1.0 }
    }

    /// SLIME_GREEN
    #[classattr]
    pub fn SLIME_GREEN() -> Color {
        Color { r: 0.6, g: 0.8, b: 0.01568627450980392, a: 1.0 }
    }

    /// BABY_POOP
    #[classattr]
    pub fn BABY_POOP() -> Color {
        Color { r: 0.5764705882352941, g: 0.48627450980392156, b: 0.0, a: 1.0 }
    }

    /// IRISH_GREEN
    #[classattr]
    pub fn IRISH_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.5843137254901961, b: 0.1607843137254902, a: 1.0 }
    }

    /// PINK_PURPLE
    #[classattr]
    pub fn PINK_PURPLE() -> Color {
        Color { r: 0.9372549019607843, g: 0.11372549019607843, b: 0.9058823529411765, a: 1.0 }
    }

    /// DARK_NAVY
    #[classattr]
    pub fn DARK_NAVY() -> Color {
        Color { r: 0.0, g: 0.01568627450980392, b: 0.20784313725490197, a: 1.0 }
    }

    /// GREENY_BLUE
    #[classattr]
    pub fn GREENY_BLUE() -> Color {
        Color { r: 0.25882352941176473, g: 0.7019607843137254, b: 0.5843137254901961, a: 1.0 }
    }

    /// LIGHT_PLUM
    #[classattr]
    pub fn LIGHT_PLUM() -> Color {
        Color { r: 0.615686274509804, g: 0.3411764705882353, b: 0.5137254901960784, a: 1.0 }
    }

    /// PINKISH_GREY
    #[classattr]
    pub fn PINKISH_GREY() -> Color {
        Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 }
    }

    /// DIRTY_ORANGE
    #[classattr]
    pub fn DIRTY_ORANGE() -> Color {
        Color { r: 0.7843137254901961, g: 0.4627450980392157, b: 0.023529411764705882, a: 1.0 }
    }

    /// RUST_RED
    #[classattr]
    pub fn RUST_RED() -> Color {
        Color { r: 0.6666666666666666, g: 0.15294117647058825, b: 0.01568627450980392, a: 1.0 }
    }

    /// PALE_LILAC
    #[classattr]
    pub fn PALE_LILAC() -> Color {
        Color { r: 0.8941176470588236, g: 0.796078431372549, b: 1.0, a: 1.0 }
    }

    /// ORANGEY_RED
    #[classattr]
    pub fn ORANGEY_RED() -> Color {
        Color { r: 0.9803921568627451, g: 0.25882352941176473, b: 0.1411764705882353, a: 1.0 }
    }

    /// PRIMARY_BLUE
    #[classattr]
    pub fn PRIMARY_BLUE() -> Color {
        Color { r: 0.03137254901960784, g: 0.01568627450980392, b: 0.9764705882352941, a: 1.0 }
    }

    /// KERMIT_GREEN
    #[classattr]
    pub fn KERMIT_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.6980392156862745, b: 0.0, a: 1.0 }
    }

    /// BROWNISH_PURPLE
    #[classattr]
    pub fn BROWNISH_PURPLE() -> Color {
        Color { r: 0.4627450980392157, g: 0.25882352941176473, b: 0.3058823529411765, a: 1.0 }
    }

    /// MURKY_GREEN
    #[classattr]
    pub fn MURKY_GREEN() -> Color {
        Color { r: 0.4235294117647059, g: 0.47843137254901963, b: 0.054901960784313725, a: 1.0 }
    }

    /// WHEAT
    #[classattr]
    pub fn WHEAT() -> Color {
        Color { r: 0.984313725490196, g: 0.8666666666666667, b: 0.49411764705882355, a: 1.0 }
    }

    /// VERY_DARK_PURPLE
    #[classattr]
    pub fn VERY_DARK_PURPLE() -> Color {
        Color { r: 0.16470588235294117, g: 0.00392156862745098, b: 0.20392156862745098, a: 1.0 }
    }

    /// BOTTLE_GREEN
    #[classattr]
    pub fn BOTTLE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.2901960784313726, b: 0.0196078431372549, a: 1.0 }
    }

    /// WATERMELON
    #[classattr]
    pub fn WATERMELON() -> Color {
        Color { r: 0.9921568627450981, g: 0.27450980392156865, b: 0.34901960784313724, a: 1.0 }
    }

    /// DEEP_SKY_BLUE
    #[classattr]
    pub fn DEEP_SKY_BLUE() -> Color {
        Color { r: 0.050980392156862744, g: 0.4588235294117647, b: 0.9725490196078431, a: 1.0 }
    }

    /// FIRE_ENGINE_RED
    #[classattr]
    pub fn FIRE_ENGINE_RED() -> Color {
        Color { r: 0.996078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// YELLOW_OCHRE
    #[classattr]
    pub fn YELLOW_OCHRE() -> Color {
        Color { r: 0.796078431372549, g: 0.615686274509804, b: 0.023529411764705882, a: 1.0 }
    }

    /// PUMPKIN_ORANGE
    #[classattr]
    pub fn PUMPKIN_ORANGE() -> Color {
        Color { r: 0.984313725490196, g: 0.49019607843137253, b: 0.027450980392156862, a: 1.0 }
    }

    /// PALE_OLIVE
    #[classattr]
    pub fn PALE_OLIVE() -> Color {
        Color { r: 0.7254901960784313, g: 0.8, b: 0.5058823529411764, a: 1.0 }
    }

    /// LIGHT_LILAC
    #[classattr]
    pub fn LIGHT_LILAC() -> Color {
        Color { r: 0.9294117647058824, g: 0.7843137254901961, b: 1.0, a: 1.0 }
    }

    /// LIGHTISH_GREEN
    #[classattr]
    pub fn LIGHTISH_GREEN() -> Color {
        Color { r: 0.3803921568627451, g: 0.8823529411764706, b: 0.3764705882352941, a: 1.0 }
    }

    /// CAROLINA_BLUE
    #[classattr]
    pub fn CAROLINA_BLUE() -> Color {
        Color { r: 0.5411764705882353, g: 0.7215686274509804, b: 0.996078431372549, a: 1.0 }
    }

    /// MULBERRY
    #[classattr]
    pub fn MULBERRY() -> Color {
        Color { r: 0.5725490196078431, g: 0.0392156862745098, b: 0.3058823529411765, a: 1.0 }
    }

    /// SHOCKING_PINK
    #[classattr]
    pub fn SHOCKING_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00784313725490196, b: 0.6352941176470588, a: 1.0 }
    }

    /// AUBURN
    #[classattr]
    pub fn AUBURN() -> Color {
        Color { r: 0.6039215686274509, g: 0.18823529411764706, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_LIME_GREEN
    #[classattr]
    pub fn BRIGHT_LIME_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.996078431372549, b: 0.03137254901960784, a: 1.0 }
    }

    /// CELADON
    #[classattr]
    pub fn CELADON() -> Color {
        Color { r: 0.7450980392156863, g: 0.9921568627450981, b: 0.7176470588235294, a: 1.0 }
    }

    /// PINKISH_BROWN
    #[classattr]
    pub fn PINKISH_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.4470588235294118, b: 0.3803921568627451, a: 1.0 }
    }

    /// POO_BROWN
    #[classattr]
    pub fn POO_BROWN() -> Color {
        Color { r: 0.5333333333333333, g: 0.37254901960784315, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_SKY_BLUE
    #[classattr]
    pub fn BRIGHT_SKY_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.8, b: 0.996078431372549, a: 1.0 }
    }

    /// CELERY
    #[classattr]
    pub fn CELERY() -> Color {
        Color { r: 0.7568627450980392, g: 0.9921568627450981, b: 0.5843137254901961, a: 1.0 }
    }

    /// DIRT_BROWN
    #[classattr]
    pub fn DIRT_BROWN() -> Color {
        Color { r: 0.5137254901960784, g: 0.396078431372549, b: 0.2235294117647059, a: 1.0 }
    }

    /// STRAWBERRY
    #[classattr]
    pub fn STRAWBERRY() -> Color {
        Color { r: 0.984313725490196, g: 0.1607843137254902, b: 0.2627450980392157, a: 1.0 }
    }

    /// DARK_LIME
    #[classattr]
    pub fn DARK_LIME() -> Color {
        Color { r: 0.5176470588235295, g: 0.7176470588235294, b: 0.00392156862745098, a: 1.0 }
    }

    /// COPPER
    #[classattr]
    pub fn COPPER() -> Color {
        Color { r: 0.7137254901960784, g: 0.38823529411764707, b: 0.1450980392156863, a: 1.0 }
    }

    /// MEDIUM_BROWN
    #[classattr]
    pub fn MEDIUM_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.3176470588235294, b: 0.07058823529411765, a: 1.0 }
    }

    /// MUTED_GREEN
    #[classattr]
    pub fn MUTED_GREEN() -> Color {
        Color { r: 0.37254901960784315, g: 0.6274509803921569, b: 0.3215686274509804, a: 1.0 }
    }

    /// ROBINS_EGG
    #[classattr]
    pub fn ROBINS_EGG() -> Color {
        Color { r: 0.42745098039215684, g: 0.9294117647058824, b: 0.9921568627450981, a: 1.0 }
    }

    /// BRIGHT_AQUA
    #[classattr]
    pub fn BRIGHT_AQUA() -> Color {
        Color { r: 0.043137254901960784, g: 0.9764705882352941, b: 0.9176470588235294, a: 1.0 }
    }

    /// BRIGHT_LAVENDER
    #[classattr]
    pub fn BRIGHT_LAVENDER() -> Color {
        Color { r: 0.7803921568627451, g: 0.3764705882352941, b: 1.0, a: 1.0 }
    }

    /// IVORY
    #[classattr]
    pub fn IVORY() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.796078431372549, a: 1.0 }
    }

    /// VERY_LIGHT_PURPLE
    #[classattr]
    pub fn VERY_LIGHT_PURPLE() -> Color {
        Color { r: 0.9647058823529412, g: 0.807843137254902, b: 0.9882352941176471, a: 1.0 }
    }

    /// LIGHT_NAVY
    #[classattr]
    pub fn LIGHT_NAVY() -> Color {
        Color { r: 0.08235294117647059, g: 0.3137254901960784, b: 0.5176470588235295, a: 1.0 }
    }

    /// PINK_RED
    #[classattr]
    pub fn PINK_RED() -> Color {
        Color { r: 0.9607843137254902, g: 0.0196078431372549, b: 0.30980392156862746, a: 1.0 }
    }

    /// OLIVE_BROWN
    #[classattr]
    pub fn OLIVE_BROWN() -> Color {
        Color { r: 0.39215686274509803, g: 0.32941176470588235, b: 0.011764705882352941, a: 1.0 }
    }

    /// POOP_BROWN
    #[classattr]
    pub fn POOP_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.34901960784313724, b: 0.00392156862745098, a: 1.0 }
    }

    /// MUSTARD_GREEN
    #[classattr]
    pub fn MUSTARD_GREEN() -> Color {
        Color { r: 0.6588235294117647, g: 0.7098039215686275, b: 0.01568627450980392, a: 1.0 }
    }

    /// OCEAN_GREEN
    #[classattr]
    pub fn OCEAN_GREEN() -> Color {
        Color { r: 0.23921568627450981, g: 0.6, b: 0.45098039215686275, a: 1.0 }
    }

    /// VERY_DARK_BLUE
    #[classattr]
    pub fn VERY_DARK_BLUE() -> Color {
        Color { r: 0.0, g: 0.00392156862745098, b: 0.2, a: 1.0 }
    }

    /// DUSTY_GREEN
    #[classattr]
    pub fn DUSTY_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.6627450980392157, b: 0.45098039215686275, a: 1.0 }
    }

    /// LIGHT_NAVY_BLUE
    #[classattr]
    pub fn LIGHT_NAVY_BLUE() -> Color {
        Color { r: 0.1803921568627451, g: 0.35294117647058826, b: 0.5333333333333333, a: 1.0 }
    }

    /// MINTY_GREEN
    #[classattr]
    pub fn MINTY_GREEN() -> Color {
        Color { r: 0.043137254901960784, g: 0.9686274509803922, b: 0.49019607843137253, a: 1.0 }
    }

    /// ADOBE
    #[classattr]
    pub fn ADOBE() -> Color {
        Color { r: 0.7411764705882353, g: 0.4235294117647059, b: 0.2823529411764706, a: 1.0 }
    }

    /// BARNEY
    #[classattr]
    pub fn BARNEY() -> Color {
        Color { r: 0.6745098039215687, g: 0.11372549019607843, b: 0.7215686274509804, a: 1.0 }
    }

    /// JADE_GREEN
    #[classattr]
    pub fn JADE_GREEN() -> Color {
        Color { r: 0.16862745098039217, g: 0.6862745098039216, b: 0.41568627450980394, a: 1.0 }
    }

    /// BRIGHT_LIGHT_BLUE
    #[classattr]
    pub fn BRIGHT_LIGHT_BLUE() -> Color {
        Color { r: 0.14901960784313725, g: 0.9686274509803922, b: 0.9921568627450981, a: 1.0 }
    }

    /// LIGHT_LIME
    #[classattr]
    pub fn LIGHT_LIME() -> Color {
        Color { r: 0.6823529411764706, g: 0.9921568627450981, b: 0.4235294117647059, a: 1.0 }
    }

    /// DARK_KHAKI
    #[classattr]
    pub fn DARK_KHAKI() -> Color {
        Color { r: 0.6078431372549019, g: 0.5607843137254902, b: 0.3333333333333333, a: 1.0 }
    }

    /// ORANGE_YELLOW
    #[classattr]
    pub fn ORANGE_YELLOW() -> Color {
        Color { r: 1.0, g: 0.6784313725490196, b: 0.00392156862745098, a: 1.0 }
    }

    /// OCRE
    #[classattr]
    pub fn OCRE() -> Color {
        Color { r: 0.7764705882352941, g: 0.611764705882353, b: 0.01568627450980392, a: 1.0 }
    }

    /// MAIZE
    #[classattr]
    pub fn MAIZE() -> Color {
        Color { r: 0.9568627450980393, g: 0.8156862745098039, b: 0.32941176470588235, a: 1.0 }
    }

    /// FADED_PINK
    #[classattr]
    pub fn FADED_PINK() -> Color {
        Color { r: 0.8705882352941177, g: 0.615686274509804, b: 0.6745098039215687, a: 1.0 }
    }

    /// BRITISH_RACING_GREEN
    #[classattr]
    pub fn BRITISH_RACING_GREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.2823529411764706, b: 0.050980392156862744, a: 1.0 }
    }

    /// SANDSTONE
    #[classattr]
    pub fn SANDSTONE() -> Color {
        Color { r: 0.788235294117647, g: 0.6823529411764706, b: 0.4549019607843137, a: 1.0 }
    }

    /// MUD_BROWN
    #[classattr]
    pub fn MUD_BROWN() -> Color {
        Color { r: 0.3764705882352941, g: 0.27450980392156865, b: 0.058823529411764705, a: 1.0 }
    }

    /// LIGHT_SEA_GREEN
    #[classattr]
    pub fn LIGHT_SEA_GREEN() -> Color {
        Color { r: 0.596078431372549, g: 0.9647058823529412, b: 0.6901960784313725, a: 1.0 }
    }

    /// ROBIN_EGG_BLUE
    #[classattr]
    pub fn ROBIN_EGG_BLUE() -> Color {
        Color { r: 0.5411764705882353, g: 0.9450980392156862, b: 0.996078431372549, a: 1.0 }
    }

    /// AQUA_MARINE
    #[classattr]
    pub fn AQUA_MARINE() -> Color {
        Color { r: 0.1803921568627451, g: 0.9098039215686274, b: 0.7333333333333333, a: 1.0 }
    }

    /// DARK_SEA_GREEN
    #[classattr]
    pub fn DARK_SEA_GREEN() -> Color {
        Color { r: 0.06666666666666667, g: 0.5294117647058824, b: 0.36470588235294116, a: 1.0 }
    }

    /// SOFT_PINK
    #[classattr]
    pub fn SOFT_PINK() -> Color {
        Color { r: 0.9921568627450981, g: 0.6901960784313725, b: 0.7529411764705882, a: 1.0 }
    }

    /// ORANGEY_BROWN
    #[classattr]
    pub fn ORANGEY_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.3764705882352941, b: 0.00784313725490196, a: 1.0 }
    }

    /// CHERRY_RED
    #[classattr]
    pub fn CHERRY_RED() -> Color {
        Color { r: 0.9686274509803922, g: 0.00784313725490196, b: 0.16470588235294117, a: 1.0 }
    }

    /// BURNT_YELLOW
    #[classattr]
    pub fn BURNT_YELLOW() -> Color {
        Color { r: 0.8352941176470589, g: 0.6705882352941176, b: 0.03529411764705882, a: 1.0 }
    }

    /// BROWNISH_GREY
    #[classattr]
    pub fn BROWNISH_GREY() -> Color {
        Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 }
    }

    /// CAMEL
    #[classattr]
    pub fn CAMEL() -> Color {
        Color { r: 0.7764705882352941, g: 0.6235294117647059, b: 0.34901960784313724, a: 1.0 }
    }

    /// PURPLISH_GREY
    #[classattr]
    pub fn PURPLISH_GREY() -> Color {
        Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 }
    }

    /// MARINE
    #[classattr]
    pub fn MARINE() -> Color {
        Color { r: 0.01568627450980392, g: 0.1803921568627451, b: 0.3764705882352941, a: 1.0 }
    }

    /// GREYISH_PINK
    #[classattr]
    pub fn GREYISH_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 }
    }

    /// PALE_TURQUOISE
    #[classattr]
    pub fn PALE_TURQUOISE() -> Color {
        Color { r: 0.6470588235294118, g: 0.984313725490196, b: 0.8352941176470589, a: 1.0 }
    }

    /// PASTEL_YELLOW
    #[classattr]
    pub fn PASTEL_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.44313725490196076, a: 1.0 }
    }

    /// BLUEY_PURPLE
    #[classattr]
    pub fn BLUEY_PURPLE() -> Color {
        Color { r: 0.3843137254901961, g: 0.2549019607843137, b: 0.7803921568627451, a: 1.0 }
    }

    /// CANARY_YELLOW
    #[classattr]
    pub fn CANARY_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.25098039215686274, a: 1.0 }
    }

    /// FADED_RED
    #[classattr]
    pub fn FADED_RED() -> Color {
        Color { r: 0.8274509803921568, g: 0.28627450980392155, b: 0.3058823529411765, a: 1.0 }
    }

    /// SEPIA
    #[classattr]
    pub fn SEPIA() -> Color {
        Color { r: 0.596078431372549, g: 0.3686274509803922, b: 0.16862745098039217, a: 1.0 }
    }

    /// COFFEE
    #[classattr]
    pub fn COFFEE() -> Color {
        Color { r: 0.6509803921568628, g: 0.5058823529411764, b: 0.2980392156862745, a: 1.0 }
    }

    /// BRIGHT_MAGENTA
    #[classattr]
    pub fn BRIGHT_MAGENTA() -> Color {
        Color { r: 1.0, g: 0.03137254901960784, b: 0.9098039215686274, a: 1.0 }
    }

    /// MOCHA
    #[classattr]
    pub fn MOCHA() -> Color {
        Color { r: 0.615686274509804, g: 0.4627450980392157, b: 0.3176470588235294, a: 1.0 }
    }

    /// ECRU
    #[classattr]
    pub fn ECRU() -> Color {
        Color { r: 0.996078431372549, g: 1.0, b: 0.792156862745098, a: 1.0 }
    }

    /// PURPLEISH
    #[classattr]
    pub fn PURPLEISH() -> Color {
        Color { r: 0.596078431372549, g: 0.33725490196078434, b: 0.5529411764705883, a: 1.0 }
    }

    /// CRANBERRY
    #[classattr]
    pub fn CRANBERRY() -> Color {
        Color { r: 0.6196078431372549, g: 0.0, b: 0.22745098039215686, a: 1.0 }
    }

    /// DARKISH_GREEN
    #[classattr]
    pub fn DARKISH_GREEN() -> Color {
        Color { r: 0.1568627450980392, g: 0.48627450980392156, b: 0.21568627450980393, a: 1.0 }
    }

    /// BROWN_ORANGE
    #[classattr]
    pub fn BROWN_ORANGE() -> Color {
        Color { r: 0.7254901960784313, g: 0.4117647058823529, b: 0.00784313725490196, a: 1.0 }
    }

    /// DUSKY_ROSE
    #[classattr]
    pub fn DUSKY_ROSE() -> Color {
        Color { r: 0.7294117647058823, g: 0.40784313725490196, b: 0.45098039215686275, a: 1.0 }
    }

    /// MELON
    #[classattr]
    pub fn MELON() -> Color {
        Color { r: 1.0, g: 0.47058823529411764, b: 0.3333333333333333, a: 1.0 }
    }

    /// SICKLY_GREEN
    #[classattr]
    pub fn SICKLY_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6980392156862745, b: 0.10980392156862745, a: 1.0 }
    }

    /// SILVER
    #[classattr]
    pub fn SILVER() -> Color {
        Color { r: 0.7725490196078432, g: 0.788235294117647, b: 0.7803921568627451, a: 1.0 }
    }

    /// PURPLY_BLUE
    #[classattr]
    pub fn PURPLY_BLUE() -> Color {
        Color { r: 0.4, g: 0.10196078431372549, b: 0.9333333333333333, a: 1.0 }
    }

    /// PURPLEISH_BLUE
    #[classattr]
    pub fn PURPLEISH_BLUE() -> Color {
        Color { r: 0.3803921568627451, g: 0.25098039215686274, b: 0.9372549019607843, a: 1.0 }
    }

    /// HOSPITAL_GREEN
    #[classattr]
    pub fn HOSPITAL_GREEN() -> Color {
        Color { r: 0.6078431372549019, g: 0.8980392156862745, b: 0.6666666666666666, a: 1.0 }
    }

    /// SHIT_BROWN
    #[classattr]
    pub fn SHIT_BROWN() -> Color {
        Color { r: 0.4823529411764706, g: 0.34509803921568627, b: 0.01568627450980392, a: 1.0 }
    }

    /// MID_BLUE
    #[classattr]
    pub fn MID_BLUE() -> Color {
        Color { r: 0.15294117647058825, g: 0.41568627450980394, b: 0.7019607843137254, a: 1.0 }
    }

    /// AMBER
    #[classattr]
    pub fn AMBER() -> Color {
        Color { r: 0.996078431372549, g: 0.7019607843137254, b: 0.03137254901960784, a: 1.0 }
    }

    /// EASTER_GREEN
    #[classattr]
    pub fn EASTER_GREEN() -> Color {
        Color { r: 0.5490196078431373, g: 0.9921568627450981, b: 0.49411764705882355, a: 1.0 }
    }

    /// SOFT_BLUE
    #[classattr]
    pub fn SOFT_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.5333333333333333, b: 0.9176470588235294, a: 1.0 }
    }

    /// CERULEAN_BLUE
    #[classattr]
    pub fn CERULEAN_BLUE() -> Color {
        Color { r: 0.0196078431372549, g: 0.43137254901960786, b: 0.9333333333333333, a: 1.0 }
    }

    /// GOLDEN_BROWN
    #[classattr]
    pub fn GOLDEN_BROWN() -> Color {
        Color { r: 0.6980392156862745, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 }
    }

    /// BRIGHT_TURQUOISE
    #[classattr]
    pub fn BRIGHT_TURQUOISE() -> Color {
        Color { r: 0.058823529411764705, g: 0.996078431372549, b: 0.9764705882352941, a: 1.0 }
    }

    /// RED_PINK
    #[classattr]
    pub fn RED_PINK() -> Color {
        Color { r: 0.9803921568627451, g: 0.16470588235294117, b: 0.3333333333333333, a: 1.0 }
    }

    /// RED_PURPLE
    #[classattr]
    pub fn RED_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.027450980392156862, b: 0.2784313725490196, a: 1.0 }
    }

    /// GREYISH_BROWN
    #[classattr]
    pub fn GREYISH_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 }
    }

    /// VERMILLION
    #[classattr]
    pub fn VERMILLION() -> Color {
        Color { r: 0.9568627450980393, g: 0.19607843137254902, b: 0.047058823529411764, a: 1.0 }
    }

    /// RUSSET
    #[classattr]
    pub fn RUSSET() -> Color {
        Color { r: 0.6313725490196078, g: 0.2235294117647059, b: 0.0196078431372549, a: 1.0 }
    }

    /// STEEL_GREY
    #[classattr]
    pub fn STEEL_GREY() -> Color {
        Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 }
    }

    /// LIGHTER_PURPLE
    #[classattr]
    pub fn LIGHTER_PURPLE() -> Color {
        Color { r: 0.6470588235294118, g: 0.35294117647058826, b: 0.9568627450980393, a: 1.0 }
    }

    /// BRIGHT_VIOLET
    #[classattr]
    pub fn BRIGHT_VIOLET() -> Color {
        Color { r: 0.6784313725490196, g: 0.0392156862745098, b: 0.9921568627450981, a: 1.0 }
    }

    /// PRUSSIAN_BLUE
    #[classattr]
    pub fn PRUSSIAN_BLUE() -> Color {
        Color { r: 0.0, g: 0.27058823529411763, b: 0.4666666666666667, a: 1.0 }
    }

    /// SLATE_GREEN
    #[classattr]
    pub fn SLATE_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.5529411764705883, b: 0.42745098039215684, a: 1.0 }
    }

    /// DIRTY_PINK
    #[classattr]
    pub fn DIRTY_PINK() -> Color {
        Color { r: 0.792156862745098, g: 0.4823529411764706, b: 0.5019607843137255, a: 1.0 }
    }

    /// DARK_BLUE_GREEN
    #[classattr]
    pub fn DARK_BLUE_GREEN() -> Color {
        Color { r: 0.0, g: 0.3215686274509804, b: 0.28627450980392155, a: 1.0 }
    }

    /// PINE
    #[classattr]
    pub fn PINE() -> Color {
        Color { r: 0.16862745098039217, g: 0.36470588235294116, b: 0.20392156862745098, a: 1.0 }
    }

    /// YELLOWY_GREEN
    #[classattr]
    pub fn YELLOWY_GREEN() -> Color {
        Color { r: 0.7490196078431373, g: 0.9450980392156862, b: 0.1568627450980392, a: 1.0 }
    }

    /// DARK_GOLD
    #[classattr]
    pub fn DARK_GOLD() -> Color {
        Color { r: 0.7098039215686275, g: 0.5803921568627451, b: 0.06274509803921569, a: 1.0 }
    }

    /// BLUISH
    #[classattr]
    pub fn BLUISH() -> Color {
        Color { r: 0.1607843137254902, g: 0.4627450980392157, b: 0.7333333333333333, a: 1.0 }
    }

    /// DARKISH_BLUE
    #[classattr]
    pub fn DARKISH_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.2549019607843137, b: 0.5098039215686274, a: 1.0 }
    }

    /// DULL_RED
    #[classattr]
    pub fn DULL_RED() -> Color {
        Color { r: 0.7333333333333333, g: 0.24705882352941178, b: 0.24705882352941178, a: 1.0 }
    }

    /// PINKY_RED
    #[classattr]
    pub fn PINKY_RED() -> Color {
        Color { r: 0.9882352941176471, g: 0.14901960784313725, b: 0.2784313725490196, a: 1.0 }
    }

    /// BRONZE
    #[classattr]
    pub fn BRONZE() -> Color {
        Color { r: 0.6588235294117647, g: 0.4745098039215686, b: 0.0, a: 1.0 }
    }

    /// PALE_TEAL
    #[classattr]
    pub fn PALE_TEAL() -> Color {
        Color { r: 0.5098039215686274, g: 0.796078431372549, b: 0.6980392156862745, a: 1.0 }
    }

    /// MILITARY_GREEN
    #[classattr]
    pub fn MILITARY_GREEN() -> Color {
        Color { r: 0.4, g: 0.48627450980392156, b: 0.24313725490196078, a: 1.0 }
    }

    /// BARBIE_PINK
    #[classattr]
    pub fn BARBIE_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.27450980392156865, b: 0.6470588235294118, a: 1.0 }
    }

    /// BUBBLEGUM_PINK
    #[classattr]
    pub fn BUBBLEGUM_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.5137254901960784, b: 0.8, a: 1.0 }
    }

    /// PEA_SOUP_GREEN
    #[classattr]
    pub fn PEA_SOUP_GREEN() -> Color {
        Color { r: 0.5803921568627451, g: 0.6509803921568628, b: 0.09019607843137255, a: 1.0 }
    }

    /// DARK_MUSTARD
    #[classattr]
    pub fn DARK_MUSTARD() -> Color {
        Color { r: 0.6588235294117647, g: 0.5372549019607843, b: 0.0196078431372549, a: 1.0 }
    }

    /// SHIT
    #[classattr]
    pub fn SHIT() -> Color {
        Color { r: 0.4980392156862745, g: 0.37254901960784315, b: 0.0, a: 1.0 }
    }

    /// MEDIUM_PURPLE
    #[classattr]
    pub fn MEDIUM_PURPLE() -> Color {
        Color { r: 0.6196078431372549, g: 0.2627450980392157, b: 0.6352941176470588, a: 1.0 }
    }

    /// VERY_DARK_GREEN
    #[classattr]
    pub fn VERY_DARK_GREEN() -> Color {
        Color { r: 0.023529411764705882, g: 0.1803921568627451, b: 0.011764705882352941, a: 1.0 }
    }

    /// DIRT
    #[classattr]
    pub fn DIRT() -> Color {
        Color { r: 0.5411764705882353, g: 0.43137254901960786, b: 0.27058823529411763, a: 1.0 }
    }

    /// DUSKY_PINK
    #[classattr]
    pub fn DUSKY_PINK() -> Color {
        Color { r: 0.8, g: 0.47843137254901963, b: 0.5450980392156862, a: 1.0 }
    }

    /// RED_VIOLET
    #[classattr]
    pub fn RED_VIOLET() -> Color {
        Color { r: 0.6196078431372549, g: 0.00392156862745098, b: 0.40784313725490196, a: 1.0 }
    }

    /// LEMON_YELLOW
    #[classattr]
    pub fn LEMON_YELLOW() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.2196078431372549, a: 1.0 }
    }

    /// PISTACHIO
    #[classattr]
    pub fn PISTACHIO() -> Color {
        Color { r: 0.7529411764705882, g: 0.9803921568627451, b: 0.5450980392156862, a: 1.0 }
    }

    /// DULL_YELLOW
    #[classattr]
    pub fn DULL_YELLOW() -> Color {
        Color { r: 0.9333333333333333, g: 0.8627450980392157, b: 0.3568627450980392, a: 1.0 }
    }

    /// DARK_LIME_GREEN
    #[classattr]
    pub fn DARK_LIME_GREEN() -> Color {
        Color { r: 0.49411764705882355, g: 0.7411764705882353, b: 0.00392156862745098, a: 1.0 }
    }

    /// DENIM_BLUE
    #[classattr]
    pub fn DENIM_BLUE() -> Color {
        Color { r: 0.23137254901960785, g: 0.3568627450980392, b: 0.5725490196078431, a: 1.0 }
    }

    /// TEAL_BLUE
    #[classattr]
    pub fn TEAL_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.5333333333333333, b: 0.6235294117647059, a: 1.0 }
    }

    /// LIGHTISH_BLUE
    #[classattr]
    pub fn LIGHTISH_BLUE() -> Color {
        Color { r: 0.23921568627450981, g: 0.47843137254901963, b: 0.9921568627450981, a: 1.0 }
    }

    /// PURPLEY_BLUE
    #[classattr]
    pub fn PURPLEY_BLUE() -> Color {
        Color { r: 0.37254901960784315, g: 0.20392156862745098, b: 0.9058823529411765, a: 1.0 }
    }

    /// LIGHT_INDIGO
    #[classattr]
    pub fn LIGHT_INDIGO() -> Color {
        Color { r: 0.42745098039215684, g: 0.35294117647058826, b: 0.8117647058823529, a: 1.0 }
    }

    /// SWAMP_GREEN
    #[classattr]
    pub fn SWAMP_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.5215686274509804, b: 0.0, a: 1.0 }
    }

    /// BROWN_GREEN
    #[classattr]
    pub fn BROWN_GREEN() -> Color {
        Color { r: 0.4392156862745098, g: 0.4235294117647059, b: 0.06666666666666667, a: 1.0 }
    }

    /// DARK_MAROON
    #[classattr]
    pub fn DARK_MAROON() -> Color {
        Color { r: 0.23529411764705882, g: 0.0, b: 0.03137254901960784, a: 1.0 }
    }

    /// HOT_PURPLE
    #[classattr]
    pub fn HOT_PURPLE() -> Color {
        Color { r: 0.796078431372549, g: 0.0, b: 0.9607843137254902, a: 1.0 }
    }

    /// DARK_FOREST_GREEN
    #[classattr]
    pub fn DARK_FOREST_GREEN() -> Color {
        Color { r: 0.0, g: 0.17647058823529413, b: 0.01568627450980392, a: 1.0 }
    }

    /// FADED_BLUE
    #[classattr]
    pub fn FADED_BLUE() -> Color {
        Color { r: 0.396078431372549, g: 0.5490196078431373, b: 0.7333333333333333, a: 1.0 }
    }

    /// DRAB_GREEN
    #[classattr]
    pub fn DRAB_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.5843137254901961, b: 0.3176470588235294, a: 1.0 }
    }

    /// LIGHT_LIME_GREEN
    #[classattr]
    pub fn LIGHT_LIME_GREEN() -> Color {
        Color { r: 0.7254901960784313, g: 1.0, b: 0.4, a: 1.0 }
    }

    /// SNOT_GREEN
    #[classattr]
    pub fn SNOT_GREEN() -> Color {
        Color { r: 0.615686274509804, g: 0.7568627450980392, b: 0.0, a: 1.0 }
    }

    /// YELLOWISH
    #[classattr]
    pub fn YELLOWISH() -> Color {
        Color { r: 0.9803921568627451, g: 0.9333333333333333, b: 0.4, a: 1.0 }
    }

    /// LIGHT_BLUE_GREEN
    #[classattr]
    pub fn LIGHT_BLUE_GREEN() -> Color {
        Color { r: 0.49411764705882355, g: 0.984313725490196, b: 0.7019607843137254, a: 1.0 }
    }

    /// BORDEAUX
    #[classattr]
    pub fn BORDEAUX() -> Color {
        Color { r: 0.4823529411764706, g: 0.0, b: 0.17254901960784313, a: 1.0 }
    }

    /// LIGHT_MAUVE
    #[classattr]
    pub fn LIGHT_MAUVE() -> Color {
        Color { r: 0.7607843137254902, g: 0.5725490196078431, b: 0.6313725490196078, a: 1.0 }
    }

    /// OCEAN
    #[classattr]
    pub fn OCEAN() -> Color {
        Color { r: 0.00392156862745098, g: 0.4823529411764706, b: 0.5725490196078431, a: 1.0 }
    }

    /// MARIGOLD
    #[classattr]
    pub fn MARIGOLD() -> Color {
        Color { r: 0.9882352941176471, g: 0.7529411764705882, b: 0.023529411764705882, a: 1.0 }
    }

    /// MUDDY_GREEN
    #[classattr]
    pub fn MUDDY_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.4549019607843137, b: 0.19607843137254902, a: 1.0 }
    }

    /// DULL_ORANGE
    #[classattr]
    pub fn DULL_ORANGE() -> Color {
        Color { r: 0.8470588235294118, g: 0.5254901960784314, b: 0.23137254901960785, a: 1.0 }
    }

    /// STEEL
    #[classattr]
    pub fn STEEL() -> Color {
        Color { r: 0.45098039215686275, g: 0.5215686274509804, b: 0.5843137254901961, a: 1.0 }
    }

    /// ELECTRIC_PURPLE
    #[classattr]
    pub fn ELECTRIC_PURPLE() -> Color {
        Color { r: 0.6666666666666666, g: 0.13725490196078433, b: 1.0, a: 1.0 }
    }

    /// FLUORESCENT_GREEN
    #[classattr]
    pub fn FLUORESCENT_GREEN() -> Color {
        Color { r: 0.03137254901960784, g: 1.0, b: 0.03137254901960784, a: 1.0 }
    }

    /// YELLOWISH_BROWN
    #[classattr]
    pub fn YELLOWISH_BROWN() -> Color {
        Color { r: 0.6078431372549019, g: 0.47843137254901963, b: 0.00392156862745098, a: 1.0 }
    }

    /// BLUSH
    #[classattr]
    pub fn BLUSH() -> Color {
        Color { r: 0.9490196078431372, g: 0.6196078431372549, b: 0.5568627450980392, a: 1.0 }
    }

    /// SOFT_GREEN
    #[classattr]
    pub fn SOFT_GREEN() -> Color {
        Color { r: 0.43529411764705883, g: 0.7607843137254902, b: 0.4627450980392157, a: 1.0 }
    }

    /// BRIGHT_ORANGE
    #[classattr]
    pub fn BRIGHT_ORANGE() -> Color {
        Color { r: 1.0, g: 0.3568627450980392, b: 0.0, a: 1.0 }
    }

    /// LEMON
    #[classattr]
    pub fn LEMON() -> Color {
        Color { r: 0.9921568627450981, g: 1.0, b: 0.3215686274509804, a: 1.0 }
    }

    /// PURPLE_GREY
    #[classattr]
    pub fn PURPLE_GREY() -> Color {
        Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 }
    }

    /// ACID_GREEN
    #[classattr]
    pub fn ACID_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.996078431372549, b: 0.03529411764705882, a: 1.0 }
    }

    /// PALE_LAVENDER
    #[classattr]
    pub fn PALE_LAVENDER() -> Color {
        Color { r: 0.9333333333333333, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 }
    }

    /// VIOLET_BLUE
    #[classattr]
    pub fn VIOLET_BLUE() -> Color {
        Color { r: 0.3176470588235294, g: 0.0392156862745098, b: 0.788235294117647, a: 1.0 }
    }

    /// LIGHT_FOREST_GREEN
    #[classattr]
    pub fn LIGHT_FOREST_GREEN() -> Color {
        Color { r: 0.30980392156862746, g: 0.5686274509803921, b: 0.3254901960784314, a: 1.0 }
    }

    /// BURNT_RED
    #[classattr]
    pub fn BURNT_RED() -> Color {
        Color { r: 0.6235294117647059, g: 0.13725490196078433, b: 0.0196078431372549, a: 1.0 }
    }

    /// KHAKI_GREEN
    #[classattr]
    pub fn KHAKI_GREEN() -> Color {
        Color { r: 0.4470588235294118, g: 0.5254901960784314, b: 0.2235294117647059, a: 1.0 }
    }

    /// CERISE
    #[classattr]
    pub fn CERISE() -> Color {
        Color { r: 0.8705882352941177, g: 0.047058823529411764, b: 0.3843137254901961, a: 1.0 }
    }

    /// FADED_PURPLE
    #[classattr]
    pub fn FADED_PURPLE() -> Color {
        Color { r: 0.5686274509803921, g: 0.43137254901960786, b: 0.6, a: 1.0 }
    }

    /// APRICOT
    #[classattr]
    pub fn APRICOT() -> Color {
        Color { r: 1.0, g: 0.6941176470588235, b: 0.42745098039215684, a: 1.0 }
    }

    /// DARK_OLIVE_GREEN
    #[classattr]
    pub fn DARK_OLIVE_GREEN() -> Color {
        Color { r: 0.23529411764705882, g: 0.30196078431372547, b: 0.011764705882352941, a: 1.0 }
    }

    /// GREY_BROWN
    #[classattr]
    pub fn GREY_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREEN_GREY
    #[classattr]
    pub fn GREEN_GREY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 }
    }

    /// TRUE_BLUE
    #[classattr]
    pub fn TRUE_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.058823529411764705, b: 0.8, a: 1.0 }
    }

    /// PALE_VIOLET
    #[classattr]
    pub fn PALE_VIOLET() -> Color {
        Color { r: 0.807843137254902, g: 0.6823529411764706, b: 0.9803921568627451, a: 1.0 }
    }

    /// PERIWINKLE_BLUE
    #[classattr]
    pub fn PERIWINKLE_BLUE() -> Color {
        Color { r: 0.5607843137254902, g: 0.6, b: 0.984313725490196, a: 1.0 }
    }

    /// LIGHT_SKY_BLUE
    #[classattr]
    pub fn LIGHT_SKY_BLUE() -> Color {
        Color { r: 0.7764705882352941, g: 0.9882352941176471, b: 1.0, a: 1.0 }
    }

    /// BLURPLE
    #[classattr]
    pub fn BLURPLE() -> Color {
        Color { r: 0.3333333333333333, g: 0.2235294117647059, b: 0.8, a: 1.0 }
    }

    /// GREEN_BROWN
    #[classattr]
    pub fn GREEN_BROWN() -> Color {
        Color { r: 0.32941176470588235, g: 0.3058823529411765, b: 0.011764705882352941, a: 1.0 }
    }

    /// BLUEGREEN
    #[classattr]
    pub fn BLUEGREEN() -> Color {
        Color { r: 0.00392156862745098, g: 0.47843137254901963, b: 0.4745098039215686, a: 1.0 }
    }

    /// BRIGHT_TEAL
    #[classattr]
    pub fn BRIGHT_TEAL() -> Color {
        Color { r: 0.00392156862745098, g: 0.9764705882352941, b: 0.7764705882352941, a: 1.0 }
    }

    /// BROWNISH_YELLOW
    #[classattr]
    pub fn BROWNISH_YELLOW() -> Color {
        Color { r: 0.788235294117647, g: 0.6901960784313725, b: 0.011764705882352941, a: 1.0 }
    }

    /// PEA_SOUP
    #[classattr]
    pub fn PEA_SOUP() -> Color {
        Color { r: 0.5725490196078431, g: 0.6, b: 0.00392156862745098, a: 1.0 }
    }

    /// FOREST
    #[classattr]
    pub fn FOREST() -> Color {
        Color { r: 0.043137254901960784, g: 0.3333333333333333, b: 0.03529411764705882, a: 1.0 }
    }

    /// BARNEY_PURPLE
    #[classattr]
    pub fn BARNEY_PURPLE() -> Color {
        Color { r: 0.6274509803921569, g: 0.01568627450980392, b: 0.596078431372549, a: 1.0 }
    }

    /// ULTRAMARINE
    #[classattr]
    pub fn ULTRAMARINE() -> Color {
        Color { r: 0.12549019607843137, g: 0.0, b: 0.6941176470588235, a: 1.0 }
    }

    /// PURPLISH
    #[classattr]
    pub fn PURPLISH() -> Color {
        Color { r: 0.5803921568627451, g: 0.33725490196078434, b: 0.5490196078431373, a: 1.0 }
    }

    /// PUKE_YELLOW
    #[classattr]
    pub fn PUKE_YELLOW() -> Color {
        Color { r: 0.7607843137254902, g: 0.7450980392156863, b: 0.054901960784313725, a: 1.0 }
    }

    /// BLUISH_GREY
    #[classattr]
    pub fn BLUISH_GREY() -> Color {
        Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 }
    }

    /// DARK_PERIWINKLE
    #[classattr]
    pub fn DARK_PERIWINKLE() -> Color {
        Color { r: 0.4, g: 0.37254901960784315, b: 0.8196078431372549, a: 1.0 }
    }

    /// DARK_LILAC
    #[classattr]
    pub fn DARK_LILAC() -> Color {
        Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.6470588235294118, a: 1.0 }
    }

    /// REDDISH
    #[classattr]
    pub fn REDDISH() -> Color {
        Color { r: 0.7686274509803922, g: 0.25882352941176473, b: 0.25098039215686274, a: 1.0 }
    }

    /// LIGHT_MAROON
    #[classattr]
    pub fn LIGHT_MAROON() -> Color {
        Color { r: 0.6352941176470588, g: 0.2823529411764706, b: 0.3411764705882353, a: 1.0 }
    }

    /// DUSTY_PURPLE
    #[classattr]
    pub fn DUSTY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.37254901960784315, b: 0.5294117647058824, a: 1.0 }
    }

    /// TERRA_COTTA
    #[classattr]
    pub fn TERRA_COTTA() -> Color {
        Color { r: 0.788235294117647, g: 0.39215686274509803, b: 0.23137254901960785, a: 1.0 }
    }

    /// AVOCADO
    #[classattr]
    pub fn AVOCADO() -> Color {
        Color { r: 0.5647058823529412, g: 0.6941176470588235, b: 0.20392156862745098, a: 1.0 }
    }

    /// MARINE_BLUE
    #[classattr]
    pub fn MARINE_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.2196078431372549, b: 0.41568627450980394, a: 1.0 }
    }

    /// TEAL_GREEN
    #[classattr]
    pub fn TEAL_GREEN() -> Color {
        Color { r: 0.1450980392156863, g: 0.6392156862745098, b: 0.43529411764705883, a: 1.0 }
    }

    /// SLATE_GREY
    #[classattr]
    pub fn SLATE_GREY() -> Color {
        Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 }
    }

    /// LIGHTER_GREEN
    #[classattr]
    pub fn LIGHTER_GREEN() -> Color {
        Color { r: 0.4588235294117647, g: 0.9921568627450981, b: 0.38823529411764707, a: 1.0 }
    }

    /// ELECTRIC_GREEN
    #[classattr]
    pub fn ELECTRIC_GREEN() -> Color {
        Color { r: 0.12941176470588237, g: 0.9882352941176471, b: 0.050980392156862744, a: 1.0 }
    }

    /// DUSTY_BLUE
    #[classattr]
    pub fn DUSTY_BLUE() -> Color {
        Color { r: 0.35294117647058826, g: 0.5254901960784314, b: 0.6784313725490196, a: 1.0 }
    }

    /// GOLDEN_YELLOW
    #[classattr]
    pub fn GOLDEN_YELLOW() -> Color {
        Color { r: 0.996078431372549, g: 0.7764705882352941, b: 0.08235294117647059, a: 1.0 }
    }

    /// BRIGHT_YELLOW
    #[classattr]
    pub fn BRIGHT_YELLOW() -> Color {
        Color { r: 1.0, g: 0.9921568627450981, b: 0.00392156862745098, a: 1.0 }
    }

    /// LIGHT_LAVENDER
    #[classattr]
    pub fn LIGHT_LAVENDER() -> Color {
        Color { r: 0.8745098039215686, g: 0.7725490196078432, b: 0.996078431372549, a: 1.0 }
    }

    /// UMBER
    #[classattr]
    pub fn UMBER() -> Color {
        Color { r: 0.6980392156862745, g: 0.39215686274509803, b: 0.0, a: 1.0 }
    }

    /// POOP
    #[classattr]
    pub fn POOP() -> Color {
        Color { r: 0.4980392156862745, g: 0.3686274509803922, b: 0.0, a: 1.0 }
    }

    /// DARK_PEACH
    #[classattr]
    pub fn DARK_PEACH() -> Color {
        Color { r: 0.8705882352941177, g: 0.49411764705882355, b: 0.36470588235294116, a: 1.0 }
    }

    /// JUNGLE_GREEN
    #[classattr]
    pub fn JUNGLE_GREEN() -> Color {
        Color { r: 0.01568627450980392, g: 0.5098039215686274, b: 0.2627450980392157, a: 1.0 }
    }

    /// EGGSHELL
    #[classattr]
    pub fn EGGSHELL() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8313725490196079, a: 1.0 }
    }

    /// DENIM
    #[classattr]
    pub fn DENIM() -> Color {
        Color { r: 0.23137254901960785, g: 0.38823529411764707, b: 0.5490196078431373, a: 1.0 }
    }

    /// YELLOW_BROWN
    #[classattr]
    pub fn YELLOW_BROWN() -> Color {
        Color { r: 0.7176470588235294, g: 0.5803921568627451, b: 0.0, a: 1.0 }
    }

    /// DULL_PURPLE
    #[classattr]
    pub fn DULL_PURPLE() -> Color {
        Color { r: 0.5176470588235295, g: 0.34901960784313724, b: 0.49411764705882355, a: 1.0 }
    }

    /// CHOCOLATE_BROWN
    #[classattr]
    pub fn CHOCOLATE_BROWN() -> Color {
        Color { r: 0.2549019607843137, g: 0.09803921568627451, b: 0.0, a: 1.0 }
    }

    /// WINE_RED
    #[classattr]
    pub fn WINE_RED() -> Color {
        Color { r: 0.4823529411764706, g: 0.011764705882352941, b: 0.13725490196078433, a: 1.0 }
    }

    /// NEON_BLUE
    #[classattr]
    pub fn NEON_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.8509803921568627, b: 1.0, a: 1.0 }
    }

    /// DIRTY_GREEN
    #[classattr]
    pub fn DIRTY_GREEN() -> Color {
        Color { r: 0.4, g: 0.49411764705882355, b: 0.17254901960784313, a: 1.0 }
    }

    /// LIGHT_TAN
    #[classattr]
    pub fn LIGHT_TAN() -> Color {
        Color { r: 0.984313725490196, g: 0.9333333333333333, b: 0.6745098039215687, a: 1.0 }
    }

    /// ICE_BLUE
    #[classattr]
    pub fn ICE_BLUE() -> Color {
        Color { r: 0.8431372549019608, g: 1.0, b: 0.996078431372549, a: 1.0 }
    }

    /// CADET_BLUE
    #[classattr]
    pub fn CADET_BLUE() -> Color {
        Color { r: 0.3058823529411765, g: 0.4549019607843137, b: 0.5882352941176471, a: 1.0 }
    }

    /// DARK_MAUVE
    #[classattr]
    pub fn DARK_MAUVE() -> Color {
        Color { r: 0.5294117647058824, g: 0.2980392156862745, b: 0.3843137254901961, a: 1.0 }
    }

    /// VERY_LIGHT_BLUE
    #[classattr]
    pub fn VERY_LIGHT_BLUE() -> Color {
        Color { r: 0.8352941176470589, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// GREY_PURPLE
    #[classattr]
    pub fn GREY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 }
    }

    /// PASTEL_PINK
    #[classattr]
    pub fn PASTEL_PINK() -> Color {
        Color { r: 1.0, g: 0.7294117647058823, b: 0.803921568627451, a: 1.0 }
    }

    /// VERY_LIGHT_GREEN
    #[classattr]
    pub fn VERY_LIGHT_GREEN() -> Color {
        Color { r: 0.8196078431372549, g: 1.0, b: 0.7411764705882353, a: 1.0 }
    }

    /// DARK_SKY_BLUE
    #[classattr]
    pub fn DARK_SKY_BLUE() -> Color {
        Color { r: 0.26666666666666666, g: 0.5568627450980392, b: 0.8941176470588236, a: 1.0 }
    }

    /// EVERGREEN
    #[classattr]
    pub fn EVERGREEN() -> Color {
        Color { r: 0.0196078431372549, g: 0.2784313725490196, b: 0.16470588235294117, a: 1.0 }
    }

    /// DULL_PINK
    #[classattr]
    pub fn DULL_PINK() -> Color {
        Color { r: 0.8352941176470589, g: 0.5254901960784314, b: 0.615686274509804, a: 1.0 }
    }

    /// AUBERGINE
    #[classattr]
    pub fn AUBERGINE() -> Color {
        Color { r: 0.23921568627450981, g: 0.027450980392156862, b: 0.20392156862745098, a: 1.0 }
    }

    /// MAHOGANY
    #[classattr]
    pub fn MAHOGANY() -> Color {
        Color { r: 0.2901960784313726, g: 0.00392156862745098, b: 0.0, a: 1.0 }
    }

    /// REDDISH_ORANGE
    #[classattr]
    pub fn REDDISH_ORANGE() -> Color {
        Color { r: 0.9725490196078431, g: 0.2823529411764706, b: 0.10980392156862745, a: 1.0 }
    }

    /// DEEP_GREEN
    #[classattr]
    pub fn DEEP_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.34901960784313724, b: 0.058823529411764705, a: 1.0 }
    }

    /// VOMIT_GREEN
    #[classattr]
    pub fn VOMIT_GREEN() -> Color {
        Color { r: 0.5372549019607843, g: 0.6352941176470588, b: 0.011764705882352941, a: 1.0 }
    }

    /// DUSTY_PINK
    #[classattr]
    pub fn DUSTY_PINK() -> Color {
        Color { r: 0.8352941176470589, g: 0.5411764705882353, b: 0.5803921568627451, a: 1.0 }
    }

    /// FADED_GREEN
    #[classattr]
    pub fn FADED_GREEN() -> Color {
        Color { r: 0.4823529411764706, g: 0.6980392156862745, b: 0.4549019607843137, a: 1.0 }
    }

    /// CAMO_GREEN
    #[classattr]
    pub fn CAMO_GREEN() -> Color {
        Color { r: 0.3215686274509804, g: 0.396078431372549, b: 0.1450980392156863, a: 1.0 }
    }

    /// PINKY_PURPLE
    #[classattr]
    pub fn PINKY_PURPLE() -> Color {
        Color { r: 0.788235294117647, g: 0.2980392156862745, b: 0.7450980392156863, a: 1.0 }
    }

    /// BROWNISH_RED
    #[classattr]
    pub fn BROWNISH_RED() -> Color {
        Color { r: 0.6196078431372549, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_ROSE
    #[classattr]
    pub fn DARK_ROSE() -> Color {
        Color { r: 0.7098039215686275, g: 0.2823529411764706, b: 0.36470588235294116, a: 1.0 }
    }

    /// MUD
    #[classattr]
    pub fn MUD() -> Color {
        Color { r: 0.45098039215686275, g: 0.3607843137254902, b: 0.07058823529411765, a: 1.0 }
    }

    /// BROWNISH
    #[classattr]
    pub fn BROWNISH() -> Color {
        Color { r: 0.611764705882353, g: 0.42745098039215684, b: 0.3411764705882353, a: 1.0 }
    }

    /// EMERALD_GREEN
    #[classattr]
    pub fn EMERALD_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.5607843137254902, b: 0.11764705882352941, a: 1.0 }
    }

    /// PALE_BROWN
    #[classattr]
    pub fn PALE_BROWN() -> Color {
        Color { r: 0.6941176470588235, g: 0.5686274509803921, b: 0.43137254901960786, a: 1.0 }
    }

    /// DULL_BLUE
    #[classattr]
    pub fn DULL_BLUE() -> Color {
        Color { r: 0.28627450980392155, g: 0.4588235294117647, b: 0.611764705882353, a: 1.0 }
    }

    /// BURNT_UMBER
    #[classattr]
    pub fn BURNT_UMBER() -> Color {
        Color { r: 0.6274509803921569, g: 0.27058823529411763, b: 0.054901960784313725, a: 1.0 }
    }

    /// MEDIUM_GREEN
    #[classattr]
    pub fn MEDIUM_GREEN() -> Color {
        Color { r: 0.2235294117647059, g: 0.6784313725490196, b: 0.2823529411764706, a: 1.0 }
    }

    /// CLAY
    #[classattr]
    pub fn CLAY() -> Color {
        Color { r: 0.7137254901960784, g: 0.41568627450980394, b: 0.3137254901960784, a: 1.0 }
    }

    /// LIGHT_AQUA
    #[classattr]
    pub fn LIGHT_AQUA() -> Color {
        Color { r: 0.5490196078431373, g: 1.0, b: 0.8588235294117647, a: 1.0 }
    }

    /// LIGHT_OLIVE_GREEN
    #[classattr]
    pub fn LIGHT_OLIVE_GREEN() -> Color {
        Color { r: 0.6431372549019608, g: 0.7450980392156863, b: 0.3607843137254902, a: 1.0 }
    }

    /// BROWNISH_ORANGE
    #[classattr]
    pub fn BROWNISH_ORANGE() -> Color {
        Color { r: 0.796078431372549, g: 0.4666666666666667, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_AQUA
    #[classattr]
    pub fn DARK_AQUA() -> Color {
        Color { r: 0.0196078431372549, g: 0.4117647058823529, b: 0.4196078431372549, a: 1.0 }
    }

    /// PURPLISH_PINK
    #[classattr]
    pub fn PURPLISH_PINK() -> Color {
        Color { r: 0.807843137254902, g: 0.36470588235294116, b: 0.6823529411764706, a: 1.0 }
    }

    /// DARK_SALMON
    #[classattr]
    pub fn DARK_SALMON() -> Color {
        Color { r: 0.7843137254901961, g: 0.35294117647058826, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREENISH_GREY
    #[classattr]
    pub fn GREENISH_GREY() -> Color {
        Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 }
    }

    /// JADE
    #[classattr]
    pub fn JADE() -> Color {
        Color { r: 0.12156862745098039, g: 0.6549019607843137, b: 0.4549019607843137, a: 1.0 }
    }

    /// UGLY_GREEN
    #[classattr]
    pub fn UGLY_GREEN() -> Color {
        Color { r: 0.47843137254901963, g: 0.592156862745098, b: 0.011764705882352941, a: 1.0 }
    }

    /// DARK_BEIGE
    #[classattr]
    pub fn DARK_BEIGE() -> Color {
        Color { r: 0.6745098039215687, g: 0.5764705882352941, b: 0.3843137254901961, a: 1.0 }
    }

    /// EMERALD
    #[classattr]
    pub fn EMERALD() -> Color {
        Color { r: 0.00392156862745098, g: 0.6274509803921569, b: 0.28627450980392155, a: 1.0 }
    }

    /// PALE_RED
    #[classattr]
    pub fn PALE_RED() -> Color {
        Color { r: 0.8509803921568627, g: 0.32941176470588235, b: 0.30196078431372547, a: 1.0 }
    }

    /// LIGHT_MAGENTA
    #[classattr]
    pub fn LIGHT_MAGENTA() -> Color {
        Color { r: 0.9803921568627451, g: 0.37254901960784315, b: 0.9686274509803922, a: 1.0 }
    }

    /// SKY
    #[classattr]
    pub fn SKY() -> Color {
        Color { r: 0.5098039215686274, g: 0.792156862745098, b: 0.9882352941176471, a: 1.0 }
    }

    /// LIGHT_CYAN
    #[classattr]
    pub fn LIGHT_CYAN() -> Color {
        Color { r: 0.6745098039215687, g: 1.0, b: 0.9882352941176471, a: 1.0 }
    }

    /// YELLOW_ORANGE
    #[classattr]
    pub fn YELLOW_ORANGE() -> Color {
        Color { r: 0.9882352941176471, g: 0.6901960784313725, b: 0.00392156862745098, a: 1.0 }
    }

    /// REDDISH_PURPLE
    #[classattr]
    pub fn REDDISH_PURPLE() -> Color {
        Color { r: 0.5686274509803921, g: 0.03529411764705882, b: 0.3176470588235294, a: 1.0 }
    }

    /// REDDISH_PINK
    #[classattr]
    pub fn REDDISH_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.17254901960784313, b: 0.32941176470588235, a: 1.0 }
    }

    /// ORCHID
    #[classattr]
    pub fn ORCHID() -> Color {
        Color { r: 0.7843137254901961, g: 0.4588235294117647, b: 0.7686274509803922, a: 1.0 }
    }

    /// DIRTY_YELLOW
    #[classattr]
    pub fn DIRTY_YELLOW() -> Color {
        Color { r: 0.803921568627451, g: 0.7725490196078432, b: 0.0392156862745098, a: 1.0 }
    }

    /// ORANGE_RED
    #[classattr]
    pub fn ORANGE_RED() -> Color {
        Color { r: 0.9921568627450981, g: 0.2549019607843137, b: 0.11764705882352941, a: 1.0 }
    }

    /// DEEP_RED
    #[classattr]
    pub fn DEEP_RED() -> Color {
        Color { r: 0.6039215686274509, g: 0.00784313725490196, b: 0.0, a: 1.0 }
    }

    /// ORANGE_BROWN
    #[classattr]
    pub fn ORANGE_BROWN() -> Color {
        Color { r: 0.7450980392156863, g: 0.39215686274509803, b: 0.0, a: 1.0 }
    }

    /// COBALT_BLUE
    #[classattr]
    pub fn COBALT_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.0392156862745098, b: 0.6549019607843137, a: 1.0 }
    }

    /// NEON_PINK
    #[classattr]
    pub fn NEON_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6039215686274509, a: 1.0 }
    }

    /// ROSE_PINK
    #[classattr]
    pub fn ROSE_PINK() -> Color {
        Color { r: 0.9686274509803922, g: 0.5294117647058824, b: 0.6039215686274509, a: 1.0 }
    }

    /// GREYISH_PURPLE
    #[classattr]
    pub fn GREYISH_PURPLE() -> Color {
        Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 }
    }

    /// RASPBERRY
    #[classattr]
    pub fn RASPBERRY() -> Color {
        Color { r: 0.6901960784313725, g: 0.00392156862745098, b: 0.28627450980392155, a: 1.0 }
    }

    /// AQUA_GREEN
    #[classattr]
    pub fn AQUA_GREEN() -> Color {
        Color { r: 0.07058823529411765, g: 0.8823529411764706, b: 0.5764705882352941, a: 1.0 }
    }

    /// SALMON_PINK
    #[classattr]
    pub fn SALMON_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.4823529411764706, b: 0.48627450980392156, a: 1.0 }
    }

    /// TANGERINE
    #[classattr]
    pub fn TANGERINE() -> Color {
        Color { r: 1.0, g: 0.5803921568627451, b: 0.03137254901960784, a: 1.0 }
    }

    /// BROWNISH_GREEN
    #[classattr]
    pub fn BROWNISH_GREEN() -> Color {
        Color { r: 0.41568627450980394, g: 0.43137254901960786, b: 0.03529411764705882, a: 1.0 }
    }

    /// RED_BROWN
    #[classattr]
    pub fn RED_BROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.1803921568627451, b: 0.08627450980392157, a: 1.0 }
    }

    /// GREENISH_BROWN
    #[classattr]
    pub fn GREENISH_BROWN() -> Color {
        Color { r: 0.4117647058823529, g: 0.3803921568627451, b: 0.07058823529411765, a: 1.0 }
    }

    /// PUMPKIN
    #[classattr]
    pub fn PUMPKIN() -> Color {
        Color { r: 0.8823529411764706, g: 0.4666666666666667, b: 0.00392156862745098, a: 1.0 }
    }

    /// PINE_GREEN
    #[classattr]
    pub fn PINE_GREEN() -> Color {
        Color { r: 0.0392156862745098, g: 0.2823529411764706, b: 0.11764705882352941, a: 1.0 }
    }

    /// CHARCOAL
    #[classattr]
    pub fn CHARCOAL() -> Color {
        Color { r: 0.20392156862745098, g: 0.2196078431372549, b: 0.21568627450980393, a: 1.0 }
    }

    /// BABY_PINK
    #[classattr]
    pub fn BABY_PINK() -> Color {
        Color { r: 1.0, g: 0.7176470588235294, b: 0.807843137254902, a: 1.0 }
    }

    /// CORNFLOWER
    #[classattr]
    pub fn CORNFLOWER() -> Color {
        Color { r: 0.41568627450980394, g: 0.4745098039215686, b: 0.9686274509803922, a: 1.0 }
    }

    /// BLUE_VIOLET
    #[classattr]
    pub fn BLUE_VIOLET() -> Color {
        Color { r: 0.36470588235294116, g: 0.023529411764705882, b: 0.9137254901960784, a: 1.0 }
    }

    /// CHOCOLATE
    #[classattr]
    pub fn CHOCOLATE() -> Color {
        Color { r: 0.23921568627450981, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 }
    }

    /// GREYISH_GREEN
    #[classattr]
    pub fn GREYISH_GREEN() -> Color {
        Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 }
    }

    /// SCARLET
    #[classattr]
    pub fn SCARLET() -> Color {
        Color { r: 0.7450980392156863, g: 0.00392156862745098, b: 0.09803921568627451, a: 1.0 }
    }

    /// DARK_OLIVE
    #[classattr]
    pub fn DARK_OLIVE() -> Color {
        Color { r: 0.21568627450980393, g: 0.24313725490196078, b: 0.00784313725490196, a: 1.0 }
    }

    /// SIENNA
    #[classattr]
    pub fn SIENNA() -> Color {
        Color { r: 0.6627450980392157, g: 0.33725490196078434, b: 0.11764705882352941, a: 1.0 }
    }

    /// PASTEL_PURPLE
    #[classattr]
    pub fn PASTEL_PURPLE() -> Color {
        Color { r: 0.792156862745098, g: 0.6274509803921569, b: 1.0, a: 1.0 }
    }

    /// TERRACOTTA
    #[classattr]
    pub fn TERRACOTTA() -> Color {
        Color { r: 0.792156862745098, g: 0.4, b: 0.2549019607843137, a: 1.0 }
    }

    /// AQUA_BLUE
    #[classattr]
    pub fn AQUA_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.8470588235294118, b: 0.9137254901960784, a: 1.0 }
    }

    /// SAGE_GREEN
    #[classattr]
    pub fn SAGE_GREEN() -> Color {
        Color { r: 0.5333333333333333, g: 0.7019607843137254, b: 0.47058823529411764, a: 1.0 }
    }

    /// BLOOD_RED
    #[classattr]
    pub fn BLOOD_RED() -> Color {
        Color { r: 0.596078431372549, g: 0.0, b: 0.00784313725490196, a: 1.0 }
    }

    /// DEEP_PINK
    #[classattr]
    pub fn DEEP_PINK() -> Color {
        Color { r: 0.796078431372549, g: 0.00392156862745098, b: 0.3843137254901961, a: 1.0 }
    }

    /// GRASS
    #[classattr]
    pub fn GRASS() -> Color {
        Color { r: 0.3607843137254902, g: 0.6745098039215687, b: 0.17647058823529413, a: 1.0 }
    }

    /// MOSS
    #[classattr]
    pub fn MOSS() -> Color {
        Color { r: 0.4627450980392157, g: 0.6, b: 0.34509803921568627, a: 1.0 }
    }

    /// PASTEL_BLUE
    #[classattr]
    pub fn PASTEL_BLUE() -> Color {
        Color { r: 0.6352941176470588, g: 0.7490196078431373, b: 0.996078431372549, a: 1.0 }
    }

    /// BLUISH_GREEN
    #[classattr]
    pub fn BLUISH_GREEN() -> Color {
        Color { r: 0.06274509803921569, g: 0.6509803921568628, b: 0.4549019607843137, a: 1.0 }
    }

    /// DARK_TAN
    #[classattr]
    pub fn DARK_TAN() -> Color {
        Color { r: 0.6862745098039216, g: 0.5333333333333333, b: 0.2901960784313726, a: 1.0 }
    }

    /// GREENISH_BLUE
    #[classattr]
    pub fn GREENISH_BLUE() -> Color {
        Color { r: 0.043137254901960784, g: 0.5450980392156862, b: 0.5294117647058824, a: 1.0 }
    }

    /// PALE_ORANGE
    #[classattr]
    pub fn PALE_ORANGE() -> Color {
        Color { r: 1.0, g: 0.6549019607843137, b: 0.33725490196078434, a: 1.0 }
    }

    /// VOMIT
    #[classattr]
    pub fn VOMIT() -> Color {
        Color { r: 0.6352941176470588, g: 0.6431372549019608, b: 0.08235294117647059, a: 1.0 }
    }

    /// FORREST_GREEN
    #[classattr]
    pub fn FORREST_GREEN() -> Color {
        Color { r: 0.08235294117647059, g: 0.26666666666666666, b: 0.023529411764705882, a: 1.0 }
    }

    /// DARK_LAVENDER
    #[classattr]
    pub fn DARK_LAVENDER() -> Color {
        Color { r: 0.5215686274509804, g: 0.403921568627451, b: 0.596078431372549, a: 1.0 }
    }

    /// DARK_VIOLET
    #[classattr]
    pub fn DARK_VIOLET() -> Color {
        Color { r: 0.20392156862745098, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// DARK_CYAN
    #[classattr]
    pub fn DARK_CYAN() -> Color {
        Color { r: 0.0392156862745098, g: 0.5333333333333333, b: 0.5411764705882353, a: 1.0 }
    }

    /// OLIVE_DRAB
    #[classattr]
    pub fn OLIVE_DRAB() -> Color {
        Color { r: 0.43529411764705883, g: 0.4627450980392157, b: 0.19607843137254902, a: 1.0 }
    }

    /// PINKISH
    #[classattr]
    pub fn PINKISH() -> Color {
        Color { r: 0.8313725490196079, g: 0.41568627450980394, b: 0.49411764705882355, a: 1.0 }
    }

    /// COBALT
    #[classattr]
    pub fn COBALT() -> Color {
        Color { r: 0.11764705882352941, g: 0.2823529411764706, b: 0.5607843137254902, a: 1.0 }
    }

    /// NEON_PURPLE
    #[classattr]
    pub fn NEON_PURPLE() -> Color {
        Color { r: 0.7372549019607844, g: 0.07450980392156863, b: 0.996078431372549, a: 1.0 }
    }

    /// LIGHT_TURQUOISE
    #[classattr]
    pub fn LIGHT_TURQUOISE() -> Color {
        Color { r: 0.49411764705882355, g: 0.9568627450980393, b: 0.8, a: 1.0 }
    }

    /// APPLE_GREEN
    #[classattr]
    pub fn APPLE_GREEN() -> Color {
        Color { r: 0.4627450980392157, g: 0.803921568627451, b: 0.14901960784313725, a: 1.0 }
    }

    /// DULL_GREEN
    #[classattr]
    pub fn DULL_GREEN() -> Color {
        Color { r: 0.4549019607843137, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 }
    }

    /// WINE
    #[classattr]
    pub fn WINE() -> Color {
        Color { r: 0.5019607843137255, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// POWDER_BLUE
    #[classattr]
    pub fn POWDER_BLUE() -> Color {
        Color { r: 0.6941176470588235, g: 0.8196078431372549, b: 0.9882352941176471, a: 1.0 }
    }

    /// OFF_WHITE
    #[classattr]
    pub fn OFF_WHITE() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8941176470588236, a: 1.0 }
    }

    /// ELECTRIC_BLUE
    #[classattr]
    pub fn ELECTRIC_BLUE() -> Color {
        Color { r: 0.023529411764705882, g: 0.3215686274509804, b: 1.0, a: 1.0 }
    }

    /// DARK_TURQUOISE
    #[classattr]
    pub fn DARK_TURQUOISE() -> Color {
        Color { r: 0.01568627450980392, g: 0.3607843137254902, b: 0.35294117647058826, a: 1.0 }
    }

    /// AZURE
    #[classattr]
    pub fn AZURE() -> Color {
        Color { r: 0.023529411764705882, g: 0.6039215686274509, b: 0.9529411764705882, a: 1.0 }
    }

    /// BRIGHT_RED
    #[classattr]
    pub fn BRIGHT_RED() -> Color {
        Color { r: 1.0, g: 0.0, b: 0.050980392156862744, a: 1.0 }
    }

    /// PINKISH_RED
    #[classattr]
    pub fn PINKISH_RED() -> Color {
        Color { r: 0.9450980392156862, g: 0.047058823529411764, b: 0.27058823529411763, a: 1.0 }
    }

    /// CORNFLOWER_BLUE
    #[classattr]
    pub fn CORNFLOWER_BLUE() -> Color {
        Color { r: 0.3176470588235294, g: 0.4392156862745098, b: 0.8431372549019608, a: 1.0 }
    }

    /// LIGHT_OLIVE
    #[classattr]
    pub fn LIGHT_OLIVE() -> Color {
        Color { r: 0.6745098039215687, g: 0.7490196078431373, b: 0.4117647058823529, a: 1.0 }
    }

    /// GRAPE
    #[classattr]
    pub fn GRAPE() -> Color {
        Color { r: 0.4235294117647059, g: 0.20392156862745098, b: 0.3803921568627451, a: 1.0 }
    }

    /// GREYISH_BLUE
    #[classattr]
    pub fn GREYISH_BLUE() -> Color {
        Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 }
    }

    /// PURPLISH_BLUE
    #[classattr]
    pub fn PURPLISH_BLUE() -> Color {
        Color { r: 0.3764705882352941, g: 0.11764705882352941, b: 0.9764705882352941, a: 1.0 }
    }

    /// YELLOWISH_GREEN
    #[classattr]
    pub fn YELLOWISH_GREEN() -> Color {
        Color { r: 0.6901960784313725, g: 0.8666666666666667, b: 0.08627450980392157, a: 1.0 }
    }

    /// GREENISH_YELLOW
    #[classattr]
    pub fn GREENISH_YELLOW() -> Color {
        Color { r: 0.803921568627451, g: 0.9921568627450981, b: 0.00784313725490196, a: 1.0 }
    }

    /// MEDIUM_BLUE
    #[classattr]
    pub fn MEDIUM_BLUE() -> Color {
        Color { r: 0.17254901960784313, g: 0.43529411764705883, b: 0.7333333333333333, a: 1.0 }
    }

    /// DUSTY_ROSE
    #[classattr]
    pub fn DUSTY_ROSE() -> Color {
        Color { r: 0.7529411764705882, g: 0.45098039215686275, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHT_VIOLET
    #[classattr]
    pub fn LIGHT_VIOLET() -> Color {
        Color { r: 0.8392156862745098, g: 0.7058823529411765, b: 0.9882352941176471, a: 1.0 }
    }

    /// MIDNIGHT_BLUE
    #[classattr]
    pub fn MIDNIGHT_BLUE() -> Color {
        Color { r: 0.00784313725490196, g: 0.0, b: 0.20784313725490197, a: 1.0 }
    }

    /// BLUISH_PURPLE
    #[classattr]
    pub fn BLUISH_PURPLE() -> Color {
        Color { r: 0.4392156862745098, g: 0.23137254901960785, b: 0.9058823529411765, a: 1.0 }
    }

    /// RED_ORANGE
    #[classattr]
    pub fn RED_ORANGE() -> Color {
        Color { r: 0.9921568627450981, g: 0.23529411764705882, b: 0.023529411764705882, a: 1.0 }
    }

    /// DARK_MAGENTA
    #[classattr]
    pub fn DARK_MAGENTA() -> Color {
        Color { r: 0.5882352941176471, g: 0.0, b: 0.33725490196078434, a: 1.0 }
    }

    /// GREENISH
    #[classattr]
    pub fn GREENISH() -> Color {
        Color { r: 0.25098039215686274, g: 0.6392156862745098, b: 0.40784313725490196, a: 1.0 }
    }

    /// OCEAN_BLUE
    #[classattr]
    pub fn OCEAN_BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.44313725490196076, b: 0.611764705882353, a: 1.0 }
    }

    /// CORAL
    #[classattr]
    pub fn CORAL() -> Color {
        Color { r: 0.9882352941176471, g: 0.35294117647058826, b: 0.3137254901960784, a: 1.0 }
    }

    /// CREAM
    #[classattr]
    pub fn CREAM() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.7607843137254902, a: 1.0 }
    }

    /// REDDISH_BROWN
    #[classattr]
    pub fn REDDISH_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.16862745098039217, b: 0.0392156862745098, a: 1.0 }
    }

    /// BURNT_SIENNA
    #[classattr]
    pub fn BURNT_SIENNA() -> Color {
        Color { r: 0.6901960784313725, g: 0.3058823529411765, b: 0.058823529411764705, a: 1.0 }
    }

    /// BRICK
    #[classattr]
    pub fn BRICK() -> Color {
        Color { r: 0.6274509803921569, g: 0.21176470588235294, b: 0.13725490196078433, a: 1.0 }
    }

    /// SAGE
    #[classattr]
    pub fn SAGE() -> Color {
        Color { r: 0.5294117647058824, g: 0.6823529411764706, b: 0.45098039215686275, a: 1.0 }
    }

    /// WHITE
    #[classattr]
    pub fn WHITE() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// ROBINS_EGG_BLUE
    #[classattr]
    pub fn ROBINS_EGG_BLUE() -> Color {
        Color { r: 0.596078431372549, g: 0.9372549019607843, b: 0.9764705882352941, a: 1.0 }
    }

    /// MOSS_GREEN
    #[classattr]
    pub fn MOSS_GREEN() -> Color {
        Color { r: 0.396078431372549, g: 0.5450980392156862, b: 0.2196078431372549, a: 1.0 }
    }

    /// STEEL_BLUE
    #[classattr]
    pub fn STEEL_BLUE() -> Color {
        Color { r: 0.35294117647058826, g: 0.49019607843137253, b: 0.6039215686274509, a: 1.0 }
    }

    /// EGGPLANT
    #[classattr]
    pub fn EGGPLANT() -> Color {
        Color { r: 0.2196078431372549, g: 0.03137254901960784, b: 0.20784313725490197, a: 1.0 }
    }

    /// LIGHT_YELLOW
    #[classattr]
    pub fn LIGHT_YELLOW() -> Color {
        Color { r: 1.0, g: 0.996078431372549, b: 0.47843137254901963, a: 1.0 }
    }

    /// LEAF_GREEN
    #[classattr]
    pub fn LEAF_GREEN() -> Color {
        Color { r: 0.3607843137254902, g: 0.6627450980392157, b: 0.01568627450980392, a: 1.0 }
    }

    /// LIGHT_GREY
    #[classattr]
    pub fn LIGHT_GREY() -> Color {
        Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 }
    }

    /// PUKE
    #[classattr]
    pub fn PUKE() -> Color {
        Color { r: 0.6470588235294118, g: 0.6470588235294118, b: 0.00784313725490196, a: 1.0 }
    }

    /// PINKISH_PURPLE
    #[classattr]
    pub fn PINKISH_PURPLE() -> Color {
        Color { r: 0.8392156862745098, g: 0.2823529411764706, b: 0.8431372549019608, a: 1.0 }
    }

    /// SEA_BLUE
    #[classattr]
    pub fn SEA_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.4549019607843137, b: 0.5843137254901961, a: 1.0 }
    }

    /// PALE_PURPLE
    #[classattr]
    pub fn PALE_PURPLE() -> Color {
        Color { r: 0.7176470588235294, g: 0.5647058823529412, b: 0.8313725490196079, a: 1.0 }
    }

    /// SLATE_BLUE
    #[classattr]
    pub fn SLATE_BLUE() -> Color {
        Color { r: 0.3568627450980392, g: 0.48627450980392156, b: 0.6, a: 1.0 }
    }

    /// HUNTER_GREEN
    #[classattr]
    pub fn HUNTER_GREEN() -> Color {
        Color { r: 0.043137254901960784, g: 0.25098039215686274, b: 0.03137254901960784, a: 1.0 }
    }

    /// FUCHSIA
    #[classattr]
    pub fn FUCHSIA() -> Color {
        Color { r: 0.9294117647058824, g: 0.050980392156862744, b: 0.8509803921568627, a: 1.0 }
    }

    /// CRIMSON
    #[classattr]
    pub fn CRIMSON() -> Color {
        Color { r: 0.5490196078431373, g: 0.0, b: 0.058823529411764705, a: 1.0 }
    }

    /// PALE_YELLOW
    #[classattr]
    pub fn PALE_YELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.5176470588235295, a: 1.0 }
    }

    /// OCHRE
    #[classattr]
    pub fn OCHRE() -> Color {
        Color { r: 0.7490196078431373, g: 0.5647058823529412, b: 0.0196078431372549, a: 1.0 }
    }

    /// MUSTARD_YELLOW
    #[classattr]
    pub fn MUSTARD_YELLOW() -> Color {
        Color { r: 0.8235294117647058, g: 0.7411764705882353, b: 0.0392156862745098, a: 1.0 }
    }

    /// LIGHT_RED
    #[classattr]
    pub fn LIGHT_RED() -> Color {
        Color { r: 1.0, g: 0.2784313725490196, b: 0.2980392156862745, a: 1.0 }
    }

    /// CERULEAN
    #[classattr]
    pub fn CERULEAN() -> Color {
        Color { r: 0.01568627450980392, g: 0.5215686274509804, b: 0.8196078431372549, a: 1.0 }
    }

    /// PALE_PINK
    #[classattr]
    pub fn PALE_PINK() -> Color {
        Color { r: 1.0, g: 0.8117647058823529, b: 0.8627450980392157, a: 1.0 }
    }

    /// DEEP_BLUE
    #[classattr]
    pub fn DEEP_BLUE() -> Color {
        Color { r: 0.01568627450980392, g: 0.00784313725490196, b: 0.45098039215686275, a: 1.0 }
    }

    /// RUST
    #[classattr]
    pub fn RUST() -> Color {
        Color { r: 0.6588235294117647, g: 0.23529411764705882, b: 0.03529411764705882, a: 1.0 }
    }

    /// LIGHT_TEAL
    #[classattr]
    pub fn LIGHT_TEAL() -> Color {
        Color { r: 0.5647058823529412, g: 0.8941176470588236, b: 0.7568627450980392, a: 1.0 }
    }

    /// SLATE
    #[classattr]
    pub fn SLATE() -> Color {
        Color { r: 0.3176470588235294, g: 0.396078431372549, b: 0.4470588235294118, a: 1.0 }
    }

    /// GOLDENROD
    #[classattr]
    pub fn GOLDENROD() -> Color {
        Color { r: 0.9803921568627451, g: 0.7607843137254902, b: 0.0196078431372549, a: 1.0 }
    }

    /// DARK_YELLOW
    #[classattr]
    pub fn DARK_YELLOW() -> Color {
        Color { r: 0.8352941176470589, g: 0.7137254901960784, b: 0.0392156862745098, a: 1.0 }
    }

    /// DARK_GREY
    #[classattr]
    pub fn DARK_GREY() -> Color {
        Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 }
    }

    /// ARMY_GREEN
    #[classattr]
    pub fn ARMY_GREEN() -> Color {
        Color { r: 0.29411764705882354, g: 0.36470588235294116, b: 0.08627450980392157, a: 1.0 }
    }

    /// SEAFOAM
    #[classattr]
    pub fn SEAFOAM() -> Color {
        Color { r: 0.5019607843137255, g: 0.9764705882352941, b: 0.6784313725490196, a: 1.0 }
    }

    /// PUCE
    #[classattr]
    pub fn PUCE() -> Color {
        Color { r: 0.6470588235294118, g: 0.49411764705882355, b: 0.3215686274509804, a: 1.0 }
    }

    /// SPRING_GREEN
    #[classattr]
    pub fn SPRING_GREEN() -> Color {
        Color { r: 0.6627450980392157, g: 0.9764705882352941, b: 0.44313725490196076, a: 1.0 }
    }

    /// DARK_ORANGE
    #[classattr]
    pub fn DARK_ORANGE() -> Color {
        Color { r: 0.7764705882352941, g: 0.3176470588235294, b: 0.00784313725490196, a: 1.0 }
    }

    /// SAND
    #[classattr]
    pub fn SAND() -> Color {
        Color { r: 0.8862745098039215, g: 0.792156862745098, b: 0.4627450980392157, a: 1.0 }
    }

    /// PASTEL_GREEN
    #[classattr]
    pub fn PASTEL_GREEN() -> Color {
        Color { r: 0.6901960784313725, g: 1.0, b: 0.615686274509804, a: 1.0 }
    }

    /// MINT
    #[classattr]
    pub fn MINT() -> Color {
        Color { r: 0.6235294117647059, g: 0.996078431372549, b: 0.6901960784313725, a: 1.0 }
    }

    /// LIGHT_ORANGE
    #[classattr]
    pub fn LIGHT_ORANGE() -> Color {
        Color { r: 0.9921568627450981, g: 0.6666666666666666, b: 0.2823529411764706, a: 1.0 }
    }

    /// BRIGHT_PINK
    #[classattr]
    pub fn BRIGHT_PINK() -> Color {
        Color { r: 0.996078431372549, g: 0.00392156862745098, b: 0.6941176470588235, a: 1.0 }
    }

    /// CHARTREUSE
    #[classattr]
    pub fn CHARTREUSE() -> Color {
        Color { r: 0.7568627450980392, g: 0.9725490196078431, b: 0.0392156862745098, a: 1.0 }
    }

    /// DEEP_PURPLE
    #[classattr]
    pub fn DEEP_PURPLE() -> Color {
        Color { r: 0.21176470588235294, g: 0.00392156862745098, b: 0.24705882352941178, a: 1.0 }
    }

    /// DARK_BROWN
    #[classattr]
    pub fn DARK_BROWN() -> Color {
        Color { r: 0.20392156862745098, g: 0.10980392156862745, b: 0.00784313725490196, a: 1.0 }
    }

    /// TAUPE
    #[classattr]
    pub fn TAUPE() -> Color {
        Color { r: 0.7254901960784313, g: 0.6352941176470588, b: 0.5058823529411764, a: 1.0 }
    }

    /// PEA_GREEN
    #[classattr]
    pub fn PEA_GREEN() -> Color {
        Color { r: 0.5568627450980392, g: 0.6705882352941176, b: 0.07058823529411765, a: 1.0 }
    }

    /// PUKE_GREEN
    #[classattr]
    pub fn PUKE_GREEN() -> Color {
        Color { r: 0.6039215686274509, g: 0.6823529411764706, b: 0.027450980392156862, a: 1.0 }
    }

    /// KELLY_GREEN
    #[classattr]
    pub fn KELLY_GREEN() -> Color {
        Color { r: 0.00784313725490196, g: 0.6705882352941176, b: 0.1803921568627451, a: 1.0 }
    }

    /// SEAFOAM_GREEN
    #[classattr]
    pub fn SEAFOAM_GREEN() -> Color {
        Color { r: 0.47843137254901963, g: 0.9764705882352941, b: 0.6705882352941176, a: 1.0 }
    }

    /// KHAKI
    #[classattr]
    pub fn KHAKI() -> Color {
        Color { r: 0.6666666666666666, g: 0.6509803921568628, b: 0.3843137254901961, a: 1.0 }
    }

    /// BURGUNDY
    #[classattr]
    pub fn BURGUNDY() -> Color {
        Color { r: 0.3803921568627451, g: 0.0, b: 0.13725490196078433, a: 1.0 }
    }

    /// DARK_TEAL
    #[classattr]
    pub fn DARK_TEAL() -> Color {
        Color { r: 0.00392156862745098, g: 0.30196078431372547, b: 0.3058823529411765, a: 1.0 }
    }

    /// BRICK_RED
    #[classattr]
    pub fn BRICK_RED() -> Color {
        Color { r: 0.5607843137254902, g: 0.0784313725490196, b: 0.00784313725490196, a: 1.0 }
    }

    /// ROYAL_PURPLE
    #[classattr]
    pub fn ROYAL_PURPLE() -> Color {
        Color { r: 0.29411764705882354, g: 0.0, b: 0.43137254901960786, a: 1.0 }
    }

    /// PLUM
    #[classattr]
    pub fn PLUM() -> Color {
        Color { r: 0.34509803921568627, g: 0.058823529411764705, b: 0.2549019607843137, a: 1.0 }
    }

    /// MINT_GREEN
    #[classattr]
    pub fn MINT_GREEN() -> Color {
        Color { r: 0.5607843137254902, g: 1.0, b: 0.6235294117647059, a: 1.0 }
    }

    /// GOLD
    #[classattr]
    pub fn GOLD() -> Color {
        Color { r: 0.8588235294117647, g: 0.7058823529411765, b: 0.047058823529411764, a: 1.0 }
    }

    /// BABY_BLUE
    #[classattr]
    pub fn BABY_BLUE() -> Color {
        Color { r: 0.6352941176470588, g: 0.8117647058823529, b: 0.996078431372549, a: 1.0 }
    }

    /// BRIGHT_PURPLE
    #[classattr]
    pub fn BRIGHT_PURPLE() -> Color {
        Color { r: 0.7450980392156863, g: 0.011764705882352941, b: 0.9921568627450981, a: 1.0 }
    }

    /// DARK_RED
    #[classattr]
    pub fn DARK_RED() -> Color {
        Color { r: 0.5176470588235295, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// PALE_BLUE
    #[classattr]
    pub fn PALE_BLUE() -> Color {
        Color { r: 0.8156862745098039, g: 0.996078431372549, b: 0.996078431372549, a: 1.0 }
    }

    /// GRASS_GREEN
    #[classattr]
    pub fn GRASS_GREEN() -> Color {
        Color { r: 0.24705882352941178, g: 0.6078431372549019, b: 0.043137254901960784, a: 1.0 }
    }

    /// NAVY
    #[classattr]
    pub fn NAVY() -> Color {
        Color { r: 0.00392156862745098, g: 0.08235294117647059, b: 0.24313725490196078, a: 1.0 }
    }

    /// AQUAMARINE
    #[classattr]
    pub fn AQUAMARINE() -> Color {
        Color { r: 0.01568627450980392, g: 0.8470588235294118, b: 0.6980392156862745, a: 1.0 }
    }

    /// BURNT_ORANGE
    #[classattr]
    pub fn BURNT_ORANGE() -> Color {
        Color { r: 0.7529411764705882, g: 0.3058823529411765, b: 0.00392156862745098, a: 1.0 }
    }

    /// NEON_GREEN
    #[classattr]
    pub fn NEON_GREEN() -> Color {
        Color { r: 0.047058823529411764, g: 1.0, b: 0.047058823529411764, a: 1.0 }
    }

    /// BRIGHT_BLUE
    #[classattr]
    pub fn BRIGHT_BLUE() -> Color {
        Color { r: 0.00392156862745098, g: 0.396078431372549, b: 0.9882352941176471, a: 1.0 }
    }

    /// ROSE
    #[classattr]
    pub fn ROSE() -> Color {
        Color { r: 0.8117647058823529, g: 0.3843137254901961, b: 0.4588235294117647, a: 1.0 }
    }

    /// LIGHT_PINK
    #[classattr]
    pub fn LIGHT_PINK() -> Color {
        Color { r: 1.0, g: 0.8196078431372549, b: 0.8745098039215686, a: 1.0 }
    }

    /// MUSTARD
    #[classattr]
    pub fn MUSTARD() -> Color {
        Color { r: 0.807843137254902, g: 0.7019607843137254, b: 0.00392156862745098, a: 1.0 }
    }

    /// INDIGO
    #[classattr]
    pub fn INDIGO() -> Color {
        Color { r: 0.2196078431372549, g: 0.00784313725490196, b: 0.5098039215686274, a: 1.0 }
    }

    /// LIME
    #[classattr]
    pub fn LIME() -> Color {
        Color { r: 0.6666666666666666, g: 1.0, b: 0.19607843137254902, a: 1.0 }
    }

    /// SEA_GREEN
    #[classattr]
    pub fn SEA_GREEN() -> Color {
        Color { r: 0.3254901960784314, g: 0.9882352941176471, b: 0.6313725490196078, a: 1.0 }
    }

    /// PERIWINKLE
    #[classattr]
    pub fn PERIWINKLE() -> Color {
        Color { r: 0.5568627450980392, g: 0.5098039215686274, b: 0.996078431372549, a: 1.0 }
    }

    /// DARK_PINK
    #[classattr]
    pub fn DARK_PINK() -> Color {
        Color { r: 0.796078431372549, g: 0.2549019607843137, b: 0.4196078431372549, a: 1.0 }
    }

    /// OLIVE_GREEN
    #[classattr]
    pub fn OLIVE_GREEN() -> Color {
        Color { r: 0.403921568627451, g: 0.47843137254901963, b: 0.01568627450980392, a: 1.0 }
    }

    /// PEACH
    #[classattr]
    pub fn PEACH() -> Color {
        Color { r: 1.0, g: 0.6901960784313725, b: 0.48627450980392156, a: 1.0 }
    }

    /// PALE_GREEN
    #[classattr]
    pub fn PALE_GREEN() -> Color {
        Color { r: 0.7803921568627451, g: 0.9921568627450981, b: 0.7098039215686275, a: 1.0 }
    }

    /// LIGHT_BROWN
    #[classattr]
    pub fn LIGHT_BROWN() -> Color {
        Color { r: 0.6784313725490196, g: 0.5058823529411764, b: 0.3137254901960784, a: 1.0 }
    }

    /// HOT_PINK
    #[classattr]
    pub fn HOT_PINK() -> Color {
        Color { r: 1.0, g: 0.00784313725490196, b: 0.5529411764705883, a: 1.0 }
    }

    /// BLACK
    #[classattr]
    pub fn BLACK() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// LILAC
    #[classattr]
    pub fn LILAC() -> Color {
        Color { r: 0.807843137254902, g: 0.6352941176470588, b: 0.9921568627450981, a: 1.0 }
    }

    /// NAVY_BLUE
    #[classattr]
    pub fn NAVY_BLUE() -> Color {
        Color { r: 0.0, g: 0.06666666666666667, b: 0.27450980392156865, a: 1.0 }
    }

    /// ROYAL_BLUE
    #[classattr]
    pub fn ROYAL_BLUE() -> Color {
        Color { r: 0.0196078431372549, g: 0.01568627450980392, b: 0.6666666666666666, a: 1.0 }
    }

    /// BEIGE
    #[classattr]
    pub fn BEIGE() -> Color {
        Color { r: 0.9019607843137255, g: 0.8549019607843137, b: 0.6509803921568628, a: 1.0 }
    }

    /// SALMON
    #[classattr]
    pub fn SALMON() -> Color {
        Color { r: 1.0, g: 0.4745098039215686, b: 0.4235294117647059, a: 1.0 }
    }

    /// OLIVE
    #[classattr]
    pub fn OLIVE() -> Color {
        Color { r: 0.43137254901960786, g: 0.4588235294117647, b: 0.054901960784313725, a: 1.0 }
    }

    /// MAROON
    #[classattr]
    pub fn MAROON() -> Color {
        Color { r: 0.396078431372549, g: 0.0, b: 0.12941176470588237, a: 1.0 }
    }

    /// BRIGHT_GREEN
    #[classattr]
    pub fn BRIGHT_GREEN() -> Color {
        Color { r: 0.00392156862745098, g: 1.0, b: 0.027450980392156862, a: 1.0 }
    }

    /// DARK_PURPLE
    #[classattr]
    pub fn DARK_PURPLE() -> Color {
        Color { r: 0.20784313725490197, g: 0.023529411764705882, b: 0.24313725490196078, a: 1.0 }
    }

    /// MAUVE
    #[classattr]
    pub fn MAUVE() -> Color {
        Color { r: 0.6823529411764706, g: 0.44313725490196076, b: 0.5058823529411764, a: 1.0 }
    }

    /// FOREST_GREEN
    #[classattr]
    pub fn FOREST_GREEN() -> Color {
        Color { r: 0.023529411764705882, g: 0.2784313725490196, b: 0.047058823529411764, a: 1.0 }
    }

    /// AQUA
    #[classattr]
    pub fn AQUA() -> Color {
        Color { r: 0.07450980392156863, g: 0.9176470588235294, b: 0.788235294117647, a: 1.0 }
    }

    /// CYAN
    #[classattr]
    pub fn CYAN() -> Color {
        Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// TAN
    #[classattr]
    pub fn TAN() -> Color {
        Color { r: 0.8196078431372549, g: 0.6980392156862745, b: 0.43529411764705883, a: 1.0 }
    }

    /// DARK_BLUE
    #[classattr]
    pub fn DARK_BLUE() -> Color {
        Color { r: 0.0, g: 0.011764705882352941, b: 0.3568627450980392, a: 1.0 }
    }

    /// LAVENDER
    #[classattr]
    pub fn LAVENDER() -> Color {
        Color { r: 0.7803921568627451, g: 0.6235294117647059, b: 0.9372549019607843, a: 1.0 }
    }

    /// TURQUOISE
    #[classattr]
    pub fn TURQUOISE() -> Color {
        Color { r: 0.023529411764705882, g: 0.7607843137254902, b: 0.6745098039215687, a: 1.0 }
    }

    /// DARK_GREEN
    #[classattr]
    pub fn DARK_GREEN() -> Color {
        Color { r: 0.011764705882352941, g: 0.20784313725490197, b: 0.0, a: 1.0 }
    }

    /// VIOLET
    #[classattr]
    pub fn VIOLET() -> Color {
        Color { r: 0.6039215686274509, g: 0.054901960784313725, b: 0.9176470588235294, a: 1.0 }
    }

    /// LIGHT_PURPLE
    #[classattr]
    pub fn LIGHT_PURPLE() -> Color {
        Color { r: 0.7490196078431373, g: 0.4666666666666667, b: 0.9647058823529412, a: 1.0 }
    }

    /// LIME_GREEN
    #[classattr]
    pub fn LIME_GREEN() -> Color {
        Color { r: 0.5372549019607843, g: 0.996078431372549, b: 0.0196078431372549, a: 1.0 }
    }

    /// GREY
    #[classattr]
    pub fn GREY() -> Color {
        Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 }
    }

    /// SKY_BLUE
    #[classattr]
    pub fn SKY_BLUE() -> Color {
        Color { r: 0.4588235294117647, g: 0.7333333333333333, b: 0.9921568627450981, a: 1.0 }
    }

    /// YELLOW
    #[classattr]
    pub fn YELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.0784313725490196, a: 1.0 }
    }

    /// MAGENTA
    #[classattr]
    pub fn MAGENTA() -> Color {
        Color { r: 0.7607843137254902, g: 0.0, b: 0.47058823529411764, a: 1.0 }
    }

    /// LIGHT_GREEN
    #[classattr]
    pub fn LIGHT_GREEN() -> Color {
        Color { r: 0.5882352941176471, g: 0.9764705882352941, b: 0.4823529411764706, a: 1.0 }
    }

    /// ORANGE
    #[classattr]
    pub fn ORANGE() -> Color {
        Color { r: 0.9764705882352941, g: 0.45098039215686275, b: 0.023529411764705882, a: 1.0 }
    }

    /// TEAL
    #[classattr]
    pub fn TEAL() -> Color {
        Color { r: 0.00784313725490196, g: 0.5764705882352941, b: 0.5254901960784314, a: 1.0 }
    }

    /// LIGHT_BLUE
    #[classattr]
    pub fn LIGHT_BLUE() -> Color {
        Color { r: 0.5843137254901961, g: 0.8156862745098039, b: 0.9882352941176471, a: 1.0 }
    }

    /// RED
    #[classattr]
    pub fn RED() -> Color {
        Color { r: 0.8980392156862745, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// BROWN
    #[classattr]
    pub fn BROWN() -> Color {
        Color { r: 0.396078431372549, g: 0.21568627450980393, b: 0.0, a: 1.0 }
    }

    /// PINK
    #[classattr]
    pub fn PINK() -> Color {
        Color { r: 1.0, g: 0.5058823529411764, b: 0.7529411764705882, a: 1.0 }
    }

    /// BLUE
    #[classattr]
    pub fn BLUE() -> Color {
        Color { r: 0.011764705882352941, g: 0.2627450980392157, b: 0.8745098039215686, a: 1.0 }
    }

    /// GREEN
    #[classattr]
    pub fn GREEN() -> Color {
        Color { r: 0.08235294117647059, g: 0.6901960784313725, b: 0.10196078431372549, a: 1.0 }
    }

    /// PURPLE
    #[classattr]
    pub fn PURPLE() -> Color {
        Color { r: 0.49411764705882355, g: 0.11764705882352941, b: 0.611764705882353, a: 1.0 }
    }

    /// GRAY_TEAL
    #[classattr]
    pub fn GRAY_TEAL() -> Color {
        Color { r: 0.3686274509803922, g: 0.6078431372549019, b: 0.5411764705882353, a: 1.0 }
    }

    /// PURPLEY_GRAY
    #[classattr]
    pub fn PURPLEY_GRAY() -> Color {
        Color { r: 0.5803921568627451, g: 0.49411764705882355, b: 0.5803921568627451, a: 1.0 }
    }

    /// LIGHT_GRAY_GREEN
    #[classattr]
    pub fn LIGHT_GRAY_GREEN() -> Color {
        Color { r: 0.7176470588235294, g: 0.8823529411764706, b: 0.6313725490196078, a: 1.0 }
    }

    /// REDDISH_GRAY
    #[classattr]
    pub fn REDDISH_GRAY() -> Color {
        Color { r: 0.6, g: 0.4588235294117647, b: 0.4392156862745098, a: 1.0 }
    }

    /// BATTLESHIP_GRAY
    #[classattr]
    pub fn BATTLESHIP_GRAY() -> Color {
        Color { r: 0.4196078431372549, g: 0.48627450980392156, b: 0.5215686274509804, a: 1.0 }
    }

    /// CHARCOAL_GRAY
    #[classattr]
    pub fn CHARCOAL_GRAY() -> Color {
        Color { r: 0.23529411764705882, g: 0.2549019607843137, b: 0.25882352941176473, a: 1.0 }
    }

    /// GRAYISH_TEAL
    #[classattr]
    pub fn GRAYISH_TEAL() -> Color {
        Color { r: 0.44313725490196076, g: 0.6235294117647059, b: 0.5686274509803921, a: 1.0 }
    }

    /// GRAY_GREEN
    #[classattr]
    pub fn GRAY_GREEN() -> Color {
        Color { r: 0.5254901960784314, g: 0.6313725490196078, b: 0.49019607843137253, a: 1.0 }
    }

    /// COOL_GRAY
    #[classattr]
    pub fn COOL_GRAY() -> Color {
        Color { r: 0.5843137254901961, g: 0.6392156862745098, b: 0.6509803921568628, a: 1.0 }
    }

    /// DARK_BLUE_GRAY
    #[classattr]
    pub fn DARK_BLUE_GRAY() -> Color {
        Color { r: 0.12156862745098039, g: 0.23137254901960785, b: 0.30196078431372547, a: 1.0 }
    }

    /// BLUEY_GRAY
    #[classattr]
    pub fn BLUEY_GRAY() -> Color {
        Color { r: 0.5372549019607843, g: 0.6274509803921569, b: 0.6901960784313725, a: 1.0 }
    }

    /// GREENY_GRAY
    #[classattr]
    pub fn GREENY_GRAY() -> Color {
        Color { r: 0.49411764705882355, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// BLUEGRAY
    #[classattr]
    pub fn BLUEGRAY() -> Color {
        Color { r: 0.5215686274509804, g: 0.6392156862745098, b: 0.6980392156862745, a: 1.0 }
    }

    /// LIGHT_BLUE_GRAY
    #[classattr]
    pub fn LIGHT_BLUE_GRAY() -> Color {
        Color { r: 0.7176470588235294, g: 0.788235294117647, b: 0.8862745098039215, a: 1.0 }
    }

    /// GRAY_BLUE
    #[classattr]
    pub fn GRAY_BLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.49019607843137253, b: 0.5568627450980392, a: 1.0 }
    }

    /// BROWN_GRAY
    #[classattr]
    pub fn BROWN_GRAY() -> Color {
        Color { r: 0.5529411764705883, g: 0.5176470588235295, b: 0.40784313725490196, a: 1.0 }
    }

    /// BLUE_GRAY
    #[classattr]
    pub fn BLUE_GRAY() -> Color {
        Color { r: 0.4588235294117647, g: 0.5529411764705883, b: 0.6392156862745098, a: 1.0 }
    }

    /// GRAYBLUE
    #[classattr]
    pub fn GRAYBLUE() -> Color {
        Color { r: 0.4666666666666667, g: 0.6313725490196078, b: 0.7098039215686275, a: 1.0 }
    }

    /// DARK_GRAY_BLUE
    #[classattr]
    pub fn DARK_GRAY_BLUE() -> Color {
        Color { r: 0.1607843137254902, g: 0.27450980392156865, b: 0.3568627450980392, a: 1.0 }
    }

    /// GRAYISH
    #[classattr]
    pub fn GRAYISH() -> Color {
        Color { r: 0.6588235294117647, g: 0.6431372549019608, b: 0.5843137254901961, a: 1.0 }
    }

    /// LIGHT_GRAY_BLUE
    #[classattr]
    pub fn LIGHT_GRAY_BLUE() -> Color {
        Color { r: 0.615686274509804, g: 0.7372549019607844, b: 0.8313725490196079, a: 1.0 }
    }

    /// PALE_GRAY
    #[classattr]
    pub fn PALE_GRAY() -> Color {
        Color { r: 0.9921568627450981, g: 0.9921568627450981, b: 0.996078431372549, a: 1.0 }
    }

    /// WARM_GRAY
    #[classattr]
    pub fn WARM_GRAY() -> Color {
        Color { r: 0.592156862745098, g: 0.5411764705882353, b: 0.5176470588235295, a: 1.0 }
    }

    /// GRAY_PINK
    #[classattr]
    pub fn GRAY_PINK() -> Color {
        Color { r: 0.7647058823529411, g: 0.5647058823529412, b: 0.6078431372549019, a: 1.0 }
    }

    /// MEDIUM_GRAY
    #[classattr]
    pub fn MEDIUM_GRAY() -> Color {
        Color { r: 0.49019607843137253, g: 0.4980392156862745, b: 0.48627450980392156, a: 1.0 }
    }

    /// PINKISH_GRAY
    #[classattr]
    pub fn PINKISH_GRAY() -> Color {
        Color { r: 0.7843137254901961, g: 0.6745098039215687, b: 0.6627450980392157, a: 1.0 }
    }

    /// BROWNISH_GRAY
    #[classattr]
    pub fn BROWNISH_GRAY() -> Color {
        Color { r: 0.5254901960784314, g: 0.4666666666666667, b: 0.37254901960784315, a: 1.0 }
    }

    /// PURPLISH_GRAY
    #[classattr]
    pub fn PURPLISH_GRAY() -> Color {
        Color { r: 0.47843137254901963, g: 0.40784313725490196, b: 0.4980392156862745, a: 1.0 }
    }

    /// GRAYISH_PINK
    #[classattr]
    pub fn GRAYISH_PINK() -> Color {
        Color { r: 0.7843137254901961, g: 0.5529411764705883, b: 0.5803921568627451, a: 1.0 }
    }

    /// GRAYISH_BROWN
    #[classattr]
    pub fn GRAYISH_BROWN() -> Color {
        Color { r: 0.47843137254901963, g: 0.41568627450980394, b: 0.30980392156862746, a: 1.0 }
    }

    /// STEEL_GRAY
    #[classattr]
    pub fn STEEL_GRAY() -> Color {
        Color { r: 0.43529411764705883, g: 0.5098039215686274, b: 0.5411764705882353, a: 1.0 }
    }

    /// PURPLE_GRAY
    #[classattr]
    pub fn PURPLE_GRAY() -> Color {
        Color { r: 0.5254901960784314, g: 0.43529411764705883, b: 0.5215686274509804, a: 1.0 }
    }

    /// GRAY_BROWN
    #[classattr]
    pub fn GRAY_BROWN() -> Color {
        Color { r: 0.4980392156862745, g: 0.4392156862745098, b: 0.3254901960784314, a: 1.0 }
    }

    /// GREEN_GRAY
    #[classattr]
    pub fn GREEN_GRAY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5725490196078431, b: 0.43529411764705883, a: 1.0 }
    }

    /// BLUISH_GRAY
    #[classattr]
    pub fn BLUISH_GRAY() -> Color {
        Color { r: 0.4549019607843137, g: 0.5450980392156862, b: 0.592156862745098, a: 1.0 }
    }

    /// SLATE_GRAY
    #[classattr]
    pub fn SLATE_GRAY() -> Color {
        Color { r: 0.34901960784313724, g: 0.396078431372549, b: 0.42745098039215684, a: 1.0 }
    }

    /// GRAY_PURPLE
    #[classattr]
    pub fn GRAY_PURPLE() -> Color {
        Color { r: 0.5098039215686274, g: 0.42745098039215684, b: 0.5490196078431373, a: 1.0 }
    }

    /// GREENISH_GRAY
    #[classattr]
    pub fn GREENISH_GRAY() -> Color {
        Color { r: 0.5882352941176471, g: 0.6823529411764706, b: 0.5529411764705883, a: 1.0 }
    }

    /// GRAYISH_PURPLE
    #[classattr]
    pub fn GRAYISH_PURPLE() -> Color {
        Color { r: 0.5333333333333333, g: 0.44313725490196076, b: 0.5686274509803921, a: 1.0 }
    }

    /// GRAYISH_GREEN
    #[classattr]
    pub fn GRAYISH_GREEN() -> Color {
        Color { r: 0.5098039215686274, g: 0.6509803921568628, b: 0.49019607843137253, a: 1.0 }
    }

    /// GRAYISH_BLUE
    #[classattr]
    pub fn GRAYISH_BLUE() -> Color {
        Color { r: 0.3686274509803922, g: 0.5058823529411764, b: 0.615686274509804, a: 1.0 }
    }

    /// LIGHT_GRAY
    #[classattr]
    pub fn LIGHT_GRAY() -> Color {
        Color { r: 0.8470588235294118, g: 0.8627450980392157, b: 0.8392156862745098, a: 1.0 }
    }

    /// DARK_GRAY
    #[classattr]
    pub fn DARK_GRAY() -> Color {
        Color { r: 0.21176470588235294, g: 0.21568627450980393, b: 0.21568627450980393, a: 1.0 }
    }

    /// GRAY
    #[classattr]
    pub fn GRAY() -> Color {
        Color { r: 0.5725490196078431, g: 0.5843137254901961, b: 0.5686274509803921, a: 1.0 }
    }

    /// ALICEBLUE
    #[classattr]
    pub fn ALICEBLUE() -> Color {
        Color { r: 0.9411764705882353, g: 0.9725490196078431, b: 1.0, a: 1.0 }
    }

    /// ANTIQUEWHITE
    #[classattr]
    pub fn ANTIQUEWHITE() -> Color {
        Color { r: 0.9803921568627451, g: 0.9215686274509803, b: 0.8431372549019608, a: 1.0 }
    }

    /// BISQUE
    #[classattr]
    pub fn BISQUE() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.7686274509803922, a: 1.0 }
    }

    /// BLANCHEDALMOND
    #[classattr]
    pub fn BLANCHEDALMOND() -> Color {
        Color { r: 1.0, g: 0.9215686274509803, b: 0.803921568627451, a: 1.0 }
    }

    /// BLUEVIOLET
    #[classattr]
    pub fn BLUEVIOLET() -> Color {
        Color { r: 0.5411764705882353, g: 0.16862745098039217, b: 0.8862745098039215, a: 1.0 }
    }

    /// BURLYWOOD
    #[classattr]
    pub fn BURLYWOOD() -> Color {
        Color { r: 0.8705882352941177, g: 0.7215686274509804, b: 0.5294117647058824, a: 1.0 }
    }

    /// CADETBLUE
    #[classattr]
    pub fn CADETBLUE() -> Color {
        Color { r: 0.37254901960784315, g: 0.6196078431372549, b: 0.6274509803921569, a: 1.0 }
    }

    /// CORNFLOWERBLUE
    #[classattr]
    pub fn CORNFLOWERBLUE() -> Color {
        Color { r: 0.39215686274509803, g: 0.5843137254901961, b: 0.9294117647058824, a: 1.0 }
    }

    /// CORNSILK
    #[classattr]
    pub fn CORNSILK() -> Color {
        Color { r: 1.0, g: 0.9725490196078431, b: 0.8627450980392157, a: 1.0 }
    }

    /// DARKCYAN
    #[classattr]
    pub fn DARKCYAN() -> Color {
        Color { r: 0.0, g: 0.5450980392156862, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKGOLDENROD
    #[classattr]
    pub fn DARKGOLDENROD() -> Color {
        Color { r: 0.7215686274509804, g: 0.5254901960784314, b: 0.043137254901960784, a: 1.0 }
    }

    /// DARKGRAY
    #[classattr]
    pub fn DARKGRAY() -> Color {
        Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 }
    }

    /// DARKGREY
    #[classattr]
    pub fn DARKGREY() -> Color {
        Color { r: 0.6627450980392157, g: 0.6627450980392157, b: 0.6627450980392157, a: 1.0 }
    }

    /// DARKKHAKI
    #[classattr]
    pub fn DARKKHAKI() -> Color {
        Color { r: 0.7411764705882353, g: 0.7176470588235294, b: 0.4196078431372549, a: 1.0 }
    }

    /// DARKMAGENTA
    #[classattr]
    pub fn DARKMAGENTA() -> Color {
        Color { r: 0.5450980392156862, g: 0.0, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKOLIVEGREEN
    #[classattr]
    pub fn DARKOLIVEGREEN() -> Color {
        Color { r: 0.3333333333333333, g: 0.4196078431372549, b: 0.1843137254901961, a: 1.0 }
    }

    /// DARKORANGE
    #[classattr]
    pub fn DARKORANGE() -> Color {
        Color { r: 1.0, g: 0.5490196078431373, b: 0.0, a: 1.0 }
    }

    /// DARKORCHID
    #[classattr]
    pub fn DARKORCHID() -> Color {
        Color { r: 0.6, g: 0.19607843137254902, b: 0.8, a: 1.0 }
    }

    /// DARKRED
    #[classattr]
    pub fn DARKRED() -> Color {
        Color { r: 0.5450980392156862, g: 0.0, b: 0.0, a: 1.0 }
    }

    /// DARKSALMON
    #[classattr]
    pub fn DARKSALMON() -> Color {
        Color { r: 0.9137254901960784, g: 0.5882352941176471, b: 0.47843137254901963, a: 1.0 }
    }

    /// DARKSEAGREEN
    #[classattr]
    pub fn DARKSEAGREEN() -> Color {
        Color { r: 0.5607843137254902, g: 0.7372549019607844, b: 0.5607843137254902, a: 1.0 }
    }

    /// DARKSLATEBLUE
    #[classattr]
    pub fn DARKSLATEBLUE() -> Color {
        Color { r: 0.2823529411764706, g: 0.23921568627450981, b: 0.5450980392156862, a: 1.0 }
    }

    /// DARKSLATEGRAY
    #[classattr]
    pub fn DARKSLATEGRAY() -> Color {
        Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 }
    }

    /// DARKSLATEGREY
    #[classattr]
    pub fn DARKSLATEGREY() -> Color {
        Color { r: 0.1843137254901961, g: 0.30980392156862746, b: 0.30980392156862746, a: 1.0 }
    }

    /// DARKTURQUOISE
    #[classattr]
    pub fn DARKTURQUOISE() -> Color {
        Color { r: 0.0, g: 0.807843137254902, b: 0.8196078431372549, a: 1.0 }
    }

    /// DARKVIOLET
    #[classattr]
    pub fn DARKVIOLET() -> Color {
        Color { r: 0.5803921568627451, g: 0.0, b: 0.8274509803921568, a: 1.0 }
    }

    /// DEEPPINK
    #[classattr]
    pub fn DEEPPINK() -> Color {
        Color { r: 1.0, g: 0.0784313725490196, b: 0.5764705882352941, a: 1.0 }
    }

    /// DEEPSKYBLUE
    #[classattr]
    pub fn DEEPSKYBLUE() -> Color {
        Color { r: 0.0, g: 0.7490196078431373, b: 1.0, a: 1.0 }
    }

    /// DIMGRAY
    #[classattr]
    pub fn DIMGRAY() -> Color {
        Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 }
    }

    /// DIMGREY
    #[classattr]
    pub fn DIMGREY() -> Color {
        Color { r: 0.4117647058823529, g: 0.4117647058823529, b: 0.4117647058823529, a: 1.0 }
    }

    /// DODGERBLUE
    #[classattr]
    pub fn DODGERBLUE() -> Color {
        Color { r: 0.11764705882352941, g: 0.5647058823529412, b: 1.0, a: 1.0 }
    }

    /// FIREBRICK
    #[classattr]
    pub fn FIREBRICK() -> Color {
        Color { r: 0.6980392156862745, g: 0.13333333333333333, b: 0.13333333333333333, a: 1.0 }
    }

    /// FLORALWHITE
    #[classattr]
    pub fn FLORALWHITE() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.9411764705882353, a: 1.0 }
    }

    /// FORESTGREEN
    #[classattr]
    pub fn FORESTGREEN() -> Color {
        Color { r: 0.13333333333333333, g: 0.5450980392156862, b: 0.13333333333333333, a: 1.0 }
    }

    /// GAINSBORO
    #[classattr]
    pub fn GAINSBORO() -> Color {
        Color { r: 0.8627450980392157, g: 0.8627450980392157, b: 0.8627450980392157, a: 1.0 }
    }

    /// GHOSTWHITE
    #[classattr]
    pub fn GHOSTWHITE() -> Color {
        Color { r: 0.9725490196078431, g: 0.9725490196078431, b: 1.0, a: 1.0 }
    }

    /// GREENYELLOW
    #[classattr]
    pub fn GREENYELLOW() -> Color {
        Color { r: 0.6784313725490196, g: 1.0, b: 0.1843137254901961, a: 1.0 }
    }

    /// HONEYDEW
    #[classattr]
    pub fn HONEYDEW() -> Color {
        Color { r: 0.9411764705882353, g: 1.0, b: 0.9411764705882353, a: 1.0 }
    }

    /// HOTPINK
    #[classattr]
    pub fn HOTPINK() -> Color {
        Color { r: 1.0, g: 0.4117647058823529, b: 0.7058823529411765, a: 1.0 }
    }

    /// INDIANRED
    #[classattr]
    pub fn INDIANRED() -> Color {
        Color { r: 0.803921568627451, g: 0.3607843137254902, b: 0.3607843137254902, a: 1.0 }
    }

    /// LAVENDERBLUSH
    #[classattr]
    pub fn LAVENDERBLUSH() -> Color {
        Color { r: 1.0, g: 0.9411764705882353, b: 0.9607843137254902, a: 1.0 }
    }

    /// LAWNGREEN
    #[classattr]
    pub fn LAWNGREEN() -> Color {
        Color { r: 0.48627450980392156, g: 0.9882352941176471, b: 0.0, a: 1.0 }
    }

    /// LEMONCHIFFON
    #[classattr]
    pub fn LEMONCHIFFON() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.803921568627451, a: 1.0 }
    }

    /// LIGHTCORAL
    #[classattr]
    pub fn LIGHTCORAL() -> Color {
        Color { r: 0.9411764705882353, g: 0.5019607843137255, b: 0.5019607843137255, a: 1.0 }
    }

    /// LIGHTCYAN
    #[classattr]
    pub fn LIGHTCYAN() -> Color {
        Color { r: 0.8784313725490196, g: 1.0, b: 1.0, a: 1.0 }
    }

    /// LIGHTGOLDENRODYELLOW
    #[classattr]
    pub fn LIGHTGOLDENRODYELLOW() -> Color {
        Color { r: 0.9803921568627451, g: 0.9803921568627451, b: 0.8235294117647058, a: 1.0 }
    }

    /// LIGHTGRAY
    #[classattr]
    pub fn LIGHTGRAY() -> Color {
        Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 }
    }

    /// LIGHTGREY
    #[classattr]
    pub fn LIGHTGREY() -> Color {
        Color { r: 0.8274509803921568, g: 0.8274509803921568, b: 0.8274509803921568, a: 1.0 }
    }

    /// LIGHTPINK
    #[classattr]
    pub fn LIGHTPINK() -> Color {
        Color { r: 1.0, g: 0.7137254901960784, b: 0.7568627450980392, a: 1.0 }
    }

    /// LIGHTSALMON
    #[classattr]
    pub fn LIGHTSALMON() -> Color {
        Color { r: 1.0, g: 0.6274509803921569, b: 0.47843137254901963, a: 1.0 }
    }

    /// LIGHTSEAGREEN
    #[classattr]
    pub fn LIGHTSEAGREEN() -> Color {
        Color { r: 0.12549019607843137, g: 0.6980392156862745, b: 0.6666666666666666, a: 1.0 }
    }

    /// LIGHTSKYBLUE
    #[classattr]
    pub fn LIGHTSKYBLUE() -> Color {
        Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9803921568627451, a: 1.0 }
    }

    /// LIGHTSLATEGRAY
    #[classattr]
    pub fn LIGHTSLATEGRAY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 }
    }

    /// LIGHTSLATEGREY
    #[classattr]
    pub fn LIGHTSLATEGREY() -> Color {
        Color { r: 0.4666666666666667, g: 0.5333333333333333, b: 0.6, a: 1.0 }
    }

    /// LIGHTSTEELBLUE
    #[classattr]
    pub fn LIGHTSTEELBLUE() -> Color {
        Color { r: 0.6901960784313725, g: 0.7686274509803922, b: 0.8705882352941177, a: 1.0 }
    }

    /// LIGHTYELLOW
    #[classattr]
    pub fn LIGHTYELLOW() -> Color {
        Color { r: 1.0, g: 1.0, b: 0.8784313725490196, a: 1.0 }
    }

    /// LIMEGREEN
    #[classattr]
    pub fn LIMEGREEN() -> Color {
        Color { r: 0.19607843137254902, g: 0.803921568627451, b: 0.19607843137254902, a: 1.0 }
    }

    /// LINEN
    #[classattr]
    pub fn LINEN() -> Color {
        Color { r: 0.9803921568627451, g: 0.9411764705882353, b: 0.9019607843137255, a: 1.0 }
    }

    /// MEDIUMAQUAMARINE
    #[classattr]
    pub fn MEDIUMAQUAMARINE() -> Color {
        Color { r: 0.4, g: 0.803921568627451, b: 0.6666666666666666, a: 1.0 }
    }

    /// MEDIUMBLUE
    #[classattr]
    pub fn MEDIUMBLUE() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.803921568627451, a: 1.0 }
    }

    /// MEDIUMORCHID
    #[classattr]
    pub fn MEDIUMORCHID() -> Color {
        Color { r: 0.7294117647058823, g: 0.3333333333333333, b: 0.8274509803921568, a: 1.0 }
    }

    /// MEDIUMPURPLE
    #[classattr]
    pub fn MEDIUMPURPLE() -> Color {
        Color { r: 0.5764705882352941, g: 0.4392156862745098, b: 0.8588235294117647, a: 1.0 }
    }

    /// MEDIUMSEAGREEN
    #[classattr]
    pub fn MEDIUMSEAGREEN() -> Color {
        Color { r: 0.23529411764705882, g: 0.7019607843137254, b: 0.44313725490196076, a: 1.0 }
    }

    /// MEDIUMSLATEBLUE
    #[classattr]
    pub fn MEDIUMSLATEBLUE() -> Color {
        Color { r: 0.4823529411764706, g: 0.40784313725490196, b: 0.9333333333333333, a: 1.0 }
    }

    /// MEDIUMSPRINGGREEN
    #[classattr]
    pub fn MEDIUMSPRINGGREEN() -> Color {
        Color { r: 0.0, g: 0.9803921568627451, b: 0.6039215686274509, a: 1.0 }
    }

    /// MEDIUMTURQUOISE
    #[classattr]
    pub fn MEDIUMTURQUOISE() -> Color {
        Color { r: 0.2823529411764706, g: 0.8196078431372549, b: 0.8, a: 1.0 }
    }

    /// MEDIUMVIOLETRED
    #[classattr]
    pub fn MEDIUMVIOLETRED() -> Color {
        Color { r: 0.7803921568627451, g: 0.08235294117647059, b: 0.5215686274509804, a: 1.0 }
    }

    /// MIDNIGHTBLUE
    #[classattr]
    pub fn MIDNIGHTBLUE() -> Color {
        Color { r: 0.09803921568627451, g: 0.09803921568627451, b: 0.4392156862745098, a: 1.0 }
    }

    /// MINTCREAM
    #[classattr]
    pub fn MINTCREAM() -> Color {
        Color { r: 0.9607843137254902, g: 1.0, b: 0.9803921568627451, a: 1.0 }
    }

    /// MISTYROSE
    #[classattr]
    pub fn MISTYROSE() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.8823529411764706, a: 1.0 }
    }

    /// MOCCASIN
    #[classattr]
    pub fn MOCCASIN() -> Color {
        Color { r: 1.0, g: 0.8941176470588236, b: 0.7098039215686275, a: 1.0 }
    }

    /// NAVAJOWHITE
    #[classattr]
    pub fn NAVAJOWHITE() -> Color {
        Color { r: 1.0, g: 0.8705882352941177, b: 0.6784313725490196, a: 1.0 }
    }

    /// OLDLACE
    #[classattr]
    pub fn OLDLACE() -> Color {
        Color { r: 0.9921568627450981, g: 0.9607843137254902, b: 0.9019607843137255, a: 1.0 }
    }

    /// OLIVEDRAB
    #[classattr]
    pub fn OLIVEDRAB() -> Color {
        Color { r: 0.4196078431372549, g: 0.5568627450980392, b: 0.13725490196078433, a: 1.0 }
    }

    /// PALEGOLDENROD
    #[classattr]
    pub fn PALEGOLDENROD() -> Color {
        Color { r: 0.9333333333333333, g: 0.9098039215686274, b: 0.6666666666666666, a: 1.0 }
    }

    /// PALEGREEN
    #[classattr]
    pub fn PALEGREEN() -> Color {
        Color { r: 0.596078431372549, g: 0.984313725490196, b: 0.596078431372549, a: 1.0 }
    }

    /// PALETURQUOISE
    #[classattr]
    pub fn PALETURQUOISE() -> Color {
        Color { r: 0.6862745098039216, g: 0.9333333333333333, b: 0.9333333333333333, a: 1.0 }
    }

    /// PALEVIOLETRED
    #[classattr]
    pub fn PALEVIOLETRED() -> Color {
        Color { r: 0.8588235294117647, g: 0.4392156862745098, b: 0.5764705882352941, a: 1.0 }
    }

    /// PAPAYAWHIP
    #[classattr]
    pub fn PAPAYAWHIP() -> Color {
        Color { r: 1.0, g: 0.9372549019607843, b: 0.8352941176470589, a: 1.0 }
    }

    /// PEACHPUFF
    #[classattr]
    pub fn PEACHPUFF() -> Color {
        Color { r: 1.0, g: 0.8549019607843137, b: 0.7254901960784313, a: 1.0 }
    }

    /// PERU
    #[classattr]
    pub fn PERU() -> Color {
        Color { r: 0.803921568627451, g: 0.5215686274509804, b: 0.24705882352941178, a: 1.0 }
    }

    /// POWDERBLUE
    #[classattr]
    pub fn POWDERBLUE() -> Color {
        Color { r: 0.6901960784313725, g: 0.8784313725490196, b: 0.9019607843137255, a: 1.0 }
    }

    /// REBECCAPURPLE
    #[classattr]
    pub fn REBECCAPURPLE() -> Color {
        Color { r: 0.4, g: 0.2, b: 0.6, a: 1.0 }
    }

    /// ROSYBROWN
    #[classattr]
    pub fn ROSYBROWN() -> Color {
        Color { r: 0.7372549019607844, g: 0.5607843137254902, b: 0.5607843137254902, a: 1.0 }
    }

    /// ROYALBLUE
    #[classattr]
    pub fn ROYALBLUE() -> Color {
        Color { r: 0.2549019607843137, g: 0.4117647058823529, b: 0.8823529411764706, a: 1.0 }
    }

    /// SADDLEBROWN
    #[classattr]
    pub fn SADDLEBROWN() -> Color {
        Color { r: 0.5450980392156862, g: 0.27058823529411763, b: 0.07450980392156863, a: 1.0 }
    }

    /// SANDYBROWN
    #[classattr]
    pub fn SANDYBROWN() -> Color {
        Color { r: 0.9568627450980393, g: 0.6431372549019608, b: 0.3764705882352941, a: 1.0 }
    }

    /// SEAGREEN
    #[classattr]
    pub fn SEAGREEN() -> Color {
        Color { r: 0.1803921568627451, g: 0.5450980392156862, b: 0.3411764705882353, a: 1.0 }
    }

    /// SEASHELL
    #[classattr]
    pub fn SEASHELL() -> Color {
        Color { r: 1.0, g: 0.9607843137254902, b: 0.9333333333333333, a: 1.0 }
    }

    /// SKYBLUE
    #[classattr]
    pub fn SKYBLUE() -> Color {
        Color { r: 0.5294117647058824, g: 0.807843137254902, b: 0.9215686274509803, a: 1.0 }
    }

    /// SLATEBLUE
    #[classattr]
    pub fn SLATEBLUE() -> Color {
        Color { r: 0.41568627450980394, g: 0.35294117647058826, b: 0.803921568627451, a: 1.0 }
    }

    /// SLATEGRAY
    #[classattr]
    pub fn SLATEGRAY() -> Color {
        Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 }
    }

    /// SLATEGREY
    #[classattr]
    pub fn SLATEGREY() -> Color {
        Color { r: 0.4392156862745098, g: 0.5019607843137255, b: 0.5647058823529412, a: 1.0 }
    }

    /// SNOW
    #[classattr]
    pub fn SNOW() -> Color {
        Color { r: 1.0, g: 0.9803921568627451, b: 0.9803921568627451, a: 1.0 }
    }

    /// SPRINGGREEN
    #[classattr]
    pub fn SPRINGGREEN() -> Color {
        Color { r: 0.0, g: 1.0, b: 0.4980392156862745, a: 1.0 }
    }

    /// STEELBLUE
    #[classattr]
    pub fn STEELBLUE() -> Color {
        Color { r: 0.27450980392156865, g: 0.5098039215686274, b: 0.7058823529411765, a: 1.0 }
    }

    /// THISTLE
    #[classattr]
    pub fn THISTLE() -> Color {
        Color { r: 0.8470588235294118, g: 0.7490196078431373, b: 0.8470588235294118, a: 1.0 }
    }

    /// WHITESMOKE
    #[classattr]
    pub fn WHITESMOKE() -> Color {
        Color { r: 0.9607843137254902, g: 0.9607843137254902, b: 0.9607843137254902, a: 1.0 }
    }

    /// INVISIBLE
    #[classattr]
    pub fn INVISIBLE() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }

    /// HALF_TRANSPARENT
    #[classattr]
    pub fn HALF_TRANSPARENT() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 }
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
