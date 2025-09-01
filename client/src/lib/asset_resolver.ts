export const RESOLVE_COLOR: { [key: string]: { item: string, name: string } } = {
    '#F9FFFE': {
        item: "white",
        name: "White Dye",
    },
    '#F9801D': {
        item: "orange",
        name: "Orange Dye"
    },
    '#C74EBD': {
        item: "magenta",
        name: "Magenta Dye"
    },
    '#3AB3DA': {
        item: "light_blue",
        name: "Light Blue Dye"
    },
    '#FED83D': {
        item: "yellow",
        name: "Yellow Dye"
    },
    '#80C71F': {
        item: "lime",
        name: "Lime Dye"
    },
    '#F38BAA': {
        item: "pink",
        name: "Pink Dye"
    },
    '#474F52': {
        item: "gray",
        name: "Gray Dye"
    },
    '#9D9D97': {
        item: "light_gray",
        name: "Light Gray Dye"
    },
    '#169C9C': {
        item: "cyan",
        name: "Cyan Dye"
    },
    '#8932B8': {
        item: "purple",
        name: "Purple Dye"
    },
    '#3C44AA': {
        item: "blue",
        name: "Blue Dye"
    },
    '#835432': {
        item: "brown",
        name: "Brown Dye"
    },
    '#5E7C16': {
        item: "green",
        name: "Green Dye"
    },
    '#B02E26': {
        item: "red",
        name: "Red Dye"
    },
    '#1D1D21': {
        item: "black",
        name: "Black Dye"
    }
} as const;

// these may be out of order on linux vs windows
// since the backend reads the files from rusts "read_dir"
// and if so these will be mismatched, if so make sure its ordered on the rust side
export const RESOLVE_PATTERN: { [key: string]: { name: string, pattern_item?: string } } = {
    'border': {
        name: "Bordure",
    },
    'bricks': {
        name: "Bricks",
        pattern_item: "field_masoned"
    },
    'circle': {
        name: "Roundel"
    },
    'creeper': {
        name: "Creeper Charge",
        pattern_item: "creeper"
    },
    'cross': {
        name: "Saltire"
    },
    'curly_border': {
        name: "Bordure Indented",
        pattern_item: "bordure_indented"
    },
    'diagonal_left': {
        name: "Per Bend Sinister"
    },
    'diagonal_right': {
        name: "Per Bend"
    },
    'diagonal_up_left': {
        name: "Per Bend Inverted"
    },
    'diagonal_up_right': {
        name: "Per Bend Sinister Inverted"
    },
    'flow': {
        name: "Flow",
        pattern_item: "flow"
    },
    'flower': {
        name: "Flower Charge",
        pattern_item: "flower"
    },
    'globe': {
        name: "Globe",
        pattern_item: "globe"
    },
    'gradient': {
        name: "Gradient"
    },
    'gradient_up': {
        name: "Base Gradient"
    },
    'guster': {
        name: "Guster",
        pattern_item: "guster"
    },
    'half_horizontal': {
        name: "Per Fess"
    },
    'half_horizontal_bottom': {
        name: "Per Fess Inverted",
    },
    'half_vertical': {
        name: "Per Pale"
    },
    'half_vertical_right': {
        name: "Per Pale Inverted"
    },
    'mojang': {
        name: "Thing",
        pattern_item: "mojang"
    },
    'piglin': {
        name: "Snout",
        pattern_item: "piglin"
    },
    'rhombus': {
        name: "Lozenge"
    },
    'skull': {
        name: "Skull Charge",
        pattern_item: "skull"
    },
    'small_stripes': {
        name: "Paly"
    },
    'square_bottom_left': {
        name: "Base Dexter Canton"
    },
    'square_bottom_right': {
        name: "Base Sinister Canton"
    },
    'square_top_left': {
        name: "Chief Dexter Canton"
    },
    'square_top_right': {
        name: "Chief Sinister Canton"
    },
    'straight_cross': {
        name: "Cross"
    },
    'stripe_bottom': {
        name: "Base"
    },
    'stripe_center': {
        name: "Pale"
    },
    'stripe_downleft': {
        name: "Bend Sinister"
    },
    'stripe_downright': {
        name: "Bend"
    },
    'stripe_left': {
        name: "Pale Dexter"
    },
    'stripe_middle': {
        name: "Fess"
    },
    'stripe_right': {
        name: "Pale Sinister"
    },
    'stripe_top': {
        name: "Chief"
    },
    'triangle_bottom': {
        name: "Chevron"
    },
    'triangle_top': {
        name: "Inverted Chevron"
    },
    'triangles_bottom': {
        name: "Base Indented"
    },
    'triangles_top': {
        name: "Chief Indented"
    },
} as const;