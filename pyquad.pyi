
import string
from tokenize import String
from typing import Any, Callable, TypeVar, overload
from typing import overload, Optional

from typing import Any, ClassVar
from typing import List
def draw_rectangle(x: float, y: float, w: float, h: float,
                      color: Color) -> None:
    """🚨 Should only be called inside py_callback, not directly. """
    ...


@overload
def activate_engine() -> None: ...
    
@overload
def activate_engine(config: Config) -> None: ...

def activate_engine(config: Optional[Config] = None) -> None:
    """Trigger the main game loop. 
    Can be called with no arguments for default settings, 
    or with a Config object for custom settings.
    """
    ...

def clear_background(color: Color) -> None:
    """🚨 Should only be called inside py_callback, not directly. """
    ...



def next_frame()-> None:
    """Next Frame"""
    ...


def write_texture(input: str)-> None:
    """Get a texture from a link"""
    ...


def draw_text(text: str, x: float, y: float, font_size: float, color: Color)-> None:
    ...

def draw_circle(x: float, y: float, r: float, color: Color) -> None:
    ...



def get_fps()-> float:
    ...


def get_keys_down()-> KeyCodeSet:
    ...
def get_keys_pressed()-> KeyCodeSet:
    ...
def get_keys_released()-> KeyCodeSet:
    ...
_all__ = [
    "draw_rectangle",
    "activate_engine",
    "next_frame",
    "clear_background",
    "write_texture",
    "draw_text",
    "get_fps",
    "get_keys_down",
    "get_keys_pressed",
    "get_keys_released",
]

#colors 
# RED: Color
# GREEN: Color
# BLUE: Color
# WHITE: Color
# BLACK: Color
# GRAY: Color
# TRANSPARENT: Color
from enum import IntEnum
from typing import Set


class KeyCode(IntEnum):
    Space = 0x0020
    Apostrophe = 0x0027
    Comma = 0x002c
    Minus = 0x002d
    Period = 0x002e
    Slash = 0x002f
    Key0 = 0x0030
    Key1 = 0x0031
    Key2 = 0x0032
    Key3 = 0x0033
    Key4 = 0x0034
    Key5 = 0x0035
    Key6 = 0x0036
    Key7 = 0x0037
    Key8 = 0x0038
    Key9 = 0x0039
    Semicolon = 0x003b
    Equal = 0x003d
    A = 0x0041
    B = 0x0042
    C = 0x0043
    D = 0x0044
    E = 0x0045
    F = 0x0046
    G = 0x0047
    H = 0x0048
    I = 0x0049
    J = 0x004a
    K = 0x004b
    L = 0x004c
    M = 0x004d
    N = 0x004e
    O = 0x004f
    P = 0x0050
    Q = 0x0051
    R = 0x0052
    S = 0x0053
    T = 0x0054
    U = 0x0055
    V = 0x0056
    W = 0x0057
    X = 0x0058
    Y = 0x0059
    Z = 0x005a
    LeftBracket = 0x005b
    Backslash = 0x005c
    RightBracket = 0x005d
    GraveAccent = 0x0060
    World1 = 0x0100
    World2 = 0x0101
    Escape = 0xff1b
    Enter = 0xff0d
    Tab = 0xff09
    Backspace = 0xff08
    Insert = 0xff63
    Delete = 0xffff
    Right = 0xff53
    Left = 0xff51
    Down = 0xff54
    Up = 0xff52
    PageUp = 0xff55
    PageDown = 0xff56
    Home = 0xff50
    End = 0xff57
    CapsLock = 0xffe5
    ScrollLock = 0xff14
    NumLock = 0xff7f
    PrintScreen = 0xfd1d
    Pause = 0xff13
    F1 = 0xffbe
    F2 = 0xffbf
    F3 = 0xffc0
    F4 = 0xffc1
    F5 = 0xffc2
    F6 = 0xffc3
    F7 = 0xffc4
    F8 = 0xffc5
    F9 = 0xffc6
    F10 = 0xffc7
    F11 = 0xffc8
    F12 = 0xffc9
    F13 = 0xffca
    F14 = 0xffcb
    F15 = 0xffcc
    F16 = 0xffcd
    F17 = 0xffce
    F18 = 0xffcf
    F19 = 0xffd0
    F20 = 0xffd1
    F21 = 0xffd2
    F22 = 0xffd3
    F23 = 0xffd4
    F24 = 0xffd5
    F25 = 0xffd6
    Kp0 = 0xffb0
    Kp1 = 0xffb1
    Kp2 = 0xffb2
    Kp3 = 0xffb3
    Kp4 = 0xffb4
    Kp5 = 0xffb5
    Kp6 = 0xffb6
    Kp7 = 0xffb7
    Kp8 = 0xffb8
    Kp9 = 0xffb9
    KpDecimal = 0xffae
    KpDivide = 0xffaf
    KpMultiply = 0xffaa
    KpSubtract = 0xffad
    KpAdd = 0xffab
    KpEnter = 0xff8d
    KpEqual = 0xffbd
    LeftShift = 0xffe1
    LeftControl = 0xffe3
    LeftAlt = 0xffe9
    LeftSuper = 0xffeb
    RightShift = 0xffe2
    RightControl = 0xffe4
    RightAlt = 0xffea
    RightSuper = 0xffec
    Menu = 0xff67
    Back = 0xff04
    Unknown = 0x01ff



#more colors

CLOUDY_BLUE: Color
DARK_PASTEL_GREEN: Color
DUST: Color
ELECTRIC_LIME: Color
FRESH_GREEN: Color
LIGHT_EGGPLANT: Color
NASTY_GREEN: Color
REALLY_LIGHT_BLUE: Color
TEA: Color
WARM_PURPLE: Color
YELLOWISH_TAN: Color
CEMENT: Color
DARK_GRASS_GREEN: Color
DUSTY_TEAL: Color
GREY_TEAL: Color
MACARONI_AND_CHEESE: Color
PINKISH_TAN: Color
SPRUCE: Color
STRONG_BLUE: Color
TOXIC_GREEN: Color
WINDOWS_BLUE: Color
BLUE_BLUE: Color
BLUE_WITH_A_HINT_OF_PURPLE: Color
BOOGER: Color
BRIGHT_SEA_GREEN: Color
DARK_GREEN_BLUE: Color
DEEP_TURQUOISE: Color
GREEN_TEAL: Color
STRONG_PINK: Color
BLAND: Color
DEEP_AQUA: Color
LAVENDER_PINK: Color
LIGHT_MOSS_GREEN: Color
LIGHT_SEAFOAM_GREEN: Color
OLIVE_YELLOW: Color
PIG_PINK: Color
DEEP_LILAC: Color
DESERT: Color
DUSTY_LAVENDER: Color
PURPLEY_GREY: Color
PURPLY: Color
CANDY_PINK: Color
LIGHT_PASTEL_GREEN: Color
BORING_GREEN: Color
KIWI_GREEN: Color
LIGHT_GREY_GREEN: Color
ORANGE_PINK: Color
TEA_GREEN: Color
VERY_LIGHT_BROWN: Color
EGG_SHELL: Color
EGGPLANT_PURPLE: Color
POWDER_PINK: Color
REDDISH_GREY: Color
BABY_SHIT_BROWN: Color
LILIAC: Color
STORMY_BLUE: Color
UGLY_BROWN: Color
CUSTARD: Color
DARKISH_PINK: Color
DEEP_BROWN: Color
GREENISH_BEIGE: Color
MANILLA: Color
OFF_BLUE: Color
BATTLESHIP_GREY: Color
BROWNY_GREEN: Color
BRUISE: Color
KELLEY_GREEN: Color
SICKLY_YELLOW: Color
SUNNY_YELLOW: Color
AZUL: Color
DARKGREEN: Color
GREEN_YELLOW: Color
LICHEN: Color
LIGHT_LIGHT_GREEN: Color
PALE_GOLD: Color
SUN_YELLOW: Color
TAN_GREEN: Color
BURPLE: Color
BUTTERSCOTCH: Color
TOUPE: Color
DARK_CREAM: Color
INDIAN_RED: Color
LIGHT_LAVENDAR: Color
POISON_GREEN: Color
BABY_PUKE_GREEN: Color
BRIGHT_YELLOW_GREEN: Color
CHARCOAL_GREY: Color
SQUASH: Color
CINNAMON: Color
LIGHT_PEA_GREEN: Color
RADIOACTIVE_GREEN: Color
RAW_SIENNA: Color
BABY_PURPLE: Color
COCOA: Color
LIGHT_ROYAL_BLUE: Color
ORANGEISH: Color
RUST_BROWN: Color
SAND_BROWN: Color
SWAMP: Color
TEALISH_GREEN: Color
BURNT_SIENA: Color
CAMO: Color
DUSK_BLUE: Color
FERN: Color
OLD_ROSE: Color
PALE_LIGHT_GREEN: Color
PEACHY_PINK: Color
ROSY_PINK: Color
LIGHT_BLUISH_GREEN: Color
LIGHT_BRIGHT_GREEN: Color
LIGHT_NEON_GREEN: Color
LIGHT_SEAFOAM: Color
TIFFANY_BLUE: Color
WASHED_OUT_GREEN: Color
BROWNY_ORANGE: Color
NICE_BLUE: Color
SAPPHIRE: Color
GREYISH_TEAL: Color
ORANGEY_YELLOW: Color
PARCHMENT: Color
STRAW: Color
VERY_DARK_BROWN: Color
TERRACOTA: Color
UGLY_BLUE: Color
CLEAR_BLUE: Color
CREME: Color
FOAM_GREEN: Color
GREY_GREEN: Color
LIGHT_GOLD: Color
SEAFOAM_BLUE: Color
TOPAZ: Color
VIOLET_PINK: Color
WINTERGREEN: Color
YELLOW_TAN: Color
DARK_FUCHSIA: Color
INDIGO_BLUE: Color
LIGHT_YELLOWISH_GREEN: Color
PALE_MAGENTA: Color
RICH_PURPLE: Color
SUNFLOWER_YELLOW: Color
GREEN_BLUE: Color
LEATHER: Color
RACING_GREEN: Color
VIVID_PURPLE: Color
DARK_ROYAL_BLUE: Color
HAZEL: Color
MUTED_PINK: Color
BOOGER_GREEN: Color
CANARY: Color
COOL_GREY: Color
DARK_TAUPE: Color
DARKISH_PURPLE: Color
TRUE_GREEN: Color
CORAL_PINK: Color
DARK_SAGE: Color
DARK_SLATE_BLUE: Color
FLAT_BLUE: Color
MUSHROOM: Color
RICH_BLUE: Color
DIRTY_PURPLE: Color
GREENBLUE: Color
ICKY_GREEN: Color
LIGHT_KHAKI: Color
WARM_BLUE: Color
DARK_HOT_PINK: Color
DEEP_SEA_BLUE: Color
CARMINE: Color
DARK_YELLOW_GREEN: Color
PALE_PEACH: Color
PLUM_PURPLE: Color
GOLDEN_ROD: Color
NEON_RED: Color
OLD_PINK: Color
VERY_PALE_BLUE: Color
BLOOD_ORANGE: Color
GRAPEFRUIT: Color
SAND_YELLOW: Color
CLAY_BROWN: Color
DARK_BLUE_GREY: Color
FLAT_GREEN: Color
LIGHT_GREEN_BLUE: Color
WARM_PINK: Color
DODGER_BLUE: Color
GROSS_GREEN: Color
ICE: Color
METALLIC_BLUE: Color
PALE_SALMON: Color
SAP_GREEN: Color
ALGAE: Color
BLUEY_GREY: Color
GREENY_GREY: Color
HIGHLIGHTER_GREEN: Color
LIGHT_LIGHT_BLUE: Color
LIGHT_MINT: Color
RAW_UMBER: Color
VIVID_BLUE: Color
DEEP_LAVENDER: Color
DULL_TEAL: Color
LIGHT_GREENISH_BLUE: Color
MUD_GREEN: Color
PINKY: Color
RED_WINE: Color
SHIT_GREEN: Color
TAN_BROWN: Color
DARKBLUE: Color
ROSA: Color
LIPSTICK: Color
PALE_MAUVE: Color
CLARET: Color
DANDELION: Color
ORANGERED: Color
POOP_GREEN: Color
RUBY: Color
DARK: Color
GREENISH_TURQUOISE: Color
PASTEL_RED: Color
PISS_YELLOW: Color
BRIGHT_CYAN: Color
DARK_CORAL: Color
ALGAE_GREEN: Color
DARKISH_RED: Color
REDDY_BROWN: Color
BLUSH_PINK: Color
CAMOUFLAGE_GREEN: Color
LAWN_GREEN: Color
PUTTY: Color
VIBRANT_BLUE: Color
DARK_SAND: Color
PURPLE_BLUE: Color
SAFFRON: Color
TWILIGHT: Color
WARM_BROWN: Color
BLUEGREY: Color
BUBBLE_GUM_PINK: Color
DUCK_EGG_BLUE: Color
GREENISH_CYAN: Color
PETROL: Color
ROYAL: Color
BUTTER: Color
DUSTY_ORANGE: Color
OFF_YELLOW: Color
PALE_OLIVE_GREEN: Color
ORANGISH: Color
LEAF: Color
LIGHT_BLUE_GREY: Color
DRIED_BLOOD: Color
LIGHTISH_PURPLE: Color
RUSTY_RED: Color
LAVENDER_BLUE: Color
LIGHT_GRASS_GREEN: Color
LIGHT_MINT_GREEN: Color
SUNFLOWER: Color
VELVET: Color
BRICK_ORANGE: Color
LIGHTISH_RED: Color
PURE_BLUE: Color
TWILIGHT_BLUE: Color
VIOLET_RED: Color
YELLOWY_BROWN: Color
CARNATION: Color
MUDDY_YELLOW: Color
DARK_SEAFOAM_GREEN: Color
DEEP_ROSE: Color
DUSTY_RED: Color
GREY_BLUE: Color
LEMON_LIME: Color
PURPLE_PINK: Color
BROWN_YELLOW: Color
PURPLE_BROWN: Color
WISTERIA: Color
BANANA_YELLOW: Color
LIPSTICK_RED: Color
WATER_BLUE: Color
BROWN_GREY: Color
VIBRANT_PURPLE: Color
BABY_GREEN: Color
BARF_GREEN: Color
EGGSHELL_BLUE: Color
SANDY_YELLOW: Color
COOL_GREEN: Color
PALE: Color
BLUE_GREY: Color
HOT_MAGENTA: Color
GREYBLUE: Color
PURPLEY: Color
BABY_SHIT_GREEN: Color
BROWNISH_PINK: Color
DARK_AQUAMARINE: Color
DIARRHEA: Color
LIGHT_MUSTARD: Color
PALE_SKY_BLUE: Color
TURTLE_GREEN: Color
BRIGHT_OLIVE: Color
DARK_GREY_BLUE: Color
GREENY_BROWN: Color
LEMON_GREEN: Color
LIGHT_PERIWINKLE: Color
SEAWEED_GREEN: Color
SUNSHINE_YELLOW: Color
UGLY_PURPLE: Color
MEDIUM_PINK: Color
PUKE_BROWN: Color
VERY_LIGHT_PINK: Color
VIRIDIAN: Color
BILE: Color
FADED_YELLOW: Color
VERY_PALE_GREEN: Color
VIBRANT_GREEN: Color
BRIGHT_LIME: Color
SPEARMINT: Color
LIGHT_AQUAMARINE: Color
LIGHT_SAGE: Color
YELLOWGREEN: Color
BABY_POO: Color
DARK_SEAFOAM: Color
DEEP_TEAL: Color
HEATHER: Color
RUST_ORANGE: Color
DIRTY_BLUE: Color
FERN_GREEN: Color
BRIGHT_LILAC: Color
WEIRD_GREEN: Color
PEACOCK_BLUE: Color
AVOCADO_GREEN: Color
FADED_ORANGE: Color
GRAPE_PURPLE: Color
HOT_GREEN: Color
LIME_YELLOW: Color
MANGO: Color
SHAMROCK: Color
BUBBLEGUM: Color
PURPLISH_BROWN: Color
VOMIT_YELLOW: Color
PALE_CYAN: Color
KEY_LIME: Color
TOMATO_RED: Color
LIGHTGREEN: Color
MERLOT: Color
NIGHT_BLUE: Color
PURPLEISH_PINK: Color
APPLE: Color
BABY_POOP_GREEN: Color
GREEN_APPLE: Color
HELIOTROPE: Color
YELLOW_GREEN: Color
ALMOST_BLACK: Color
COOL_BLUE: Color
LEAFY_GREEN: Color
MUSTARD_BROWN: Color
DUSK: Color
DULL_BROWN: Color
FROG_GREEN: Color
VIVID_GREEN: Color
BRIGHT_LIGHT_GREEN: Color
FLURO_GREEN: Color
KIWI: Color
SEAWEED: Color
NAVY_GREEN: Color
ULTRAMARINE_BLUE: Color
IRIS: Color
PASTEL_ORANGE: Color
YELLOWISH_ORANGE: Color
PERRYWINKLE: Color
TEALISH: Color
DARK_PLUM: Color
PEAR: Color
PINKISH_ORANGE: Color
MIDNIGHT_PURPLE: Color
LIGHT_URPLE: Color
DARK_MINT: Color
GREENISH_TAN: Color
LIGHT_BURGUNDY: Color
TURQUOISE_BLUE: Color
UGLY_PINK: Color
SANDY: Color
ELECTRIC_PINK: Color
MUTED_PURPLE: Color
MID_GREEN: Color
GREYISH: Color
NEON_YELLOW: Color
BANANA: Color
CARNATION_PINK: Color
TOMATO: Color
SEA: Color
MUDDY_BROWN: Color
TURQUOISE_GREEN: Color
BUFF: Color
FAWN: Color
MUTED_BLUE: Color
PALE_ROSE: Color
DARK_MINT_GREEN: Color
AMETHYST: Color
BLUE_GREEN: Color
CHESTNUT: Color
SICK_GREEN: Color
PEA: Color
RUSTY_ORANGE: Color
STONE: Color
ROSE_RED: Color
PALE_AQUA: Color
DEEP_ORANGE: Color
EARTH: Color
MOSSY_GREEN: Color
GRASSY_GREEN: Color
PALE_LIME_GREEN: Color
LIGHT_GREY_BLUE: Color
PALE_GREY: Color
ASPARAGUS: Color
BLUEBERRY: Color
PURPLE_RED: Color
PALE_LIME: Color
GREENISH_TEAL: Color
CARAMEL: Color
DEEP_MAGENTA: Color
LIGHT_PEACH: Color
MILK_CHOCOLATE: Color
OCHER: Color
OFF_GREEN: Color
PURPLY_PINK: Color
LIGHTBLUE: Color
DUSKY_BLUE: Color
GOLDEN: Color
LIGHT_BEIGE: Color
BUTTER_YELLOW: Color
DUSKY_PURPLE: Color
FRENCH_BLUE: Color
UGLY_YELLOW: Color
GREENY_YELLOW: Color
ORANGISH_RED: Color
SHAMROCK_GREEN: Color
ORANGISH_BROWN: Color
TREE_GREEN: Color
DEEP_VIOLET: Color
GUNMETAL: Color
BLUE_PURPLE: Color
CHERRY: Color
SANDY_BROWN: Color
WARM_GREY: Color
DARK_INDIGO: Color
MIDNIGHT: Color
BLUEY_GREEN: Color
GREY_PINK: Color
SOFT_PURPLE: Color
BLOOD: Color
BROWN_RED: Color
MEDIUM_GREY: Color
BERRY: Color
POO: Color
PURPLEY_PINK: Color
LIGHT_SALMON: Color
SNOT: Color
EASTER_PURPLE: Color
LIGHT_YELLOW_GREEN: Color
DARK_NAVY_BLUE: Color
DRAB: Color
LIGHT_ROSE: Color
ROUGE: Color
PURPLISH_RED: Color
SLIME_GREEN: Color
BABY_POOP: Color
IRISH_GREEN: Color
PINK_PURPLE: Color
DARK_NAVY: Color
GREENY_BLUE: Color
LIGHT_PLUM: Color
PINKISH_GREY: Color
DIRTY_ORANGE: Color
RUST_RED: Color
PALE_LILAC: Color
ORANGEY_RED: Color
PRIMARY_BLUE: Color
KERMIT_GREEN: Color
BROWNISH_PURPLE: Color
MURKY_GREEN: Color
WHEAT: Color
VERY_DARK_PURPLE: Color
BOTTLE_GREEN: Color
WATERMELON: Color
DEEP_SKY_BLUE: Color
FIRE_ENGINE_RED: Color
YELLOW_OCHRE: Color
PUMPKIN_ORANGE: Color
PALE_OLIVE: Color
LIGHT_LILAC: Color
LIGHTISH_GREEN: Color
CAROLINA_BLUE: Color
MULBERRY: Color
SHOCKING_PINK: Color
AUBURN: Color
BRIGHT_LIME_GREEN: Color
CELADON: Color
PINKISH_BROWN: Color
POO_BROWN: Color
BRIGHT_SKY_BLUE: Color
CELERY: Color
DIRT_BROWN: Color
STRAWBERRY: Color
DARK_LIME: Color
COPPER: Color
MEDIUM_BROWN: Color
MUTED_GREEN: Color
ROBINS_EGG: Color
BRIGHT_AQUA: Color
BRIGHT_LAVENDER: Color
IVORY: Color
VERY_LIGHT_PURPLE: Color
LIGHT_NAVY: Color
PINK_RED: Color
OLIVE_BROWN: Color
POOP_BROWN: Color
MUSTARD_GREEN: Color
OCEAN_GREEN: Color
VERY_DARK_BLUE: Color
DUSTY_GREEN: Color
LIGHT_NAVY_BLUE: Color
MINTY_GREEN: Color
ADOBE: Color
BARNEY: Color
JADE_GREEN: Color
BRIGHT_LIGHT_BLUE: Color
LIGHT_LIME: Color
DARK_KHAKI: Color
ORANGE_YELLOW: Color
OCRE: Color
MAIZE: Color
FADED_PINK: Color
BRITISH_RACING_GREEN: Color
SANDSTONE: Color
MUD_BROWN: Color
LIGHT_SEA_GREEN: Color
ROBIN_EGG_BLUE: Color
AQUA_MARINE: Color
DARK_SEA_GREEN: Color
SOFT_PINK: Color
ORANGEY_BROWN: Color
CHERRY_RED: Color
BURNT_YELLOW: Color
BROWNISH_GREY: Color
CAMEL: Color
PURPLISH_GREY: Color
MARINE: Color
GREYISH_PINK: Color
PALE_TURQUOISE: Color
PASTEL_YELLOW: Color
BLUEY_PURPLE: Color
CANARY_YELLOW: Color
FADED_RED: Color
SEPIA: Color
COFFEE: Color
BRIGHT_MAGENTA: Color
MOCHA: Color
ECRU: Color
PURPLEISH: Color
CRANBERRY: Color
DARKISH_GREEN: Color
BROWN_ORANGE: Color
DUSKY_ROSE: Color
MELON: Color
SICKLY_GREEN: Color
SILVER: Color
PURPLY_BLUE: Color
PURPLEISH_BLUE: Color
HOSPITAL_GREEN: Color
SHIT_BROWN: Color
MID_BLUE: Color
AMBER: Color
EASTER_GREEN: Color
SOFT_BLUE: Color
CERULEAN_BLUE: Color
GOLDEN_BROWN: Color
BRIGHT_TURQUOISE: Color
RED_PINK: Color
RED_PURPLE: Color
GREYISH_BROWN: Color
VERMILLION: Color
RUSSET: Color
STEEL_GREY: Color
LIGHTER_PURPLE: Color
BRIGHT_VIOLET: Color
PRUSSIAN_BLUE: Color
SLATE_GREEN: Color
DIRTY_PINK: Color
DARK_BLUE_GREEN: Color
PINE: Color
YELLOWY_GREEN: Color
DARK_GOLD: Color
BLUISH: Color
DARKISH_BLUE: Color
DULL_RED: Color
PINKY_RED: Color
BRONZE: Color
PALE_TEAL: Color
MILITARY_GREEN: Color
BARBIE_PINK: Color
BUBBLEGUM_PINK: Color
PEA_SOUP_GREEN: Color
DARK_MUSTARD: Color
SHIT: Color
MEDIUM_PURPLE: Color
VERY_DARK_GREEN: Color
DIRT: Color
DUSKY_PINK: Color
RED_VIOLET: Color
LEMON_YELLOW: Color
PISTACHIO: Color
DULL_YELLOW: Color
DARK_LIME_GREEN: Color
DENIM_BLUE: Color
TEAL_BLUE: Color
LIGHTISH_BLUE: Color
PURPLEY_BLUE: Color
LIGHT_INDIGO: Color
SWAMP_GREEN: Color
BROWN_GREEN: Color
DARK_MAROON: Color
HOT_PURPLE: Color
DARK_FOREST_GREEN: Color
FADED_BLUE: Color
DRAB_GREEN: Color
LIGHT_LIME_GREEN: Color
SNOT_GREEN: Color
YELLOWISH: Color
LIGHT_BLUE_GREEN: Color
BORDEAUX: Color
LIGHT_MAUVE: Color
OCEAN: Color
MARIGOLD: Color
MUDDY_GREEN: Color
DULL_ORANGE: Color
STEEL: Color
ELECTRIC_PURPLE: Color
FLUORESCENT_GREEN: Color
YELLOWISH_BROWN: Color
BLUSH: Color
SOFT_GREEN: Color
BRIGHT_ORANGE: Color
LEMON: Color
PURPLE_GREY: Color
ACID_GREEN: Color
PALE_LAVENDER: Color
VIOLET_BLUE: Color
LIGHT_FOREST_GREEN: Color
BURNT_RED: Color
KHAKI_GREEN: Color
CERISE: Color
FADED_PURPLE: Color
APRICOT: Color
DARK_OLIVE_GREEN: Color
GREY_BROWN: Color
GREEN_GREY: Color
TRUE_BLUE: Color
PALE_VIOLET: Color
PERIWINKLE_BLUE: Color
LIGHT_SKY_BLUE: Color
BLURPLE: Color
GREEN_BROWN: Color
BLUEGREEN: Color
BRIGHT_TEAL: Color
BROWNISH_YELLOW: Color
PEA_SOUP: Color
FOREST: Color
BARNEY_PURPLE: Color
ULTRAMARINE: Color
PURPLISH: Color
PUKE_YELLOW: Color
BLUISH_GREY: Color
DARK_PERIWINKLE: Color
DARK_LILAC: Color
REDDISH: Color
LIGHT_MAROON: Color
DUSTY_PURPLE: Color
TERRA_COTTA: Color
AVOCADO: Color
MARINE_BLUE: Color
TEAL_GREEN: Color
SLATE_GREY: Color
LIGHTER_GREEN: Color
ELECTRIC_GREEN: Color
DUSTY_BLUE: Color
GOLDEN_YELLOW: Color
BRIGHT_YELLOW: Color
LIGHT_LAVENDER: Color
UMBER: Color
POOP: Color
DARK_PEACH: Color
JUNGLE_GREEN: Color
EGGSHELL: Color
DENIM: Color
YELLOW_BROWN: Color
DULL_PURPLE: Color
CHOCOLATE_BROWN: Color
WINE_RED: Color
NEON_BLUE: Color
DIRTY_GREEN: Color
LIGHT_TAN: Color
ICE_BLUE: Color
CADET_BLUE: Color
DARK_MAUVE: Color
VERY_LIGHT_BLUE: Color
GREY_PURPLE: Color
PASTEL_PINK: Color
VERY_LIGHT_GREEN: Color
DARK_SKY_BLUE: Color
EVERGREEN: Color
DULL_PINK: Color
AUBERGINE: Color
MAHOGANY: Color
REDDISH_ORANGE: Color
DEEP_GREEN: Color
VOMIT_GREEN: Color
PURPLE_PINK: Color
DUSTY_PINK: Color
FADED_GREEN: Color
CAMO_GREEN: Color
PINKY_PURPLE: Color
PINK_PURPLE: Color
BROWNISH_RED: Color
DARK_ROSE: Color
MUD: Color
BROWNISH: Color
EMERALD_GREEN: Color
PALE_BROWN: Color
DULL_BLUE: Color
BURNT_UMBER: Color
MEDIUM_GREEN: Color
CLAY: Color
LIGHT_AQUA: Color
LIGHT_OLIVE_GREEN: Color
BROWNISH_ORANGE: Color
DARK_AQUA: Color
PURPLISH_PINK: Color
DARK_SALMON: Color
GREENISH_GREY: Color
JADE: Color
UGLY_GREEN: Color
DARK_BEIGE: Color
EMERALD: Color
PALE_RED: Color
LIGHT_MAGENTA: Color
SKY: Color
LIGHT_CYAN: Color
YELLOW_ORANGE: Color
REDDISH_PURPLE: Color
REDDISH_PINK: Color
ORCHID: Color
DIRTY_YELLOW: Color
ORANGE_RED: Color
DEEP_RED: Color
ORANGE_BROWN: Color
COBALT_BLUE: Color
NEON_PINK: Color
ROSE_PINK: Color
GREYISH_PURPLE: Color
RASPBERRY: Color
AQUA_GREEN: Color
SALMON_PINK: Color
TANGERINE: Color
BROWNISH_GREEN: Color
RED_BROWN: Color
GREENISH_BROWN: Color
PUMPKIN: Color
PINE_GREEN: Color
CHARCOAL: Color
BABY_PINK: Color
CORNFLOWER: Color
BLUE_VIOLET: Color
CHOCOLATE: Color
GREYISH_GREEN: Color
SCARLET: Color
GREEN_YELLOW: Color
DARK_OLIVE: Color
SIENNA: Color
PASTEL_PURPLE: Color
TERRACOTTA: Color
AQUA_BLUE: Color
SAGE_GREEN: Color
BLOOD_RED: Color
DEEP_PINK: Color
GRASS: Color
MOSS: Color
PASTEL_BLUE: Color
BLUISH_GREEN: Color
GREEN_BLUE: Color
DARK_TAN: Color
GREENISH_BLUE: Color
PALE_ORANGE: Color
VOMIT: Color
FORREST_GREEN: Color
DARK_LAVENDER: Color
DARK_VIOLET: Color
PURPLE_BLUE: Color
DARK_CYAN: Color
OLIVE_DRAB: Color
PINKISH: Color
COBALT: Color
NEON_PURPLE: Color
LIGHT_TURQUOISE: Color
APPLE_GREEN: Color
DULL_GREEN: Color
WINE: Color
POWDER_BLUE: Color
OFF_WHITE: Color
ELECTRIC_BLUE: Color
DARK_TURQUOISE: Color
BLUE_PURPLE: Color
AZURE: Color
BRIGHT_RED: Color
PINKISH_RED: Color
CORNFLOWER_BLUE: Color
LIGHT_OLIVE: Color
GRAPE: Color
GREYISH_BLUE: Color
PURPLISH_BLUE: Color
YELLOWISH_GREEN: Color
GREENISH_YELLOW: Color
MEDIUM_BLUE: Color
DUSTY_ROSE: Color
LIGHT_VIOLET: Color
MIDNIGHT_BLUE: Color
BLUISH_PURPLE: Color
RED_ORANGE: Color
DARK_MAGENTA: Color
GREENISH: Color
OCEAN_BLUE: Color
CORAL: Color
CREAM: Color
REDDISH_BROWN: Color
BURNT_SIENNA: Color
BRICK: Color
SAGE: Color
GREY_GREEN: Color
WHITE: Color
ROBINS_EGG_BLUE: Color
MOSS_GREEN: Color
STEEL_BLUE: Color
EGGPLANT: Color
LIGHT_YELLOW: Color
LEAF_GREEN: Color
LIGHT_GREY: Color
PUKE: Color
PINKISH_PURPLE: Color
SEA_BLUE: Color
PALE_PURPLE: Color
SLATE_BLUE: Color
BLUE_GREY: Color
HUNTER_GREEN: Color
FUCHSIA: Color
CRIMSON: Color
PALE_YELLOW: Color
OCHRE: Color
MUSTARD_YELLOW: Color
LIGHT_RED: Color
CERULEAN: Color
PALE_PINK: Color
DEEP_BLUE: Color
RUST: Color
LIGHT_TEAL: Color
SLATE: Color
GOLDENROD: Color
DARK_YELLOW: Color
DARK_GREY: Color
ARMY_GREEN: Color
GREY_BLUE: Color
SEAFOAM: Color
PUCE: Color
SPRING_GREEN: Color
DARK_ORANGE: Color
SAND: Color
PASTEL_GREEN: Color
MINT: Color
LIGHT_ORANGE: Color
BRIGHT_PINK: Color
CHARTREUSE: Color
DEEP_PURPLE: Color
DARK_BROWN: Color
TAUPE: Color
PEA_GREEN: Color
PUKE_GREEN: Color
KELLY_GREEN: Color
SEAFOAM_GREEN: Color
BLUE_GREEN: Color
KHAKI: Color
BURGUNDY: Color
DARK_TEAL: Color
BRICK_RED: Color
ROYAL_PURPLE: Color
PLUM: Color
MINT_GREEN: Color
GOLD: Color
BABY_BLUE: Color
YELLOW_GREEN: Color
BRIGHT_PURPLE: Color
DARK_RED: Color
PALE_BLUE: Color
GRASS_GREEN: Color
NAVY: Color
AQUAMARINE: Color
BURNT_ORANGE: Color
NEON_GREEN: Color
BRIGHT_BLUE: Color
ROSE: Color
LIGHT_PINK: Color
MUSTARD: Color
INDIGO: Color
LIME: Color
SEA_GREEN: Color
PERIWINKLE: Color
DARK_PINK: Color
OLIVE_GREEN: Color
PEACH: Color
PALE_GREEN: Color
LIGHT_BROWN: Color
HOT_PINK: Color
BLACK: Color
LILAC: Color
NAVY_BLUE: Color
ROYAL_BLUE: Color
BEIGE: Color
SALMON: Color
OLIVE: Color
MAROON: Color
BRIGHT_GREEN: Color
DARK_PURPLE: Color
MAUVE: Color
FOREST_GREEN: Color
AQUA: Color
CYAN: Color
TAN: Color
DARK_BLUE: Color
LAVENDER: Color
TURQUOISE: Color
DARK_GREEN: Color
VIOLET: Color
LIGHT_PURPLE: Color
LIME_GREEN: Color
GREY: Color
SKY_BLUE: Color
YELLOW: Color
MAGENTA: Color
LIGHT_GREEN: Color
ORANGE: Color
TEAL: Color
LIGHT_BLUE: Color
RED: Color
BROWN: Color
PINK: Color
BLUE: Color
GREEN: Color
PURPLE: Color
GRAY_TEAL: Color
PURPLEY_GRAY: Color
LIGHT_GRAY_GREEN: Color
REDDISH_GRAY: Color
BATTLESHIP_GRAY: Color
CHARCOAL_GRAY: Color
GRAYISH_TEAL: Color
GRAY_GREEN: Color
COOL_GRAY: Color
DARK_BLUE_GRAY: Color
BLUEY_GRAY: Color
GREENY_GRAY: Color
BLUEGRAY: Color
LIGHT_BLUE_GRAY: Color
GRAY_BLUE: Color
BROWN_GRAY: Color
BLUE_GRAY: Color
GRAYBLUE: Color
DARK_GRAY_BLUE: Color
GRAYISH: Color
LIGHT_GRAY_BLUE: Color
PALE_GRAY: Color
WARM_GRAY: Color
GRAY_PINK: Color
MEDIUM_GRAY: Color
PINKISH_GRAY: Color
BROWNISH_GRAY: Color
PURPLISH_GRAY: Color
GRAYISH_PINK: Color
GRAYISH_BROWN: Color
STEEL_GRAY: Color
PURPLE_GRAY: Color
GRAY_BROWN: Color
GREEN_GRAY: Color
BLUISH_GRAY: Color
SLATE_GRAY: Color
GRAY_PURPLE: Color
GREENISH_GRAY: Color
GRAYISH_PURPLE: Color
GRAYISH_GREEN: Color
GRAYISH_BLUE: Color
GRAY_GREEN: Color
LIGHT_GRAY: Color
BLUE_GRAY: Color
DARK_GRAY: Color
GRAY_BLUE: Color
GRAY: Color
ALICEBLUE: Color
ANTIQUEWHITE: Color
AQUA: Color
AQUAMARINE: Color
AZURE: Color
BEIGE: Color
BISQUE: Color
BLACK: Color
BLANCHEDALMOND: Color
BLUE: Color
BLUEVIOLET: Color
BROWN: Color
BURLYWOOD: Color
CADETBLUE: Color
CHARTREUSE: Color
CHOCOLATE: Color
CORAL: Color
CORNFLOWERBLUE: Color
CORNSILK: Color
CRIMSON: Color
CYAN: Color
DARKBLUE: Color
DARKCYAN: Color
DARKGOLDENROD: Color
DARKGRAY: Color
DARKGREEN: Color
DARKGREY: Color
DARKKHAKI: Color
DARKMAGENTA: Color
DARKOLIVEGREEN: Color
DARKORANGE: Color
DARKORCHID: Color
DARKRED: Color
DARKSALMON: Color
DARKSEAGREEN: Color
DARKSLATEBLUE: Color
DARKSLATEGRAY: Color
DARKSLATEGREY: Color
DARKTURQUOISE: Color
DARKVIOLET: Color
DEEPPINK: Color
DEEPSKYBLUE: Color
DIMGRAY: Color
DIMGREY: Color
DODGERBLUE: Color
FIREBRICK: Color
FLORALWHITE: Color
FORESTGREEN: Color
FUCHSIA: Color
GAINSBORO: Color
GHOSTWHITE: Color
GOLD: Color
GOLDENROD: Color
GRAY: Color
GREEN: Color
GREENYELLOW: Color
GREY: Color
HONEYDEW: Color
HOTPINK: Color
INDIANRED: Color
INDIGO: Color
IVORY: Color
KHAKI: Color
LAVENDER: Color
LAVENDERBLUSH: Color
LAWNGREEN: Color
LEMONCHIFFON: Color
LIGHTBLUE: Color
LIGHTCORAL: Color
LIGHTCYAN: Color
LIGHTGOLDENRODYELLOW: Color
LIGHTGRAY: Color
LIGHTGREEN: Color
LIGHTGREY: Color
LIGHTPINK: Color
LIGHTSALMON: Color
LIGHTSEAGREEN: Color
LIGHTSKYBLUE: Color
LIGHTSLATEGRAY: Color
LIGHTSLATEGREY: Color
LIGHTSTEELBLUE: Color
LIGHTYELLOW: Color
LIME: Color
LIMEGREEN: Color
LINEN: Color
MAGENTA: Color
MAROON: Color
MEDIUMAQUAMARINE: Color
MEDIUMBLUE: Color
MEDIUMORCHID: Color
MEDIUMPURPLE: Color
MEDIUMSEAGREEN: Color
MEDIUMSLATEBLUE: Color
MEDIUMSPRINGGREEN: Color
MEDIUMTURQUOISE: Color
MEDIUMVIOLETRED: Color
MIDNIGHTBLUE: Color
MINTCREAM: Color
MISTYROSE: Color
MOCCASIN: Color
NAVAJOWHITE: Color
NAVY: Color
OLDLACE: Color
OLIVE: Color
OLIVEDRAB: Color
ORANGE: Color
ORANGERED: Color
ORCHID: Color
PALEGOLDENROD: Color
PALEGREEN: Color
PALETURQUOISE: Color
PALEVIOLETRED: Color
PAPAYAWHIP: Color
PEACHPUFF: Color
PERU: Color
PINK: Color
PLUM: Color
POWDERBLUE: Color
PURPLE: Color
REBECCAPURPLE: Color
RED: Color
ROSYBROWN: Color
ROYALBLUE: Color
SADDLEBROWN: Color
SALMON: Color
SANDYBROWN: Color
SEAGREEN: Color
SEASHELL: Color
SIENNA: Color
SILVER: Color
SKYBLUE: Color
SLATEBLUE: Color
SLATEGRAY: Color
SLATEGREY: Color
SNOW: Color
SPRINGGREEN: Color
STEELBLUE: Color
TAN: Color
TEAL: Color
THISTLE: Color
TOMATO: Color
TURQUOISE: Color
VIOLET: Color
WHEAT: Color
WHITE: Color
WHITESMOKE: Color
YELLOW: Color
YELLOWGREEN: Color
BLUE: Color
ORANGE: Color
GREEN: Color
RED: Color
PURPLE: Color
BROWN: Color
PINK: Color
GRAY: Color
OLIVE: Color
CYAN: Color
GREY: Color

#end of functions

# --- Auto-generated class definitions start ---
from typing import Any, ClassVar

class Circle:
    def __init__(self, x: float, y: float, r: float) -> None: ...

    x: float
    y: float
    r: float

    def __repr__(self) -> str: ...


class Color:
    def __init__(self, r: float, g: float, b: float, a: float) -> None: ...

    r: float
    g: float
    b: float
    a: float

    def __repr__(self) -> str: ...


class Config:
    def __init__(self, window_title: str, window_width: int, window_height: int, fullscreen: bool, sample_count: int, window_resizable: bool, stop_pyton_when_closing_window: bool) -> None: ...

    window_title: str
    window_width: int
    window_height: int
    fullscreen: bool
    sample_count: int
    window_resizable: bool
    stop_pyton_when_closing_window: bool

    def __repr__(self) -> str: ...


class DMat2:
    def __init__(self, x_axis: Any, y_axis: Any) -> None: ...

    x_axis: Any
    y_axis: Any

    def __repr__(self) -> str: ...


class DMat3:
    def __init__(self, x_axis: Any, y_axis: Any, z_axis: Any) -> None: ...

    x_axis: Any
    y_axis: Any
    z_axis: Any

    def __repr__(self) -> str: ...


class DMat4:
    def __init__(self, x_axis: Any, y_axis: Any, z_axis: Any, w_axis: Any) -> None: ...

    x_axis: Any
    y_axis: Any
    z_axis: Any
    w_axis: Any

    def __repr__(self) -> str: ...


class DVec2:
    def __init__(self, x: float, y: float) -> None: ...

    x: float
    y: float

    def __repr__(self) -> str: ...


class DVec3:
    def __init__(self, x: float, y: float, z: float) -> None: ...

    x: float
    y: float
    z: float

    def __repr__(self) -> str: ...


class DVec4:
    def __init__(self, x: float, y: float, z: float, w: float) -> None: ...

    x: float
    y: float
    z: float
    w: float

    def __repr__(self) -> str: ...


class I16Vec2:
    def __init__(self, x: int, y: int) -> None: ...

    x: int
    y: int

    def __repr__(self) -> str: ...


class I16Vec3:
    def __init__(self, x: int, y: int, z: int) -> None: ...

    x: int
    y: int
    z: int

    def __repr__(self) -> str: ...


class I16Vec4:
    def __init__(self, x: int, y: int, z: int, w: int) -> None: ...

    x: int
    y: int
    z: int
    w: int

    def __repr__(self) -> str: ...


class I64Vec2:
    def __init__(self, x: int, y: int) -> None: ...

    x: int
    y: int

    def __repr__(self) -> str: ...


class I64Vec3:
    def __init__(self, x: int, y: int, z: int) -> None: ...

    x: int
    y: int
    z: int

    def __repr__(self) -> str: ...


class I64Vec4:
    def __init__(self, x: int, y: int, z: int, w: int) -> None: ...

    x: int
    y: int
    z: int
    w: int

    def __repr__(self) -> str: ...


class IVec2:
    def __init__(self, x: int, y: int) -> None: ...

    x: int
    y: int

    def __repr__(self) -> str: ...


class IVec3:
    def __init__(self, x: int, y: int, z: int) -> None: ...

    x: int
    y: int
    z: int

    def __repr__(self) -> str: ...


class IVec4:
    def __init__(self, x: int, y: int, z: int, w: int) -> None: ...

    x: int
    y: int
    z: int
    w: int

    def __repr__(self) -> str: ...


class Image:
    def __init__(self, bytes: List, width: int, height: int) -> None: ...

    bytes: List
    width: int
    height: int

    def __repr__(self) -> str: ...


class Quat:
    def __init__(self, x: float, y: float, z: float, w: float) -> None: ...

    x: float
    y: float
    z: float
    w: float

    def __repr__(self) -> str: ...


class Rect:
    def __init__(self, x: float, y: float, w: float, h: float) -> None: ...

    x: float
    y: float
    w: float
    h: float

    def __repr__(self) -> str: ...


class U16Vec2:
    def __init__(self, x: int, y: int) -> None: ...

    x: int
    y: int

    def __repr__(self) -> str: ...


class U16Vec3:
    def __init__(self, x: int, y: int, z: int) -> None: ...

    x: int
    y: int
    z: int

    def __repr__(self) -> str: ...


class U16Vec4:
    def __init__(self, x: int, y: int, z: int, w: int) -> None: ...

    x: int
    y: int
    z: int
    w: int

    def __repr__(self) -> str: ...


class U64Vec2:
    def __init__(self, x: int, y: int) -> None: ...

    x: int
    y: int

    def __repr__(self) -> str: ...


class U64Vec3:
    def __init__(self, x: int, y: int, z: int) -> None: ...

    x: int
    y: int
    z: int

    def __repr__(self) -> str: ...


class U64Vec4:
    def __init__(self, x: int, y: int, z: int, w: int) -> None: ...

    x: int
    y: int
    z: int
    w: int

    def __repr__(self) -> str: ...


class UVec2:
    def __init__(self, x: int, y: int) -> None: ...

    x: int
    y: int

    def __repr__(self) -> str: ...


class UVec3:
    def __init__(self, x: int, y: int, z: int) -> None: ...

    x: int
    y: int
    z: int

    def __repr__(self) -> str: ...


class UVec4:
    def __init__(self, x: int, y: int, z: int, w: int) -> None: ...

    x: int
    y: int
    z: int
    w: int

    def __repr__(self) -> str: ...





class Vec4:
    def __init__(self, x: float, y: float, z: float, w: float) -> None: ...

    x: float
    y: float
    z: float
    w: float

    def __repr__(self) -> str: ...


# --- Auto-generated class definitions end ---