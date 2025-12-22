<%namespace name="helpers" file="/helpers.mako.rs" />

// MSO (Microsoft Office) related CSS properties
// These properties are used for compatibility with Microsoft Office documents

// MSO ASCII Font Family - font family for ASCII characters
${helpers.single_keyword(
    "mso-ascii-font-family",
    "auto cursive fantasy monospace sans-serif serif",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Border Alt - alternative border styles with extensive decorative options
${helpers.single_keyword(
    "mso-border-alt",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Bottom Alt - alternative bottom border styles
${helpers.single_keyword(
    "mso-border-bottom-alt",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium MM moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Color Alt - alternative border color
${helpers.single_keyword(
    "mso-border-color-alt",
    "auto windowtext",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Inside Horizontal - placeholder property
${helpers.single_keyword(
    "mso-border-insideh",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Inside Vertical - placeholder property
${helpers.single_keyword(
    "mso-border-insidev",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Left Alt - alternative left border styles
${helpers.single_keyword(
    "mso-border-left-alt",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium MM moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Right Alt - alternative right border styles
${helpers.single_keyword(
    "mso-border-right-alt",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium MM moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Border Top Alt - alternative top border styles
${helpers.single_keyword(
    "mso-border-top-alt",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Character Indent Count - number of character indentations
${helpers.single_keyword(
    "mso-char-indent-count",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Character Type - type of characters for formatting
${helpers.single_keyword(
    "mso-char-type",
    "hiragana katakana narrow-katakana none symbol",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Diagonal Down - diagonal border going down
${helpers.single_keyword(
    "mso-diagonal-down",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Diagonal Up - diagonal border going up
${helpers.single_keyword(
    "mso-diagonal-up",
    "apples arched-scallops auto baby-pacifier baby-rattle balloons-3-colors balloons-hot-air basic-black-dashes basic-black-dots basic-black-squares basic-thin-lines basic-white-dashes basic-white-dots basic-white-squares basic-wide-inline basic-wide-midline basic-wide-outline bats birds birds-flight cabins cake-slice candy-corn celtic-knotwork certificate-banner chain-link champagne-bottle checked-bar-black checked-bar-color checkered christmas-tree circles-lines circles-rectangles classical-wave clocks compass confetti confetti-grays confetti-outline confetti-streamers confetti-white corner-triangles coupon-cutout-dashes coupon-cutout-dots crazy-maze creatures-butterfly creatures-fish creatures-insects creatures-lady-bug cross-stitch cup dash-dot-stroked dashed dash-large-gap dash-small-gap deco-arch deco-arch-color deco-blocks diamonds-gray dot-dash dot-dash-slanted dot-dot-dash dotted double double-d double-diamonds double-wave earth-1 earth-2 eclipsing-squares-1 eclipsing-squares-2 eggs-black emboss-3d engrave-3d fans film firecrackers flowers-block-print flowers-daisies flowers-modern-1 flowers-modern-2 flowers-pansy flowers-red-rose flowers-roses flowers-teacup flowers-tiny gems gingerbread-man gradient groove hairline handmade-1 handmade-2 heart-balloon heart-gray hearts heebie-jeebies holly house-funky hypnotic ice-cream-cones inset light-bulb lightning-1 lightning-2 maple-leaf maple-muffins map-pins marquee marquee-toothed medium moons mosaic music-notes none northwest outset ovals packages palms-black palms-color paper-clips papyrus party-favor party-glass pencils people people-hats people-waving poinsettias postage-stamp pumpkin-1 push-pin-note-1 push-pin-note-2 pyramids pyramids-above quadrants ridge rings safari sawtooth sawtooth-gray scared-cat seattle shadowed-squares sharks-teeth shorebird-tracks short-dash skyrocket snowflake-fancy snowflakes solid solid-thick sombrero southwest stars stars-3d stars-black stars-shadowed stars-top sun swirligig thick thick-thin-large-gap thick-thin-medium-gap thick-thin-small-gap thick-thin-thick-small-gap thin thin-thick-large-gap thin-thick-medium-gap thin-thick-small-gap thin-thick-thin-large-gap thin-thick-thin-medium-gap thin-thick-thin-small-gap three-d-emboss three-d-engrave torn-paper torn-paper-black trees triangle-party triangles tribal-1 tribal-2 tribal-3 tribal-4 tribal-5 tribal-6 triple twisted-lines-1 twisted-lines-2 vine wave waveline weaving-angles weaving-braid weaving-ribbon weaving-strips white-flowers windowtext woodwork x-illusions zany-triangles zig-zag zig-zag-stitch",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Far East Font Family - font family for Far East characters
${helpers.single_keyword(
    "mso-fareast-font-family",
    "auto cursive fantasy monospace sans-serif serif",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Font Width - width adjustment for fonts
${helpers.predefined_type(
    "mso-font-width",
    "LengthPercentage",
    "computed::LengthPercentage::zero_percent()",
    engines="servo",
    animation_type="normal",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Highlight - highlighting color
${helpers.single_keyword(
    "mso-highlight",
    "auto windowtext",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Ignore - elements to ignore during processing
${helpers.single_keyword(
    "mso-ignore",
    "align bgcolor color colspan colspan-obo colspan-obt colspan-rowspan padding style vglayout vglayout2 visibility",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Level Text - text content for list levels
${helpers.single_keyword(
    "mso-level-text",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO List - list level or type specification
${helpers.single_keyword(
    "mso-list",
    "level1 level2 level3 level4 level5 level6 level7 level8 level9 none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Level Number Format - formatting for list numbers
${helpers.single_keyword(
    "mso-level-number-format",
    "aiueo aiueo-full-width alpha-lower alpha-upper arabic arabic-abjad arabic-alpha arabic-leading-zero bullet cardinal-text chicago chinese-counting chinese-counting-thousand chinese-legal-simplified chosung decimal decimal-enclosed-circle decimal-enclosed-circle-chinese decimal-enclosed-fullstop decimal-enclosed-paren decimal-full-width decimal-half-width decimal-zero ganada hangul-digital hebrew-1 hebrew-2 hexadecimal ideograph-digital ideograph-enclosed-circle ideograph-legal-traditional ideograph-traditional ideograph-zodiak ideograph-zodiak-traditional iroha iroha-full-width japanese-counting japanese-digital-ten-thousand japanese-legal korean-counting korean-digital korean-legal lower-alpha lower-roman none ordinal ordinal-text roman-lower roman-upper taiwanese-counting taiwanese-counting-thousand taiwanese-digital upper-alpha upper-roman",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Level Suffix - suffix for list level items
${helpers.single_keyword(
    "mso-level-suffix",
    "none space tab",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO List Type - type of list structure
${helpers.single_keyword(
    "mso-list-type",
    "hybrid multilevel simple",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Line Height Alt - alternative line height specification
${helpers.predefined_type(
    "mso-line-height-alt",
    "LineHeight",
    initial_value="computed::LineHeight::Normal",
    engines="servo",
    animation_type="normal",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Line Height Rule - rule for applying line height
${helpers.single_keyword(
    "mso-line-height-rule",
    "at-least exactly",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Pattern - background pattern specification
${helpers.single_keyword(
    "mso-pattern",
    "auto diag-cross diag-stripe gray-025 gray-0625 gray-075 gray-10 gray-125 gray-15 gray-175 gray-20 gray-225 gray-25 gray-275 gray-30 gray-325 gray-35 gray-375 gray-40 gray-425 gray-45 gray-475 gray-5 gray-50 gray-525 gray-55 gray-575 gray-60 gray-625 gray-65 gray-675 gray-70 gray-725 gray-75 gray-775 gray-80 gray-825 gray-85 gray-875 gray-90 gray-925 gray-95 gray-975 horz-cross horz-stripe none reverse-diag-stripe solid thick-diag-cross thin-diag-cross thin-diag-stripe thin-horz-cross thin-horz-stripe thin-reverse-diag-stripe thin-vert-stripe vert-stripe windowtext",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Position Horizontal - horizontal positioning (placeholder)
${helpers.single_keyword(
    "mso-position-horizontal",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Position Horizontal Relative - relative horizontal positioning (placeholder)
${helpers.single_keyword(
    "mso-position-horizontal-relative",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Position Vertical - vertical positioning (placeholder)
${helpers.single_keyword(
    "mso-position-vertical",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Position Vertical Relative - relative vertical positioning (placeholder)
${helpers.single_keyword(
    "mso-position-vertical-relative",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Rotate - rotation angle for elements
${helpers.single_keyword(
    "mso-rotate",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Symbol Font Family - font family for symbol characters
${helpers.single_keyword(
    "mso-symbol-font-family",
    "auto cursive fantasy monospace sans-serif serif",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Tab Count - tab stops configuration
${helpers.predefined_type(
    "mso-tab-count",
    "Integer",
    "0",
    engines="servo",
    parse_method="parse_non_negative",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Table Anchor Horizontal - horizontal table anchoring
${helpers.single_keyword(
    "mso-table-anchor-horizontal",
    "column margin page",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Table Anchor Vertical - vertical table anchoring
${helpers.single_keyword(
    "mso-table-anchor-vertical",
    "margin page paragraph",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Table Left - left positioning for tables
${helpers.single_keyword(
    "mso-table-left",
    "center inside left outside right",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Table Top - top positioning for tables
${helpers.single_keyword(
    "mso-table-top",
    "bottom inside middle outside top",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Text Indent Alt - alternative text indentation
${helpers.predefined_type(
    "mso-text-indent-alt",
    "LengthPercentage",
    "computed::LengthPercentage::zero()",
    engines="servo",
    animation_type="normal",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

// MSO Shading - background shading color
${helpers.single_keyword(
    "mso-shading",
    "auto transparent windowtext",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="paint"
)}

// MSO Spacerun - space handling behavior
${helpers.single_keyword(
    "mso-spacerun",
    "no yes",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects=""
)}

// MSO Wrap Style - text wrapping style (placeholder)
${helpers.single_keyword(
    "mso-wrap-style",
    "none",
    engines="servo",
    animation_type="discrete",
    spec="https://docs.microsoft.com/en-us/office/",
    affects="layout"
)}

